use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use crate::display::spawn_anim;
use crate::spell::{ASpell, SpellBox};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    sprite_index: usize,
}

impl Creature {
    fn to_entity(
        &self,
        v: Vec2,
        commands: &mut Commands,
        texture_atlas_handle: Handle<TextureAtlas>
    ) -> Entity {
        spawn_anim(commands, texture_atlas_handle, v, self.sprite_index, 4)
    }
    pub fn to_spell(&self) -> SpellBox {
        Box::new(CreatureSpell{creature: self.clone()})
    }
}

#[derive(Clone)]
pub struct CreatureSpell {
    creature: Creature
}

impl ASpell for CreatureSpell {
    fn name(&self) -> String {
        self.creature.name.clone()
    }
    fn get_sep(&self) -> &str {
        "-"
    }
    fn clone(&self) -> SpellBox {
        Box::new(std::clone::Clone::clone(self))
    }
    fn cast(&self, pos: Vec2, commands: &mut Commands, tah: Handle<TextureAtlas>) {
        self.creature.to_entity(pos, commands, tah);
    }
    fn reusable(&self) -> bool {
        false
    }
}

pub fn load_creatures() -> HashMap<String, Creature> {
    let f = File::open("assets/creatures.ron").unwrap();
    ron::de::from_reader(f).unwrap()
}
