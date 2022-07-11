use specs::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

use std::collections::VecDeque;

// use crate::components::*;
use crate::{ScreenInfo, MouseInfo, MouseCommand, UIAction};

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, MouseInfo>,
    WriteExpect<'a, Option<MouseCommand>>,
    WriteExpect<'a, VecDeque<UIAction>>
);

pub fn run(mut data: SystemData) {
    let screen = &*data.0;
    let mouse_command = match &*data.2 {
        Some(mouse_command) => mouse_command,
        None => return
    };

    // TODO: This should only be handled when an entity w/ actions is selected
    match mouse_command {
        &MouseCommand::Click(point) => {
            let ui_commands = &mut *data.3;
            let x = screen.width - 80;
            let y = screen.height - 80;
            if (point.x > x && point.x < x + 60) && (point.y > y && point.y < y + 60) {
                ui_commands.push_back(UIAction::ActionButton(0));
                // Consume the click so it doesn't get processed by other systems
                let mouse_command = &mut *data.2;
                *mouse_command = Option::None;
            }
        }
    }
}

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    let screen = &*data.0;
    let mouse = &*data.1;

    // TODO: This should only be handled when an entity w/ actions is selected
    let x = screen.width - 80;
    let y = screen.height - 80;
    let screen_rect = Rect::new(x, y, 60, 60);
    if (mouse.x > x && mouse.x < x + 60) && (mouse.y > y && mouse.y < y + 60) {
        canvas.copy(&textures[1], None, screen_rect)?;
    }
    else {
        canvas.copy(&textures[0], None, screen_rect)?;
    }

    Ok(())
}