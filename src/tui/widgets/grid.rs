use crate::engine::fleet::Ship;
use crate::engine::grid::{Cell, CellState, Grid};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect, Spacing};
use ratatui::prelude::{Stylize, Widget};
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, Paragraph};

pub enum Layer {
    Ship(Ship),
    Shots(Vec<Cell>),
}

impl Layer {
    fn apply<'block>(&self, cell: &Cell, state: &CellState, block: Block<'block>) -> Block<'block> {
        match self {
            Self::Ship(ship) => match state {
                CellState::Empty if ship.occupied_cells().contains(cell) => block.on_yellow(),
                CellState::Occupied if ship.occupied_cells().contains(cell) => block.on_red(),
                _ => block,
            },
            Self::Shots(cells) => match state {
                CellState::Empty if cells.contains(cell) => block.on_magenta(),
                CellState::Occupied if cells.contains(cell) => block.on_red(),
                _ => block,
            },
        }
    }
}

/// The state for a grid widget
///
/// Besides the grid itself, the model also keeps track of the cursor position and layers.
/// Cursor is a grid position used to highlight the current active cell.
/// Layers are used to overlay additional information on the grid. Layers are designed as a stack:
///  the last pushed layer is rendered on top of all other layers. You can push or pop layers.
pub struct GridModel {
    grid: Grid,
    cursor: Option<Cell>,
    layers: Vec<Layer>,
}

impl GridModel {
    /// Creates a new grid model with the given grid.
    ///
    /// The model does not have a cursor enabled by default.
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            cursor: None,
            layers: Vec::new(),
        }
    }

    /// Set a new cursor position for this grid.
    pub fn set_cursor(&mut self, p0: &Cell) {
        self.cursor = Some(p0.clone());
    }

    /// Returns the cursor cell of this grid.
    pub fn cursor(&self) -> Option<&Cell> {
        self.cursor.as_ref()
    }

    /// Returns a widget that renders this grid.
    ///
    /// This grid must have a lifetime that outlives the widget.
    pub fn widget<'model, 'widget>(&'model self) -> GridWidget<'widget>
    where
        'model: 'widget,
    {
        GridWidget::new(self)
    }

    /// Enables the cursor for this grid.
    ///
    /// The cursor will be set to the first cell of the grid if it is not already set.
    pub fn enable_cursor(&mut self) {
        if self.cursor.is_none() {
            self.cursor = Some(Cell::bounded(0, 0));
        }
    }

    /// Disables the cursor for this grid.
    pub fn disable_cursor(&mut self) {
        self.cursor = None;
    }

    /// Adds a layer over the grid.
    ///
    /// The layer will be added to the top of the grid and all other layers
    pub fn push_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Removes the topmost layer from the grid.
    pub fn pop_layer(&mut self) -> Option<Layer> {
        self.layers.pop()
    }

    /// Add a ship to the grid
    pub fn add_ship(&mut self, ship: &Ship) {
        self.grid.add_ship(ship);
    }

    /// Applies the given function to the cursor cell, if any.
    /// If the cursor is not set, it will be set to the first cell of the grid.
    pub fn move_cursor<MoveFn>(&mut self, move_func: MoveFn)
    where
        MoveFn: FnOnce(&mut Cell),
    {
        if let Some(cursor) = &mut self.cursor {
            move_func(cursor);
        } else {
            self.enable_cursor()
        }
    }
}

/// A widget that renders a grid.
pub struct GridWidget<'app> {
    grid_model: &'app GridModel,
}

impl<'app> GridWidget<'app> {
    /// Creates a new grid widget.
    pub fn new(grid_model: &'app GridModel) -> Self {
        Self { grid_model }
    }

    fn header_block<'c>(&'app self, cell_block: Block<'c>) -> Block<'c> {
        cell_block.on_dark_gray().white()
    }

    fn cell_block<'c>(&'app self, cell: &Cell, cell_block: Block<'c>) -> Block<'c> {
        let block = match self.grid_model.grid.at(cell) {
            CellState::Empty => cell_block.on_light_blue(),
            CellState::Occupied => cell_block.on_light_green(),
            CellState::Miss => cell_block.on_light_cyan(),
            CellState::Hit => cell_block.on_light_red(),
        };

        self.grid_model.layers.iter().fold(block, |block, layer| {
            layer.apply(cell, self.grid_model.grid.at(cell), block)
        })
    }
}

impl<'app> Widget for &GridWidget<'app> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Splits the area into rows and columns. There are 10x10 blocks that represent the grid.
        // An additional row and column are reserved for the column and row labels. Therefore, there are 11x11 blocks.
        // Pay attention that the block in position 0,0 is empty: it is not a part of the grid, neither is a label.
        let horizontal = Layout::horizontal([Constraint::Fill(1); 11]).spacing(Spacing::Overlap(1));
        let vertical = Layout::vertical([Constraint::Fill(1); 11]).spacing(Spacing::Overlap(1));

        let rows = vertical.split(area);
        rows.iter().enumerate().for_each(|(row, area)| {
            let cells = horizontal.split(*area).to_vec();
            for (col, cell_area) in cells.iter().enumerate() {
                // Choose the block content and color based on its role and position in the grid
                let mut cell_block = Block::bordered().merge_borders(MergeStrategy::Exact);
                let content = match (row, col) {
                    // the first row of the grid is reserved for the column labels (letters A to J)
                    (0, col) if col > 0 => {
                        cell_block = self.header_block(cell_block);
                        format!("{}", char::from_u32('A' as u32 + col as u32 - 1).unwrap())
                    }

                    // the first column of the grid is reserved for the row labels (numbers from 1 to 10)
                    (row, 0) if row > 0 => {
                        cell_block = self.header_block(cell_block);
                        format!("{:02}", row)
                    }

                    // render the content of the grid's cells
                    (row, col) if row > 0 || col > 0 => {
                        let current_cell = Cell::new(col as u8 - 1, row as u8 - 1).unwrap();
                        cell_block = self.cell_block(&current_cell, cell_block);

                        if let Some(cursor) = &self.grid_model.cursor
                            && current_cell == *cursor
                        {
                            "X".to_string()
                        } else {
                            String::new()
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

                // Render the cell's border and background
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
