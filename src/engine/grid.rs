//! This module represents a battleship space for naval battle game.
//!
//! The naval battle game is played on a 10x10 grid where players place ships and take notes of hits and misses.
//! Each cell in the grid can be empty, occupied by a ship, or report a shoot result: miss, hit, or sunk.
//! When a shoot is made, you can hit a ship or miss it. Whenever a ship is completely hit, it is considered sunk.
//!
//! The battleship grid is divided into cells, each represented by the `Cell` struct with x and y coordinates.
//! The `Grid` struct represents the entire 10x10 grid and maintains the state of each cell using the `CellState` enum.
//! The `CellState` enum has four variants: `Empty`, `Occupied`, `Hit`, and `Sunk`.
//!
use crate::engine::fleet::Ship;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// Represents the state of a cell in the battleship grid.
///
/// A cell can be empty, occupied by a ship part or report a shoot result: miss or hit.
/// A hit occurs when you shoot toward a cell with was occupied, a miss if it wasn't.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum CellState {
    /// The default state of a cell, indicating that it is empty and has not been shot at.
    #[default]
    Empty,

    /// Indicates that the cell is occupied by a ship but has not been hit yet.
    Occupied,

    /// Indicates that the cell has been beaten by a shoot, but the cell was empty.
    Miss,

    /// Indicates that the cell has been hit by a shoot and was occupied by a ship.
    Hit,
}

/// Represents a Cell error.
///
/// A cell is defined only between (0,0) -> (9,9). Any other coordinate is invalid
///
#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum Error {
    /// The X value is out of range
    #[error("{0} is not a valid X coordinate")]
    InvalidX(u8),

    /// The Y value is out of range
    #[error("{0} is not a valid Y coordinate")]
    InvalidY(u8),

    /// Both X and Y coordinates are out of range
    #[error("<{0}, {1}> is not a valid cell on the grid")]
    InvalidCoordinates(u8, u8),

    /// The string doesn't represent a valid cell.
    #[error("{0} does not represent a valid cell")]
    InvalidFormat(String),
}

/// Represents a cell in the battleship grid with x and y coordinates.
///
/// The x coordinate corresponds to the column (0-9) and the y coordinate corresponds to the row (0-9).
/// From the player's perspective, (0,0) is the top-left corner of the grid and (9,9) is the bottom-right corner.
/// Moreover, the x coordinate is usually represented by letters A-J, and the y one uses numbers from 1 to 10.
///
/// For example, the cell at (0,0) is represented as "A1", and the cell at (9,9) is "J10".
///
/// A cell simply holds the coordinates and provides methods for creation and string representation.
/// It doesn't manage any state or behavior related to ships or shooting; that is handled by the `Grid` struct.
///
/// A cell out of bounds will be clamped to the nearest valid value (0-9).
///
/// # Examples
/// ```rust
/// use crate::cell::Cell;
/// use std::str::FromStr;
/// let cell = Cell::from_str("A1").unwrap();
/// assert_eq!(cell, Cell::new(0, 0).unwrap());
/// let cell = Cell::from_str("J10").unwrap();
/// assert_eq!(cell, Cell::new(9, 9).unwrap());
/// let cell = Cell::new(5, 7).unwrap();
/// assert_eq!(format!("{}", cell), "F8");
/// ```
///
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Cell {
    /// The x coordinate (0-9)
    x: u8,

    /// The y coordinate (0-9)
    y: u8,
}

impl Cell {
    const MAX_X: u8 = 9;
    const MAX_Y: u8 = 9;

    /// Creates a new Cell with the given x and y coordinates.
    ///
    /// If x and/or y are out of range, an error was returned
    pub fn new(x: u8, y: u8) -> Result<Self, Error> {
        if x > Self::MAX_X {
            if y > Self::MAX_Y {
                Err(Error::InvalidCoordinates(x, y))
            } else {
                Err(Error::InvalidX(x))
            }
        } else if y > Self::MAX_Y {
            Err(Error::InvalidY(y))
        } else {
            Ok(Self { x, y })
        }
    }

    /// Creates a new Cell with the given x and y coordinates.
    ///
    /// If the coordinates are out of bounds, they will be clamped to the nearest valid value (0-9).
    ///
    /// # Examples
    /// ```rust
    /// use crate::cell::Cell;
    ///
    /// let cell = Cell::new(0, 0);
    /// assert_eq!(cell.x(), 0);
    /// assert_eq!(cell.y(), 0);
    ///
    /// let cell = Cell::new(10, 15);
    /// assert_eq!(cell.x(), 9);
    /// assert_eq!(cell.y(), 9);
    ///
    /// let cell = Cell::new(5, 7);
    /// assert_eq!(cell.x(), 5);
    /// assert_eq!(cell.y(), 7);
    /// ```
    pub fn bounded(x: u8, y: u8) -> Self {
        let x = min(x, Self::MAX_X);
        let y = min(y, Self::MAX_Y);

        Cell { x, y }
    }

    /// Return a cell using random coordinates.
    pub fn random() -> Self {
        let x = rand::random::<u8>() % Self::MAX_X;
        let y = rand::random::<u8>() % Self::MAX_Y;

        Cell { x, y }
    }

    /// Returns the x coordinate of this cell.
    pub fn x(&self) -> u8 {
        self.x
    }

    /// Returns the y coordinate of this cell.
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Moves this cell to the left.
    ///
    /// This method automatically wraps around if the cell is at the leftmost position.
    pub fn move_left(&mut self) {
        self.x = self.x.checked_sub(1).unwrap_or(Self::MAX_X);
    }

    /// Moves this cell to the right.
    ///
    /// This method automatically wraps around if the cell is at the rightmost position.
    pub fn move_right(&mut self) {
        if self.x == Self::MAX_X {
            self.x = 0;
        } else {
            self.x = self.x.saturating_add(1);
        }
    }

    /// Moves this cell up
    ///
    /// This method automatically wraps around if the cell is on the top
    pub fn move_up(&mut self) {
        self.y = self.y.checked_sub(1).unwrap_or(Self::MAX_Y);
    }

    /// Moves this cell down
    ///
    /// This method automatically wraps around if the cell is on the bottom
    pub fn move_down(&mut self) {
        if self.y == Self::MAX_Y {
            self.y = 0;
        } else {
            self.y = self.y.saturating_add(1);
        }
    }
}

impl FromStr for Cell {
    type Err = Error;

    /// Parses a string representation of a cell into a Cell struct.
    ///
    /// The string should be in the format "A1" to "J10", where the letter represents the x coordinate (A-J)
    /// and the number represents the y coordinate (1-10).
    /// It is supposed to be case-insensitive, so "a1" is also valid. No spaces or other characters are allowed.
    /// Therefore, " A1", "A 1", "A11", "K1", "A0", " d6  are considered invalid, but it is admitted to have
    /// the number with leading zeros, e.g., "A01" is valid and equivalent to "A1".
    /// Anyway the parsed coordinates must be valid, so only values between 1 and 10 (inclusive) will be accepted.
    ///
    /// The string must be well-formed; otherwise, an error is returned.
    /// The error is always and [`Error::InvalidFormat`], reporting which was the original string submitted.
    ///
    /// For example:
    /// ```rust
    /// use crate::cell::Cell;
    /// use std::str::FromStr;
    ///
    /// let cell = Cell::from_str("A1").unwrap();
    /// assert_eq!(cell, Cell::new(0, 0).unwrap());
    ///
    /// let cell = Cell::from_str("J10").unwrap();
    /// assert_eq!(cell, Cell::new(9, 9).unwrap());
    ///
    /// let cell = Cell::from_str("d6").unwrap();
    /// assert_eq!(cell, Cell::new(3, 5).unwrap());
    ///
    /// /// let cell = Cell::from_str("d06").unwrap();
    /// assert_eq!(cell, Cell::new(3, 5).unwrap());
    ///
    /// assert!(Cell::from_str("K1").is_err());
    /// assert!(Cell::from_str("A0").is_err());
    /// assert!(Cell::from_str("  A5  ").is_err());
    /// ```
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let x_char = chars.next().ok_or(Error::InvalidFormat(s.to_string()))?;
        let x = x_char.to_ascii_uppercase() as u8;
        let y = chars
            .as_str()
            .parse::<u8>()
            .map_err(|_| Error::InvalidFormat(s.to_string()))?;

        if !x_char.is_ascii_alphabetic()
            || (x - b'A') > Self::MAX_Y
            || y == 0
            || (y - 1) > Self::MAX_Y
        {
            Err(Error::InvalidFormat(s.to_string()))
        } else {
            Ok(Cell::bounded(x - b'A', y - 1))
        }
    }
}

impl Display for Cell {
    /// Formats the cell as a string in the format "A1" to "J10".
    ///
    /// The y coordinate is represented without leading zeros, while the x coordinate is
    /// represented by letters A-J, as usual.
    ///
    /// # Examples
    /// ```rust
    /// use crate::cell::Cell;
    ///
    /// let cell = Cell::new(0, 0);
    /// assert_eq!(format!("{}", cell), "A1");
    ///
    /// let cell = Cell::new(9, 9);
    /// assert_eq!(format!("{}", cell), "J10");
    ///
    /// let cell = Cell::new(5, 7);
    /// assert_eq!(format!("{}", cell), "F8");
    /// ```
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = (self.x + b'A') as char;
        write!(f, "{}{}", x, self.y + 1)
    }
}

/// Represents the battleship grid for the naval battle game.
///
/// The grid is a 10x10 matrix of cells, where each cell can be in one of the states defined by the `CellState` enum.
/// The default state of the grid is empty, with all cells set to [`CellState::Empty`].
/// The grid just record the state of each cell; it doesn't manage any behavior related to ships or shooting.
/// Therefore, when you set a cell state, it doesn't check if the transition is valid or not (e.g. from empty to hit).
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Grid {
    cells: [[CellState; 10]; 10],
}

impl Grid {
    /// Build a new grid with only empty or occupied cells.
    ///
    /// The occupied cells match the position and the size of every ship in the slice passed as argument
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ships = vec![ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap()];
    /// let grid = Grid::from_ships(ships.as_slice();
    /// assert!(!grid.is_empty())
    /// ```
    ///
    pub fn from_ships(ships: &[Ship]) -> Self {
        let mut grid = Grid::default();
        ships.iter().for_each(|ship| grid.add_ship(ship));
        grid
    }

    /// Check for all the cells in the grid are empty.
    ///
    /// Return `true` if all the cells in the grid are marked as [CellState::Empty], `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| cell == &CellState::Empty))
    }

    /// The state of the passed cell
    pub fn at(&self, cell: &Cell) -> &CellState {
        &self.cells[cell.y as usize][cell.x as usize]
    }

    /// Overwrite the chosen cell with the passed state, it doesn't mind which was its previous state.
    pub fn mark(&mut self, cell: &Cell, state: CellState) {
        self.cells[cell.y as usize][cell.x as usize] = state;
    }

    /// Add a ship to the grid.
    pub fn add_ship(&mut self, ship: &Ship) {
        for cell in ship.occupied_cells().iter() {
            self.mark(cell, CellState::Occupied);
        }
    }
}

impl Display for Grid {
    /// Format the grid in a table 10x10 with references.
    ///
    /// The output consists in a ascii representation of the grid in a way like this:
    ///
    /// ```text
    ///     A B C D E F G H I J
    ///  00
    ///  01
    ///  02    ................
    ///  ..    ..Grid content..
    ///  09    ................
    ///  10
    /// ```
    ///
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = "   A B C D E F G H I J \n".to_string();
        for (y, row) in self.cells.iter().enumerate() {
            output = format!("{output}{:02} ", y + 1);
            for cell in row.iter() {
                output.push(match cell {
                    CellState::Empty => ' ',
                    CellState::Occupied => '#',
                    CellState::Miss => 'O',
                    CellState::Hit => 'X',
                });
                output.push(' ');
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fleet::ShipKind;
    use crate::engine::fleet::ShipOrientation;
    use rstest::rstest;

    #[rstest]
    #[case(Cell::bounded(0, 0), 0, 0)]
    #[case(Cell::bounded(9, 9), 9, 9)]
    #[case(Cell::bounded(5, 7), 5, 7)]
    #[case(Cell::bounded(10, 15), 9, 9)]
    #[case(Cell::bounded(10, 5), 9, 5)]
    #[case(Cell::bounded(7, 15), 7, 9)]
    #[case(Cell::bounded(255, 255), 9, 9)]
    fn test_bounded_cell(#[case] cell: Cell, #[case] expected_x: u8, #[case] expected_y: u8) {
        assert_eq!(cell.x(), expected_x);
        assert_eq!(cell.y(), expected_y);
    }

    #[rstest]
    #[case(Cell::new(0, 0), 0, 0)]
    #[case(Cell::new(9, 9), 9, 9)]
    #[case(Cell::new(5, 7), 5, 7)]
    fn test_new_cell(
        #[case] cell: Result<Cell, Error>,
        #[case] expected_x: u8,
        #[case] expected_y: u8,
    ) {
        let cell = cell.unwrap();
        assert_eq!(cell.x(), expected_x);
        assert_eq!(cell.y(), expected_y);
    }

    #[rstest]
    #[case(Cell::new(10, 15), Error::InvalidCoordinates(10, 15))]
    #[case(Cell::new(10, 5), Error::InvalidX(10))]
    #[case(Cell::new(7, 15), Error::InvalidY(15))]
    #[case(Cell::new(255, 255), Error::InvalidCoordinates(255, 255))]
    fn test_error_cell(#[case] cell: Result<Cell, Error>, #[case] expected_err: Error) {
        let cell = cell.err().unwrap();
        assert_eq!(cell, expected_err);
    }

    #[rstest]
    #[case("A1", Cell::new(0, 0))]
    #[case("J10", Cell::new(9, 9))]
    #[case("d6", Cell::new(3, 5))]
    #[case("D06", Cell::new(3, 5))]
    #[case("e0001", Cell::new(4, 0))]
    fn test_cell_from_str(#[case] s: &str, #[case] expected: Result<Cell, Error>) {
        let cell = Cell::from_str(s).unwrap();
        assert_eq!(cell, expected.unwrap());
    }

    #[rstest]
    #[case("K1")]
    #[case("A0")]
    #[case("  A5  ")]
    #[case("A15")]
    fn test_cell_from_str_errors(#[case] s: &str) {
        assert!(matches!(
            Cell::from_str(s),
            Err(e) if e == Error::InvalidFormat(s.to_string())
        ));
    }

    #[rstest]
    #[case(Cell::bounded(0, 0), "A1")]
    #[case(Cell::bounded(9, 9), "J10")]
    #[case(Cell::bounded(5, 7), "F8")]
    fn test_cell_display(#[case] cell: Cell, #[case] expected: &str) {
        assert_eq!(format!("{}", cell), expected);
    }

    #[rstest]
    #[case(Cell::bounded(5, 5), Cell::bounded(4, 5))]
    #[case(Cell::bounded(0, 5), Cell::bounded(9, 5))]
    fn test_move_left(#[case] mut cell: Cell, #[case] expected: Cell) {
        cell.move_left();
        assert_eq!(cell, expected);
    }

    #[rstest]
    #[case(Cell::bounded(5, 5), Cell::bounded(6, 5))]
    #[case(Cell::bounded(9, 5), Cell::bounded(0, 5))]
    fn test_move_right(#[case] mut cell: Cell, #[case] expected: Cell) {
        cell.move_right();
        assert_eq!(cell, expected);
    }

    #[rstest]
    #[case(Cell::bounded(5, 5), Cell::bounded(5, 4))]
    #[case(Cell::bounded(5, 0), Cell::bounded(5, 9))]
    fn test_move_up(#[case] mut cell: Cell, #[case] expected: Cell) {
        cell.move_up();
        assert_eq!(cell, expected);
    }

    #[rstest]
    #[case(Cell::bounded(5, 5), Cell::bounded(5, 6))]
    #[case(Cell::bounded(5, 9), Cell::bounded(5, 0))]
    fn test_move_down(#[case] mut cell: Cell, #[case] expected: Cell) {
        cell.move_down();
        assert_eq!(cell, expected);
    }

    #[rstest]
    fn test_new_grid_is_always_empty() {
        assert!(Grid::default().is_empty());
    }

    #[rstest]
    fn test_grid_from_ships() {
        let ships = [
            ShipKind::AircraftCarrier
                .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
                .unwrap(),
            ShipKind::Destroyer
                .ship(Cell::bounded(3, 3), ShipOrientation::Vertical)
                .unwrap(),
        ];

        let grid = Grid::from_ships(ships.as_slice());
        assert_eq!(*grid.at(&Cell::bounded(0, 0)), CellState::Occupied);
        assert_eq!(*grid.at(&Cell::bounded(1, 0)), CellState::Occupied);
        assert_eq!(*grid.at(&Cell::bounded(3, 3)), CellState::Occupied);
        assert_eq!(*grid.at(&Cell::bounded(3, 4)), CellState::Occupied);
        assert_eq!(*grid.at(&Cell::bounded(4, 3)), CellState::Empty);
        assert_eq!(*grid.at(&Cell::bounded(2, 3)), CellState::Empty);
    }

    #[rstest]
    fn test_grid_mark_and_at() {
        let mut grid = Grid::default();
        assert_eq!(*grid.at(&Cell::bounded(3, 3)), CellState::Empty);
        grid.mark(&Cell::bounded(3, 3), CellState::Occupied);
        assert_eq!(*grid.at(&Cell::bounded(3, 3)), CellState::Occupied);
        grid.mark(&Cell::bounded(3, 3), CellState::Miss);
        assert_eq!(*grid.at(&Cell::bounded(3, 3)), CellState::Miss);
    }

    #[rustfmt::skip]
    #[rstest]
    fn test_display_grid() {
        let mut grid = Grid::default();
        assert_eq!(
            format!("{}", grid),
                  "   A B C D E F G H I J \n".to_owned()
                + "01                     \n"
                + "02                     \n"
                + "03                     \n"
                + "04                     \n"
                + "05                     \n"
                + "06                     \n"
                + "07                     \n"
                + "08                     \n"
                + "09                     \n"
                + "10                     \n"
        );
    grid.mark(&Cell::bounded(0, 0), CellState::Occupied);
    grid.mark(&Cell::bounded(1, 1), CellState::Miss);
    grid.mark(&Cell::bounded(2, 2), CellState::Hit);
    assert_eq!(
            format!("{}", grid),
                  "   A B C D E F G H I J \n".to_owned()
                + "01 #                   \n"
                + "02   O                 \n"
                + "03     X               \n"
                + "04                     \n"
                + "05                     \n"
                + "06                     \n"
                + "07                     \n"
                + "08                     \n"
                + "09                     \n"
                + "10                     \n"
        );
    }
}
