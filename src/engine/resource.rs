use sdl2::{render::{TextureCreator, Texture}, video::WindowContext, image::LoadTexture};

pub struct TextureManager<'creator> {
    pub creator : &'creator TextureCreator<WindowContext>,
    pub textures : Vec<Texture<'creator>>
}

impl<'creator> TextureManager<'creator> {
    pub fn new(creator : &'creator TextureCreator<WindowContext>) -> Self {
        TextureManager {
            creator : creator,
            textures : Vec::<Texture<'creator>>::new()
        }
    }

    pub fn load(&mut self, filename : &str) {
        self.textures.push(self.creator.load_texture(filename).unwrap());
    }
}