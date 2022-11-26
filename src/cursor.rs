use bevy::math::vec2;
use bevy::{prelude::*};
use super::constants::{ANIMATION_TICK, WIDTH, HEIGHT, CURSOR_Z};
use super::Game;
use crate::display;

const CURSOR_SPRITE_ID: usize = 164;
pub const CURSOR_SPELL: usize = 0;
pub const CURSOR_BOX: usize = 1;
pub const CURSOR_FLY: usize = 2;
//pub const CURSOR_TARGET: usize = 3;


pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CursorMovedEvent>()
            .add_startup_system(cursor_setup.at_end())
            .add_system(keyboard_input)
            .add_system(animate_cursor);
    }
}

#[derive(Deref)]
pub struct CursorMovedEvent(Vec2);

#[derive(Component)]
pub struct CursorEntity;

#[derive(Default)]
pub struct Cursor {
    cursor: usize,
    visible: bool,
    x: f32,
    y: f32,
    flash_timer: Timer,
    moved: bool,
    redraw: bool,
    hide_till_moved: bool,
}

impl Cursor {
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(&mut self) {
        self.visible = true;
        self.redraw = true;
    }
    pub fn set_invisible(&mut self) {
        self.visible = false;
        self.redraw = true;
    }
    pub fn set_type(&mut self, t: usize) {
        self.cursor = t;
        self.redraw = true;
    }
    pub fn set_pos(&mut self, v: Vec2) {
        self.x = v.x;
        self.y = v.y;
        self.redraw = true;
    }
    pub fn get_pos_v(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
    pub fn hide_till_moved(&mut self) {
        println!("SET HIDE TILL MOVED");
        self.hide_till_moved = true;
    }
}

fn cursor_setup(
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    let mut sprite = display::get_sprite_sheet_bundle_z(game.tah(), Vec2::new(0.0, 0.0), CURSOR_SPRITE_ID, display::WHITE, CURSOR_Z);
    sprite.visibility.is_visible = false;
    commands.spawn(sprite).insert(CursorEntity);
    game.cursor = Cursor{
        cursor: CURSOR_BOX,
        visible: false,
        x: 0.0,
        y: 0.0,
        flash_timer: Timer::from_seconds(ANIMATION_TICK/2.0, TimerMode::Repeating),
        moved: false,
        hide_till_moved: false,
        redraw: true,
    };
}

#[allow(clippy::useless_let_if_seq)]
fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut ev_cursor_moved: EventWriter<CursorMovedEvent>,
) {
    let mut cursor = &mut game.cursor;
    if keys.just_pressed(KeyCode::Left) && cursor.x > 0.0 {
        cursor.x -= 1.0;
        cursor.moved = true;
    }
    if keys.just_pressed(KeyCode::Right) && cursor.x < WIDTH as f32 -2.0 {
        cursor.x += 1.0;
        cursor.moved = true;
    }
    if keys.just_pressed(KeyCode::Up) && cursor.y < HEIGHT as f32 -3.0 {
        cursor.y += 1.0;
        cursor.moved = true;
    }
    if keys.just_pressed(KeyCode::Down) && cursor.y > 0.0 {
        cursor.y -= 1.0;
        cursor.moved = true;
    }
    if cursor.moved {
        cursor.hide_till_moved = false;
        println!("SEND cursor moved event");
        ev_cursor_moved.send(CursorMovedEvent(Vec2::new(cursor.x, cursor.y)));
    }
}

fn animate_cursor(
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut Transform, &mut TextureAtlasSprite), With<CursorEntity>>,
) {
    let item = query.single_mut();
    let mut vis = item.0;
    let mut transform = item.1;
    let mut sprite = item.2;
    let mut cursor = &mut game.cursor;
    if cursor.moved || cursor.redraw || cursor.hide_till_moved {
        sprite.index = cursor.cursor + CURSOR_SPRITE_ID;
        if cursor.hide_till_moved {
            vis.is_visible = false;
        } else {
            vis.is_visible = cursor.is_visible();
        }
        *transform = transform.with_translation(vec2(cursor.x, cursor.y).extend(CURSOR_Z));
        cursor.moved = false;
        cursor.redraw = false;
    }
    if !cursor.is_visible() || cursor.hide_till_moved {
        return;
    }
    cursor.flash_timer.tick(time.delta());
    if cursor.flash_timer.just_finished() {
        if vis.is_visible {
            vis.is_visible = false;
        } else {
            vis.is_visible = true;
        }
    }
}
