// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    InitialMenu,
    PlayerNameMenu,
    PlayerNameMenuTransition,
    PlayerMenu,
    PlayerMenuExamineSpell,
    PlayerMenuExamineOneSpell,
    PlayerMenuSelectSpell,
    PlayerMenuExamineBoard,
    PlayerMenuTransition,
    Game,
    GameCastSpell,
}

