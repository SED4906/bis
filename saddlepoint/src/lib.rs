use sdl3::{
    Sdl,
    gpu::{
        Buffer, BufferBinding, BufferRegion, BufferUsageFlags, ColorTargetDescription,
        ColorTargetInfo, CommandBuffer, DepthStencilState, DepthStencilTargetInfo, Device,
        GraphicsPipeline, GraphicsPipelineTargetInfo, IndexElementSize, LoadOp, RasterizerState,
        Sampler, SamplerCreateInfo, Shader, ShaderFormat, ShaderStage, StoreOp, Texture,
        TextureCreateInfo, TextureFormat, TextureRegion, TextureSamplerBinding,
        TextureTransferInfo, TextureUsage, TransferBufferLocation, TransferBufferUsage,
        VertexAttribute, VertexBufferDescription, VertexInputState,
    },
    pixels::Color,
    sys::gpu::{
        SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2, SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3,
        SDL_GPUVertexAttribute,
    },
    video::Window,
};
use tailmaw::geometry::{
    space::{Matrix4, Vector2, Vector3},
    transform::{look_at, perspective},
};

pub struct RenderingEngine {
    device: Device,
    window: Window,
    pub sdl: Sdl,
}

pub struct Geometry {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    material: Material,
    pub transform: Matrix4,
}

pub struct Material {
    pub graphics_pipeline: GraphicsPipeline,
    pub texture: Texture<'static>,
    pub sampler: Sampler,
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: YOrZ,
    pub fov: f32,
}

#[derive(Clone, Copy)]
pub enum YOrZ {
    Y,
    Z,
}

impl RenderingEngine {
    pub fn new(title: &'static str) -> Self {
        let sdl = sdl3::init().expect("sdl init failed");
        let video = sdl.video().expect("video init failed");
        let window = video
            .window(title, 640, 360)
            .resizable()
            .build()
            .expect("window init failed");
        let device = Device::new(ShaderFormat::SPIRV, cfg!(debug_assertions))
            .expect("device init failed")
            .with_window(&window)
            .expect("claim window failed");
        Self {
            device,
            window,
            sdl,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let _ = self.window.set_title(title);
    }

    pub fn create_geometry<V: VertexData>(
        &mut self,
        vertices: &[V],
        indices: &[u32],
        material: Material,
        transformation_matrix: Option<Matrix4>,
    ) -> Geometry {
        let command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");
        let vertex_size = (vertices.len() * size_of::<V>()) as u32;
        let index_size = (indices.len() * 4) as u32;
        let vertex_buffer = self
            .device
            .create_buffer()
            .with_size(vertex_size)
            .with_usage(BufferUsageFlags::VERTEX)
            .build()
            .expect("create vertex buffer failed");
        let index_buffer = self
            .device
            .create_buffer()
            .with_size(index_size)
            .with_usage(BufferUsageFlags::INDEX)
            .build()
            .expect("create index buffer failed");
        let copy_pass = self
            .device
            .begin_copy_pass(&command_buffer)
            .expect("begin copy pass failed");
        let transfer_buffer = self
            .device
            .create_transfer_buffer()
            .with_size(vertex_size + index_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("create transfer buffer failed");
        transfer_buffer.map(&self.device, true).mem_mut()[0..vertices.len()]
            .copy_from_slice(vertices);
        transfer_buffer.map(&self.device, true).mem_mut()[vertex_size as usize / 4..]
            .copy_from_slice(indices);
        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_transfer_buffer(&transfer_buffer)
                .with_offset(0),
            BufferRegion::new()
                .with_buffer(&vertex_buffer)
                .with_offset(0)
                .with_size(vertex_size),
            true,
        );
        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_transfer_buffer(&transfer_buffer)
                .with_offset(vertex_size),
            BufferRegion::new()
                .with_buffer(&index_buffer)
                .with_offset(0)
                .with_size(vertex_size),
            true,
        );
        self.device.end_copy_pass(copy_pass);
        self.complete_gpu_operation(command_buffer);
        Geometry {
            vertex_buffer,
            index_buffer,
            material,
            transform: transformation_matrix.unwrap_or(Matrix4::identity()),
        }
    }

    pub fn draw_geometry(
        &mut self,
        camera: Option<Camera>,
        geometry: &[&Geometry],
        target: Option<Texture>,
    ) {
        let (width, height) = self.window.size();
        let mut command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");
        let texture = target.unwrap_or_else(|| {
            command_buffer
                .wait_and_acquire_swapchain_texture(&self.window)
                .expect("acquire swapchain texture failed")
        });
        let aspect_ratio = texture.width() as f32 / texture.height() as f32;
        let color_target_info = ColorTargetInfo::default()
            .with_clear_color(Color::RGB(0xbb, 0x11, 0x55))
            .with_load_op(LoadOp::CLEAR)
            .with_store_op(StoreOp::STORE)
            .with_texture(&texture);
        let mut depth_texture = self
            .device
            .create_texture(
                TextureCreateInfo::new()
                    .with_usage(TextureUsage::DEPTH_STENCIL_TARGET)
                    .with_format(TextureFormat::D16Unorm)
                    .with_width(width)
                    .with_height(height)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1),
            )
            .unwrap();
        let render_pass = self
            .device
            .begin_render_pass(
                &command_buffer,
                &[color_target_info],
                Some(
                    &DepthStencilTargetInfo::new()
                        .with_clear_depth(10000.0)
                        .with_cycle(true)
                        .with_load_op(LoadOp::CLEAR)
                        .with_stencil_load_op(LoadOp::CLEAR)
                        .with_store_op(StoreOp::DONT_CARE)
                        .with_stencil_store_op(StoreOp::DONT_CARE)
                        .with_texture(&mut depth_texture),
                ),
            )
            .expect("begin render pass failed");
        let camera_matrix = if let Some(camera) = camera {
            look_at(
                camera.position,
                camera.target,
                match camera.up {
                    YOrZ::Y => Vector3::new([0.0, 1.0, 0.0]),
                    YOrZ::Z => Vector3::new([0.0, 0.0, 1.0]),
                },
            ) * perspective(camera.fov, aspect_ratio, 0.01, 10000.0)
        } else {
            Matrix4::identity()
        };
        for geometry in geometry {
            render_pass.bind_graphics_pipeline(&geometry.material.graphics_pipeline);
            render_pass.bind_vertex_buffers(
                0,
                &[BufferBinding::new()
                    .with_buffer(&geometry.vertex_buffer)
                    .with_offset(0)],
            );
            render_pass.bind_index_buffer(
                &BufferBinding::new()
                    .with_buffer(&geometry.index_buffer)
                    .with_offset(0),
                IndexElementSize::_32BIT,
            );
            render_pass.bind_fragment_samplers(
                0,
                &[TextureSamplerBinding::new()
                    .with_sampler(&geometry.material.sampler)
                    .with_texture(&geometry.material.texture)],
            );
            command_buffer
                .push_vertex_uniform_data(0, &(geometry.transform * camera_matrix));
            render_pass.draw_indexed_primitives(geometry.index_buffer.len() / 4, 1, 0, 0, 0);
        }
        self.device.end_render_pass(render_pass);
        self.complete_gpu_operation(command_buffer);
    }

    fn complete_gpu_operation(&self, command_buffer: CommandBuffer) {
        let fence = command_buffer
            .submit_and_acquire_fence(&self.device)
            .expect("submit and acquire fence failed");
        self.device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
    }

    pub fn create_graphics_pipeline<V: VertexData>(
        &self,
        vertex_shader: Shader,
        fragment_shader: Shader,
    ) -> GraphicsPipeline {
        self.device
            .create_graphics_pipeline()
            .with_vertex_input_state(
                VertexInputState::new()
                    .with_vertex_attributes(V::attributes())
                    .with_vertex_buffer_descriptions(&[V::buffer_description()]),
            )
            .with_vertex_shader(&vertex_shader)
            .with_fragment_shader(&fragment_shader)
            .with_primitive_type(sdl3::gpu::PrimitiveType::TriangleList)
            .with_rasterizer_state(
                RasterizerState::new()
                    .with_cull_mode(sdl3::gpu::CullMode::Back)
                    .with_front_face(sdl3::gpu::FrontFace::Clockwise)
                    .with_fill_mode(sdl3::gpu::FillMode::Fill),
            )
            .with_depth_stencil_state(
                DepthStencilState::new()
                    .with_enable_depth_test(true)
                    .with_enable_depth_write(true)
                    .with_compare_op(sdl3::gpu::CompareOp::Less),
            )
            .with_target_info(
                GraphicsPipelineTargetInfo::new()
                    .with_color_target_descriptions(&[ColorTargetDescription::new()
                        .with_format(self.device.get_swapchain_texture_format(&self.window))])
                    .with_has_depth_stencil_target(true)
                    .with_depth_stencil_format(TextureFormat::D16Unorm),
            )
            .build()
            .expect("create graphics pipeline failed")
    }

    pub fn create_sampler(&self) -> Sampler {
        self.device
            .create_sampler(
                SamplerCreateInfo::new()
                    .with_address_mode_u(sdl3::gpu::SamplerAddressMode::Repeat)
                    .with_address_mode_v(sdl3::gpu::SamplerAddressMode::Repeat)
                    .with_address_mode_w(sdl3::gpu::SamplerAddressMode::Repeat)
                    .with_mag_filter(sdl3::gpu::Filter::Linear)
                    .with_mipmap_mode(sdl3::gpu::SamplerMipmapMode::Nearest),
            )
            .expect("create sampler failed")
    }

    pub fn create_shader(&self, code: &[u8], stage: ShaderStage) -> Shader {
        self.device
            .create_shader()
            .with_code(ShaderFormat::SPIRV, code, stage)
            .with_entrypoint(c"main")
            .with_samplers(match stage {
                ShaderStage::Vertex => 0,
                ShaderStage::Fragment => 1,
            })
            .with_uniform_buffers(1)
            .build()
            .expect("create shader failed")
    }

    pub fn create_window_texture(&self) -> Texture<'static> {
        self.device
            .create_texture(
                TextureCreateInfo::new()
                    .with_format(self.device.get_swapchain_texture_format(&self.window))
                    .with_width(self.window.size().0)
                    .with_height(self.window.size().1)
                    .with_usage(TextureUsage::COLOR_TARGET | TextureUsage::SAMPLER)
                    .with_type(sdl3::gpu::TextureType::_2D)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1)
                    .with_sample_count(sdl3::gpu::SampleCount::NoMultiSampling),
            )
            .expect("create window texture failed")
    }

    pub fn create_sized_texture(&self, width: u32, height: u32) -> Texture<'static> {
        self.device
            .create_texture(
                TextureCreateInfo::new()
                    .with_format(TextureFormat::R8g8b8a8Unorm)
                    .with_width(width)
                    .with_height(height)
                    .with_usage(TextureUsage::COLOR_TARGET | TextureUsage::SAMPLER)
                    .with_type(sdl3::gpu::TextureType::_2D)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1)
                    .with_sample_count(sdl3::gpu::SampleCount::NoMultiSampling),
            )
            .expect("create window texture failed")
    }

    pub fn load_texture(&self, texture: &Texture, data: &[u8]) {
        let command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");
        let copy_pass = self
            .device
            .begin_copy_pass(&command_buffer)
            .expect("begin copy pass failed");
        let transfer_buffer = self
            .device
            .create_transfer_buffer()
            .with_size(data.len() as u32)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("create transfer buffer failed");
        transfer_buffer
            .map(&self.device, true)
            .mem_mut()
            .copy_from_slice(data);
        copy_pass.upload_to_gpu_texture(
            TextureTransferInfo::new()
                .with_offset(0)
                .with_pixels_per_row(texture.width())
                .with_rows_per_layer(texture.height())
                .with_transfer_buffer(&transfer_buffer),
            TextureRegion::new()
                .with_texture(&texture)
                .with_width(texture.width())
                .with_height(texture.height())
                .with_depth(1)
                .with_layer(0),
            true,
        );
        self.device.end_copy_pass(copy_pass);
        self.complete_gpu_operation(command_buffer);
    }
}

pub trait VertexData: Sized + Copy {
    fn attributes() -> &'static [VertexAttribute];
    fn buffer_description() -> VertexBufferDescription {
        VertexBufferDescription::new()
            .with_input_rate(sdl3::gpu::VertexInputRate::Vertex)
            .with_pitch(size_of::<Self>() as u32)
    }
}

#[derive(Clone, Copy)]
pub struct VertexPositionUV {
    pub position: Vector3,
    pub uv: Vector2,
}

impl VertexData for VertexPositionUV {
    fn attributes() -> &'static [VertexAttribute] {
        unsafe {
            core::mem::transmute::<&[SDL_GPUVertexAttribute], &[VertexAttribute]>(&[
                SDL_GPUVertexAttribute {
                    location: 0,
                    buffer_slot: 0,
                    format: SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3,
                    offset: 8,
                },
                SDL_GPUVertexAttribute {
                    location: 1,
                    buffer_slot: 0,
                    format: SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2,
                    offset: 0,
                },
            ])
        }
    }
}
