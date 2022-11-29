use bevy::prelude::*;
use crate::player::Player;
use crate::cursor::Cursor;
use crate::constants::*;

#[derive(Default, Resource)]
pub struct Game {
    tah: Handle<TextureAtlas>,
    fah: Handle<TextureAtlas>,
    pub players: u8,
    pub ai_level: u8,
    pub player_info: Vec<Player>,
    pub player_turn: u8,
}

impl Game {
    pub fn tah(&self) -> Handle<TextureAtlas> {
        self.tah.clone()
    }
    pub fn fah(&self) -> Handle<TextureAtlas> {
        self.fah.clone()
    }
    pub fn get_player(&self) -> &Player {
        &self.player_info[self.player_turn as usize]
    }
    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player_info[self.player_turn as usize]
    }
}

fn setup_game(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32), 10, 41, None, None);
    game.tah = texture_atlases.add(texture_atlas);
    let font_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new((SPRITE_SIZE/2) as f32, SPRITE_SIZE as f32), 20, 41, None, None);
    game.fah = texture_atlases.add(font_atlas);
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_startup_system(setup_game);
    }
}
