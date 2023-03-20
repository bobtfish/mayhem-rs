use bevy::{prelude::*, math::{vec3, vec2}};

use crate::{game::Game, vec::Vec2I};
use crate::constants::*;
const WIZARD_IDX: usize = 170;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_event::<BottomTextEvent>()
        .add_system(manage_text_bottom)
        .add_system(animate_sprite)

        .add_event::<StartExplosion>()
        .add_event::<FinishedExplosion>()
        .add_system(start_explosion)
        .add_system(animate_explosion)
        ;
    }
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(vec3(1.0/(SCALE*SPRITE_SIZE as f32), 1.0/(SCALE*SPRITE_SIZE as f32), 1.0))
            .with_translation(vec3((WIDTH/2) as f32-1.0, (HEIGHT/2) as f32-2.0, CAMERA_Z)),
        ..default()
    });
}

pub fn get_border(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>
) {
    commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(-0.5, -0.5), BORDER_BOTTOMLEFT, BLUE));
    commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(-0.5, (HEIGHT-1) as f32-1.5), BORDER_TOPLEFT, BLUE));
    commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH) as f32-1.5, -0.5), BORDER_BOTTOMRIGHT, BLUE));
    commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH) as f32-1.5, HEIGHT as f32-2.5), BORDER_TOPRIGHT, BLUE));
    for n in 2..HEIGHT-1 {
        commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(-0.5, n as f32-1.5), BORDER_LEFT, BLUE));
        commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new((WIDTH) as f32-1.5, n as f32-1.5), BORDER_RIGHT, BLUE));
    }
    for n in 1..WIDTH-1 {
        commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(n as f32-0.5, -0.5), BORDER_BOTTOM, BLUE));
        commands.spawn(get_sprite_sheet_bundle(texture_atlas_handle.clone(), Vec2::new(n as f32-0.5, HEIGHT as f32-2.5), BORDER_TOP, BLUE));
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

/* 
pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgba(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0)
}
*/

pub const WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
pub const BLUE: Color = Color::rgba(0.0, 0.0, 1.0, 1.0);
pub const GREEN: Color = Color::rgba(0.0, 1.0, 0.0, 1.0);
pub const RED: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);
pub const AQUA: Color = Color::rgba(0.0, 1.0, 1.0, 1.0);
pub const YELLOW: Color = Color::rgba(1.0, 1.0, 0.0, 1.0);
pub const PURPLE: Color = Color::rgba(1.0, 0.0, 1.0, 1.0);
pub const MIDYELLOW: Color = Color::rgba(204.0/255.0, 204.0/255.0, 0.0, 1.0);
pub const GREY: Color = Color::rgba(204.0/255.0, 204.0/255.0, 204.0/255.0, 1.0);

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


pub fn print_text(str: &str, commands: &mut Commands, fah: Handle<TextureAtlas>, v: Vec2, color: Color, component: impl Component + std::marker::Copy) -> Vec<Entity> {
    let mut entities = Vec::new();
    for (i,ch) in str.chars().enumerate() {
        let mut new_v = v;
        new_v.x += i as f32/2.0;
        let new = commands.spawn(get_sprite_sheet_bundle(fah.clone(), new_v, char_to_pos(ch), color))
        .insert(component).id();
        trace!("print_text spawned entity {new:?} for char {ch}");
        entities.push(new);
    }
    entities
}

#[derive(Default, PartialEq, Eq)]
pub enum BottomTextState {
    #[default]
    Cleared,
    Set
}

#[derive(Component, Copy, Clone)]
pub struct BottomText;

#[derive(Debug, Deref)]
pub struct BottomTextEvent(Option<String>);
impl BottomTextEvent {
    pub fn from(s: &str) -> Self {
        debug!("MAKE BOTTOM TEXT {}", s);
        Self(Some(String::from(s)))
    }
    pub fn clear() -> Self {
        debug!("Generate clear event for bottom text");
        Self(None)
    }
}

pub fn manage_text_bottom(
    mut commands: Commands,
    game: Res<Game>,
    mut ev_text: EventReader<BottomTextEvent>,
    to_despawn: Query<Entity, With<BottomText>>,
    mut state: Local<BottomTextState>,
) {
    for ev in ev_text.iter() {
        debug!("Bottom text event {:#?}", ev);
        if *state == BottomTextState::Cleared {
            debug!("Skipping clearing bottom text state as already cleared");
        } else {
            debug!("DESPAWN BOTTOM TEXT");
            for entity in &to_despawn {
                commands.entity(entity).despawn_recursive();
            }
            *state = BottomTextState::Cleared;
        }
        if ev.is_some() {
            debug!("PRINT NEW BOTTOM TEXT {}", ev.as_ref().unwrap());
            print_text(ev.as_ref().unwrap(), &mut commands, game.fah(), vec2(0.0, -1.5), WHITE, BottomText);
            *state = BottomTextState::Set;
        }
    }
}

pub fn print_wizard(commands: &mut Commands, tah: Handle<TextureAtlas>, v: Vec2, idx: usize, color: Color, component: impl Component + std::marker::Copy) {
    commands.spawn(get_sprite_sheet_bundle(tah, v, WIZARD_IDX + idx, color))
    .insert(component);
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

impl RepeatAnimation {
    pub fn new(init: usize, num: usize) -> Self {
        Self {
            max: init+num-1,
            init,
            timer: Timer::from_seconds(ANIMATION_TICK, TimerMode::Repeating),
        }
    }
}

pub fn spawn_anim(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    num: usize,
    color: Color
) -> Entity {
    return commands
        .spawn(get_sprite_sheet_bundle(texture_atlas_handle, v, init, color))
        .insert(RepeatAnimation::new(init, num)).id();
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
                // This if for cases where we just added a completely new repeat animation which doesn't
                // share the same base sprite as we had before. E.g. player getting a magic bow.
                if index < repeater.init {
                    index = repeater.init;
                }
                sprite.index = index;
            } else {
                sprite.index = repeater.max + 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Explosion {
    max: usize,
    timer: Timer,
}

impl Explosion {
    pub fn new(init: usize, num: usize) -> Self {
        Self {
            max: init+num-1,
            timer: Timer::from_seconds(ANIMATION_TICK, TimerMode::Repeating),
        }
    }
}

#[derive(Debug)]
pub struct StartExplosion {
    pub at: Vec2I,
    pub idx: usize,
}

#[derive(Debug)]
pub struct FinishedExplosion();

pub fn start_explosion(
    game: Res<Game>,
    mut commands: Commands,
    mut ev: EventReader<StartExplosion>,
) {
    for e in ev.iter() {
        let color = Color::rgba(1.0, 1.0, 1.0, 1.0);

        let sprite_index = 130 + 10 * e.idx;
        info!("Spawn animation at {:?}", e.at);
        commands
            .spawn(get_sprite_sheet_bundle_z(game.tah(), Into::into(e.at), sprite_index, color, 2.0))
            .insert(Explosion::new(sprite_index, 8));
    }
}


pub fn animate_explosion(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut Explosion,
    )>,
    mut commands: Commands,
    mut ev: EventWriter<FinishedExplosion>,
) {
    for (e, mut sprite, mut repeater) in &mut query {
        repeater.timer.tick(time.delta());
        if repeater.timer.just_finished() {
                let index = sprite.index + 1;
                if index > repeater.max {
                    commands.entity(e).despawn();
                    ev.send(FinishedExplosion());
                } else {
                    sprite.index = index;
                }
        }
    }
}