use bevy::{prelude::*, math::vec3};
use rand::Rng;

use crate::constants::SPRITE_SIZE;

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
    return SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(v.extend(z)).with_scale(vec3(1.0/SPRITE_SIZE as f32, 1.0/SPRITE_SIZE as f32, 1.0)),
        sprite: sprite,
        ..default()
    };
}


pub fn print_text(str: &str, commands: &mut Commands, fah: Handle<TextureAtlas>, v: Vec2, c: impl Component + std::marker::Copy) {
    for (i,ch) in str.chars().enumerate() {
        let mut new_v = v.clone();
        new_v.x = new_v.x + (i as f32/2.0);
        commands.spawn_bundle(get_sprite_sheet_bundle(fah.clone(), new_v, char_to_pos(ch)))
        .insert(c);
    }
}

fn char_to_pos(c: char) -> usize {
    let d = c as u32;
    if d >=33 && d <= 126{
        return (d - 31) as usize;
    }
    if c == ' ' {
        return 1;
    }
    return 0;
}

