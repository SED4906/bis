use image::DynamicImage;
use jxl_oxide::integration::JxlDecoder;
use sdl3::gpu::ShaderStage;

use crate::base::Engine;

impl Engine {
    pub(super) fn load_texture_data(&mut self, name: &str) -> Option<(Vec<u8>,u32,u32)> {
        if name.is_empty() {
            let jxl = JxlDecoder::new(&include_bytes!("../res/missing.jxl")[..]).expect("jxl decoder failed");
            let img = DynamicImage::from_decoder(jxl).expect("image decode failed");
            return Some((img.as_bytes().to_vec(),img.width(),img.height()));
        }
        None
    }

    pub(super) fn load_shader_data(&mut self, name: &str) -> Option<(Vec<u8>, ShaderStage)> {
        match name {
            "default.vert.spv" => {
                return Some((include_bytes!("../res/default.vert.spv").to_vec(), ShaderStage::Vertex));
            }
            "default.frag.spv" => {
                return Some((include_bytes!("../res/default.frag.spv").to_vec(), ShaderStage::Fragment));
            }
            _ => {}
        }
        None
    }
}