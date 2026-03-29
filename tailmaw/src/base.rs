use std::{collections::HashMap, time::Instant};

use sdl3::{
    Sdl,
    event::Event,
    gpu::{
        BufferBinding, ColorTargetInfo, DepthStencilTargetInfo, Device, IndexElementSize, LoadOp,
        ShaderFormat, StoreOp, TextureCreateInfo, TextureFormat, TextureSamplerBinding,
        TextureUsage,
    },
    pixels::Color,
    video::Window,
};

use crate::{
    geometry::{
        space::{Matrix4, Quaternion, Vector2, Vector3},
        transform::{look_at, perspective, transformation},
    },
    hammerspace::Hammerspace,
    model::Model,
};

pub struct Engine {
    // Graphics
    device: Device,
    window: Window,
    sdl: Sdl,
    width: u32,
    height: u32,
    // Assets
    hammerspace: Hammerspace,
    // Logic
    next_entity: u128,
    next_system: u128,
    entities: HashMap<u128, HashMap<String, Component>>,
    systems: HashMap<u128, fn(&mut Engine, u128)>,
    // Timing
    timer: f32,
    delta_ns: u128,
    instant: Instant,
    // Input
    last_mouse_screen_position: Vector2,
    mouse_screen_position: Vector2,
    mouse_clicked_buttons: u8,
}

#[derive(Clone)]
pub enum Component {
    Model {
        model: String,
        visible: bool,
        texture_override: Option<String>,
    },
    Transformation {
        position: Vector3,
        rotation: Quaternion,
        scale: Vector3,
    },
    Camera {
        cam: Vector3,
        target: Vector3,
        up: Vector3,
        fov: f32,
    },
    Reference {
        entity: u128,
        camera: bool,
        transformation: bool,
        model: bool,
    },
}

struct RenderStep {
    pub model: Model,
    pub transformation: Matrix4,
    pub camera: Matrix4,
}

const DEBUG_MODE: bool = true;

impl Engine {
    pub fn new() -> Self {
        let sdl = sdl3::init().expect("sdl init failed");
        let video = sdl.video().expect("sdl video failed");
        //let audio = sdl.audio().expect("sdl audio failed");
        let window = video
            .window("bis game engine", 640, 360)
            .position_centered()
            .resizable()
            .build()
            .expect("sdl window failed");
        let device = Device::new(ShaderFormat::SPIRV, DEBUG_MODE)
            .expect("gpu init failed")
            .with_window(&window)
            .expect("gpu window failed");
        Self {
            device,
            window,
            sdl,
            width: 640,
            height: 360,
            hammerspace: Hammerspace::new(),
            next_entity: 0,
            next_system: 0,
            entities: HashMap::new(),
            systems: HashMap::new(),
            timer: 0.0,
            delta_ns: 0,
            instant: Instant::now(),
            last_mouse_screen_position: Vector2::fill(0.5),
            mouse_screen_position: Vector2::fill(0.5),
            mouse_clicked_buttons: 0,
        }
    }

    pub fn retitle_window(&mut self, title: &str) {
        self.window
            .set_title(title)
            .expect("failed to set window title");
    }

    pub fn update(&mut self) -> bool {
        let mut event_pump = self.sdl.event_pump().expect("sdl event pump failed");
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    sdl3::mouse::MouseButton::Left => self.mouse_clicked_buttons |= 1 << 1,
                    sdl3::mouse::MouseButton::Middle => self.mouse_clicked_buttons |= 1 << 2,
                    sdl3::mouse::MouseButton::Right => self.mouse_clicked_buttons |= 1 << 3,
                    _ => {}
                },
                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    sdl3::mouse::MouseButton::Left => self.mouse_clicked_buttons &= !(1 << 1),
                    sdl3::mouse::MouseButton::Middle => self.mouse_clicked_buttons &= !(1 << 2),
                    sdl3::mouse::MouseButton::Right => self.mouse_clicked_buttons &= !(1 << 3),
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    self.last_mouse_screen_position = self.mouse_screen_position;
                    self.mouse_screen_position.values[0] = x / self.window.size().0 as f32;
                    self.mouse_screen_position.values[1] = y / self.window.size().1 as f32;
                }
                Event::Quit { .. } => {
                    return false;
                }
                _ => {}
            }
        }
        for system in self.systems.clone().values() {
            for entity in self.entities.clone().keys() {
                system(self, *entity);
            }
        }
        self.render();
        let instant = Instant::now();
        self.delta_ns = (instant - self.instant).as_nanos();
        self.timer += self.delta_ns as f32 / 1e9;
        self.instant = instant;
        true
    }

    fn render(&mut self) {
        let mut command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");
        let swapchain_texture = command_buffer
            .wait_and_acquire_swapchain_texture(&self.window)
            .expect("swapchain texture failed");
        self.width = swapchain_texture.width();
        self.height = swapchain_texture.height();
        let color_target_info = ColorTargetInfo::default()
            .with_texture(&swapchain_texture.clone())
            .with_load_op(LoadOp::CLEAR)
            .with_store_op(StoreOp::STORE)
            .with_clear_color(Color::RGB(0xbb, 0x11, 0x55));
        let mut depth_target = self
            .device
            .create_texture(
                TextureCreateInfo::new()
                    .with_usage(TextureUsage::DEPTH_STENCIL_TARGET)
                    .with_format(TextureFormat::D16Unorm)
                    .with_width(self.width)
                    .with_height(self.height)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1),
            )
            .unwrap();

        let tiled_sampler = self.hammerspace.sampler(&self.device, "tiled").unwrap();
        let default_graphics_pipeline = self
            .hammerspace
            .graphics_pipeline(&self.window, &self.device, "default")
            .unwrap();
        let render_pass = self
            .device
            .begin_render_pass(
                &command_buffer,
                &[color_target_info],
                Some(
                    &DepthStencilTargetInfo::new()
                        .with_load_op(LoadOp::CLEAR)
                        .with_store_op(StoreOp::DONT_CARE)
                        .with_clear_depth(10000.0)
                        .with_texture(&mut depth_target),
                ),
            )
            .expect("begin render pass failed");
        render_pass.bind_graphics_pipeline(&default_graphics_pipeline);
        let mut render_steps = vec![];
        for entity in self.entities.clone().keys() {
            let mut render_model = None;
            let mut render_camera = None;
            let mut render_transformation = None;
            let mut referenced = vec![*entity];
            self.render_step(
                *entity,
                &mut render_model,
                &mut render_transformation,
                &mut render_camera,
                &mut referenced,
            );
            if let Some(model) = render_model
                && let Some(camera) = render_camera
                && let Some(transformation) = render_transformation
            {
                render_steps.push(RenderStep {
                    model,
                    transformation,
                    camera,
                });
            }
        }
        for render_step in render_steps {
            render_pass.bind_fragment_samplers(
                0,
                &[TextureSamplerBinding::new()
                    .with_texture(&render_step.model.texture)
                    .with_sampler(&tiled_sampler)],
            );
            render_pass.bind_vertex_buffers(
                0,
                &[BufferBinding::new().with_buffer(&render_step.model.vertex_buffer)],
            );
            render_pass.bind_index_buffer(
                &BufferBinding::new().with_buffer(&render_step.model.index_buffer),
                IndexElementSize::_32BIT,
            );
            let matrix = render_step.transformation * render_step.camera;
            command_buffer.push_vertex_uniform_data(0, &matrix);
            render_pass.draw_indexed_primitives(
                render_step.model.index_buffer.len() / 4,
                1,
                0,
                0,
                0,
            );
        }
        self.device.end_render_pass(render_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(&self.device)
            .expect("submit and acquire fence failed");
        self.device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
    }

    fn render_step(
        &mut self,
        entity: u128,
        render_model: &mut Option<Model>,
        render_transformation: &mut Option<Matrix4>,
        render_camera: &mut Option<Matrix4>,
        referenced: &mut Vec<u128>,
    ) {
        let Some(components) = self.entities.get(&entity).cloned() else {
            return;
        };
        for (_,component) in components {
            match component {
                Component::Model {
                    model,
                    visible,
                    texture_override,
                } if visible => {
                    let mut model = self.hammerspace.model(&self.device, &model).unwrap();
                    if let Some(texture_override) = texture_override {
                        model.texture = self
                            .hammerspace
                            .texture(&self.device, &texture_override)
                            .unwrap();
                    }
                    *render_model = Some(model);
                }
                Component::Camera {
                    cam,
                    target,
                    up,
                    fov,
                } => {
                    *render_camera = Some(
                        look_at(cam, target, up)
                            * perspective(
                                fov,
                                self.width as f32 / self.height as f32,
                                0.01,
                                10000.0,
                            ),
                    )
                }
                Component::Transformation {
                    position,
                    rotation,
                    scale,
                } => *render_transformation = Some(transformation(position, rotation, scale)),
                Component::Reference {
                    entity,
                    camera,
                    transformation,
                    model,
                } => {
                    if !referenced.contains(&entity) {
                        referenced.push(entity);
                        let (mut dummy_model, mut dummy_transformation, mut dummy_camera) =
                            (None, None, None);
                        self.render_step(
                            entity,
                            if model {
                                render_model
                            } else {
                                &mut dummy_model
                            },
                            if transformation {
                                render_transformation
                            } else {
                                &mut dummy_transformation
                            },
                            if camera {
                                render_camera
                            } else {
                                &mut dummy_camera
                            },
                            referenced,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    pub fn load_default_assets(&mut self) {
        self.hammerspace
            .texture(&self.device, "")
            .expect("failed to load the missing texture");
        self.hammerspace
            .shader(&self.device, "default.vert.spv")
            .expect("failed to load the default vertex shader");
        self.hammerspace
            .shader(&self.device, "default.frag.spv")
            .expect("failed to load the default fragment shader");
        self.hammerspace
            .sampler(&self.device, "tiled")
            .expect("failed to load tiled sampler");
        self.hammerspace
            .sampler(&self.device, "pixel")
            .expect("failed to load pixel sampler");
        self.hammerspace
            .graphics_pipeline(&self.window, &self.device, "default")
            .expect("failed to load the default graphics pipeline");
        self.hammerspace
            .model(&self.device, "test_quad")
            .expect("failed to load test quad model");
        self.hammerspace
            .model(&self.device, "test_cube")
            .expect("failed to load test cube model");
    }

    pub fn create_entity(&mut self) -> u128 {
        self.next_entity += 1;
        let _ = self.entities.insert(self.next_entity, HashMap::new());
        self.next_entity
    }

    pub fn destroy_entity(&mut self, which: u128) {
        let _ = self.entities.remove(&which);
    }

    pub fn insert_component(&mut self, entity: u128, name: &str, data: Component) {
        if let Some(components) = self.entities.get_mut(&entity) {
            let _ = components.insert(name.to_string(), data);
        }
    }

    pub fn remove_component(&mut self, entity: u128, name: &str) {
        if let Some(components) = self.entities.get_mut(&entity) {
            let _ = components.remove(name);
        }
    }

    pub fn get_component(&self, entity: u128, name: &str) -> Option<Component> {
        self.entities.get(&entity)?.get(name).cloned()
    }

    pub fn get_component_mut(&mut self, entity: u128, name: &str) -> Option<&mut Component> {
        self.entities.get_mut(&entity)?.get_mut(name)
    }

    pub fn create_system(&mut self, system: fn(&mut Self, u128)) -> u128 {
        self.next_system += 1;
        let _ = self.systems.insert(self.next_system, system);
        self.next_system
    }

    pub fn destroy_system(&mut self, which: u128) {
        let _ = self.systems.remove(&which);
    }

    pub fn left_mouse_button_pressed(&self) -> bool {
        self.mouse_clicked_buttons & (1 << 1) != 0
    }

    pub fn middle_mouse_button_pressed(&self) -> bool {
        self.mouse_clicked_buttons & (1 << 2) != 0
    }

    pub fn right_mouse_button_pressed(&self) -> bool {
        self.mouse_clicked_buttons & (1 << 3) != 0
    }

    pub fn get_timer(&self) -> f32 {
        self.timer
    }
}
