use bevy::prelude::*;
use crate::display::BottomTextEvent;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
    ev_text.send(BottomTextEvent::clear());
}

#[derive(Component)]
pub struct Named {
    pub name: String
}

#[derive(Component)]
pub struct BelongsToPlayer {
    pub player_entity: Entity
}

#[derive(Component)]
pub struct BoardEntity;

pub fn hide_board_entities (
    to_hide: Query<&mut Visibility, With<BoardEntity>>,
) {
    toggle_board_entities(to_hide, false);
}

pub fn show_board_entities (
    to_show: Query<&mut Visibility, With<BoardEntity>>,
) {
    toggle_board_entities(to_show, true);
}

pub fn toggle_board_entities (
    mut q: Query<&mut Visibility, With<BoardEntity>>,
    set: bool
) {
    for mut vis in q.iter_mut() {
        vis.is_visible = set;
    }
}