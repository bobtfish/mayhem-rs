use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use crate::board::MoveableComponent;
use crate::display::spawn_anim;
use crate::player::Player;
use crate::spell::{ASpell, SpellBox};
use crate::system::{BoardEntity, Named, RangedCombat, CanDefend, CanAttack};

fn default_as_zero() -> u8 {
    0
}
fn default_as_zero_signed() -> i8 {
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
    #[serde(default = "default_as_zero")]
    combat: u8,
    #[serde(default = "default_as_zero")]
    ranged_combat: u8,
    #[serde(default = "default_as_zero")]
    range: u8,
    #[serde(default = "default_as_zero")]
    defence: u8,
    #[serde(default = "default_as_false")]
    mountable: bool,
    #[serde(default = "default_as_false")]
    can_be_illusion: bool,
    #[serde(default = "default_as_zero_signed")]
    law_chaos: i8,
    casting_chance: u8,
    manoeuvre: u8,
    magical_resistance: u8,
    color_r: u8,
    color_g: u8,
    color_b: u8,
}

#[derive(Component)]
pub struct CreatureComponent {
    pub is_illusion: bool,
    pub mountable: bool,
}

impl Creature {
    fn to_entity(
        &self,
        illusion: bool,
        v: Vec2,
        commands: &mut Commands,
        texture_atlas_handle: Handle<TextureAtlas>
    ) -> Entity {
        let color = Color::rgba(f32::from(self.color_r) / 255.0, f32::from(self.color_g) / 255.0, f32::from(self.color_b) / 255.0, 1.0);
        let e = spawn_anim(commands, texture_atlas_handle, v, self.sprite_index, 4, color);
        let mut ec = commands.get_entity(e).unwrap();
        ec.insert(CreatureComponent{
            is_illusion: illusion,
            mountable: self.mountable,
        });
        ec.insert(CanAttack{
            combat: self.combat,
        });
        ec.insert(CanDefend{
            defence: self.defence,
        });
        ec.insert(MoveableComponent{
            movement: self.movement,
            flying: self.flying,
        });
        ec.insert(BoardEntity);
        ec.insert(Named{ name: self.name.clone() });
        if self.ranged_combat > 0 {
            ec.insert(RangedCombat{
                ranged_combat: self.ranged_combat,
                range: self.range,
            });
        }
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
    fn law_rating(&self) -> i8 {
        self.creature.law_chaos
    }
    fn clone(&self) -> SpellBox {
        Box::new(std::clone::Clone::clone(self))
    }
    fn cast(&self, illusion: bool, _player: &Player, pos: Vec2, commands: &mut Commands, tah: Handle<TextureAtlas>) -> Option<Entity> {
        Some(self.creature.to_entity(illusion, pos, commands, tah))
    }
    fn reusable(&self) -> bool {
        false
    }
    fn cast_range(&self) -> u8 {
        1
    }
    fn can_be_illusion(&self) -> bool {
        self.creature.can_be_illusion
    }
    fn casting_chance(&self) -> u8 {
        self.creature.casting_chance
    }
    fn get_description(&self) -> Vec<String> {
        vec![
            format!("Combat={}", self.creature.combat),
            format!("Ranged Combat={} Range={}", self.creature.ranged_combat, self.creature.range),
            format!("Defence={}", self.creature.defence),
            format!("Movement Allowance={}", self.creature.movement),
            format!("Manoeuver Rating={}", self.creature.manoeuvre),
            format!("Magic Resistance={}", self.creature.magical_resistance),
            format!("Casting Chance={}%", self.casting_chance()),
        ]
    }
}

pub fn load_creatures() -> HashMap<String, Creature> {
    let f = File::open("assets/creatures.ron").unwrap();
    ron::de::from_reader(f).unwrap()
}
