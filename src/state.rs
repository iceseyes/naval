use crate::player::Player;
use crate::widgets::battle::BattleStateModel;
use crate::widgets::setup::SetupStateModel;
use crossterm::event::{Event, KeyEvent};
use ratatui::prelude::{Buffer, Rect, Widget};
use std::default::Default;

/// Trait for all application model states.
///
/// A Model state is responsible for handling user input and build the corresponding UI widget.
pub trait StateModel {
    /// Handles user input events according to the current state.
    fn handle_key_events(&mut self, key_event: KeyEvent);

    /// Updates player data according to the current state.
    ///
    /// It consumes the current players and returns the updated players.
    /// First it updates the computer player, then the human player.
    fn update(&mut self, computer: Player, human: Option<Player>) -> (Player, Option<Player>);

    /// Builds the corresponding UI widget for the current state.
    fn widget(&self) -> impl Widget;
}

/// The application states: Setup or Battle.
///
/// Setup state allows the user to deploy their fleet on the grid.
/// Battle state allows the user to play against the computer.
pub enum NavalBattleState {
    Setup(SetupStateModel),
    Battle(BattleStateModel),
}

impl NavalBattleState {
    /// Creates a new setup state with an empty self.deploy_grid, ready to be populated by the user.
    pub fn setup() -> Self {
        Self::Setup(SetupStateModel::default())
    }

    /// Creates a new battle state ready to start the battle between the computer and the user.
    pub fn battle() -> Self {
        Self::Battle(BattleStateModel::default())
    }

    /// Dispatches events to be handled according to the current state.
    pub fn handle_events(&mut self, event: Event) {
        if let Event::Key(key_event) = event {
            match self {
                NavalBattleState::Setup(state) => state.handle_key_events(key_event),
                NavalBattleState::Battle(state) => state.handle_key_events(key_event),
            }
        }
    }

    pub fn update(&mut self, computer: Player, human: Option<Player>) -> (Player, Option<Player>) {
        match self {
            NavalBattleState::Setup(state) => state.update(computer, human),
            NavalBattleState::Battle(state) => state.update(computer, human),
        }
    }

    /// Render the current state into the given area
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            NavalBattleState::Setup(state) => state.widget().render(area, buf),
            NavalBattleState::Battle(state) => state.widget().render(area, buf),
        }
    }
}

/// The default state is the setup screen
impl Default for NavalBattleState {
    fn default() -> Self {
        NavalBattleState::setup()
    }
}
