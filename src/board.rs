
use bevy::{prelude::*, utils::HashMap};
use crate::{constants::{WIDTH, HEIGHT}, vec::Vec2I};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<BoardPutEntity>()
        .add_system(put_entity)
        .add_event::<BoardMove>()
        .add_system(move_entity)
        .insert_resource(GameBoard::new());
    }
}

#[derive(Component)]
pub struct MoveableComponent {
    pub movement: u8,
    pub flying: bool,
}

pub struct BoardPutEntity {
    pub entity: Entity,
    pub pos: Vec2,
}

fn put_entity(
    mut ev: EventReader<BoardPutEntity>,
    mut board: ResMut<GameBoard>,
) {
    for e in ev.iter() {
        board.put_entity(e.pos, e.entity);
    }
}

pub struct BoardMove {
    pub entity: Entity,
    pub to: Vec2,
}

fn move_entity(
    mut ev: EventReader<BoardMove>,
    mut board: ResMut<GameBoard>,
    mut query: Query<&mut Transform>,
) {
    for e in ev.iter() {
        let current_pos = Vec2::from(board.get_entity_pos(e.entity));
        println!("Do Board move from {current_pos} to {}", e.to);
        let entity = board.pop_entity(current_pos);
        println!("pop entity {entity:?}");
        let mut transform = query.get_mut(entity).unwrap();
        *transform = transform.with_translation(e.to.extend(1.0));
        board.put_entity(e.to, entity);
    }
}

#[derive(Resource)]
pub struct GameBoard([GameColumn; WIDTH], HashMap<Entity, Vec2I>);
struct GameColumn([GameSquare; HEIGHT]);
struct GameSquare(Vec<Entity>);

#[allow(clippy::cast_sign_loss)]
impl GameBoard {
    fn new() -> Self {
        Self([GameColumn; WIDTH].map(|_| GameColumn::new()), HashMap::new())
    }
    fn put_entity(&mut self, pos: Vec2, e: Entity) {
        self.0[pos.x as usize].0[pos.y as usize].0.push(e);
        self.1.insert(e, Vec2I::from(pos));
    }
    pub fn has_entity_at(&self, pos: Vec2) -> bool {
        self.get_entity(pos).is_some()
    }
    pub fn _has_entity(&self, e: Entity) -> bool {
        self.1.contains_key(&e)
    }
    pub fn get_entity(&self, pos: Vec2) -> Option<Entity> {
        let stack = &self.0[pos.x as usize].0[pos.y as usize].0;
        if stack.is_empty() {
            return None;
        }
        Some(stack[stack.len()-1])
    }
    pub fn get_entity_pos(&self, e: Entity) -> Vec2I {
        *self.1.get(&e).unwrap()
    }
    fn pop_entity(&mut self, pos: Vec2) -> Entity {
        let stack = &mut self.0[pos.x as usize].0[pos.y as usize].0;
        stack.remove(stack.len()-1)
    }
}
impl GameColumn {
    fn new() -> Self {
        Self([GameSquare; HEIGHT].map(|_| GameSquare::new()))
    }
}
impl GameSquare {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::GameBoard;

    #[test]
    fn basic() {
        let mut b = GameBoard::new();
        b.put_entity(Vec2::new(0.0, 0.0), Entity::from_raw(1));
        assert!(b.has_entity_at(Vec2::new(0.0, 0.0)));
        let e = b.get_entity(Vec2::new(0.0, 0.0));
        assert!(e.is_some());
        assert!(b.get_entity(Vec2::new(1.0, 0.0)).is_none());
    }
    #[test]
    fn stack_top() {
        let mut b = GameBoard::new();
        b.put_entity(Vec2::new(0.0, 0.0), Entity::from_raw(1));
        b.put_entity(Vec2::new(0.0, 0.0), Entity::from_raw(2));
        let e = b.get_entity(Vec2::new(0.0, 0.0));
        assert!(e.is_some());
        assert_eq!(e.unwrap().index(), 2);
    }
}