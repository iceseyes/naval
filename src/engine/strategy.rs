use crate::engine::fleet::ShipKind;
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

    /// Notify the strategy that a ship has been hit and which was it.
    fn notify_hit(&mut self, _kind: ShipKind) {}
}

#[derive(Debug)]
pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn next_move(&mut self) -> Option<Cell> {
        Some(Cell::random())
    }
}

#[derive(Debug)]
pub struct SmartStrategy {
    moves: Vec<Cell>,
    candidates_moves: Vec<Cell>,
}

impl SmartStrategy {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            candidates_moves: Vec::new(),
        }
    }
}

impl Strategy for SmartStrategy {
    fn next_move(&mut self) -> Option<Cell> {
        let next = loop {
            let cell = if let Some(cell) = self.candidates_moves.pop() {
                cell
            } else {
                Cell::random()
            };

            if !self.moves.contains(&cell) {
                break cell;
            }
        };

        self.moves.push(next);

        Some(next)
    }

    fn notify_hit(&mut self, kind: ShipKind) {
        let size = kind.size();
        let last_move = *self.moves.last().unwrap();

        let mut new_candidates = Vec::new();

        for i in 1..size {
            if last_move.x() + i < 10
                && let Ok(cell) = Cell::new(last_move.x() + i, last_move.y())
                && !self.moves.contains(&cell)
            {
                new_candidates.push(cell);
            }

            if last_move.x() >= i
                && let Ok(cell) = Cell::new(last_move.x() - i, last_move.y())
                && !self.moves.contains(&cell)
            {
                new_candidates.push(cell);
            }

            if last_move.y() + i < 10
                && let Ok(cell) = Cell::new(last_move.x(), last_move.y() + i)
                && !self.moves.contains(&cell)
            {
                new_candidates.push(cell);
            }

            if last_move.y() >= i
                && let Ok(cell) = Cell::new(last_move.x(), last_move.y() - i)
                && !self.moves.contains(&cell)
            {
                new_candidates.push(cell);
            }
        }

        self.candidates_moves.extend(new_candidates);
    }
}
