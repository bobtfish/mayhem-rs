use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash, Resource, States, Default)]
pub enum GameState {
    #[default]
    InitialMenu,
    Help,
    HelpKeys,
    HelpSpells,
    HelpCombat,
    HelpRangedCombat,
    HelpUndead,
    HelpMounts,
    HelpVictory,
    PlayerNameMenu,
    PlayerNameMenuTransition,
    PlayerMenu,
    PlayerMenuExamineSpell,
    PlayerMenuExamineOneSpell,
    PlayerMenuSelectSpell,
    PlayerMenuExamineBoard,
    PlayerMenuTransition,
    CastSpellSetup,
    CastSpell,
    MoveSetup,
    MoveChoose,
    MoveMoving,
    RangedAttackChoose,
    RangedAttackDo,
    NextTurn,
}

