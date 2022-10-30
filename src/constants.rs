pub const SCALE: f32 = 4.0;

pub const SPRITE_SIZE: usize = 16;

pub const SCALED_SPRITE_SIZE: f32 =  SPRITE_SIZE as f32 * SCALE;

pub const HEIGHT: usize = 12;
pub const WIDTH: usize = 16;

pub const SCREEN_WIDTH: f32 = SCALE*((SPRITE_SIZE*WIDTH) as f32);
pub const SCREEN_HEIGHT: f32 = SCALE*((SPRITE_SIZE*HEIGHT) as f32);

pub const HALF_SPRITE: f32 =  SCALED_SPRITE_SIZE/2.0;

pub const BORDER_TOPLEFT: usize = 202;
pub const BORDER_TOP: usize = 203;
pub const BORDER_TOPRIGHT: usize = 204;
pub const BORDER_LEFT: usize = 205;
pub const BORDER_BOTTOMLEFT: usize = 206;
pub const BORDER_BOTTOM: usize = 207;
pub const BORDER_BOTTOMRIGHT: usize = 208;
pub const BORDER_RIGHT: usize = 209;

pub const ANIMATION_TICK: f32 = 0.5;

