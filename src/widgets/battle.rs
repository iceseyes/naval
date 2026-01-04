use crate::grid::{Cell, Grid};
use crate::player::Player;
use crate::state::StateModel;
use crate::widgets::grid::{GridModel, Layer};
use crossterm::event::{KeyCode, KeyEvent};
use rand::random;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::Block;

pub struct BattleStateModel {
    player1_start: bool,
    player1_has_shot: bool,
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
}

impl Default for BattleStateModel {
    fn default() -> Self {
        let tactical_grid = GridModel::new(Grid::default());
        let mut opponent_grid = GridModel::new(Grid::default());

        opponent_grid.enable_cursor();

        Self {
            player1_start: random(),
            player1_has_shot: false,
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
        if self.player1_has_shot {
            if self.player1_start {
                // if player1 is the first player, evaluate its shot first
                human.attack(&mut computer, self.opponent_grid.cursor().unwrap());
            }

            let shot = Cell::random();
            computer.attack(&mut human, &shot);
            self.computer_shots.push(shot);

            if !self.player1_start {
                // if player1 is the second player, evaluate its shot after
                human.attack(&mut computer, self.opponent_grid.cursor().unwrap());
            }

            self.player1_has_shot = false;
        }

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
    }
}
