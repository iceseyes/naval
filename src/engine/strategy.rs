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

#[allow(dead_code)]
#[derive(Debug)]
pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn next_move(&mut self) -> Option<Cell> {
        Some(Cell::random())
    }
}

/// A strategy that tries to avoid hitting already hit cells.
/// When a ship is hit, it adds the adjacent cells as candidate moves to try next.
#[derive(Debug)]
pub struct SmartStrategy {
    moves: Vec<Cell>,
    candidate_cells: Vec<Vec<Cell>>,
    candidate_moves: Vec<Cell>,
}

impl SmartStrategy {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            candidate_cells: Vec::new(),
            candidate_moves: Vec::new(),
        }
    }
}

impl Strategy for SmartStrategy {
    fn next_move(&mut self) -> Option<Cell> {
        let next = loop {
            if self.candidate_moves.is_empty()
                && let Some(moves) = self.candidate_cells.pop()
            {
                self.candidate_moves = moves;
            }

            let cell = if let Some(cell) = self.candidate_moves.pop() {
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

    fn notify_hit(&mut self, _kind: ShipKind) {
        let last_move = *self.moves.last().unwrap();
        let mut new_candidates = Vec::new();

        match self.candidate_moves.len() {
            3 => {
                // the first strike has hit a ship, so only the cell on the same
                // axis may be a ship-cell. I can remove the perpendicular cells
                // and mark them as moved
                self.moves.push(self.candidate_moves.remove(0));
                // after that, candidates_move is reduced to two cells, one for each axis
                self.moves.push(self.candidate_moves.remove(1));
            }
            2 => {
                // the second strike has hit a ship, so I can remove the perpendicular cells
                self.moves.push(self.candidate_moves.remove(1));
            }
            _ => {}
        }

        if last_move.x() > 0
            && let Ok(cell) = Cell::new(last_move.x() - 1, last_move.y())
            && !self.moves.contains(&cell)
            && !self.candidate_cells.iter().flatten().any(|c| *c == cell)
        {
            new_candidates.push(cell);
        }

        if last_move.y() > 0
            && let Ok(cell) = Cell::new(last_move.x(), last_move.y() - 1)
            && !self.moves.contains(&cell)
            && !self.candidate_cells.iter().flatten().any(|c| *c == cell)
        {
            new_candidates.push(cell);
        }

        if last_move.x() < 10
            && let Ok(cell) = Cell::new(last_move.x() + 1, last_move.y())
            && !self.moves.contains(&cell)
            && !self.candidate_cells.iter().flatten().any(|c| *c == cell)
        {
            new_candidates.push(cell);
        }

        if last_move.y() < 10
            && let Ok(cell) = Cell::new(last_move.x(), last_move.y() + 1)
            && !self.moves.contains(&cell)
            && !self.candidate_cells.iter().flatten().any(|c| *c == cell)
        {
            new_candidates.push(cell);
        }

        self.candidate_cells.push(new_candidates);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_smart_strategy_shortcut() {
        let mut strategy = SmartStrategy {
            moves: vec![Cell::bounded(5, 5)],
            candidate_cells: Vec::new(),
            candidate_moves: vec![
                Cell::bounded(4, 5),
                Cell::bounded(5, 4),
                Cell::bounded(6, 5),
                Cell::bounded(5, 6),
            ],
        };

        // we should get (5, 6) first because it's the first element of the first group in candidate cells
        let cell = strategy.next_move();
        assert_eq!(cell, Some(Cell::bounded(5, 6)));
        assert_eq!(
            strategy.moves,
            vec![Cell::bounded(5, 5), Cell::bounded(5, 6),]
        );
        strategy.notify_hit(ShipKind::AircraftCarrier);
        assert_eq!(strategy.candidate_moves, vec![Cell::bounded(5, 4)]);
        assert_eq!(
            strategy.moves,
            vec![
                Cell::bounded(5, 5),
                Cell::bounded(5, 6),
                Cell::bounded(4, 5),
                Cell::bounded(6, 5),
            ]
        );
    }
    #[rstest]
    fn test_smart_strategy_in_a_row() {
        let mut strategy = SmartStrategy {
            moves: vec![Cell::bounded(5, 5)],
            candidate_cells: Vec::new(),
            candidate_moves: vec![
                Cell::bounded(4, 5),
                Cell::bounded(5, 4),
                Cell::bounded(6, 5),
                Cell::bounded(5, 6),
            ],
        };

        // this move doesn't hit anything
        strategy.next_move();

        // we should get (6, 5) first because it's the first element of the first group in candidate cells
        let cell = strategy.next_move();
        assert_eq!(cell, Some(Cell::bounded(6, 5)));
        assert_eq!(
            strategy.moves,
            vec![
                Cell::bounded(5, 5),
                Cell::bounded(5, 6),
                Cell::bounded(6, 5)
            ]
        );
        assert_eq!(
            strategy.candidate_moves,
            vec![Cell::bounded(4, 5), Cell::bounded(5, 4)]
        );
        assert!(strategy.candidate_cells.is_empty());

        // notify hit to add new candidates around (4,5)
        strategy.notify_hit(ShipKind::AircraftCarrier);
        assert_eq!(strategy.candidate_moves, vec![Cell::bounded(4, 5)]);
        assert_eq!(
            strategy.candidate_cells,
            vec![vec![
                Cell::bounded(6, 4),
                Cell::bounded(7, 5),
                Cell::bounded(6, 6)
            ]]
        );

        // the next move should be (4,5)
        let cell = strategy.next_move();
        assert_eq!(cell, Some(Cell::bounded(4, 5)));
        assert_eq!(
            strategy.moves,
            vec![
                Cell::bounded(5, 5),
                Cell::bounded(5, 6),
                Cell::bounded(6, 5),
                Cell::bounded(5, 4),
                Cell::bounded(4, 5)
            ]
        );

        strategy.notify_hit(ShipKind::AircraftCarrier);
        assert_eq!(strategy.candidate_moves, vec![]);
        assert_eq!(
            strategy.candidate_cells,
            vec![
                vec![
                    Cell::bounded(6, 4),
                    Cell::bounded(7, 5),
                    Cell::bounded(6, 6)
                ],
                vec![
                    Cell::bounded(3, 5),
                    Cell::bounded(4, 4),
                    Cell::bounded(4, 6)
                ]
            ]
        );

        strategy.next_move();
        strategy.next_move();

        let cell = strategy.next_move();
        assert_eq!(cell, Some(Cell::bounded(3, 5)));
    }

    #[rstest]
    fn test_smart_strategy_first_move_is_random() {
        let mut strategy = SmartStrategy::new();
        let cell = strategy.next_move();
        assert!(cell.is_some());
    }

    #[rstest]
    fn test_smart_do_not_repeat_your_self() {
        let mut strategy = SmartStrategy::new();
        let first_move = strategy.next_move().unwrap();
        strategy.moves.push(first_move);
        strategy.candidate_moves.push(first_move);
        let next_move = strategy.next_move().unwrap();
        assert!(strategy.candidate_moves.is_empty());
        assert_ne!(first_move, next_move);
    }

    #[rstest]
    fn test_random_strategy_next_move() {
        let mut strategy = RandomStrategy;
        let cell = strategy.next_move();
        assert!(cell.is_some());
    }
}
