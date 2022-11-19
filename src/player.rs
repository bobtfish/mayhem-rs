use bevy::prelude::*;
use crate::display;
use crate::spell::{AllSpells, SpellBox, ASpell};
use crate::system::BoardEntity;
use rand::prelude::SliceRandom;

pub struct SpellList {
    pub spells: Vec<Box<dyn ASpell + Sync + Send>>,
    pub chosen_spell: Option<usize>,
    pub illusion: bool,
}
impl SpellList {
    pub fn set_chosen(&mut self, idx: usize) {
        self.chosen_spell = Some(idx);
    }
    pub fn get_chosen_spell(
        &self
    ) -> Option<&dyn ASpell> {
        self.chosen_spell?;
        Some(self.get_spell(self.chosen_spell.unwrap()))
    }
    pub fn get_spell(&self, idx: usize) -> &dyn ASpell {
        &*self.spells[idx]
    }
    pub fn len(&self) -> usize {
        self.spells.len()
    }
    pub fn pop_chosen_spell(&mut self) -> Box<dyn ASpell> {
        let idx = self.chosen_spell.unwrap();
        self.chosen_spell = None;
        if !self.spells[idx].reusable() {
            return self.spells.remove(idx);
        }
        return self.spells[idx].clone();
    }
}
pub struct Player {
    pub name: String,
    pub computer_controlled: bool,
    pub character_icon: u8,
    pub color: u8,
    pub spells: SpellList,
    pub pos: Vec2,
    pub handle: Option<Entity>,
    pub creations: Vec<Entity>,
}

pub enum CastFailed {
    OutOfRange
}

impl Player {
    pub fn new(name: String, cc: bool, icon: u8, color: u8) -> Self {
        Self {
            name,
            computer_controlled: cc,
            character_icon: icon,
            color,
            spells: SpellList {
                spells: Vec::new(),
                chosen_spell: None,
                illusion: false,
            },
            pos: Vec2{x: 0.0, y: 0.0},
            handle: None,
            creations: Vec::new(),
        }
    }
    pub fn pick_spells(&mut self, allspells: &AllSpells) {
        let mut sample: Vec<SpellBox> = Vec::new();
        for spell in allspells[1..].choose_multiple(&mut rand::thread_rng(), 13) {
            sample.push((*spell).clone());
        }
        sample.insert(0, allspells[0].clone());
        self.spells.spells = sample;

    }
    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        tah: Handle<TextureAtlas>
    ) {
        let mut ss = display::get_sprite_sheet_bundle(tah, self.pos, (169 + self.character_icon) as usize, display::WHITE);
        ss.visibility.is_visible = false;
        self.handle = Some(
            commands.spawn(ss)
            .insert(BoardEntity)
            .id()
        );
    }
    pub fn cast(
        &mut self,
        pos: Vec2,
        commands: &mut Commands,
        tah: Handle<TextureAtlas>
    ) -> Result<Option<Entity>, CastFailed> {
        let range = self.spells.get_chosen_spell().unwrap().cast_range();
        let dist = (pos - self.pos).length().floor();
        println!("RANGE IS {range} DIST IS {dist}");
        if dist > f32::from(range) {
            println!("Return too far");
            return Err(CastFailed::OutOfRange);
        }
        let spell = self.spells.pop_chosen_spell();
        let e = spell.cast(pos, commands, tah);
        if let Some(entity) = e {
            self.creations.push(entity);
        }
        Ok(e)
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