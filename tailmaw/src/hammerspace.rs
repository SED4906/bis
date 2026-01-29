use std::collections::HashMap;

use sdl3::gpu::{Shader, Texture};

use crate::model::Model;

pub struct Hammerspace<'a> {
    textures: HashMap<String, Texture<'a>>,
    models: HashMap<String, Model<'a>>,
    shaders: HashMap<String, Shader>
}

impl Hammerspace<'_> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            models: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn texture(&mut self, name: &str) -> Option<Texture<'_>> {
        None
    }

    pub fn model(&mut self, name: &str) -> Option<Model<'_>> {
        None
    }

    pub fn shader(&mut self, name: &str) -> Option<Shader> {
        None
    }
}

impl Default for Hammerspace<'_> {
    fn default() -> Self {
        Self::new()
    }
}
