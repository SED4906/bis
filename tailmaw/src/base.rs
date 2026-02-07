use std::collections::HashMap;

use sdl3::{
    Sdl,
    event::Event,
    gpu::{
        BufferBinding, BufferRegion, BufferUsageFlags, ColorTargetDescription, ColorTargetInfo, ComputePipeline, Device, GraphicsPipeline, GraphicsPipelineTargetInfo, IndexElementSize, LoadOp, Sampler, SamplerCreateInfo, Shader, ShaderFormat, ShaderStage, StoreOp, Texture, TextureCreateInfo, TextureFormat, TextureRegion, TextureSamplerBinding, TextureTransferInfo, TextureUsage, TransferBufferLocation, TransferBufferUsage, VertexAttribute, VertexBufferDescription, VertexInputState
    },
    pixels::Color,
    video::Window,
};

use crate::{
    math::{Vector2, Vector3},
    model::{LoaderModel, Mesh, Model, Vertex},
};

pub struct Engine {
    device: Device,
    window: Window,
    sdl: Sdl,
    textures: HashMap<String, Texture<'static>>,
    models: HashMap<String, Model>,
    shaders: HashMap<String, Shader>,
    samplers: HashMap<String, Sampler>,
    graphics_pipelines: HashMap<String, GraphicsPipeline>,
    compute_pipelines: HashMap<String, ComputePipeline>,
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
            textures: HashMap::new(),
            models: HashMap::new(),
            shaders: HashMap::new(),
            samplers: HashMap::new(),
            graphics_pipelines: HashMap::new(),
            compute_pipelines: HashMap::new(),
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
                Event::Quit { .. } => {
                    return false;
                }
                _ => {}
            }
        }
        self.render();
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
        let color_target_info = ColorTargetInfo::default()
            .with_texture(&swapchain_texture)
            .with_load_op(LoadOp::CLEAR)
            .with_store_op(StoreOp::STORE)
            .with_clear_color(Color::RGB(0xbb, 0x11, 0x55));

        let render_pass = self
            .device
            .begin_render_pass(&command_buffer, &[color_target_info], None)
            .expect("begin render pass failed");

        let default_texture = self.texture("").unwrap();
        let tiled_sampler = self.sampler("tiled").unwrap();
        let default_graphics_pipeline = self.graphics_pipeline("default").unwrap();
        let test_model = self.model("test_quad").unwrap();
        render_pass.bind_graphics_pipeline(&default_graphics_pipeline);
        render_pass.bind_fragment_samplers(
            0,
            &[TextureSamplerBinding::new()
                .with_texture(&default_texture)
                .with_sampler(&tiled_sampler)],
        );
        render_pass.bind_vertex_buffers(0, &[BufferBinding::new().with_buffer(&test_model.vertex_buffer)]);
        render_pass.bind_index_buffer(&BufferBinding::new().with_buffer(&test_model.index_buffer),IndexElementSize::_32BIT);
        render_pass.draw_indexed_primitives(6, 1, 0, 0, 0);
        self.device.end_render_pass(render_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(&self.device)
            .expect("submit and acquire fence failed");
        self.device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
    }

    pub fn load_default_assets(&mut self) {
        self.texture("").expect("failed to load the missing texture");
        self.shader("default.vert.spv").expect("failed to load the default vertex shader");
        self.shader("default.frag.spv").expect("failed to load the default fragment shader");
        self.sampler("tiled").expect("failed to load tiled sampler");
        self.sampler("pixel").expect("failed to load pixel sampler");
        self.graphics_pipeline("default").expect("failed to load the default graphics pipeline");
        self.model("test_quad").expect("failed to load test quad model");
    }

    pub fn texture(&mut self, name: &str) -> Option<Texture<'static>> {
        if self.textures.contains_key(name) {
            return self.textures.get(name).cloned();
        }
        let Some((data, width, height)) = self.load_texture_data(name) else {
            return self.textures.get("").cloned();
        };
        let texture = self
            .device
            .create_texture(
                TextureCreateInfo::new()
                    .with_width(width)
                    .with_height(height)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1)
                    .with_type(sdl3::gpu::TextureType::_2D)
                    .with_usage(TextureUsage::SAMPLER)
                    .with_format(TextureFormat::R8g8b8a8Unorm),
            )
            .expect("failed to create texture");
        let command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");

        let copy_pass = self
            .device
            .begin_copy_pass(&command_buffer)
            .expect("failed to begin copy pass");
        let transfer_buffer = self
            .device
            .create_transfer_buffer()
            .with_size(data.len() as u32)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("failed to create transfer buffer");
        transfer_buffer
            .map(&self.device, true)
            .mem_mut()
            .clone_from_slice(data.as_slice());
        copy_pass.upload_to_gpu_texture(
            TextureTransferInfo::new()
                .with_offset(0)
                .with_pixels_per_row(width)
                .with_rows_per_layer(height)
                .with_transfer_buffer(&transfer_buffer),
            TextureRegion::new()
                .with_width(width)
                .with_height(height)
                .with_texture(&texture),
            true,
        );
        self.device.end_copy_pass(copy_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(&self.device)
            .expect("submit and acquire fence failed");
        self.device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
        let _ = self.textures.insert(name.into(), texture.clone());
        Some(texture)
    }

    pub fn model(&mut self, name: &str) -> Option<Model> {
        if self.models.contains_key(name) {
            return self.models.get(name).cloned();
        }
        let loader_model = match name {
            "test_quad" => LoaderModel {
                mesh: Mesh {
                    vertices: vec![
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, 0.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, 0.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, 0.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, -1.0, 0.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                    ],
                    indices: vec![0, 1, 2, 0, 2, 3],
                }
                .into(),
                texture: "".to_string().into(),
            },
            _ => return None,
        };
        let vertex_buffer_size = loader_model.mesh.vertices.len() as u32 * 20;
        let index_buffer_size = loader_model.mesh.indices.len() as u32 * 4;
        let vertex_buffer = self.device.create_buffer().with_size(vertex_buffer_size).with_usage(BufferUsageFlags::VERTEX).build().ok()?;
        let index_buffer = self.device.create_buffer().with_size(index_buffer_size).with_usage(BufferUsageFlags::INDEX).build().ok()?;
        let command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");

        let copy_pass = self
            .device
            .begin_copy_pass(&command_buffer)
            .expect("failed to begin copy pass");
        let vertex_transfer_buffer = self
            .device
            .create_transfer_buffer()
            .with_size(vertex_buffer_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("failed to create transfer buffer");
        let index_transfer_buffer = self
            .device
            .create_transfer_buffer()
            .with_size(index_buffer_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("failed to create transfer buffer");
        vertex_transfer_buffer
            .map(&self.device, true)
            .mem_mut()
            .clone_from_slice(loader_model.mesh.vertices.as_slice());
        index_transfer_buffer
            .map(&self.device, true)
            .mem_mut()
            .clone_from_slice(loader_model.mesh.indices.as_slice());
        copy_pass.upload_to_gpu_buffer(TransferBufferLocation::new().with_offset(0).with_transfer_buffer(&vertex_transfer_buffer),BufferRegion::new().with_buffer(&vertex_buffer).with_offset(0).with_size(vertex_buffer_size),true);
        copy_pass.upload_to_gpu_buffer(TransferBufferLocation::new().with_offset(0).with_transfer_buffer(&index_transfer_buffer),BufferRegion::new().with_buffer(&index_buffer).with_offset(0).with_size(index_buffer_size),true);
        self.device.end_copy_pass(copy_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(&self.device)
            .expect("submit and acquire fence failed");
        self.device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
        let texture = self.texture(&loader_model.texture)?;
        let model = Model { vertex_buffer, index_buffer, texture };
        let _ = self.models.insert(name.into(), model.clone());
        Some(model)
    }

    pub fn shader(&mut self, name: &str) -> Option<Shader> {
        if self.shaders.contains_key(name) {
            return self.shaders.get(name).cloned();
        }
        let Some((data, stage)) = self.load_shader_data(name) else {
            return None;
        };
        let shader = self
            .device
            .create_shader()
            .with_code(ShaderFormat::SPIRV, &data, stage)
            .with_entrypoint(c"main")
            .with_samplers(if stage == ShaderStage::Vertex { 0 } else { 1 })
            .build()
            .ok()?;
        let _ = self.shaders.insert(name.into(), shader.clone());
        Some(shader)
    }

    pub fn sampler(&mut self, name: &str) -> Option<Sampler> {
        if self.samplers.contains_key(name) {
            return self.samplers.get(name).cloned();
        }
        let sampler_create_info = match name {
            "tiled" => SamplerCreateInfo::new()
                .with_address_mode_u(sdl3::gpu::SamplerAddressMode::Repeat)
                .with_address_mode_v(sdl3::gpu::SamplerAddressMode::Repeat)
                .with_address_mode_w(sdl3::gpu::SamplerAddressMode::Repeat)
                .with_mag_filter(sdl3::gpu::Filter::Linear)
                .with_mipmap_mode(sdl3::gpu::SamplerMipmapMode::Linear),
            "pixel" => SamplerCreateInfo::new()
                .with_address_mode_u(sdl3::gpu::SamplerAddressMode::Repeat)
                .with_address_mode_v(sdl3::gpu::SamplerAddressMode::Repeat)
                .with_address_mode_w(sdl3::gpu::SamplerAddressMode::Repeat)
                .with_mag_filter(sdl3::gpu::Filter::Nearest)
                .with_mipmap_mode(sdl3::gpu::SamplerMipmapMode::Nearest),
            _ => return None,
        };
        let sampler = self.device.create_sampler(sampler_create_info).ok()?;
        let _ = self.samplers.insert(name.into(), sampler.clone());
        Some(sampler)
    }

    pub fn graphics_pipeline(&mut self, name: &str) -> Option<GraphicsPipeline> {
        if self.graphics_pipelines.contains_key(name) {
            return self.graphics_pipelines.get(name).cloned();
        }
        let graphics_pipeline = match name {
            "default" => self
                .device
                .create_graphics_pipeline()
                .with_vertex_shader(self.shaders.get("default.vert.spv")?)
                .with_fragment_shader(self.shaders.get("default.frag.spv")?)
                .with_primitive_type(sdl3::gpu::PrimitiveType::TriangleList)
                .with_target_info(
                    GraphicsPipelineTargetInfo::new()
                        .with_color_target_descriptions(&[ColorTargetDescription::new()
                            .with_format(self.device.get_swapchain_texture_format(&self.window))]),
                )
                .with_vertex_input_state(
                    VertexInputState::new()
                        .with_vertex_buffer_descriptions(&[VertexBufferDescription::new()
                            .with_input_rate(sdl3::gpu::VertexInputRate::Vertex)
                            .with_pitch(20)])
                        .with_vertex_attributes(&[
                            VertexAttribute::new()
                                .with_format(sdl3::gpu::VertexElementFormat::Float2)
                                .with_location(1)
                                .with_offset(0),
                            VertexAttribute::new()
                                .with_format(sdl3::gpu::VertexElementFormat::Float3)
                                .with_location(0)
                                .with_offset(8),
                        ]),
                )
                .build()
                .ok()?,
            _ => return None,
        };
        let _ = self
            .graphics_pipelines
            .insert(name.into(), graphics_pipeline.clone());
        Some(graphics_pipeline)
    }

    pub fn compute_pipeline(&mut self, name: &str) -> Option<ComputePipeline> {
        if self.compute_pipelines.contains_key(name) {
            return self.compute_pipelines.get(name).cloned();
        }
        None
    }
}
