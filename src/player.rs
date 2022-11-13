use bevy::prelude::*;
use crate::display;
use crate::spell::{AllSpells, SpellBox, ASpell};
use rand::prelude::SliceRandom;

pub struct Player {
    pub name: String,
    pub computer_controlled: bool,
    pub character_icon: u8,
    pub color: u8,
    pub chosen_spell: Option<usize>,
    pub spells: Vec<Box<dyn ASpell + Sync + Send>>,
    pub x: f32,
    pub y: f32,
    pub handle: Option<Entity>
}

impl Player {
    pub fn pick_spells(&mut self, allspells: &AllSpells) {
        let mut sample: Vec<SpellBox> = Vec::new();
        for spell in allspells[1..].choose_multiple(&mut rand::thread_rng(), 13) {
            sample.push((*spell).clone());
        }
        sample.insert(0, allspells[0].clone());
        self.spells = sample;

    }
    pub fn get_chosen_spell(
        &self
    ) -> Option<&dyn ASpell> {
        self.chosen_spell?;
        Some(&*self.spells[self.chosen_spell.unwrap()])
    }
    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        tah: Handle<TextureAtlas>
    ) {
        self.handle = Some(commands.spawn(display::get_sprite_sheet_bundle(tah, Vec2::new(self.x, self.y), (169 + self.character_icon) as usize, display::WHITE)).id());
    }
}

pub fn get_start_positions(num: usize) -> Result<Vec<Vec2>, &'static str> {
    match num {
        2 => Ok(vec![
            Vec2::new(1.0, 5.0),
            Vec2::new(13.0, 5.0),
        ]),
        3 => Ok(vec![
            Vec2::new(7.0, 8.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(13.0, 1.0),
        ]),
        4 => Ok(vec![
            Vec2::new(1.0, 8.0),
            Vec2::new(13.0, 8.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(13.0, 1.0),
        ]),
        5 => Ok(vec![
            Vec2::new(7.0, 9.0),
            Vec2::new(0.0, 6.0),
            Vec2::new(14.0, 6.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(11.0, 0.0),
        ]),
        6 => Ok(vec![
            Vec2::new(7.0, 9.0),
            Vec2::new(0.0, 8.0),
            Vec2::new(14.0, 8.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(7.0, 0.0),
            Vec2::new(14.0, 1.0),
        ]),
        7 => Ok(vec![
            Vec2::new(7.0, 9.0),
            Vec2::new(1.0, 8.0),
            Vec2::new(13.0, 8.0),
            Vec2::new(0.0, 3.0),
            Vec2::new(14.0, 4.0),
            Vec2::new(4.0, 0.0),
            Vec2::new(10.0, 0.0),
        ]),
        8 => Ok(vec![
            Vec2::new(0.0, 9.0),
            Vec2::new(7.0, 9.0),
            Vec2::new(14.0, 9.0),
            Vec2::new(0.0, 5.0),
            Vec2::new(14.0, 5.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(7.0, 0.0),
            Vec2::new(14.0, 0.0),
        ]),
        _ => Err("invalid number of players"),
    }
}