use sdl2::pixels::PixelFormatEnum;
use specs::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas};

use crate::{components::*, CameraInfo};
use crate::engine::resource::TextureManager;
use crate::engine::text::TextEngine;
use crate::TownInfo;

#[derive(Clone, Copy)]
pub enum UIEvent {
    Build,
    Collect
}

enum ButtonState {
    DEFAULT,
    HOVER
}

struct BackgroundData {
    x : i32,
    y : i32,
    w : u32,
    h : u32
}

struct ButtonData {
    x : i32,
    y : i32,
    w : u32,
    h : u32,
    image : usize,
    state : ButtonState,
    event : UIEvent
}

enum UIElement {
    Background(BackgroundData),
    Button(ButtonData)
}

pub struct Hud {
    controls : Vec<UIElement>
}

impl Hud {
    pub fn new() -> Hud {
        return Hud { controls : vec![] };
    }
    
    pub fn init(&mut self) {
        self.controls.push(UIElement::Background(BackgroundData{ x : 0, y : 0, w : 40, h : 720}));
        self.controls.push(UIElement::Button(ButtonData{ x : 10, y : 10, w : 60, h : 60, image : 1, state : ButtonState::DEFAULT, event : UIEvent::Build }));
        self.controls.push(UIElement::Button(ButtonData{ x : 10, y : 80, w : 60, h : 60, image : 2, state : ButtonState::DEFAULT, event : UIEvent::Collect }));
    }

    pub fn handle_mouse_click(&mut self, x : i32, y : i32, world: &mut World) -> bool{
        for element in &mut self.controls {
            if let UIElement::Button(data) = element {
                if x > data.x && x < data.x + data.w as i32 {
                    if y > data.y && y < data.y + data.h as i32 {
                        let mut eventqueue = world.write_resource::<Vec<UIEvent>>();
                        eventqueue.push(data.event);
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn handle_mouse_motion(&mut self, x : i32, y : i32) {
        for element in &mut self.controls {
            if let UIElement::Button(data) = element {
                data.state = ButtonState::DEFAULT;
                if x > data.x && x < data.x + data.w as i32 {
                    if y > data.y && y < data.y + data.h as i32 {
                        data.state = ButtonState::HOVER;
                    }
                }
            }
        }
    }

    pub fn render(&self, canvas : &mut WindowCanvas, textures: &mut TextureManager, text_engine: &TextEngine, world: &World) {
        // game entity hud data
        let positions = world.read_storage::<Position>();
        let constructions = world.read_storage::<Construction>();
        let camera_info = world.read_resource::<CameraInfo>();
        for (pos, construction) in (&positions, &constructions).join() {
            if construction.timer > 0.0 {
                let screen_x = pos.x - camera_info.x as i32;
                let screen_y = pos.y - camera_info.y as i32;
                let filled = 1.0 - construction.timer / 10.0;
                let bg_rect = Rect::new(screen_x, screen_y, 40, 10);
                canvas.copy(&textures.textures[4], None, bg_rect).expect("");
                let filled_rect = Rect::new(screen_x, screen_y, (40.0 * filled) as u32, 10);
                // filled portion
                canvas.copy(&textures.textures[3], None, filled_rect).expect("");
            }
        }
        
        for element in &self.controls {
            match element {
                UIElement::Background(data) => {
                    // minimum size for backgrounds for proper rendering
                    debug_assert!(data.w >= 20, "Background width is less than required minimum of 20");
                    debug_assert!(data.h >= 20, "Background height is less than required minimum of 20");
                    let mut render_part = | src : Rect, dst : Rect | {
                        canvas.copy(&textures.textures[0], src, dst).expect("render copy failed...");
                    };
                    // top left
                    render_part(Rect::new(0, 0, 10, 10), Rect::new(data.x, data.y, 10, 10));
                    // top middle
                    render_part(Rect::new(10, 0, 10, 10), Rect::new(data.x + 10, data.y, data.w - 20, 10));
                    // top right
                    render_part(Rect::new(20, 0, 10, 10), Rect::new(data.x + data.w as i32- 10, data.y, 10, 10));
                    // middle left
                    render_part(Rect::new(0, 10, 10, 10), Rect::new(data.x, data.y + 10, 10, data.h - 20));
                    // middle middle
                    render_part(Rect::new(10, 10, 10, 10), Rect::new(data.x + 10, data.y + 10, data.w - 20, data.h - 20));
                    // middle right
                    render_part(Rect::new(20, 10, 10, 10), Rect::new(data.x + data.w as i32 - 10, data.y + 10, 10, data.h - 20));
                    // bottom left
                    render_part(Rect::new(0, 20, 10, 10), Rect::new(data.x, data.y + data.h as i32 - 10, 10, 10));
                    // bottom middle
                    render_part(Rect::new(10, 20, 10, 10), Rect::new(data.x + 10, data.y + data.h as i32 - 10, data.w - 20, 10));
                    // bottom right
                    render_part(Rect::new(20, 20, 10, 10), Rect::new(data.x + data.w as i32 - 10, data.y + data.h as i32 - 10, 10, 10));
                },
                UIElement::Button(data) => {
                    let screen_rect = Rect::new(data.x, data.y, data.w, data.h);
                    let tint = match data.state {
                        ButtonState::DEFAULT => 255,
                        ButtonState::HOVER => 100
                    };
                    textures.textures[data.image].set_color_mod(tint, tint, tint);
                    canvas.copy(&textures.textures[data.image], None, screen_rect).expect("render copy failed...");
                }
            }
        }

        // text showing current resources
        //  TODO: Convert to UI Element for rendering
        let storages = world.read_storage::<ResourceStorage>();
        let inventories = world.read_storage::<Inventory>();
        let mut resources : Vec<(ResourceType, i32)> = vec![ (ResourceType::WOOD, 0), (ResourceType::FLINT, 0)];
        let mut add_to_resources = |t : ResourceType, n : i32| {
            for resource in &mut resources {
                if resource.0 == t {
                    resource.1 = resource.1 + n;
                }
            }
        };
        for storage in (&storages).join() {
            for resource in &storage.resources {
                add_to_resources(resource.0, resource.1);
            }
        }
        for inventory in (&inventories).join() {
            for resource in &inventory.resources {
                add_to_resources(resource.0, resource.1);
            }
        }

        // actual resource rendering
        let scale : f32 = 32.0;
        let mut draw_text = |message : &str, x : i32, y : i32| {
            let (width, height, pixel_data) = text_engine.layout_data(message, scale);
            let mut texture = textures.creator.create_texture(PixelFormatEnum::RGBA32, sdl2::render::TextureAccess::Static, width, height).expect("Texture creation failed...");
            texture.update(None, &pixel_data, 4 * width as usize).expect("texture update failed");
            texture.set_blend_mode(sdl2::render::BlendMode::Add);
            
            let dst_rect = sdl2::rect::Rect::new(x, y, width, height);
            canvas.copy(&texture, None, dst_rect).expect("text rendering failed");
        };
        draw_text(&resources[0].1.to_string(), 1280 - 100, 720 - 110);
        draw_text(&resources[1].1.to_string(), 1280 - 100, 720 - 80);

        // draw the town name
        let town_info = world.fetch::<TownInfo>();
        draw_text(&town_info.name, 1280 - 150, 720 - 40);
    }
}