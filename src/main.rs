mod cell;
mod player;
mod ship;

use crate::cell::{Cell, CellState, Grid};
use crate::player::Player;
use crate::ship::Fleet;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Spacing;
use ratatui::prelude::{Alignment, Buffer, Constraint, Direction, Layout, Line, Rect, Widget};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| NavalBattleTui::new().run(terminal))
}

struct NavalBattleTui {
    players: Vec<Player>,
    fleet_grid: GridWidget,
    exit: bool,
}

impl NavalBattleTui {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            fleet_grid: GridWidget {
                grid: Grid::from_ships(Fleet::build(|kind| kind.random()).as_ref()),
                cursor: None,
            },
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
        self.fleet_grid.render(fleet_block.inner(layout[0]), buf);
        fleet_block.render(layout[0], buf);

        let notes_block = Block::bordered()
            .title(Line::from("Notes".bold()))
            .border_set(border::THICK);

        notes_block.render(layout[1], buf);
    }
}

struct GridWidget {
    grid: Grid,
    cursor: Option<Cell>,
}

impl GridWidget {
    /// Applies the given function to the cursor cell, if any.
    /// If the cursor is not set, it will be set to the first cell of the grid.
    fn move_cursor<MoveFn>(&mut self, move_func: MoveFn)
    where
        MoveFn: FnOnce(&mut Cell),
    {
        if let Some(cursor) = &mut self.cursor {
            move_func(cursor);
        } else {
            self.cursor = Some(Cell::bounded(0, 0));
        }
    }
}

impl Widget for &GridWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Fill(1); 11]).spacing(Spacing::Overlap(1));
        let vertical = Layout::vertical([Constraint::Fill(1); 11]).spacing(Spacing::Overlap(1));

        let rows = vertical.split(area);
        rows.iter().enumerate().for_each(|(row, area)| {
            let cells = horizontal.split(*area).to_vec();
            for (col, cell_area) in cells.iter().enumerate() {
                // Choose the content and the color of the cell
                let mut cell_block = Block::bordered().merge_borders(MergeStrategy::Exact);
                let content = match (row, col) {
                    (0, col) if col > 0 => {
                        cell_block = cell_block.on_dark_gray().white();
                        format!("{}", char::from_u32('A' as u32 + col as u32 - 1).unwrap())
                    }

                    (row, 0) if row > 0 => {
                        cell_block = cell_block.on_dark_gray().white();
                        format!("{:02}", row)
                    }

                    (row, col) if row > 0 || col > 0 => {
                        let current_cell = Cell::new(col as u8 - 1, row as u8 - 1).unwrap();
                        cell_block = match self.grid.at(&current_cell) {
                            CellState::Empty => cell_block.on_light_blue(),
                            CellState::Occupied => cell_block.on_light_green(),
                            CellState::Miss => cell_block.on_light_cyan(),
                            CellState::Hit => cell_block.on_light_red(),
                        };

                        if let Some(cursor) = &self.cursor
                            && current_cell == *cursor
                        {
                            "X".to_string()
                        } else {
                            "".to_string()
                        }
                    }

                    _ => "".to_string(),
                };

                // Center vertically by creating a layout inside the cell that picks the middle line
                let inner_area = cell_block.inner(*cell_area);
                let vertical_center_layout = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ])
                .split(inner_area);
                cell_block.render(*cell_area, buf);

                // Render the content of the cell
                Paragraph::new(content)
                    .bold()
                    .alignment(Alignment::Center)
                    .render(vertical_center_layout[1], buf);
            }
        });
    }
}
