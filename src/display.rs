use bevy::{prelude::*, math::vec3};
use rand::Rng;

use crate::constants::*;
const WIZARD_IDX: usize = 170;

pub fn get_border(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>
) {
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(0.0, 1.0), BORDER_BOTTOMLEFT));
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(0.0, (HEIGHT-1) as f32), BORDER_TOPLEFT));
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, 1.0), BORDER_BOTTOMRIGHT));
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, (HEIGHT-1) as f32), BORDER_TOPRIGHT));
    for n in 2..HEIGHT-1 {
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(0.0, n as f32), BORDER_LEFT));
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, n as f32), BORDER_RIGHT));
    }
    for n in 1..WIDTH-1 {
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(n as f32, 1.0), BORDER_BOTTOM));
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(n as f32, (HEIGHT-1) as f32), BORDER_TOP));
    }
}

pub fn get_sprite_sheet_bundle(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
) -> SpriteSheetBundle {
    get_sprite_sheet_bundle_z(texture_atlas_handle, v, init, 0.0)
}

pub fn get_sprite_sheet_bundle_z(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    z: f32,
) -> SpriteSheetBundle {
    let mut rng = rand::thread_rng();
    let mut sprite = TextureAtlasSprite::new(init);
    sprite.color = Color::rgba(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0);
    SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(v.extend(z)).with_scale(vec3(1.0/SPRITE_SIZE as f32, 1.0/SPRITE_SIZE as f32, 1.0)),
        sprite,
        ..default()
    }
}


pub fn print_text(str: &str, commands: &mut Commands, fah: Handle<TextureAtlas>, v: Vec2, c: impl Component + std::marker::Copy) {
    for (i,ch) in str.chars().enumerate() {
        let mut new_v = v;
        new_v.x += i as f32/2.0;
        commands.spawn_bundle(get_sprite_sheet_bundle(fah.clone(), new_v, char_to_pos(ch)))
        .insert(c);
    }
}

pub fn print_wizard(commands: &mut Commands, tah: Handle<TextureAtlas>, v: Vec2, idx: usize, c: impl Component + std::marker::Copy) {
    commands.spawn_bundle(get_sprite_sheet_bundle(tah, v, WIZARD_IDX + idx))
    .insert(c);
}

fn char_to_pos(c: char) -> usize {
    let d = c as u32;
    if (33..=126).contains(&d) {
        return (d - 31) as usize;
    }
    if c == ' ' {
        return 1;
    }
    0
}

