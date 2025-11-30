#[derive(Debug, PartialEq, Clone)]
pub enum ShipDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        Cell { x, y }
    }
}

pub struct Ship {
    first_cell: Cell,
    ship_size: u8,
    direction: ShipDirection,
    state: u8,
}

impl Ship {
    const AIRCRAFT_CARRIER_SIZE: u8 = 5;
    const BATTLESHIP_SIZE: u8 = 4;
    const CRUISER_SIZE: u8 = 3;
    const SUBMARINE_SIZE: u8 = 3;
    const DESTROYER_SIZE: u8 = 2;

    pub fn new(ship_size: u8, first_cell: Cell, direction: &ShipDirection) -> Option<Self> {
        let (long, short) = match direction {
            ShipDirection::Horizontal => (first_cell.x, first_cell.y),
            ShipDirection::Vertical => (first_cell.y, first_cell.x),
        };

        if long + ship_size - 1 <= 9 && short <= 9 {
            Some(Ship {
                first_cell,
                ship_size,
                direction: direction.clone(),
                state: get_ship_state(ship_size),
            })
        } else {
            None
        }
    }

    pub fn aircraft_carrier(first_cell: Cell, direction: &ShipDirection) -> Option<Self> {
        Ship::new(Self::AIRCRAFT_CARRIER_SIZE, first_cell, direction)
    }

    pub fn battleship(first_cell: Cell, direction: &ShipDirection) -> Option<Self> {
        Ship::new(Self::BATTLESHIP_SIZE, first_cell, direction)
    }

    pub fn cruiser(first_cell: Cell, direction: &ShipDirection) -> Option<Self> {
        Ship::new(Self::CRUISER_SIZE, first_cell, direction)
    }

    pub fn submarine(first_cell: Cell, direction: &ShipDirection) -> Option<Self> {
        Ship::new(Self::SUBMARINE_SIZE, first_cell, direction)
    }

    pub fn destroyer(first_cell: Cell, direction: &ShipDirection) -> Option<Self> {
        Ship::new(Self::DESTROYER_SIZE, first_cell, direction)
    }

    pub fn is_sunk(&self) -> bool {
        self.state == 0
    }

    pub fn check_hit(&mut self, cell: &Cell) -> bool {
        let bit = match self.direction {
            ShipDirection::Horizontal
                if self.first_cell.y == cell.y
                    && (self.first_cell.x..(self.first_cell.x + self.ship_size))
                        .contains(&cell.x) =>
            {
                Some(cell.x - self.first_cell.x)
            }

            ShipDirection::Vertical
                if self.first_cell.x == cell.x
                    && (self.first_cell.y..(self.first_cell.y + self.ship_size))
                        .contains(&cell.y) =>
            {
                Some(cell.x - self.first_cell.x)
            }

            _ => None,
        };

        bit.map(|bit| {
            self.state &= !(1u8 << bit);
            true
        })
        .unwrap_or(false)
    }
}

fn get_ship_state(size: u8) -> u8 {
    let mut state = 0u8;
    for i in 0u8..size {
        state |= 2u8.pow(i as u32);
    }

    state
}

pub struct Battlefield {
    ships: Vec<Ship>,
    notes_table: [[u8; 10]; 10],
}

#[cfg(test)]
mod tests {
    use crate::battlefield::{Cell, Ship, ShipDirection};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(2, 2)]
    #[case(3, 0)]
    fn test_build_cell(#[case] x: u8, #[case] y: u8) {
        let cell = Cell::new(x, y);
        assert_eq!(cell.x, x);
        assert_eq!(cell.y, y);
    }

    #[rstest]
    #[case(0, 0, ShipDirection::Horizontal, true)]
    #[case(5, 0, ShipDirection::Horizontal, true)]
    #[case(0, 5, ShipDirection::Horizontal, true)]
    #[case(0, 5, ShipDirection::Vertical, true)]
    #[case(5, 5, ShipDirection::Horizontal, true)]
    #[case(5, 5, ShipDirection::Vertical, true)]
    #[case(1, 1, ShipDirection::Horizontal, true)]
    #[case(6, 1, ShipDirection::Horizontal, false)]
    #[case(1, 6, ShipDirection::Vertical, false)]
    #[case(1, 9, ShipDirection::Vertical, false)]
    #[case(6, 6, ShipDirection::Horizontal, false)]
    #[case(10, 10, ShipDirection::Horizontal, false)]
    fn test_build_carrier(
        #[case] x: u8,
        #[case] y: u8,
        #[case] direction: ShipDirection,
        #[case] expected: bool,
    ) {
        let ship = Ship::aircraft_carrier(Cell::new(x, y), &direction);
        if expected {
            assert!(ship.is_some());

            let ship = ship.unwrap();
            assert_eq!(ship.ship_size, 5);
            assert_eq!(ship.direction, direction);
            assert_eq!(ship.state, 0x1f);
            assert_eq!(ship.first_cell, Cell::new(x, y));
        } else {
            assert!(ship.is_none());
        }
    }

    #[test]
    fn test_build_battleship() {
        let ship = Ship::battleship(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
        assert_eq!(ship.ship_size, 4);
        assert_eq!(ship.state, 0x0f);
    }

    #[test]
    fn test_build_cruiser() {
        let ship = Ship::cruiser(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
        assert_eq!(ship.ship_size, 3);
        assert_eq!(ship.state, 0x07);
    }

    #[test]
    fn test_build_submarine() {
        let ship = Ship::submarine(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
        assert_eq!(ship.ship_size, 3);
        assert_eq!(ship.state, 0x07);
    }

    #[test]
    fn test_build_destroyer() {
        let ship = Ship::destroyer(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
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
        let mut ship = Ship::aircraft_carrier(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
        assert_eq!(ship.check_hit(&Cell::new(x, y)), expected);
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
        let mut ship = Ship::aircraft_carrier(Cell::new(5, 5), &ShipDirection::Horizontal).unwrap();
        assert_eq!(ship.check_hit(&Cell::new(x, y)), expected);
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
        let mut ship = Ship::aircraft_carrier(Cell::new(0, 0), &ShipDirection::Vertical).unwrap();
        assert_eq!(ship.check_hit(&Cell::new(x, y)), expected);
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
        let mut ship = Ship::aircraft_carrier(Cell::new(5, 5), &ShipDirection::Vertical).unwrap();
        assert_eq!(ship.check_hit(&Cell::new(x, y)), expected);
    }

    #[test]
    fn test_check_hit_change_state() {
        let mut ship = Ship::aircraft_carrier(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
        ship.check_hit(&Cell::new(0, 0));
        assert_eq!(ship.state, 0x1e);
        ship.check_hit(&Cell::new(5, 0));
        assert_eq!(ship.state, 0x1e);
        ship.check_hit(&Cell::new(4, 0));
        assert_eq!(ship.state, 0x0e);
    }

    #[test]
    fn test_is_sunk() {
        let mut ship = Ship::aircraft_carrier(Cell::new(0, 0), &ShipDirection::Horizontal).unwrap();
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::new(0, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::new(1, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::new(2, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::new(3, 0));
        assert!(!ship.is_sunk());
        ship.check_hit(&Cell::new(4, 0));
        assert!(ship.is_sunk());
    }
}
