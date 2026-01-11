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
//! version will focus on a single-player vs. computer opponent.
//!

use crate::engine::fleet::{Fleet, ShipKind};
use crate::engine::grid::{Cell, CellState, Grid};
use crate::engine::strategy::Strategy;

/// Defines the Player struct and associated methods for managing player-related functionalities.
#[derive(Debug)]
pub struct Player {
    name: String,
    fleet: Fleet,
    grid: Grid,
    strategy: Box<dyn Strategy>,
    human: bool,
}

impl Player {
    /// Creates a new Player instance.
    ///
    /// Initializes a new player with the given name and fleet. The player's grid is initialized to empty.
    pub fn new(name: &str, fleet: Fleet) -> Self {
        Self {
            name: name.to_string(),
            fleet,
            grid: Grid::default(),
            strategy: Box::new(NoStrategy),
            human: true,
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

    /// return the next move to play, or None if no strategy is supported (human player)
    pub fn next_move(&mut self) -> Option<Cell> {
        self.strategy.next_move()
    }

    /// Set the strategy to use for this player.
    pub fn set_strategy<ConcreteStrategy: Strategy + 'static>(
        &mut self,
        strategy: ConcreteStrategy,
    ) {
        self.strategy = Box::new(strategy);
        self.human = false;
    }

    /// A player is a human player if it has a strategy that is not NoStrategy.
    pub fn is_human(&self) -> bool {
        self.human
    }
}

#[derive(Debug)]
struct NoStrategy;

impl Strategy for NoStrategy {
    fn next_move(&mut self) -> Option<Cell> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fleet::ShipOrientation;
    use rstest::{fixture, rstest};

    #[fixture]
    pub fn player1_fleet() -> Fleet {
        let mut y_coords = (0u8..9).into_iter().step_by(2);
        Fleet::build(|kind| {
            kind.ship(
                Cell::bounded(0, y_coords.next().unwrap()),
                ShipOrientation::Horizontal,
            )
            .unwrap()
        })
    }

    #[fixture]
    pub fn player2_fleet() -> Fleet {
        let mut x_coords = (0u8..9).into_iter().step_by(2);
        Fleet::build(|kind| {
            kind.ship(
                Cell::bounded(x_coords.next().unwrap(), 0),
                ShipOrientation::Vertical,
            )
            .unwrap()
        })
    }

    #[rstest]
    pub fn test_match(player1_fleet: Fleet, player2_fleet: Fleet) {
        let shots = [
            Cell::bounded(0, 0),
            Cell::bounded(1, 0),
            Cell::bounded(2, 0),
            Cell::bounded(3, 0),
            Cell::bounded(4, 0),
            Cell::bounded(0, 2),
            Cell::bounded(1, 2),
            Cell::bounded(2, 2),
            Cell::bounded(3, 2),
            Cell::bounded(0, 4),
            Cell::bounded(1, 4),
            Cell::bounded(2, 4),
            Cell::bounded(0, 6),
            Cell::bounded(1, 6),
            Cell::bounded(2, 6),
            Cell::bounded(0, 8),
        ];
        let mut player1 = Player::new("One", player1_fleet);
        let mut player2 = Player::new("Two", player2_fleet);

        shots.iter().for_each(|shot| {
            assert!(!player1.has_lost());
            assert!(!player2.has_lost());

            player1.attack(&mut player2, shot);
            assert!(!player2.has_lost());

            player2.attack(&mut player1, shot);
            assert!(!player1.has_lost());
        });

        let attack_result = player2.attack(&mut player1, &Cell::bounded(1, 8)).unwrap();
        assert_eq!(attack_result, ShipKind::Destroyer);
        assert!(player1.fleet().get(&attack_result).is_sunk());
        assert!(player1.fleet().is_sunk());
        assert!(!player2.fleet().is_sunk());

        assert!(!player2.has_lost());
        assert!(player1.has_lost());
    }

    #[rstest]
    pub fn test_name(player1_fleet: Fleet, player2_fleet: Fleet) {
        let player1 = Player::new("One", player1_fleet);
        let player2 = Player::new("Two", player2_fleet);

        assert_eq!(player1.name(), "One");
        assert_eq!(player2.name(), "Two");
    }

    #[rstest]
    pub fn test_shots_grid(player1_fleet: Fleet, player2_fleet: Fleet) {
        let mut player1 = Player::new("One", player1_fleet);
        let mut player2 = Player::new("Two", player2_fleet);

        assert!(player1.shots_grid().is_empty());

        player1.attack(&mut player2, &Cell::bounded(0, 0));
        assert_eq!(
            player1.shots_grid().at(&Cell::bounded(0, 0)),
            &CellState::Hit
        );

        player1.attack(&mut player2, &Cell::bounded(1, 0));
        assert_eq!(
            player1.shots_grid().at(&Cell::bounded(0, 0)),
            &CellState::Hit
        );
        assert_eq!(
            player1.shots_grid().at(&Cell::bounded(1, 0)),
            &CellState::Miss
        );
    }
}
