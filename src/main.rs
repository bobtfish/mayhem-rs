use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode, math::vec3};
pub mod constants;
pub mod cursor;
pub use self::constants::*;
use crate::display::*;

#[derive(Default)]
struct Game {
    tah: Handle<TextureAtlas>,
    fah: Handle<TextureAtlas>,
    cursor: cursor::Cursor,
    players: u8,
    ai_level: u8,
    player_info: Vec<Player>,
    player_turn: u8,
}

impl Game {
    fn get_player(&self) -> &Player {
        &self.player_info[self.player_turn as usize]
    }
}

struct Player {
    name: String,
    computer_controlled: bool,
    character_icon: u8,
    color: u8,
}

impl Player {
    fn spells(&self) -> Vec<Spell> {
        return vec![
            Spell {name: "Spell one".to_string()},
            Spell {name: "Spell two".to_string()},
            Spell {name: "Spell 3".to_string()},
            Spell {name: "Spell 4".to_string()},
        ];
    }
}

struct Spell {
    name: String
}

impl Spell {
    fn get_sep(&self) -> &str {
        return "-";
    }
}

fn get_border(
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

fn setup_initial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32), 10, 41);
    game.tah = texture_atlases.add(texture_atlas);
    let font_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new((SPRITE_SIZE/2) as f32, SPRITE_SIZE as f32), 20, 41);
    game.fah = texture_atlases.add(font_atlas);

    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_scale(vec3(1.0/(SCALE*SPRITE_SIZE as f32), 1.0/(SCALE*SPRITE_SIZE as f32), 1.0))
            .with_translation(vec3((WIDTH/2) as f32-0.5, (HEIGHT/2) as f32-0.5, CAMERA_Z)),
        ..default()
    });
}

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    InitialMenu,
    PlayerNameMenu,
    PlayerNameMenuTransition,
    PlayerMenu,
    PlayerMenuExamineSpell,
    PlayerMenuExamineOneSpell,
    PlayerMenuSelectSpell,
    PlayerMenuExamineBoard,
    PlayerMenuTransition,
    Game,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(WindowDescriptor {
            title: "Mayhem!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::InitialMenu)
        .add_startup_system(setup_initial)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(choosespell::ChooseSpellPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(cursor::CursorPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

mod menu;
mod game;
mod choosespell;
mod display;