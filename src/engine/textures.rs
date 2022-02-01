use std::collections::HashMap;

use sdl2::{render::{TextureCreator, Texture}, video::WindowContext, image::LoadTexture};

pub struct Textures<'a> {
    creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<&'a str, Texture<'a>>
}

impl<'a> Textures<'a> {
    pub fn new(creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            creator,
            textures: HashMap::new()
        }
    }

    pub fn load_texture(&mut self, name: &'a str, path: &'a str) {
        let texture = self.creator.load_texture(path)
            .expect("Could not load texture");
        self.textures.insert(name, texture);
    }

    pub fn get_texture(&self, name: &str) -> &Texture {
        self.textures.get(name)
            .expect("Could not found the specified texture")
    }
}
