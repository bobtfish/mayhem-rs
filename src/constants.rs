pub const SCALE: f32 = 4.0;

pub const SPRITE_SIZE: usize = 16;

pub const HEIGHT: usize = 12;
pub const WIDTH: usize = 16;

pub const SCREEN_WIDTH: f32 = SCALE*((SPRITE_SIZE*WIDTH) as f32);
pub const SCREEN_HEIGHT: f32 = SCALE*((SPRITE_SIZE*HEIGHT) as f32);

pub const BORDER_TOPLEFT: usize = 202;
pub const BORDER_TOP: usize = 203;
pub const BORDER_TOPRIGHT: usize = 204;
pub const BORDER_LEFT: usize = 205;
pub const BORDER_BOTTOMLEFT: usize = 206;
pub const BORDER_BOTTOM: usize = 207;
pub const BORDER_BOTTOMRIGHT: usize = 208;
pub const BORDER_RIGHT: usize = 209;

pub const ANIMATION_TICK: f32 = 0.5;

pub const CAMERA_Z: f32 = 10.0;
pub const CURSOR_Z: f32 = 9.0;
