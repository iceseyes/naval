use crate::engine::grid::{Cell, Grid};
use crate::engine::player::Player;
use crate::tui::state::StateModel;
use crate::tui::widgets::grid::{GridModel, Layer};
use crossterm::event::{KeyCode, KeyEvent};
use rand::random;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Style, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

pub struct BattleStateModel {
    player1_start: bool,
    player1_has_shot: bool,
    player1_won: Option<bool>,
    tactical_grid: GridModel,
    opponent_grid: GridModel,
    computer_shots: Vec<Cell>,
}

impl BattleStateModel {
    pub fn update_grid(&mut self, _computer: &Player, human: &Player) {
        let cursor = *self.opponent_grid.cursor().unwrap();
        self.opponent_grid = GridModel::new(human.shots_grid().clone());
        self.opponent_grid.set_cursor(&cursor);

        self.tactical_grid = GridModel::new(Grid::from_ships(human.fleet().as_ref()));
        self.tactical_grid
            .push_layer(Layer::Shots(self.computer_shots.clone()));
    }

    pub fn play_turn(&mut self, computer: &mut Player, human: &mut Player) {
        if self.player1_has_shot {
            if self.player1_start {
                // if player1 is the first player, evaluate its shot first
                human.attack(computer, self.opponent_grid.cursor().unwrap());

                if computer.has_lost() {
                    self.player1_won = Some(true);
                    return;
                }
            }

            let shot = Cell::random();
            computer.attack(human, &shot);
            self.computer_shots.push(shot);

            if human.has_lost() {
                self.player1_won = Some(false);
                return;
            }

            if !self.player1_start {
                // if player1 is the second player, evaluate its shot after
                human.attack(computer, self.opponent_grid.cursor().unwrap());

                if computer.has_lost() {
                    self.player1_won = Some(true);
                    return;
                }
            }

            self.player1_has_shot = false;
        }
    }
}

impl Default for BattleStateModel {
    fn default() -> Self {
        let tactical_grid = GridModel::new(Grid::default());
        let mut opponent_grid = GridModel::new(Grid::default());

        opponent_grid.enable_cursor();

        Self {
            player1_start: random(),
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

    fn update(&mut self, computer: Player, human: Option<Player>) -> (Player, Option<Player>) {
        let mut computer = computer;
        let mut human = human.unwrap();

        self.play_turn(&mut computer, &mut human);
        self.update_grid(&computer, &human);

        (computer, Some(human))
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
