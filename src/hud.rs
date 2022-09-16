use specs::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

// use crate::components::*;

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
        self.controls.push(UIElement::Background(BackgroundData{ x : 0, y : 0, w : 20, h : 720}));
        self.controls.push(UIElement::Button(ButtonData{ x : 10, y : 10, w : 40, h : 40, image : 1, state : ButtonState::DEFAULT, event : UIEvent::Build }));
        self.controls.push(UIElement::Button(ButtonData{ x : 10, y : 60, w : 40, h : 40, image : 2, state : ButtonState::DEFAULT, event : UIEvent::Collect }));
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

    pub fn render(&self, canvas : &mut WindowCanvas, textures: &mut [Texture], _world: &World) {
        for element in &self.controls {
            match element {
                UIElement::Background(data) => {
                    // minimum size for backgrounds for proper rendering
                    assert!(data.w >= 20, "Background width is less than required minimum of 20");
                    assert!(data.h >= 20, "Background height is less than required minimum of 20");
                    let mut render_part = | src : Rect, dst : Rect | {
                        canvas.copy(&textures[0], src, dst).expect("render copy failed...");
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
                    textures[data.image].set_color_mod(tint, tint, tint);
                    canvas.copy(&textures[data.image], None, screen_rect).expect("render copy failed...");
                }
            }
        }
    }
}