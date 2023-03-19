use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{print_text, WHITE, BottomTextEvent};
use crate::system;
pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(help_setup.in_schedule(OnEnter(GameState::Help)))
            .add_system(help_keyboard_input.in_set(OnUpdate(GameState::Help)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::Help)))

            .add_system(help_keys_setup.in_schedule(OnEnter(GameState::HelpKeys)))
            .add_system(help_subscreen_keyboard_input.in_set(OnUpdate(GameState::HelpKeys)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpKeys)))


            .add_system(help_spells_setup.in_schedule(OnEnter(GameState::HelpSpells)))
            .add_system(help_subscreen_keyboard_input.in_set(OnUpdate(GameState::HelpSpells)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpSpells)))

            .add_system(help_combat_setup.in_schedule(OnEnter(GameState::HelpCombat)))
            .add_system(help_combat_keyboard_input.in_set(OnUpdate(GameState::HelpCombat)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpCombat)))

            .add_system(help_ranged_combat_setup.in_schedule(OnEnter(GameState::HelpRangedCombat)))
            .add_system(help_subscreen_keyboard_input.in_set(OnUpdate(GameState::HelpRangedCombat)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpRangedCombat)))

            .add_system(help_undead_setup.in_schedule(OnEnter(GameState::HelpUndead)))
            .add_system(help_subscreen_keyboard_input.in_set(OnUpdate(GameState::HelpUndead)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpUndead)))

            .add_system(help_mounts_setup.in_schedule(OnEnter(GameState::HelpMounts)))
            .add_system(help_subscreen_keyboard_input.in_set(OnUpdate(GameState::HelpMounts)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpMounts)))

            .add_system(help_victory_setup.in_schedule(OnEnter(GameState::HelpVictory)))
            .add_system(help_subscreen_keyboard_input.in_set(OnUpdate(GameState::HelpVictory)))
            .add_system(system::despawn_screen::<HelpScreen>.in_schedule(OnExit(GameState::HelpVictory)))
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
    debug!("in help setup");
    print_text("         Help screen", &mut commands, g.fah(), Vec2::new(0.0, 8.0), WHITE, HelpScreen);
    print_text("1. Keys", &mut commands, g.fah(), Vec2::new(0.0, 7.0), WHITE, HelpScreen);
    print_text("2. Spells", &mut commands, g.fah(), Vec2::new(0.0, 6.0), WHITE, HelpScreen);
    print_text("3. Combat", &mut commands, g.fah(), Vec2::new(0.0, 5.0), WHITE, HelpScreen);
    print_text("4. Undead", &mut commands, g.fah(), Vec2::new(0.0, 4.0), WHITE, HelpScreen);
    print_text("5. Mounts", &mut commands, g.fah(), Vec2::new(0.0, 3.0), WHITE, HelpScreen);
    print_text("6. Victory", &mut commands, g.fah(), Vec2::new(0.0, 2.0), WHITE, HelpScreen);
    debug!("printed help");
    ev_text.send(BottomTextEvent::from("Press Keys 1-6 or 0 to return"));
}

fn help_keyboard_input(
    mut state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        state.set(GameState::InitialMenu);
        return
    }
    if keys.just_pressed(KeyCode::Key1) {
        keys.reset(KeyCode::Key1);
        state.set(GameState::HelpKeys);
        return;
    }
    if keys.just_pressed(KeyCode::Key2) {
        keys.reset(KeyCode::Key2);
        state.set(GameState::HelpSpells);
        return;
    }
    if keys.just_pressed(KeyCode::Key3) {
        keys.reset(KeyCode::Key3);
        state.set(GameState::HelpCombat);
        return;
    }
    if keys.just_pressed(KeyCode::Key4) {
        keys.reset(KeyCode::Key4);
        state.set(GameState::HelpUndead);
        return;
    }
    if keys.just_pressed(KeyCode::Key5) {
        keys.reset(KeyCode::Key5);
        state.set(GameState::HelpMounts);
        return;
    }
    if keys.just_pressed(KeyCode::Key6) {
        keys.reset(KeyCode::Key6);
        state.set(GameState::HelpVictory);
    }
}

fn help_subscreen_keyboard_input(
    mut state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    let mut has_pressed = false;
    for _ in keys.get_just_pressed() {
        has_pressed = true;
    }
    if has_pressed {
        println!("Got keypress, return to main help screeen");
        keys.reset_all();
        state.set(GameState::Help);
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
}

fn help_combat_keyboard_input(
    mut state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    let mut has_pressed = false;
    for _ in keys.get_just_pressed() {
        has_pressed = true;
    }
    if has_pressed {
        println!("Set ranged combat and reset all keypresses");
        keys.reset_all();
        state.set(GameState::HelpRangedCombat);
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
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
	ev_text.send(BottomTextEvent::from("   Press any key to continue "));
}
