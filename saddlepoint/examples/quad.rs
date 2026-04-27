use std::time::Instant;

use image::DynamicImage;
use jxl_oxide::integration::JxlDecoder;
use saddlepoint::{Camera, Material, RenderingEngine, VertexPositionUV};
use sdl3::event::Event;
use tailmaw::geometry::{space::{Quaternion, Vector2, Vector3}, transform::transformation};

pub fn main() {
    let mut rendering_engine = RenderingEngine::new("Saddle Point");
    let sampler = rendering_engine.create_sampler();
    let vertex_shader = rendering_engine.create_shader(
        include_bytes!("../../tailmaw/res/default.vert.spv"),
        sdl3::gpu::ShaderStage::Vertex,
    );
    let fragment_shader = rendering_engine.create_shader(
        include_bytes!("../../tailmaw/res/default.frag.spv"),
        sdl3::gpu::ShaderStage::Fragment,
    );
    let graphics_pipeline = rendering_engine
        .create_graphics_pipeline::<VertexPositionUV>(vertex_shader, fragment_shader);
    let texture = rendering_engine.create_sized_texture(256, 256);
    let jxl = JxlDecoder::new(&include_bytes!("../../tailmaw/res/missing.jxl")[..])
        .expect("jxl decoder failed");
    let img = DynamicImage::from_decoder(jxl)
        .expect("image decode failed")
        .flipv();
    let data = img.as_bytes();
    rendering_engine.load_texture(&texture, data);
    let material = Material {
        graphics_pipeline,
        texture,
        sampler,
    };
    let mut geometry = rendering_engine.create_geometry(
        &[
            VertexPositionUV {
                position: Vector3::new([-1.0, -1.0, 0.0]),
                uv: Vector2::new([0.0, 0.0]),
            },
            VertexPositionUV {
                position: Vector3::new([-1.0, 1.0, 0.0]),
                uv: Vector2::new([0.0, 1.0]),
            },
            VertexPositionUV {
                position: Vector3::new([1.0, 1.0, 0.0]),
                uv: Vector2::new([1.0, 1.0]),
            },
            VertexPositionUV {
                position: Vector3::new([1.0, -1.0, 0.0]),
                uv: Vector2::new([1.0, 0.0]),
            },
        ],
        &[0, 1, 2, 0, 2, 3],
        material,
        None
    );
    let camera = Camera { position: Vector3::new([1.0,2.0,4.0]), target: Vector3::fill(0.0), up: saddlepoint::YOrZ::Y, fov: 1.308 };
    let mut time = 0.0;
    loop {
        let start = Instant::now();
        let mut event_pump = rendering_engine
            .sdl
            .event_pump()
            .expect("event pump failed");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return;
                }
                _ => {}
            }
        }
        rendering_engine.draw_geometry(Some(camera), &[&geometry], None);
        geometry.transform = transformation(Vector3::fill(0.0), Quaternion::axis_angle(Vector3::new([0.0,0.0,1.0]), time), Vector3::fill(1.0));
        let end = Instant::now();
        time += (end - start).as_secs_f32();
    }
}
