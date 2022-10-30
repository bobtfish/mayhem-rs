use bevy::{prelude::*};
use super::constants::ANIMATION_TICK;
use super::{AnimationTimer, get_anim};

#[derive(Component)]
pub struct Cursor {
    visible: bool,
    flash: bool,
    x: u8,
    y: u8,
}

impl Cursor {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        commands: &mut Commands,
    ) {
       let c = Cursor{
            visible: true,
            flash: true,
            x: 0,
            y: 0,
        };
        commands.spawn_bundle(get_anim(texture_atlas_handle, Vec2::splat(5.0), 165))
        .insert(AnimationTimer(Timer::from_seconds(ANIMATION_TICK/2.0, true)))
        .insert(c);
    }
}

pub fn animate_cursor(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Cursor,
    )>,
) {
    for (mut timer, mut sprite, mut cursor) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if cursor.flash || !cursor.visible {
                cursor.flash = false;
                sprite.color.set_a(0.0);
            } else {
                cursor.flash = true;
                sprite.color.set_a(1.0);
            }
        }
    }
}
