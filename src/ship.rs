use crate::cell::Cell;
use crate::orientation::ShipOrientation;
use strum::Display;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Clone, Display, EnumIter)]
pub enum ShipKind {
    #[strum(serialize = "Aircraft Carrier")]
    AircraftCarrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl ShipKind {
    const AIRCRAFT_CARRIER_SIZE: u8 = 5;
    const BATTLESHIP_SIZE: u8 = 4;
    const CRUISER_SIZE: u8 = 3;
    const SUBMARINE_SIZE: u8 = 3;
    const DESTROYER_SIZE: u8 = 2;

    pub fn ship(&self, first: Cell, orientation: ShipOrientation) -> Option<Ship> {
        Ship::new(self.size(), first, orientation)
    }

    pub fn size(&self) -> u8 {
        match self {
            ShipKind::AircraftCarrier => Self::AIRCRAFT_CARRIER_SIZE,
            ShipKind::Battleship => Self::BATTLESHIP_SIZE,
            ShipKind::Cruiser => Self::CRUISER_SIZE,
            ShipKind::Submarine => Self::SUBMARINE_SIZE,
            ShipKind::Destroyer => Self::DESTROYER_SIZE,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ship {
    first_cell: Cell,
    ship_size: u8,
    orientation: ShipOrientation,
    state: u8,
}

impl Ship {
    fn new(ship_size: u8, first_cell: Cell, direction: ShipOrientation) -> Option<Self> {
        let (long, short) = match direction {
            ShipOrientation::Horizontal => (first_cell.x(), first_cell.y()),
            ShipOrientation::Vertical => (first_cell.y(), first_cell.x()),
        };

        if long <= 9 && long + ship_size - 1 <= 9 && short <= 9 {
            Some(Ship {
                first_cell,
                ship_size,
                orientation: direction,
                state: get_ship_state(ship_size),
            })
        } else {
            None
        }
    }

    pub fn cell(&self) -> &Cell {
        &self.first_cell
    }

    pub fn orientation(&self) -> &ShipOrientation {
        &self.orientation
    }

    /// Returns all board cells occupied by this ship based on its
    /// origin cell, size and direction.
    pub fn occupied_cells(&self) -> Vec<Cell> {
        let mut cells = Vec::with_capacity(self.ship_size as usize);
        match self.orientation {
            ShipOrientation::Horizontal => {
                for dx in 0..self.ship_size {
                    cells.push(Cell::bounded(self.first_cell.x() + dx, self.first_cell.y()));
                }
            }
            ShipOrientation::Vertical => {
                for dy in 0..self.ship_size {
                    cells.push(Cell::bounded(self.first_cell.x(), self.first_cell.y() + dy));
                }
            }
        }
        cells
    }

    pub fn is_sunk(&self) -> bool {
        self.state == 0
    }

    pub fn check_hit(&mut self, cell: &Cell) -> bool {
        let bit = self.is_hit(cell);

        bit.map(|bit| {
            self.state &= !(1u8 << bit);
            true
        })
        .unwrap_or(false)
    }

    pub fn is_overlapping(&self, other: &Ship) -> bool {
        let (x_start, x_end, y_start, y_end) = match self.orientation {
            ShipOrientation::Horizontal => {
                let x_start = self.first_cell.x().saturating_sub(1);
                let x_end = (self.first_cell.x() + self.ship_size + 1).min(9);
                let y_start = self.first_cell.y().saturating_sub(1);
                let y_end = (self.first_cell.y() + 1).min(9);
                (x_start, x_end, y_start, y_end)
            }
            ShipOrientation::Vertical => {
                let x_start = self.first_cell.x().saturating_sub(1);
                let x_end = (self.first_cell.x() + 1).min(9);
                let y_start = self.first_cell.y().saturating_sub(1);
                let y_end = (self.first_cell.y() + self.ship_size + 1).min(9);
                (x_start, x_end, y_start, y_end)
            }
        };

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                if other.is_hit(&Cell::bounded(x, y)).is_some() {
                    return true;
                }
            }
        }

        false
    }

    fn is_hit(&self, cell: &Cell) -> Option<u8> {
        match self.orientation {
            ShipOrientation::Horizontal
                if self.first_cell.y() == cell.y()
                    && (self.first_cell.x()..(self.first_cell.x() + self.ship_size))
                        .contains(&cell.x()) =>
            {
                Some(cell.x() - self.first_cell.x())
            }

            ShipOrientation::Vertical
                if self.first_cell.x() == cell.x()
                    && (self.first_cell.y()..(self.first_cell.y() + self.ship_size))
                        .contains(&cell.y()) =>
            {
                Some(cell.x() - self.first_cell.x())
            }

            _ => None,
        }
    }
}

fn get_ship_state(size: u8) -> u8 {
    let mut state = 0u8;
    for i in 0u8..size {
        state |= 2u8.pow(i as u32);
    }

    state
}

pub fn validate_ships(ships: &[Ship]) -> Result<(), &'static str> {
    for (index, ship) in ships.iter().enumerate() {
        for other_ship in ships.iter().skip(index + 1) {
            if ship.is_overlapping(other_ship) {
                return Err("Ships overlap");
            }
        }
    }

    Ok(())
}

pub fn as_grid(ships: &[Ship]) -> [[bool; 10]; 10] {
    let mut grid = [[false; 10]; 10];

    for ship in ships.iter() {
        for cell in ship.occupied_cells() {
            let x = cell.x() as usize;
            let y = cell.y() as usize;
            grid[y][x] = true;
        }
    }

    grid
}

pub fn display_ships(ships: &[Ship]) -> String {
    let grid = as_grid(ships);

    let mut out = "  A B C D E F G H I J \n".to_string();
    for (index, y) in grid.iter().enumerate() {
        out.push(char::from(b'0' + index as u8));
        out.push(' ');
        y.iter().for_each(|o| {
            out.push(if *o { 'X' } else { ' ' });
            out.push(' ')
        });
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::orientation::ShipOrientation;
    use crate::ship::{display_ships, Ship, ShipKind};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, ShipOrientation::Horizontal, true)]
    #[case(5, 0, ShipOrientation::Horizontal, true)]
    #[case(0, 5, ShipOrientation::Horizontal, true)]
    #[case(0, 5, ShipOrientation::Vertical, true)]
    #[case(5, 5, ShipOrientation::Horizontal, true)]
    #[case(5, 5, ShipOrientation::Vertical, true)]
    #[case(1, 1, ShipOrientation::Horizontal, true)]
    #[case(6, 1, ShipOrientation::Horizontal, false)]
    #[case(1, 6, ShipOrientation::Vertical, false)]
    #[case(1, 9, ShipOrientation::Vertical, false)]
    #[case(6, 6, ShipOrientation::Horizontal, false)]
    #[case(10, 10, ShipOrientation::Horizontal, false)]
    fn test_build_carrier(
        #[case] x: u8,
        #[case] y: u8,
        #[case] direction: ShipOrientation,
        #[case] expected: bool,
    ) {
        let ship = ShipKind::AircraftCarrier.ship(Cell::bounded(x, y), direction.clone());
        if expected {
            assert!(ship.is_some());

            let ship = ship.unwrap();
            assert_eq!(ship.ship_size, 5);
            assert_eq!(ship.orientation, direction);
            assert_eq!(ship.state, 0x1f);
            assert_eq!(ship.first_cell, Cell::bounded(x, y));
        } else {
            assert!(ship.is_none());
        }
    }

    #[test]
    fn test_build_battleship() {
        let ship = ShipKind::Battleship
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        assert_eq!(ship.ship_size, 4);
        assert_eq!(ship.state, 0x0f);
    }

    #[test]
    fn test_build_cruiser() {
        let ship = ShipKind::Cruiser
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        assert_eq!(ship.ship_size, 3);
        assert_eq!(ship.state, 0x07);
    }

    #[test]
    fn test_build_submarine() {
        let ship = ShipKind::Submarine
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        assert_eq!(ship.ship_size, 3);
        assert_eq!(ship.state, 0x07);
    }

    #[test]
    fn test_build_destroyer() {
        let ship = ShipKind::Destroyer
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        assert_eq!(ship.ship_size, 2);
        assert_eq!(ship.state, 0x03);
    }

    #[rstest]
    #[case(0, 0, true)]
    #[case(2, 0, true)]
    #[case(4, 0, true)]
    #[case(5, 0, false)]
    #[case(9, 0, false)]
    #[case(0, 1, false)]
    #[case(4, 1, false)]
    #[case(0, 9, false)]
    fn test_check_hit_horizonal_origin(#[case] x: u8, #[case] y: u8, #[case] expected: bool) {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        assert_eq!(ship.check_hit(&Cell::bounded(x, y)), expected);
    }

    #[rstest]
    #[case(0, 0, false)]
    #[case(2, 5, false)]
    #[case(4, 5, false)]
    #[case(5, 4, false)]
    #[case(7, 4, false)]
    #[case(9, 4, false)]
    #[case(5, 5, true)]
    #[case(7, 5, true)]
    #[case(9, 5, true)]
    #[case(10, 5, false)]
    #[case(5, 6, false)]
    #[case(7, 6, false)]
    #[case(9, 6, false)]
    #[case(0, 1, false)]
    #[case(4, 1, false)]
    #[case(0, 9, false)]
    fn test_check_hit_horizonal_middle(#[case] x: u8, #[case] y: u8, #[case] expected: bool) {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(5, 5), ShipOrientation::Horizontal)
            .unwrap();
        assert_eq!(ship.check_hit(&Cell::bounded(x, y)), expected);
    }

    #[rstest]
    #[case(0, 0, true)]
    #[case(0, 2, true)]
    #[case(0, 4, true)]
    #[case(0, 5, false)]
    #[case(0, 9, false)]
    #[case(1, 0, false)]
    #[case(1, 4, false)]
    #[case(9, 0, false)]
    fn test_check_hit_vertical_origin(#[case] x: u8, #[case] y: u8, #[case] expected: bool) {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(0, 0), ShipOrientation::Vertical)
            .unwrap();
        assert_eq!(ship.check_hit(&Cell::bounded(x, y)), expected);
    }

    #[rstest]
    #[case(0, 0, false)]
    #[case(5, 2, false)]
    #[case(5, 4, false)]
    #[case(4, 5, false)]
    #[case(4, 7, false)]
    #[case(4, 9, false)]
    #[case(5, 5, true)]
    #[case(5, 7, true)]
    #[case(5, 9, true)]
    #[case(5, 10, false)]
    #[case(6, 5, false)]
    #[case(6, 7, false)]
    #[case(6, 9, false)]
    #[case(1, 0, false)]
    #[case(1, 4, false)]
    #[case(9, 0, false)]
    fn test_check_hit_vertical_middle(#[case] x: u8, #[case] y: u8, #[case] expected: bool) {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(5, 5), ShipOrientation::Vertical)
            .unwrap();
        assert_eq!(ship.check_hit(&Cell::bounded(x, y)), expected);
    }

    #[test]
    fn test_check_hit_change_state() {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        ship.check_hit(&Cell::bounded(0, 0));
        assert_eq!(ship.state, 0x1e);
        ship.check_hit(&Cell::bounded(5, 0));
        assert_eq!(ship.state, 0x1e);
        ship.check_hit(&Cell::bounded(4, 0));
        assert_eq!(ship.state, 0x0e);
    }

    #[test]
    fn test_is_sunk() {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::bounded(0, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::bounded(1, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::bounded(2, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::bounded(3, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::bounded(4, 0));
        assert!(ship.is_sunk());
    }

    #[rstest]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(4, 4).unwrap(), ShipOrientation::Horizontal).unwrap())]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(4, 4).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap())]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(4, 4).unwrap(), ShipOrientation::Vertical).unwrap())]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(4, 0).unwrap(), ShipOrientation::Vertical).unwrap())]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Vertical).unwrap(),
        ShipKind::Submarine.ship(Cell::new(0, 4).unwrap(), ShipOrientation::Horizontal).unwrap())]
    fn test_is_overlapping(#[case] ship1: Ship, #[case] ship2: Ship) {
        assert!(ship1.is_overlapping(&ship2));
    }

    #[rstest]
    fn test_display_ships() {
        let ships = [];
        display_ships(&ships);

        #[rustfmt::skip]
        assert_eq!(
            display_ships(&ships),
                  "  A B C D E F G H I J \n".to_owned()
                + "0                     \n"
                + "1                     \n"
                + "2                     \n"
                + "3                     \n"
                + "4                     \n"
                + "5                     \n"
                + "6                     \n"
                + "7                     \n"
                + "8                     \n"
                + "9                     \n"
        );

        let ships = vec![
            // A: Aircraft carrier, horizontal on row 0
            ShipKind::AircraftCarrier
                .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
                .unwrap(),
            // B: Battleship, vertical starting at (0, 2)
            ShipKind::Battleship
                .ship(Cell::bounded(0, 2), ShipOrientation::Vertical)
                .unwrap(),
            // S: Submarine, horizontal at (5, 5)
            ShipKind::Submarine
                .ship(Cell::bounded(5, 5), ShipOrientation::Horizontal)
                .unwrap(),
            // C: Cruiser, vertical at (9, 0)
            ShipKind::Cruiser
                .ship(Cell::bounded(9, 0), ShipOrientation::Vertical)
                .unwrap(),
            // D: Destroyer, horizontal at (7, 9)
            ShipKind::Destroyer
                .ship(Cell::bounded(7, 9), ShipOrientation::Horizontal)
                .unwrap(),
        ];

        #[rustfmt::skip]
        assert_eq!(
            display_ships(&ships),
                  "  A B C D E F G H I J \n".to_owned()
                + "0 X X X X X         X \n"
                + "1                   X \n"
                + "2 X                 X \n"
                + "3 X                   \n"
                + "4 X                   \n"
                + "5 X         X X X     \n"
                + "6                     \n"
                + "7                     \n"
                + "8                     \n"
                + "9               X X   \n"
        );
    }
}
