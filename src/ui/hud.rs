use specs::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

use std::collections::VecDeque;

use crate::components::*;
use crate::{ScreenInfo, MouseInfo, MouseCommand, UIAction};

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, MouseInfo>,
    WriteExpect<'a, Option<MouseCommand>>,
    WriteExpect<'a, VecDeque<UIAction>>,
    ReadStorage<'a, Selectable>,
    ReadStorage<'a, Turn>,
    ReadStorage<'a, Health>
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
            let x = screen.width - 80 - 80;
            let y = screen.height - 80;
            if (point.x > x && point.x < x + 60) && (point.y > y && point.y < y + 60) {
                ui_commands.push_back(UIAction::ActionButton(1));
                // Consume the click so it doesn't get processed by other systems
                let mouse_command = &mut *data.2;
                *mouse_command = Option::None;
            }
        }
    }
}

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &mut [Texture],
    data: SystemData
) -> Result<(), String> {
    let screen = &*data.0;
    let mouse = &*data.1;

    // Only draw actions if selected unit has turn
    let mut draw_action_hud = false;
    for (selectable, turn) in (&data.4, &data.5).join() {
        if selectable.selected && turn.current {
            draw_action_hud = true;
        }
    }

    if draw_action_hud {
        let x = screen.width - 80;
        let y = screen.height - 80;
        let screen_rect = Rect::new(x, y, 60, 60);
        if (mouse.x > x && mouse.x < x + 60) && (mouse.y > y && mouse.y < y + 60) {
            textures[0].set_color_mod(150, 150, 150);
        }
        else {
            textures[0].set_color_mod(255, 255, 255);
        }
        canvas.copy(&textures[0], None, screen_rect)?;
        let x = screen.width - 80 - 80;
        let y = screen.height - 80;
        let screen_rect = Rect::new(x, y, 60, 60);
        if (mouse.x > x && mouse.x < x + 60) && (mouse.y > y && mouse.y < y + 60) {
            textures[3].set_color_mod(150, 150, 150);
        }
        else {
            textures[3].set_color_mod(255, 255, 255);
        }
        canvas.copy(&textures[3], None, screen_rect)?;
    }

    // Health of current selected entity
    for (select, health) in (&data.4, (&data.6).maybe()).join() {
        if select.selected {
            if let Some(health) = health {
                let y = screen.height - 80;
                for index in 1..health.max_health {
                    let screen_rect = Rect::new(index * 40 + 80, y, 30, 30);
                    let texture_id = if index < health.health { 1 } else { 2 };
                    canvas.copy(&textures[texture_id], None, screen_rect)?;
                }
            }
        }
    }

    Ok(())
}