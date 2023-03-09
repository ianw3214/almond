use bevy::prelude::*;
use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler::Descriptor;

use crate::game::components::*;

pub fn update_sprite_translation(mut sprites : Query<(&mut Transform, &WorldPosition)>,) {
    for (mut transform, position) in sprites.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

pub fn update_sprite_animation(
    time : Res<Time>,
    animation_trees : Res<Assets<AnimationTree>>,
    mut query : Query<(&mut Animation, &mut TextureAtlasSprite)>) 
{
    for (mut anim, mut sprite) in &mut query {
        // handle animation events
        let anim_tree = animation_trees.get(&anim.tree).unwrap();
        let initial_state = anim.current_state.clone();
        let mut curr_state = &anim.current_state;
        for event in &anim.events {
            let state = anim_tree.states.get(curr_state).unwrap();
            let new_state = state.transitions.get(event);
            curr_state = match new_state {
                Some(name) => name,
                _ => curr_state
            };
        }
        if initial_state != curr_state.clone() {
            sprite.index = anim_tree.states.get(curr_state).unwrap().start_frame;
        }
        anim.current_state = curr_state.to_string();
        anim.events.clear();
        // update the actual animation frame
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            let next_index = sprite.index + 1;
            let curr_state = anim_tree.states.get(&anim.current_state).unwrap();
            sprite.index = if next_index > curr_state.end_frame { curr_state.start_frame } else { next_index };
        }
    }
}

pub fn update_sprite_size(mut query : Query<(&mut RenderInfo, &mut TextureAtlasSprite)>) {
    for (render_info, mut sprite) in &mut query {
        sprite.custom_size = Some(Vec2::new(render_info.screen_width, render_info.screen_height));
    }
}

pub fn fixup_sprites(
    mut asset_events : EventReader<AssetEvent<Image>>,
    mut assets : ResMut<Assets<Image>>) 
{
    for ev in asset_events.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                let mut texture = assets.get_mut(handle).unwrap();
                texture.sampler_descriptor = Descriptor(SamplerDescriptor {
                    mag_filter : FilterMode::Nearest,
                    min_filter : FilterMode::Nearest,
                    mipmap_filter : FilterMode::Nearest,
                    ..default()
                });
            }
            AssetEvent::Modified { handle : _ } => {
                println!("IMAGE ASSET MODIFIED\n");
            }
            AssetEvent::Removed { handle : _ } => {
                println!("IMAGE ASSET REMOVED\n");
            }
        }
    }
}

pub fn initialize_anim_states(
    mut asset_events : EventReader<AssetEvent<AnimationTree>>,
    animation_trees : Res<Assets<AnimationTree>>,
    mut query : Query<&mut Animation>) 
{
    for ev in asset_events.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                let anim_tree = animation_trees.get(handle).unwrap();
                for mut anim in query.iter_mut() {
                    if anim.tree == *handle {
                        anim.current_state = anim_tree.initial.clone();
                    }
                }
            }
            AssetEvent::Modified { handle : _ } => {
                println!("ANIMATION TREE ASSET MODIFIED\n");
            }
            AssetEvent::Removed { handle : _ } => {
                println!("ANIMATION TREE ASSET REMOVED\n");
            }
        }
    }
}