use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

const SCALE: f32 = 4.0;

const SPRITE_SIZE: usize = 16;

const SCALED_SPRITE_SIZE: f32 =  SPRITE_SIZE as f32 * SCALE;

const HEIGHT: usize = 12;
const WIDTH: usize = 16;

const SCREEN_WIDTH: f32 = SCALE*((SPRITE_SIZE*WIDTH) as f32);
const SCREEN_HEIGHT: f32 = SCALE*((SPRITE_SIZE*HEIGHT) as f32);

const HALF_SPRITE: f32 =  SCALED_SPRITE_SIZE/2.0;

const BORDER_TOPLEFT: usize = 202;
const BORDER_TOP: usize = 203;
const BORDER_TOPRIGHT: usize = 204;
const BORDER_LEFT: usize = 205;
const BORDER_BOTTOMLEFT: usize = 206;
const BORDER_BOTTOM: usize = 207;
const BORDER_BOTTOMRIGHT: usize = 208;
const BORDER_RIGHT: usize = 209;

const ANIMATION_TICK: f32 = 0.5;

#[derive(Default)]
struct Game {
    tah: Handle<TextureAtlas>
}

#[derive(Component)]
struct Cursor {
    visible: bool,
    flash: bool,
}

impl Cursor {
    fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        commands: &mut Commands,
    ) {
       let c = Cursor{
            visible: true,
            flash: true,
        };
        commands.spawn_bundle(get_anim(texture_atlas_handle, Vec2::splat(5.0), 165))
        .insert(AnimationTimer(Timer::from_seconds(ANIMATION_TICK/2.0, true)))
        .insert(c);
    }
}



#[derive(Component)]
struct Position { x: u8, y: u8 }
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Mortal {
    is_alive: bool
}

#[derive(Component)]
struct RepeatAnimation {
    max: usize,
    init: usize,
}


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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

fn get_anim(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
) -> SpriteSheetBundle {
    let actual_v = v.mul_add(Vec2::splat(SPRITE_SIZE as f32 * SCALE), Vec2::new(-(SCREEN_WIDTH/2.0-HALF_SPRITE), -(SCREEN_HEIGHT/2.0-HALF_SPRITE)));
    let mut sprite = TextureAtlasSprite::new(init);
    sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    return SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_xyz(actual_v.x, actual_v.y, 0.0).with_scale(Vec3::splat(SCALE)),
        sprite: sprite,
        ..default()
    };
}

fn get_border(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>
) {
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(0.0, 0.0), BORDER_BOTTOMLEFT));
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(0.0, (HEIGHT-1) as f32), BORDER_TOPLEFT));
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, 0.0), BORDER_BOTTOMRIGHT));
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, (HEIGHT-1) as f32), BORDER_TOPRIGHT));
    for n in 1..HEIGHT-1 {
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(0.0, n as f32), BORDER_LEFT));
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, n as f32), BORDER_RIGHT));
    }
    for n in 1..WIDTH-1 {
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(n as f32, 0.0), BORDER_BOTTOM));
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(n as f32, (HEIGHT-1) as f32), BORDER_TOP));
    }
}

fn spawn_anim(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    num: usize
) -> Entity {
    return commands
        .spawn_bundle(get_anim(texture_atlas_handle, v, init))
        .insert(AnimationTimer(Timer::from_seconds(ANIMATION_TICK, true)))
        .insert(RepeatAnimation {max: init+num-1, init: init}).id();
}

#[derive(Default)]
struct AtlasHandle(Handle<TextureAtlas>);

fn setup_initial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32), 10, 41);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(Game{tah: texture_atlas_handle});
}
/* 
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas: ResMut<AtlasHandle>,
) {

    
    get_border(&mut commands, atlas.clone());
    spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(2.0), 120, 8);
    spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(1.0), 180, 4);
    let creature = spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(3.0), 210, 4);
    commands.entity(creature).insert(Mortal{is_alive: false});

    let creature_map = load_creatures();
    creature_map.get("Pegasus").unwrap().to_entity(Vec2::splat(4.0), &mut commands, texture_atlas_handle.clone());

    Cursor::new(texture_atlas_handle.clone(), &mut commands);
}*/

fn load_creatures() -> HashMap<String, Creature> {
    let f = File::open("assets/creatures.ron").unwrap();
    return ron::de::from_reader(f).unwrap();
}

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Game,
}

fn main() {
    println!("WINDOW SIZE IS {} x {}", SCALE*((SPRITE_SIZE*WIDTH) as f32), SCALE*((SPRITE_SIZE*HEIGHT) as f32));


    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)
        .add_startup_system(setup_initial)
        /* .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(animate_cursor)
        .add_system(keyboard_input)*/
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

mod menu {
    use bevy::prelude::*;

    use super::{GameState, despawn_screen, Game, get_border};

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
                .add_system_set(SystemSet::on_update(GameState::Menu).with_system(keyboard_input_system))

                .add_system_set(
                    SystemSet::on_exit(GameState::Menu)
                        .with_system(despawn_screen::<OnMenuScreen>),
                );
        }
    }

     // Tag component used to tag entities added on the menu screen
     #[derive(Component)]
     struct OnMenuScreen;

    fn menu_setup(mut commands: Commands, g: Res<Game>) {
        get_border(&mut commands, g.tah.clone());
    }

    fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    
        if keyboard_input.just_pressed(KeyCode::A) {
            info!("'A' just pressed");
            state.set(GameState::Game).unwrap();
        }
    }
}

mod game {
    use bevy::{prelude::*, math::vec3};

    pub struct GamePlugin;

    use super::{GameState, despawn_screen, spawn_anim, Game, AnimationTimer, RepeatAnimation, Mortal, Cursor, SCALED_SPRITE_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, HALF_SPRITE};

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app
                .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
                .add_system_set(
                    SystemSet::on_update(GameState::Game)
                        .with_system(animate_sprite)
                        .with_system(animate_cursor)
                        .with_system(keyboard_input)
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

    fn game_setup(mut commands: Commands, g: Res<Game>) {
        spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(2.0), 120, 8);
        spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(1.0), 180, 4);
        Cursor::new(g.tah.clone(), &mut commands);
    }

    fn animate_sprite(
        time: Res<Time>,
        mut query: Query<(
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut RepeatAnimation,
            Option<&Mortal>,
        )>,
    ) {
        for (mut timer, mut sprite, repeater, mortal) in &mut query {
            timer.tick(time.delta());
            if timer.just_finished() {
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

    fn animate_cursor(
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
    
    fn keyboard_input(
        keys: Res<Input<KeyCode>>,
        mut query: Query<(
            &mut Transform,
            &mut Cursor,
        )>,
    ) {
        for (mut transform, _cursor) in &mut query {
            let mut new_cursor_position = transform.translation;
            if keys.just_pressed(KeyCode::Left) {
                new_cursor_position = transform.translation - vec3(SCALED_SPRITE_SIZE, 0.0, 0.0);
            }
            if keys.just_pressed(KeyCode::Right) {
                new_cursor_position = transform.translation + vec3(SCALED_SPRITE_SIZE, 0.0, 0.0);
            }
            if keys.just_pressed(KeyCode::Up) {
                new_cursor_position = transform.translation + vec3(0.0, SCALED_SPRITE_SIZE, 0.0);
            }
            if keys.just_pressed(KeyCode::Down) {
                new_cursor_position = transform.translation - vec3(0.0, SCALED_SPRITE_SIZE, 0.0);
            }
            transform.translation.x = new_cursor_position.x.clamp(-SCREEN_WIDTH/2.0+HALF_SPRITE+SCALED_SPRITE_SIZE, SCREEN_WIDTH/2.0-HALF_SPRITE-SCALED_SPRITE_SIZE);
            transform.translation.y = new_cursor_position.y.clamp(-SCREEN_HEIGHT/2.0+HALF_SPRITE+SCALED_SPRITE_SIZE, SCREEN_HEIGHT/2.0-HALF_SPRITE-SCALED_SPRITE_SIZE);
        }
    }
}