use bevy::prelude::*;
use crate::creature::load_creatures;

#[derive(Resource, Deref)]
pub struct AllSpells(Vec<SpellBox>);

pub type SpellBox = Box<dyn ASpell + Sync + Send>;

pub trait ASpell {
    fn name(&self) -> String;
    fn get_sep(&self) -> &str;
    fn clone(&self) -> SpellBox;
    fn cast(&self, pos: Vec2, commands: &mut Commands, tah: Handle<TextureAtlas>);
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

impl Spell {
    pub const fn get_sep(&self) -> &str {
        "-"
    }
    pub fn cast(&self, pos: Vec2, mut commands: Commands) {

    }
}

impl ASpell for Spell {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_sep(&self) -> &str {
        "-"
    }
    fn clone(&self) -> SpellBox {
        Box::new(std::clone::Clone::clone(self))
    }
    fn cast(&self, pos: Vec2, commands: Commands, tah: Handle<TextureAtlas>) {

    }
}

pub fn load_all_spells() -> AllSpells {
    let mut spells: Vec<SpellBox> = vec![
        Box::new(Spell {name: "Disbelieve".to_string(), ..Default::default()}),
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