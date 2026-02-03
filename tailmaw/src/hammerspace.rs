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
        if self.textures.contains_key(name) {
            return self.textures.get(name).cloned();
        }
        None
    }

    pub fn model(&mut self, name: &str) -> Option<Model<'_>> {
        if self.models.contains_key(name) {
            return self.models.get(name).cloned();
        }
        None
    }

    pub fn shader(&mut self, name: &str) -> Option<Shader> {
        if self.shaders.contains_key(name) {
            return self.shaders.get(name).cloned();
        }
        None
    }
}

impl Default for Hammerspace<'_> {
    fn default() -> Self {
        Self::new()
    }
}
