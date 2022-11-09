use bevy::prelude::*;
use super::{AllSpells, Spell, get_sprite_sheet_bundle};
use rand::prelude::SliceRandom;

pub struct Player {
    pub name: String,
    pub computer_controlled: bool,
    pub character_icon: u8,
    pub color: u8,
    pub chosen_spell: Option<usize>,
    pub spells: Vec<Spell>,
    pub x: f32,
    pub y: f32,
    pub handle: Option<Entity>
}

impl Player {
    pub fn pick_spells(&mut self, allspells: &AllSpells) {
        let mut sample: Vec<_> = allspells.0[1..].choose_multiple(&mut rand::thread_rng(), 13)
        .cloned().collect();
        sample.insert(0, allspells.0[0].clone());
        self.spells = sample;

    }
    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        tah: Handle<TextureAtlas>
    ) {
        self.handle = Some(commands.spawn_bundle(get_sprite_sheet_bundle(tah, Vec2::new(self.x, self.y), (169 + self.character_icon) as usize)).id());
    }
}

pub fn get_start_positions(num: usize) -> Result<Vec<Vec2>, &'static str> {
    match num {
        2 => Ok(vec![
            Vec2::new(1.0, 6.0),
            Vec2::new(13.0, 6.0),
        ]),
        3 => Ok(vec![
            Vec2::new(7.0, 9.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(13.0, 2.0),
        ]),
        4 => Ok(vec![
            Vec2::new(1.0, 9.0),
            Vec2::new(13.0, 9.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(13.0, 2.0),
        ]),
        5 => Ok(vec![
            Vec2::new(7.0, 15.0),
            Vec2::new(0.0, 7.0),
            Vec2::new(14.0, 7.0),
            Vec2::new(3.0, 1.0),
            Vec2::new(11.0, 1.0),
        ]),
        6 => Ok(vec![
            Vec2::new(7.0, 10.0),
            Vec2::new(0.0, 9.0),
            Vec2::new(14.0, 9.0),
            Vec2::new(0.0, 2.0),
            Vec2::new(7.0, 1.0),
            Vec2::new(14.0, 2.0),
        ]),
        7 => Ok(vec![
            Vec2::new(7.0, 10.0),
            Vec2::new(1.0, 9.0),
            Vec2::new(13.0, 9.0),
            Vec2::new(0.0, 4.0),
            Vec2::new(14.0, 5.0),
            Vec2::new(4.0, 1.0),
            Vec2::new(10.0, 1.0),
        ]),
        8 => Ok(vec![
            Vec2::new(0.0, 10.0),
            Vec2::new(7.0, 10.0),
            Vec2::new(14.0, 10.0),
            Vec2::new(0.0, 6.0),
            Vec2::new(14.0, 6.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(7.0, 1.0),
            Vec2::new(14.0, 1.0),
        ]),
        _ => Err("invalid number of players"),
    }
}