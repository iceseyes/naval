use crate::engine::game::Game;
use crate::{
    engine::{
        grid::{Cell, Grid},
        player::Player,
    },
    tui::{
        state::StateModel,
        widgets::grid::{GridModel, Layer},
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Line, Style, Stylize, Widget},
    symbols::border,
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Tracks how the battle goes
pub struct BattleStateModel {
    player1_has_shot: bool,
    player1_won: Option<bool>,
    tactical_grid: GridModel,
    opponent_grid: GridModel,
    computer_shots: Vec<Cell>,
}

impl BattleStateModel {
    /// Updates the grids to reflect the current state of the game
    pub fn update_grid(&mut self, _computer: &Player, human: &Player) {
        let cursor = *self.opponent_grid.cursor().unwrap();
        self.opponent_grid = GridModel::new(human.shots_grid().clone());
        self.opponent_grid.set_cursor(&cursor);

        self.tactical_grid = GridModel::new(Grid::from_ships(human.fleet().as_ref()));
        self.tactical_grid
            .push_layer(Layer::Shots(self.computer_shots.clone()));
    }
}

impl Default for BattleStateModel {
    fn default() -> Self {
        let tactical_grid = GridModel::new(Grid::default());
        let mut opponent_grid = GridModel::new(Grid::default());

        opponent_grid.enable_cursor();

        Self {
            player1_has_shot: false,
            player1_won: None,
            tactical_grid,
            opponent_grid,
            computer_shots: Vec::new(),
        }
    }
}

impl StateModel for BattleStateModel {
    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.opponent_grid.move_cursor(|c| c.move_left()),
            KeyCode::Right => self.opponent_grid.move_cursor(|c| c.move_right()),
            KeyCode::Up => self.opponent_grid.move_cursor(|c| c.move_up()),
            KeyCode::Down => self.opponent_grid.move_cursor(|c| c.move_down()),
            KeyCode::Enter => {
                self.player1_has_shot = true;
            }

            _ => {}
        }
    }

    fn update(&mut self, game: &mut Game) {
        if self.player1_has_shot {
            match game.play_turn(self.opponent_grid.cursor().unwrap()) {
                Ok(winner) => {
                    if let Some(computer_shot) = game.last_computer_move() {
                        self.computer_shots.push(*computer_shot);
                    }

                    if let Some(human) = winner {
                        self.player1_won = Some(human);
                    }
                }
                Err(e) => {
                    panic!("{e}");
                }
            }
        }

        self.player1_has_shot = false;

        self.update_grid(game.computer().unwrap(), game.human().unwrap());
    }

    fn widget(&self) -> impl Widget {
        BattleWidget(self)
    }
}

pub struct BattleWidget<'state>(&'state BattleStateModel);

impl<'state> Widget for BattleWidget<'state> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let opponent_block = Block::bordered()
            .title(Line::from("Opponent Grid".bold()))
            .border_set(border::THICK);

        self.0
            .opponent_grid
            .widget()
            .render(opponent_block.inner(layout[0]), buf);

        opponent_block.render(layout[0], buf);

        let tactical_block = Block::bordered()
            .title(Line::from("Tactical".bold()))
            .border_set(border::THICK);

        self.0
            .tactical_grid
            .widget()
            .render(tactical_block.inner(layout[1]), buf);

        tactical_block.render(layout[1], buf);

        if let Some(player1_won) = self.0.player1_won {
            let popup_area = Rect {
                x: area.width / 4,
                y: area.height / 3,
                width: area.width / 2,
                height: area.height / 3,
            };
            Clear.render(popup_area, buf);
            let bad_popup = Paragraph::new(if player1_won {
                Span::raw("You WIN!!!").bold()
            } else {
                Span::raw("You lose! :(").bold()
            })
            .wrap(Wrap { trim: true })
            .style(Style::new().black())
            .centered()
            .block(
                Block::new()
                    .title("Match Over!")
                    .title_style(Style::new().black().bold())
                    .borders(Borders::ALL)
                    .border_style(Style::new().red())
                    .on_white(),
            );

            bad_popup.render(popup_area, buf);
        }
    }
}
