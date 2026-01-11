use crate::engine::grid::Cell;
use std::fmt::Debug;

/// The Strategy trait for implementing different move strategies for players.
///
/// Every player uses its given stategy implementation to decide which is the next move.
/// If the strategy returns `None`, the game engine should ask the user for the next move.
pub trait Strategy: Debug {
    /// Return the next move for the player.
    ///
    /// It can return `None` if no move is available (e.g., for human players).
    fn next_move(&mut self) -> Option<Cell>;
}

#[derive(Debug)]
pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn next_move(&mut self) -> Option<Cell> {
        Some(Cell::random())
    }
}
