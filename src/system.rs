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
