//! Player module for managing player-related functionalities.
//!
//! In naval battle, players deploy their fleets to engage in strategic battles. Every player has
//! a grid to take notes about its attacks: if the shoot as hit, sunk, or missed the ships of the other player.
//! The game ends when a player fleet is totally sunk.
//!
//! In each turn, a player chooses another one to attack and try to hit its ships. After that, if all
//! the other player's fleets but its own are sunk, the game ends and the winner is the player with
//! the remaining fleet.
//!
//! Eventually, the game will be extended to support multiplayer and AI opponents, but the default
//! version will focus on a single-player vs computer opponent.
//!

use crate::cell::{Cell, CellState, Grid};
use crate::ship::{Fleet, ShipKind};

/// Defines the Player struct and associated methods for managing player-related functionalities.
pub struct Player {
    name: String,
    fleet: Fleet,
    grid: Grid,
}

impl Player {
    /// Creates a new Player instance.
    ///
    /// Initializes a new player with the given name and fleet. The player's grid is initialized to empty.
    pub fn new(name: String, fleet: Fleet) -> Self {
        Self {
            name,
            fleet,
            grid: Grid::default(),
        }
    }

    /// Returns the player's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the player's shots grid.
    ///
    /// This grid represents the player's shots on the opponent's fleet and the effect they have on the opponent's ships.
    pub fn shots_grid(&self) -> &Grid {
        &self.grid
    }

    /// Returns the player's fleet.
    pub fn fleet(&self) -> &Fleet {
        &self.fleet
    }

    /// Try to hit the opponent's ships.
    pub fn attack(&mut self, opponent: &mut Player, cell: &Cell) -> Option<ShipKind> {
        let ship_hit = opponent.fleet.hit_at(cell);
        if ship_hit.is_some() {
            self.grid.mark(cell, CellState::Hit);
        } else {
            self.grid.mark(cell, CellState::Miss);
        }

        ship_hit
    }

    /// Checks whether this player has lost the battle
    pub fn has_lost(&self) -> bool {
        self.fleet.is_sunk()
    }
}
