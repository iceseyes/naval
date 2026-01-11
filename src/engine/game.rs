//! This module contains the logic to play the naval battle game.
//! Every game requires 2 players: a human player and a computer one.
//! The game proceeds in turns, where each player attacks the other until one of them loses all
//! their ships.

use crate::engine::fleet::Fleet;
use crate::engine::grid::Cell;
use crate::engine::player::Player;
use crate::engine::strategy::{RandomStrategy, SmartStrategy};
use rand::random_bool;

/// The Naval Battle game
pub struct Game {
    players: Vec<Player>,
    last_computer_move: Option<Cell>,
}

impl Game {
    const HUMAN_MOVE_FIRST_PROBABILITY: f64 = 0.5;
    const COMPUTER_NAME: &'static str = "Computer";

    /// Creates a new game, not ready to play.
    ///
    /// This new game must have been set upped with 2 players
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            last_computer_move: None,
        }
    }

    /// Set human player.
    ///
    /// When a human player is set, all previous players are cleared, and the game becomes ready
    /// (computer player is added automatically). The players' order is randomly chosen.
    ///
    /// The game object takes the ownership of the given player.
    pub fn set_human_player(&mut self, player: Player) {
        let human_player_first = random_bool(Self::HUMAN_MOVE_FIRST_PROBABILITY);
        let mut computer = Player::new(Self::COMPUTER_NAME, Fleet::build(|k| k.random()));
        computer.set_strategy(SmartStrategy::new());

        self.players.clear();
        if human_player_first {
            self.players.push(player);
            self.players.push(computer);
        } else {
            self.players.push(computer);
            self.players.push(player);
        }
    }

    /// Return whether the game is over.
    ///
    /// A game is over when one of the two players has lost.
    pub fn is_over(&self) -> bool {
        self.players.len() == 2 && self.players.iter().any(|p| p.has_lost())
    }

    /// The game is ready to play when it has 2 players and none has lost yet.
    pub fn is_ready(&self) -> bool {
        self.players.len() == 2 && !self.players.iter().any(|p| p.has_lost())
    }

    /// Return the human player.
    pub fn human(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.is_human())
    }

    /// Return the computer player.
    pub fn computer(&self) -> Option<&Player> {
        self.players.iter().find(|p| !p.is_human())
    }

    /// return the last computer move made by the computer player.
    pub fn last_computer_move(&self) -> Option<&Cell> {
        self.last_computer_move.as_ref()
    }

    /// Play a turn with the given move for the human player.
    ///
    /// The computer player uses its internal policy to evaluate the next move.
    /// Return whether the game is over after this turn: in this case will be returned `true` when
    /// human wins, otherwise `false`.
    ///
    /// If the game is over or not ready, an error is returned.
    pub fn play_turn(&mut self, human_move: &Cell) -> Result<Option<bool>, String> {
        if !self.is_ready() {
            return Err("Game is not ready or already over".to_string());
        }

        self.last_computer_move = None;

        let (first, second) = self.players.split_at_mut(1);
        let (first, second) = (&mut first[0], &mut second[0]);

        let (winner, computer_move) = do_move(first, second, human_move)?;
        if let Some(winner) = winner {
            return Ok(Some(winner.is_human()));
        }

        if let Some(computer_move) = computer_move {
            self.last_computer_move = Some(computer_move);
        }

        let (winner, computer_move) = do_move(second, first, human_move)?;
        if let Some(winner) = winner {
            return Ok(Some(winner.is_human()));
        }

        if let Some(computer_move) = computer_move {
            self.last_computer_move = Some(computer_move);
        }

        Ok(None)
    }
}

fn do_move<'player>(
    player: &'player mut Player,
    opposite: &'player mut Player,
    human_move: &Cell,
) -> Result<(Option<&'player Player>, Option<Cell>), String> {
    let mut last_computer_move = None;
    let player_move = if let Some(move_) = player.next_move() {
        last_computer_move = Some(move_);
        move_
    } else {
        *human_move
    };

    player.attack(opposite, &player_move);

    if opposite.has_lost() {
        Ok((Some(player), last_computer_move))
    } else {
        Ok((None, last_computer_move))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fleet::{tests::fixed_fleet, Fleet};
    use rstest::{fixture, rstest};

    #[fixture]
    fn human_player(fixed_fleet: Fleet) -> Player {
        Player::new("Human", fixed_fleet)
    }

    #[fixture]
    fn computer_player(fixed_fleet: Fleet) -> Player {
        let mut player = Player::new(Game::COMPUTER_NAME, fixed_fleet);
        player.set_strategy(RandomStrategy);

        player
    }

    #[rstest]
    fn test_play_turn_not_ready() {
        let mut game = Game::new();
        let err = game.play_turn(&Cell::bounded(0, 0)).unwrap_err();
        assert_eq!(err, "Game is not ready or already over");
    }

    #[rstest]
    fn test_play_turn_when_over(mut human_player: Player, mut computer_player: Player) {
        // pre-hit every occupied cell of the computer fleet.
        let mut occupied = Vec::<Cell>::new();
        for ship in computer_player.fleet().as_ref().iter() {
            occupied.extend(ship.occupied_cells());
        }
        for cell in occupied {
            human_player.attack(&mut computer_player, &cell);
        }
        assert!(computer_player.has_lost());

        let mut game = Game {
            players: vec![human_player, computer_player],
            last_computer_move: None,
        };

        assert!(game.is_over());

        let err = game.play_turn(&Cell::bounded(0, 0)).unwrap_err();
        assert_eq!(err, "Game is not ready or already over");
    }

    #[rstest]
    fn test_play_turn_human_wins_immediately(
        mut human_player: Player,
        mut computer_player: Player,
    ) {
        // Pre-hit every occupied cell of the computer fleet except one, so the next human move wins.
        let mut occupied = Vec::<Cell>::new();
        for ship in computer_player.fleet().as_ref().iter() {
            occupied.extend(ship.occupied_cells());
        }
        let winning_cell = occupied.pop().expect("fleet must occupy at least one cell");
        for cell in occupied {
            human_player.attack(&mut computer_player, &cell);
        }
        assert!(!computer_player.has_lost());

        // Force order: human plays first (so the "human_move" is actually used).
        let mut game = Game {
            players: vec![human_player, computer_player],
            last_computer_move: None,
        };
        assert!(game.is_ready());

        let winner = game.play_turn(&winning_cell).unwrap();

        assert_eq!(winner, Some(true));
        assert!(game.is_over());
    }

    #[rstest]
    fn test_get_human_when_not_ready() {
        let game = Game::new();
        assert!(game.human().is_none());
    }

    #[rstest]
    fn test_get_player_when_not_ready() {
        let game = Game::new();
        assert!(game.computer().is_none());
    }

    #[rstest]
    fn test_get_player(human_player: Player, computer_player: Player) {
        let human_name = human_player.name().to_string();
        let computer_name = computer_player.name().to_string();
        let game = Game {
            players: vec![human_player, computer_player],
            last_computer_move: None,
        };
        assert_eq!(game.human().unwrap().name(), human_name);
        assert_eq!(game.computer().unwrap().name(), computer_name);
    }
}
