use bevy::{prelude::*, math::vec3};
use rand::Rng;

use crate::constants::*;
const WIZARD_IDX: usize = 170;

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_scale(vec3(1.0/(SCALE*SPRITE_SIZE as f32), 1.0/(SCALE*SPRITE_SIZE as f32), 1.0))
            .with_translation(vec3((WIDTH/2) as f32-1.0, (HEIGHT/2) as f32-2.0, CAMERA_Z)),
        ..default()
    });
}

pub fn get_border(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>
) {
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(-0.5, -0.5), BORDER_BOTTOMLEFT, WHITE));
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(-0.5, (HEIGHT-1) as f32-1.5), BORDER_TOPLEFT, WHITE));
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH) as f32-1.5, -0.5), BORDER_BOTTOMRIGHT, WHITE));
    commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH) as f32-1.5, HEIGHT as f32-2.5), BORDER_TOPRIGHT, WHITE));
    for n in 2..HEIGHT-1 {
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(-0.5, n as f32-1.5), BORDER_LEFT, WHITE));
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH) as f32-1.5, n as f32-1.5), BORDER_RIGHT, WHITE));
    }
    for n in 1..WIDTH-1 {
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(n as f32-0.5, -0.5), BORDER_BOTTOM, WHITE));
        commands.spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(n as f32-0.5, HEIGHT as f32-2.5), BORDER_TOP, WHITE));
    }
}

pub fn get_sprite_sheet_bundle(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    color: Color,
) -> SpriteSheetBundle {
    get_sprite_sheet_bundle_z(texture_atlas_handle, v, init, color, 0.0)
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgba(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0)
}

pub const WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

pub fn get_sprite_sheet_bundle_z(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    color: Color,
    z: f32,
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(init);
    sprite.color = color;
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
        commands.spawn_bundle(get_sprite_sheet_bundle(fah.clone(), new_v, char_to_pos(ch), WHITE))
        .insert(c);
    }
}

pub fn print_wizard(commands: &mut Commands, tah: Handle<TextureAtlas>, v: Vec2, idx: usize, c: impl Component + std::marker::Copy) {
    commands.spawn_bundle(get_sprite_sheet_bundle(tah, v, WIZARD_IDX + idx, WHITE))
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

#[derive(Component)]
pub struct Mortal {
    is_alive: bool
}

#[derive(Component)]
pub struct RepeatAnimation {
    max: usize,
    init: usize,
    timer: Timer,
}

pub fn spawn_anim(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    num: usize
) -> Entity {
    return commands
        .spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle, v, init, WHITE))
        .insert(RepeatAnimation {
            max: init+num-1,
            init,
            timer: Timer::from_seconds(ANIMATION_TICK, true),
        }).id();
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut RepeatAnimation,
        Option<&Mortal>,
    )>,
) {
    for (mut sprite, mut repeater, mortal) in &mut query {
        repeater.timer.tick(time.delta());
        if repeater.timer.just_finished() {
            let alive = mortal.map_or(true, |x| x.is_alive);
            if alive {
                let mut index = sprite.index + 1;
                if index > repeater.max {
                    index = repeater.init;
                }
                sprite.index = index;
            } else {
                sprite.index = repeater.max + 1;
            }
        }
    }
}