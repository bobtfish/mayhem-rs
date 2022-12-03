use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{print_text, WHITE, BottomTextEvent};
use crate::system;
pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Help).with_system(help_setup))
            .add_system_set(SystemSet::on_update(GameState::Help).with_system(help_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::Help).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpKeys).with_system(help_keys_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpKeys).with_system(help_subscreen_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpKeys).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpSpells).with_system(help_spells_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpSpells).with_system(help_subscreen_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpSpells).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpCombat).with_system(help_combat_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpCombat).with_system(help_combat_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpCombat).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpRangedCombat).with_system(help_ranged_combat_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpRangedCombat).with_system(help_subscreen_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpRangedCombat).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpUndead).with_system(help_undead_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpUndead).with_system(help_subscreen_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpUndead).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpMounts).with_system(help_mounts_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpMounts).with_system(help_subscreen_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpMounts).with_system(system::despawn_screen::<HelpScreen>))
            .add_system_set(SystemSet::on_enter(GameState::HelpVictory).with_system(help_victory_setup))
            .add_system_set(SystemSet::on_update(GameState::HelpVictory).with_system(help_subscreen_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::HelpVictory).with_system(system::despawn_screen::<HelpScreen>))
            ;
    }
}

#[derive(Component, Clone, Copy)]
struct HelpScreen;

fn help_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    println!("in help setup");
    print_text("         Help screen", &mut commands, g.fah(), Vec2::new(0.0, 8.0), WHITE, HelpScreen);
	print_text("1. Keys", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("2. Spells", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("3. Combat", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("4. Undead", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("5. Mounts", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("6. Victory", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
    println!("printed help");
	ev_text.send(BottomTextEvent::noclear("Press Keys 1-6 or 0 to return"));
}

fn help_keyboard_input(
    mut state: ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        state.set(GameState::InitialMenu).unwrap();
        return
    }
    if keys.just_pressed(KeyCode::Key1) {
        keys.reset(KeyCode::Key1);
        state.set(GameState::HelpKeys).unwrap();
        return;
    }
    if keys.just_pressed(KeyCode::Key2) {
        keys.reset(KeyCode::Key2);
        state.set(GameState::HelpSpells).unwrap();
        return;
    }
    if keys.just_pressed(KeyCode::Key3) {
        keys.reset(KeyCode::Key3);
        state.set(GameState::HelpCombat).unwrap();
        return;
    }
    if keys.just_pressed(KeyCode::Key4) {
        keys.reset(KeyCode::Key4);
        state.set(GameState::HelpUndead).unwrap();
        return;
    }
    if keys.just_pressed(KeyCode::Key5) {
        keys.reset(KeyCode::Key5);
        state.set(GameState::HelpMounts).unwrap();
        return;
    }
    if keys.just_pressed(KeyCode::Key6) {
        keys.reset(KeyCode::Key6);
        state.set(GameState::HelpVictory).unwrap();
    }
}

fn help_subscreen_keyboard_input(
    mut state: ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    let mut has_pressed = false;
    for _ in keys.get_just_pressed() {
        has_pressed = true;
    }
    if has_pressed {
        println!("Got keypress, return to main help screeen");
        keys.reset_all();
        state.set(GameState::Help).unwrap();
    }
}

fn help_keys_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("              Keys", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	print_text("AQWEDCXZ - Move in direction", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("S - Select creature/wizard", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("K - Cancel movement/attack", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("I - Show information on", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("    creature", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("1-8 - Highlight creations of", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	print_text("      player # 1-8", &mut commands, g.fah(), Vec2::new(0.0, 1.0), WHITE, HelpScreen);
	print_text("0 - End turn", &mut commands, g.fah(), Vec2::new(0.0, 0.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}

fn help_spells_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("           Spells", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	print_text("Select a spell then use", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("direction keys to choose", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("where to cast it.", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("Press S to cast.", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("Illusions always succeed but", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("can be disbelieved by others.", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	print_text("   ^=law *=chaos -=neutral", &mut commands, g.fah(), Vec2::new(0.0, 0.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}


fn help_combat_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("           Combat", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	//                                        #
	print_text("Move next to another creature", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("to engage them in combat.", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("Flying creatures can attack", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("remotely without engagement.", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("If adjacent next turn you may", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("remain engaged or may be able", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	print_text("to break away.", &mut commands, g.fah(), Vec2::new(0.0, 1.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}

fn help_combat_keyboard_input(
    mut state: ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    let mut has_pressed = false;
    for _ in keys.get_just_pressed() {
        has_pressed = true;
    }
    if has_pressed {
        println!("Set ranged combat and reset all keypresses");
        keys.reset_all();
        state.set(GameState::HelpRangedCombat).unwrap();
    }
}

fn help_ranged_combat_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
 	print_text("        Ranged Combat", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	//                                        #
	print_text("Some characters have ranged", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("combat.", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("This always happens after", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("movement (K to skip movement)", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("Target is selected with", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("direction keys, press S to", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	print_text("fire. Target must be in line", &mut commands, g.fah(), Vec2::new(0.0, 1.0), WHITE, HelpScreen);
	print_text("of sight.", &mut commands, g.fah(), Vec2::new(0.0, 0.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}

fn help_undead_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("           Undead", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	//                                        #
	print_text("Some characters are undead.", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("They can only be attacked by", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("other undead characters or", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("magic weapons.", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("The raise dead spell will", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	print_text("turn a corpse into an undead", &mut commands, g.fah(), Vec2::new(0.0, 1.0), WHITE, HelpScreen);
	print_text("creature.", &mut commands, g.fah(), Vec2::new(0.0, 0.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}


fn help_mounts_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("           Mounts", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	//                                        #
	print_text("Some characters can be ridden", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("by wizards. Simply move your", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("wizard onto the creature to", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
	print_text("mount it.", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("This allows faster movement", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("and your wizard cannot be", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	print_text("killed unless their mount is", &mut commands, g.fah(), Vec2::new(0.0, 1.0), WHITE, HelpScreen);
	print_text("killed first.", &mut commands, g.fah(), Vec2::new(0.0, 0.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}

fn help_victory_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("          Victory", &mut commands, g.fah(), Vec2::new(0.0, 9.0), WHITE, HelpScreen);
	//                                        #
	print_text("To win the game, simply kill", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
	print_text("all the other wizards.", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
	print_text("When a player is killed, all", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
	print_text("of their creations will also", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
	print_text("vanish.", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
	ev_text.send(BottomTextEvent::noclear("   Press any key to continue "));
}