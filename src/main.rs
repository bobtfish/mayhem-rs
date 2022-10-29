use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode, ecs::system::EntityCommands};

const SCALE: f32 = 4.0;

const SPRITE_SIZE: usize = 16;

const HEIGHT: usize = 12;
const WIDTH: usize = 16;

const SCREEN_WIDTH: f32 = SCALE*((SPRITE_SIZE*WIDTH) as f32);
const SCREEN_HEIGHT: f32 = SCALE*((SPRITE_SIZE*HEIGHT) as f32);

const HALF_SPRITE: f32 =  (SPRITE_SIZE/2) as f32 * SCALE;

const BORDER_TOPLEFT: usize = 202;
const BORDER_TOP: usize = 203;
const BORDER_TOPRIGHT: usize = 204;
const BORDER_LEFT: usize = 205;
const BORDER_BOTTOMLEFT: usize = 206;
const BORDER_BOTTOM: usize = 207;
const BORDER_BOTTOMRIGHT: usize = 208;
const BORDER_RIGHT: usize = 209;

const ANIMATION_TICK: f32 = 0.5;

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

fn get_anim(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
) -> SpriteSheetBundle {
    let t = Transform::from_scale(Vec3::splat(SCALE));
    // = Vec2::new((v.x * SPRITE_SIZE*SCALE)-6.0 * SPRITE_SIZE*SCALE, (v.y * SPRITE_SIZE*SCALE)-4.0 * SPRITE_SIZE * SCALE);
    let actual_v = v.mul_add(Vec2::splat(SPRITE_SIZE as f32 * SCALE), Vec2::new(-(SCREEN_WIDTH/2.0-HALF_SPRITE), -(SCREEN_HEIGHT/2.0-HALF_SPRITE)));
    return SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_xyz(actual_v.x, actual_v.y, 0.0).with_scale(Vec3::splat(SCALE)),
        sprite: TextureAtlasSprite::new(init),
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


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32), 10, 41);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());
    
    get_border(&mut commands, texture_atlas_handle.clone());
    spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(2.0), 120, 8);
    spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(1.0), 180, 4);
    let creature = spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(3.0), 210, 4);
    commands.entity(creature).insert(Mortal{is_alive: false});
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
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}