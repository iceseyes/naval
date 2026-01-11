//! This module contains the application states.
//!
//! The application state is a simple enum that represents the current application phase.
//! It is responsible for handling user input and building the corresponding UI widget for
//! the specific state.
//!
//! Every state has its own model, which is responsible for handling the state-specific logic.
//! The model is updated according to the user input, and it can be used to get the UI widget to display.
//!
//! The model is also responsible for updating the player data according to the current state.
//! For instance, the setup state model creates the new player fleet and so the player object itself.
//! The battle state model updates the player shots according to the user input and the computer ones.
//!
//! To make the state model transparent to the application, the state is enum and every variant
//! has its own model. Application sends requests to the actual state object, and this one dispatches
//! the requests to the real model.
//!
use crate::engine::game::Game;
use crate::tui::widgets::{battle::BattleStateModel, setup::SetupStateModel};
use crossterm::event::{Event, KeyEvent};
use ratatui::prelude::{Buffer, Rect, Widget};
use std::default::Default;

/// Trait for all application model states.
///
/// A Model state is responsible for handling user input and build the corresponding UI widget.
pub trait StateModel {
    /// Handles user input events according to the current state.
    fn handle_key_events(&mut self, key_event: KeyEvent);

    /// Updates the game state according to the current interface state.
    fn update(&mut self, game: &mut Game);

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
    pub fn battle(game: &Game) -> Self {
        let mut model = BattleStateModel::default();
        model.update_grid(game.computer().unwrap(), game.human().unwrap());

        Self::Battle(model)
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

    /// Updates the player objects according to the current state.
    pub fn update(&mut self, game: &mut Game) {
        match self {
            NavalBattleState::Setup(state) => state.update(game),
            NavalBattleState::Battle(state) => state.update(game),
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
