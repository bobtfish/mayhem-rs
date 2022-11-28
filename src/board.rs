
use bevy::prelude::*;
use crate::constants::{WIDTH, HEIGHT};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_startup_system(setup)
        //.add_event::<BottomTextEvent>()
        //.add_system(manage_text_bottom)
        //.add_system(animate_sprite);
        .insert_resource(GameBoard::new());
    }
}

#[derive(Resource)]
pub struct GameBoard([GameColumn; WIDTH]);
struct GameColumn([GameSquare; HEIGHT]);
struct GameSquare(Vec<Entity>);

#[allow(clippy::cast_sign_loss)]
impl GameBoard {
    pub fn new() -> Self {
        Self([GameColumn; WIDTH].map(|_| GameColumn::new()))
    }
    pub fn put_entity(&mut self, pos: Vec2, e: Entity) {
        self.0[pos.x as usize].0[pos.y as usize].0.push(e);
    }
    pub fn has_entity(&self, pos: Vec2) -> bool {
        self.get_entity(pos).is_some()
    }
    pub fn get_entity(&self, pos: Vec2) -> Option<Entity> {
        let stack = &self.0[pos.x as usize].0[pos.y as usize].0;
        if stack.is_empty() {
            return None;
        }
        Some(stack[stack.len()-1])
    }
    pub fn pop_entity(&mut self, pos: Vec2) -> Entity {
        let stack = &mut self.0[pos.x as usize].0[pos.y as usize].0;
        stack.remove(stack.len()-1)
    }
    pub fn as_ref(&self) -> &Self {
        self
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
        assert!(b.has_entity(Vec2::new(0.0, 0.0)));
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