use bevy::{prelude::*, render::texture::ImageSettings};

#[derive(Component)]
struct Position { x: u8, y: u8 }
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Alive;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Bundle)]
struct IconBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

#[derive(Component)]
struct RepeatAnimation {
    max: usize,
    init: usize,
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut RepeatAnimation,
    )>,
) {
    for (mut timer, mut sprite, repeater) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut index = sprite.index + 1;
            if index > repeater.max {
                index = repeater.init;
            }
            sprite.index = index;
        }
    }
}

fn spawn_anim(
    mut commands: Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    num: usize
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(v.x, v.y, 0.0).with_scale(Vec3::splat(6.0)),
            sprite: TextureAtlasSprite::new(init),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(RepeatAnimation {max: init+num, init: init});
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 10, 41);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle::default());
    
    spawn_anim(commands, texture_atlas_handle, Vec2::splat(0.0), 120, 8);
}


fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}