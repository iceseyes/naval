//! This module contains the Naval Battle TUI application.
//!
//! The application is based on the [ratatui](https://github.com/ratatouille-aqua/ratatui) crate.
//! It provides a terminal user interface for playing a naval battle game against a computer opponent.
//! The game consists of two main phases: setup and battle. During the setup phase, the human player deploys their fleet on a grid.
//! During the battle phase, the human player and the computer take turns attacking each other's fleets until one player wins.
//!
use crate::engine::game::Game;
use crate::tui::{state::NavalBattleState, widgets::workbench::Workbench};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub mod state;
mod widgets;

/// The Naval Battle TUI application
///
/// Basically, the battle happens between a computer player with a random fleet deployment
/// and a human player that will deploy ships manually.
///
/// The application starts in setup mode, where the human player can deploy their ships on a grid.
/// When the human player is done deploying their ships, the application switches to battle mode.
///
/// During the battle mode, the application will display one grid to input human player's shots (the opponent grid)
/// and another grid to display the computer's shots and the fleet deployment of the human player (the tactical grid).
///
/// For each player, the application asks for a shot to the current player, it evaluates if the opponent fleet is sunk or not,
/// and switch turns until one of the players has lost.
pub struct NavalBattleTui {
    game: Game,
    state: NavalBattleState,
    exit: bool,
    enter_pressed: bool,
}

impl NavalBattleTui {
    /// Creates a new Naval Battle TUI application
    ///
    /// As the application starts, a new computer player is created with a random fleet deployment.
    /// The human player is not created yet, as it will be created during the setup phase.
    /// The setup state is the default state when the application starts.
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            state: NavalBattleState::default(),
            exit: false,
            enter_pressed: false,
        }
    }

    /// Runs the application's main loop until the user quits
    ///
    /// It renders the current application state, then it is waiting for events according to the
    /// actual application state.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.state.update(&mut self.game);
            self.check_for_state_change()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let workbench = Workbench(&self.state);
        frame.render_widget(&workbench, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;
        if !self.handle_app_events(&event) {
            self.state.handle_events(event);
        }

        Ok(())
    }

    fn check_for_state_change(&mut self) -> io::Result<()> {
        // If the application is in setup mode but the human player has been created, switch to battle mode.
        // If the application is in battle mode, wait for user input.
        if let NavalBattleState::Setup { .. } = self.state
            && self.game.is_ready()
        {
            self.state = NavalBattleState::battle(&self.game);
        } else if let NavalBattleState::Battle { .. } = self.state
            && self.match_is_over()
            && self.enter_pressed
        {
            self.game = Game::new();
            self.state = NavalBattleState::setup();
            self.enter_pressed = false;
        }

        Ok(())
    }

    fn match_is_over(&self) -> bool {
        if let NavalBattleState::Battle { .. } = self.state {
            self.game.is_over()
        } else {
            false
        }
    }

    // Handles application-level events, such as quitting the application. If the event is handled, returns true.
    fn handle_app_events(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Char('Q'),
                ..
            }) => {
                self.exit();
                true
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) if self.match_is_over() => {
                self.enter_pressed = true;
                true
            }
            _ => self.match_is_over(),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
