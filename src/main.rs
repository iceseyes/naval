mod cell;
mod player;
mod ship;
mod widgets;

use crate::cell::Grid;
use crate::player::Player;
use crate::ship::{Fleet, ShipKind, ShipOrientation};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Buffer, Constraint, Direction, Layout, Line, Rect, Widget};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};
use std::io;
use widgets::grid_widget::GridModel;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| NavalBattleTui::new().run(terminal))
}

struct NavalBattleTui {
    players: Vec<Player>,
    fleet_grid: GridModel,
    exit: bool,
}

impl NavalBattleTui {
    pub fn new() -> Self {
        let grid = Grid::from_ships(Fleet::build(|kind| kind.random()).as_ref());

        Self {
            players: Vec::new(),
            fleet_grid: GridModel::new(grid),
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
            KeyCode::Left => self.fleet_grid.move_cursor(|c| c.move_left()),
            KeyCode::Right => self.fleet_grid.move_cursor(|c| c.move_right()),
            KeyCode::Up => self.fleet_grid.move_cursor(|c| c.move_up()),
            KeyCode::Down => self.fleet_grid.move_cursor(|c| c.move_down()),
            KeyCode::Char('s') => {
                self.fleet_grid.pop_layer();
                self.fleet_grid.enable_cursor();
                self.fleet_grid
                    .push_layer(widgets::grid_widget::Layer::Ship(
                        ShipKind::AircraftCarrier
                            .ship(
                                *self.fleet_grid.cursor().unwrap(),
                                ShipOrientation::Horizontal,
                            )
                            .unwrap(),
                    ))
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &NavalBattleTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Naval Battle ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        block.clone().render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(block.inner(area));

        let fleet_block = Block::bordered()
            .title(Line::from("Fleet".bold()))
            .border_set(border::THICK);
        self.fleet_grid
            .widget()
            .render(fleet_block.inner(layout[0]), buf);
        fleet_block.render(layout[0], buf);

        let notes_block = Block::bordered()
            .title(Line::from("Notes".bold()))
            .border_set(border::THICK);

        notes_block.render(layout[1], buf);
    }
}
