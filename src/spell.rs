use bevy::prelude::*;
use crate::{creature::load_creatures, display::{WHITE, GREEN, AQUA, YELLOW, FUCHSIA}};

#[derive(Resource, Deref)]
pub struct AllSpells(Vec<SpellBox>);

pub type SpellBox = Box<dyn ASpell + Sync + Send>;

pub trait ASpell {
    fn name(&self) -> String;
    fn clone(&self) -> SpellBox;
    fn cast(&self, illusion: bool, pos: Vec2, commands: &mut Commands, tah: Handle<TextureAtlas>) -> Option<Entity>;
    fn reusable(&self) -> bool {
        false
    }
    fn cast_range(&self) -> u8;
    fn can_be_illusion(&self) -> bool {
        false
    }
    fn law_rating(&self) -> i8;
    fn get_sep(&self) -> &str {
        let law_rating = self.law_rating();
        if law_rating == 0 {
            return "-";
        }
        if law_rating < 0 {
            return "*";
        }
        "^"
    }
    fn casting_chance(&self) -> u8;
    fn casting_chance_color(&self) -> Color {
        let chance = self.casting_chance();
        if chance >= 100 {
            return WHITE;
        }
        if chance >= 80 {
            return YELLOW;
        }
        if chance >= 60 {
            return AQUA;
        }
        if chance >= 40 {
            return GREEN;
        }
        FUCHSIA
    }
    fn get_description(&self) -> Vec<String>;
}

#[derive(Default, Clone)]
pub struct Spell {
    pub name: String,
    pub law_rating: i8,
    pub reusable: bool,
    pub casting_chance: u8,
    pub cast_range: u8,
    pub tries: u8,
    pub no_line_of_sight_needed: bool,
}

impl ASpell for Spell {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn law_rating(&self) -> i8 {
        self.law_rating
    }
    fn clone(&self) -> SpellBox {
        Box::new(std::clone::Clone::clone(self))
    }
    fn cast(&self, _illusion: bool, _pos: Vec2, _commands: &mut Commands, _tah: Handle<TextureAtlas>) -> Option<Entity> {
        None
    }
    fn reusable(&self) -> bool {
        self.reusable
    }
    fn cast_range(&self) -> u8 {
        self.cast_range
    }
    fn casting_chance(&self) -> u8 {
        self.casting_chance
    }
    fn get_description(&self) -> Vec<String> {
        Vec::new()
    }
}

pub fn load_all_spells() -> AllSpells {
    let mut spells: Vec<SpellBox> = vec![
        Box::new(Spell {
            name: "Disbelieve".to_string(),
            reusable: true,
            casting_chance: 100,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Raise Dead".to_string(),
            law_rating: -1,
            casting_chance: 60,
            cast_range: 4,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Subversion".to_string(),
            cast_range: 7,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Vengence".to_string(),
            casting_chance: 80,
            cast_range: 20,
            no_line_of_sight_needed: true,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Decree".to_string(),
            casting_chance: 80,
            cast_range: 20,
            law_rating: 1,
            no_line_of_sight_needed: true,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Dark Power".to_string(),
            casting_chance: 50,
            cast_range: 20,
            law_rating: -2,
            tries: 3,
            no_line_of_sight_needed: true,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Justice".to_string(),
            casting_chance: 50,
            cast_range: 20,
            law_rating: 2,
            tries: 3,
            no_line_of_sight_needed: true,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Law-1".to_string(),
            casting_chance: 100,
            law_rating: 2,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Law-2".to_string(),
            casting_chance: 100,
            law_rating: 4,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Chaos-1".to_string(),
            casting_chance: 100,
            law_rating: -2,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Chaos-2".to_string(),
            casting_chance: 100,
            law_rating: -4,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Lightning".to_string(),
            casting_chance: 100,
            cast_range: 4,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Magic Bolt".to_string(),
            casting_chance: 100,
            cast_range: 6,
            ..Default::default()
        }),
        Box::new(Spell {
            name: "Magic Wood".to_string(),
            casting_chance: 80,
            law_rating: 1,
            ..Default::default()
        })
    ];
    let creature_map = load_creatures();
    for (_, c) in creature_map {
        spells.push(c.to_spell());
    }
    AllSpells(spells)
}