use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use crate::display::spawn_anim;
use crate::spell::{ASpell, SpellBox};
use crate::system::{BoardEntity, Named};

fn default_as_zero() -> u8 {
    0
}
fn default_as_false() -> bool {
    false
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    sprite_index: usize,
    entity: Option<Entity>,
    #[serde(default = "default_as_zero")]
    movement: u8,
    #[serde(default = "default_as_false")]
    flying: bool,
}

#[derive(Component)]
pub struct CreatureComponent {
    pub is_illusion: bool,
    pub movement: u8,
    pub flying: bool,
}

impl Creature {
    fn to_entity(
        &self,
        illusion: bool,
        v: Vec2,
        commands: &mut Commands,
        texture_atlas_handle: Handle<TextureAtlas>
    ) -> Entity {
        let e = spawn_anim(commands, texture_atlas_handle, v, self.sprite_index, 4);
        commands.get_entity(e).unwrap()
            .insert(CreatureComponent{
                is_illusion: illusion,
                movement: self.movement,
                flying: self.flying,
            })
            .insert(BoardEntity)
            .insert(Named{ name: self.name.clone() });
        e
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
    fn cast(&self, illusion: bool, pos: Vec2, commands: &mut Commands, tah: Handle<TextureAtlas>) -> Option<Entity> {
        Some(self.creature.to_entity(illusion, pos, commands, tah))
    }
    fn reusable(&self) -> bool {
        false
    }
    fn cast_range(&self) -> u8 {
        1
    }
    fn can_be_illusion(&self) -> bool {
        true
    }
}

pub fn load_creatures() -> HashMap<String, Creature> {
    let f = File::open("assets/creatures.ron").unwrap();
    ron::de::from_reader(f).unwrap()
}
