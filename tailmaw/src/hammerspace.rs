use std::collections::HashMap;

use image::DynamicImage;
use jxl_oxide::integration::JxlDecoder;
use sdl3::{gpu::{BufferRegion, BufferUsageFlags, ColorTargetDescription, ComputePipeline, DepthStencilState, Device, GraphicsPipeline, GraphicsPipelineTargetInfo, RasterizerState, Sampler, SamplerCreateInfo, Shader, ShaderFormat, ShaderStage, Texture, TextureCreateInfo, TextureFormat, TextureRegion, TextureTransferInfo, TextureUsage, TransferBufferLocation, TransferBufferUsage, VertexAttribute, VertexBufferDescription, VertexInputState}, video::Window};

use crate::{formats::{vit::vit, vit2::vit2}, geometry::space::{Vector2, Vector3}, model::{Mesh, MeshGeometry, Model, Vertex}};

pub struct Hammerspace {
    textures: HashMap<String, Texture<'static>>,
    models: HashMap<String, Model>,
    shaders: HashMap<String, Shader>,
    samplers: HashMap<String, Sampler>,
    graphics_pipelines: HashMap<String, GraphicsPipeline>,
    compute_pipelines: HashMap<String, ComputePipeline>,
}

impl Hammerspace {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            models: HashMap::new(),
            shaders: HashMap::new(),
            samplers: HashMap::new(),
            graphics_pipelines: HashMap::new(),
            compute_pipelines: HashMap::new(),
        }
    }

    pub fn texture(&mut self, device: &Device, name: &str) -> Option<Texture<'static>> {
        if self.textures.contains_key(name) {
            return self.textures.get(name).cloned();
        }
        let Some((data, width, height)) = self.load_texture_data(name) else {
            return self.textures.get("").cloned();
        };
        let texture = device
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
        let command_buffer = device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");

        let copy_pass = device
            .begin_copy_pass(&command_buffer)
            .expect("failed to begin copy pass");
        let transfer_buffer = device
            .create_transfer_buffer()
            .with_size(data.len() as u32)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("failed to create transfer buffer");
        transfer_buffer
            .map(device, true)
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
        device.end_copy_pass(copy_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(device)
            .expect("submit and acquire fence failed");
        device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
        let _ = self.textures.insert(name.into(), texture.clone());
        Some(texture)
    }

    pub fn model(&mut self, device: &Device, name: &str) -> Option<Model> {
        if self.models.contains_key(name) {
            return self.models.get(name).cloned();
        }
        let loader_model = match name {
            "quad" => vit2(include_bytes!("res/quad.vit2")).unwrap(),
            "cube" => vit(include_str!("res/cube.vit")).unwrap(),
            "test_quad" => Mesh {
                mesh: MeshGeometry {
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
            "test_cube" => Mesh {
                mesh: MeshGeometry {
                    vertices: vec![
                        // +Z
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, 1.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, 1.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, 1.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, -1.0, 1.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                        // -Z
                        Vertex {
                            position: Vector3::new([1.0, -1.0, -1.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, -1.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, -1.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, -1.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                        // +X
                        Vertex {
                            position: Vector3::new([1.0, -1.0, 1.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, 1.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, -1.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, -1.0, -1.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                        // -X
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, -1.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, -1.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, 1.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, 1.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                        // +Y
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, 1.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, 1.0, -1.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, -1.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, 1.0, 1.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                        // -Y
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, -1.0]),
                            texture_coord: Vector2::new([0.0, 0.0]),
                        },
                        Vertex {
                            position: Vector3::new([-1.0, -1.0, 1.0]),
                            texture_coord: Vector2::new([0.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, -1.0, 1.0]),
                            texture_coord: Vector2::new([1.0, 1.0]),
                        },
                        Vertex {
                            position: Vector3::new([1.0, -1.0, -1.0]),
                            texture_coord: Vector2::new([1.0, 0.0]),
                        },
                    ],
                    indices: vec![
                        0, 1, 2, 0, 2, 3, // +Z
                        4, 5, 6, 4, 6, 7, // -Z
                        8, 9, 10, 8, 10, 11, // +X
                        12, 13, 14, 12, 14, 15, // -X
                        16, 17, 18, 16, 18, 19, // +Y
                        20, 21, 22, 20, 22, 23, // -Y
                    ],
                }
                .into(),
                texture: "".to_string().into(),
            },
            _ => return None,
        };
        let vertex_buffer_size = loader_model.mesh.vertices.len() as u32 * 20;
        let index_buffer_size = loader_model.mesh.indices.len() as u32 * 4;
        let vertex_buffer = device
            .create_buffer()
            .with_size(vertex_buffer_size)
            .with_usage(BufferUsageFlags::VERTEX)
            .build()
            .ok()?;
        let index_buffer = device
            .create_buffer()
            .with_size(index_buffer_size)
            .with_usage(BufferUsageFlags::INDEX)
            .build()
            .ok()?;
        let command_buffer = device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");

        let copy_pass = device
            .begin_copy_pass(&command_buffer)
            .expect("failed to begin copy pass");
        let vertex_transfer_buffer = device
            .create_transfer_buffer()
            .with_size(vertex_buffer_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("failed to create transfer buffer");
        let index_transfer_buffer = device
            .create_transfer_buffer()
            .with_size(index_buffer_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()
            .expect("failed to create transfer buffer");
        vertex_transfer_buffer
            .map(device, true)
            .mem_mut()
            .clone_from_slice(loader_model.mesh.vertices.as_slice());
        index_transfer_buffer
            .map(device, true)
            .mem_mut()
            .clone_from_slice(loader_model.mesh.indices.as_slice());
        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_offset(0)
                .with_transfer_buffer(&vertex_transfer_buffer),
            BufferRegion::new()
                .with_buffer(&vertex_buffer)
                .with_offset(0)
                .with_size(vertex_buffer_size),
            true,
        );
        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_offset(0)
                .with_transfer_buffer(&index_transfer_buffer),
            BufferRegion::new()
                .with_buffer(&index_buffer)
                .with_offset(0)
                .with_size(index_buffer_size),
            true,
        );
        device.end_copy_pass(copy_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(device)
            .expect("submit and acquire fence failed");
        device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
        let texture = self.texture(device, &loader_model.texture)?;
        let model = Model {
            vertex_buffer,
            index_buffer,
            texture,
        };
        let _ = self.models.insert(name.into(), model.clone());
        Some(model)
    }

    pub fn shader(&mut self, device: &Device, name: &str) -> Option<Shader> {
        if self.shaders.contains_key(name) {
            return self.shaders.get(name).cloned();
        }
        let Some((data, stage)) = self.load_shader_data(name) else {
            return None;
        };
        let shader = device
            .create_shader()
            .with_code(ShaderFormat::SPIRV, &data, stage)
            .with_entrypoint(c"main")
            .with_samplers(if stage == ShaderStage::Vertex { 0 } else { 1 })
            .with_uniform_buffers(1)
            .build()
            .ok()?;
        let _ = self.shaders.insert(name.into(), shader.clone());
        Some(shader)
    }

    pub fn sampler(&mut self, device: &Device, name: &str) -> Option<Sampler> {
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
        let sampler = device.create_sampler(sampler_create_info).ok()?;
        let _ = self.samplers.insert(name.into(), sampler.clone());
        Some(sampler)
    }

    pub fn graphics_pipeline(&mut self, window: &Window, device: &Device, name: &str) -> Option<GraphicsPipeline> {
        if self.graphics_pipelines.contains_key(name) {
            return self.graphics_pipelines.get(name).cloned();
        }
        let graphics_pipeline = match name {
            "default" => device
                .create_graphics_pipeline()
                .with_vertex_shader(self.shaders.get("default.vert.spv")?)
                .with_fragment_shader(self.shaders.get("default.frag.spv")?)
                .with_primitive_type(sdl3::gpu::PrimitiveType::TriangleList)
                .with_target_info(
                    GraphicsPipelineTargetInfo::new()
                        .with_color_target_descriptions(&[ColorTargetDescription::new()
                            .with_format(device.get_swapchain_texture_format(window))])
                        .with_has_depth_stencil_target(true)
                        .with_depth_stencil_format(TextureFormat::D16Unorm),
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
                .with_depth_stencil_state(
                    DepthStencilState::new()
                        .with_enable_depth_test(true)
                        .with_enable_depth_write(true)
                        .with_compare_op(sdl3::gpu::CompareOp::Less),
                )
                .with_rasterizer_state(
                    RasterizerState::new()
                        .with_front_face(sdl3::gpu::FrontFace::Clockwise)
                        .with_fill_mode(sdl3::gpu::FillMode::Fill)
                        .with_cull_mode(sdl3::gpu::CullMode::Back),
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

    pub(super) fn load_texture_data(&mut self, name: &str) -> Option<(Vec<u8>, u32, u32)> {
        if name.is_empty() {
            let jxl = JxlDecoder::new(&include_bytes!("../res/missing.jxl")[..])
                .expect("jxl decoder failed");
            let img = DynamicImage::from_decoder(jxl)
                .expect("image decode failed")
                .flipv();
            return Some((img.as_bytes().to_vec(), img.width(), img.height()));
        }
        let jxl = JxlDecoder::new(std::fs::File::open(name).ok()?).ok()?;
        let img = DynamicImage::from_decoder(jxl).ok()?.flipv();
        Some((img.as_bytes().to_vec(), img.width(), img.height()))
    }

    pub(super) fn load_shader_data(&mut self, name: &str) -> Option<(Vec<u8>, ShaderStage)> {
        match name {
            "default.vert.spv" => {
                return Some((
                    include_bytes!("../res/default.vert.spv").to_vec(),
                    ShaderStage::Vertex,
                ));
            }
            "default.frag.spv" => {
                return Some((
                    include_bytes!("../res/default.frag.spv").to_vec(),
                    ShaderStage::Fragment,
                ));
            }
            _ => {}
        }
        None
    }
}