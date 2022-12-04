use bevy::prelude::*;
use crate::board::MoveableComponent;
use crate::display;
use crate::spell::{AllSpells, SpellBox, ASpell};
use crate::system::{BoardEntity, Named, BelongsToPlayer};
use rand::prelude::SliceRandom;
use rand::Rng;

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
    pub color: Color,
    pub spells: SpellList,
    pub handle: Option<Entity>,
    pub creations: Vec<Entity>,
    pub law_chaos: i8,
    defence: u8,
    combat: u8,
    manoeuvre: u8,
    magic_resistance: u8,
}

pub enum CastFailed {
    OutOfRange,
    NotThere,
}

impl Player {
    pub fn new(name: String, cc: bool, icon: u8, color: Color) -> Self {
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
            handle: None,
            creations: Vec::new(),
            law_chaos: 0,
            defence: rand::thread_rng().gen_range(1..6),          // 1-5
            combat: rand::thread_rng().gen_range(1..6),           // 1-5
            manoeuvre: rand::thread_rng().gen_range(3..8),        // 3-7
            magic_resistance: rand::thread_rng().gen_range(6..9), // 6-8
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
        tah: Handle<TextureAtlas>,
        pos: Vec2,
    ) {
        let mut ss = display::get_sprite_sheet_bundle(tah, pos, (169 + self.character_icon) as usize, self.color);
        ss.visibility.is_visible = false;
        let entity = commands.spawn(ss)
            .insert(MoveableComponent{
                movement: 1,
                flying: false,
            })
            .insert(BoardEntity)
            .insert(Named{ name: self.name.clone() })
            .id();
        println!("Add entity {:?}", entity);
        self.handle = Some(entity);
    }
    pub fn cast(
        &mut self,
        from: Vec2,
        to: Vec2,
        commands: &mut Commands,
        tah: Handle<TextureAtlas>
    ) -> Result<Option<Entity>, CastFailed> {
        let range = self.spells.get_chosen_spell().unwrap().cast_range();
        let dist = (to - from).length().floor();
        println!("RANGE IS {range} DIST IS {dist}");
        if dist > f32::from(range) {
            println!("Return too far");
            return Err(CastFailed::OutOfRange);
        }
        let spell = self.spells.pop_chosen_spell();
        let e = spell.cast(self.spells.illusion, self, to, commands, tah);
        if let Some(entity) = e {
            commands.get_entity(e.unwrap()).unwrap()
                .insert(BelongsToPlayer{player_entity: self.handle.unwrap()});
            self.creations.push(entity);
        }
        Ok(e)
    }
    pub fn get_chosen_spell_name(&self) -> Option<String> {
        let spell = self.spells.get_chosen_spell();
        spell?;
        let spell = spell.unwrap();
        let mut text = self.name.clone();
        text.push(' ');
        text.push_str(&spell.name());
        Some(text)
    }
}


#[derive(Clone)]
pub struct PlayerSpell {
    pub name: String,
    pub law_rating: i8,
    pub casting_chance: u8,
    pub imp: fn(&Player, &mut Commands),
}

impl ASpell for PlayerSpell {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn law_rating(&self) -> i8 {
        self.law_rating
    }
    fn clone(&self) -> SpellBox {
        Box::new(std::clone::Clone::clone(self))
    }
    fn cast(&self, _illusion: bool, player: &Player, _pos: Vec2, commands: &mut Commands, _tah: Handle<TextureAtlas>) -> Option<Entity> {
        let f = self.imp;
        f(player, commands);
        None
    }
    fn reusable(&self) -> bool {
        false
    }
    fn cast_range(&self) -> u8 {
        0
    }
    fn casting_chance(&self) -> u8 {
        self.casting_chance
    }
    fn get_description(&self) -> Vec<String> {
        Vec::new()
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