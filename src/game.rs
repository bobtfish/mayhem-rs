use bevy::prelude::*;
use crate::player::Player;
use crate::cursor::Cursor;
use crate::board::GameBoard;
use crate::constants::*;

#[derive(Default, Resource)]
pub struct Game {
    tah: Handle<TextureAtlas>,
    fah: Handle<TextureAtlas>,
    pub cursor: Cursor,
    pub players: u8,
    pub ai_level: u8,
    pub player_info: Vec<Player>,
    pub player_turn: u8,
    pub board: Option<GameBoard>,
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
    pub fn board(&self) -> &GameBoard {
        self.board.as_ref().unwrap()
    }
    pub fn board_mut(&mut self) -> &mut GameBoard {
        self.board.as_mut().unwrap()
    }
    pub fn spawn_players(&mut self, commands: &mut Commands, tah: Handle<TextureAtlas>) {
        for p in &mut self.player_info {
            p.spawn(commands, tah.clone(), self.board.as_mut().unwrap());
        }
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
    game.board = Some(GameBoard::new());
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_startup_system(setup_game);
    }
}
