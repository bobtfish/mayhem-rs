use bevy::prelude::*;
use super::{GameState, despawn_screen, ANIMATION_TICK};
use crate::display::*;
use crate::player::Player;
use crate::cursor::Cursor;

#[derive(Default)]
pub struct Game {
    pub tah: Handle<TextureAtlas>,
    pub fah: Handle<TextureAtlas>,
    pub cursor: Cursor,
    pub players: u8,
    pub ai_level: u8,
    pub player_info: Vec<Player>,
    pub player_turn: u8,
}

impl Game {
    pub fn get_player(&self) -> &Player {
        &self.player_info[self.player_turn as usize]
    }
    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player_info[self.player_turn as usize]
    }
}

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
    mut g: ResMut<Game>,
) {
    let tah = g.tah.clone();
    // Wizard with bow
    //spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(2.0), 120, 8);
    // Spell/splodey thing
    //spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(1.0), 180, 4);

    //let creature = spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(3.0), 210, 4);
    //commands.entity(creature).insert(Mortal{is_alive: false});

    //let creature_map = load_creatures();
    //creature_map.get("Pegasus").unwrap().to_entity(Vec2::splat(4.0), &mut commands, g.tah.clone());
    for p in g.player_info.iter_mut() {
        p.spawn(&mut commands, tah.clone());
    }
}

pub fn spawn_anim(
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
            init,
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

