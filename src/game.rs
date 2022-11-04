use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use super::{GameState, despawn_screen, ANIMATION_TICK, Game};
use crate::display::*;

#[derive(Component)]
struct Mortal {
    is_alive: bool
}

#[derive(Component)]
struct RepeatAnimation {
    max: usize,
    init: usize,
    timer: Timer,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(animate_sprite)
                )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    g: Res<Game>,
) {
    // Wizard with bow
    spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(2.0), 120, 8);
    // Spell/splodey thing
    spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(1.0), 180, 4);

    let creature = spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(3.0), 210, 4);
    commands.entity(creature).insert(Mortal{is_alive: false});

    let creature_map = load_creatures();
    creature_map.get("Pegasus").unwrap().to_entity(Vec2::splat(4.0), &mut commands, g.tah.clone());
}

fn spawn_anim(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    num: usize
) -> Entity {
    return commands
        .spawn_bundle(get_sprite_sheet_bundle(texture_atlas_handle, v, init))
        .insert(RepeatAnimation {
            max: init+num-1,
            init: init,
            timer: Timer::from_seconds(ANIMATION_TICK, true),
        }).id();
}

fn animate_sprite(
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
            let alive = match mortal {
                // The division was valid
                Some(x) => x.is_alive,
                None    => true,
            };
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

/// TODO
/// 
///
#[derive(Debug, Deserialize, Serialize)]
struct Creature {
    name: String,
    sprite_index: usize,
}

impl Creature {
    fn to_entity(
        &self,
        v: Vec2,
        commands: &mut Commands,
        texture_atlas_handle: Handle<TextureAtlas>
    ) -> Entity {
        return spawn_anim(commands, texture_atlas_handle.clone(), v, self.sprite_index, 4)
    }
}

fn load_creatures() -> HashMap<String, Creature> {
    let f = File::open("assets/creatures.ron").unwrap();
    return ron::de::from_reader(f).unwrap();
}