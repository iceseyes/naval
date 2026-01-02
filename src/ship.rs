//! In the naval battle the main object is the Ship. Players deploy their fleet up on a [crate::cell::Grid].
//! Each fleet consists of 5 ships, each of a different kind.
//!
//! The [ShipKind] defines the different ship types, and can be used to make a new [Ship]
//! of a given type.
//!
//! A [Ship] is a set of [Cell]s in a row. User can try to hit a cell of a ship, and when all the
//! ship cells have been hit, the ship is sunk.
//!
//! You have to use a given [ShipKind] in order to create a new [Ship].
//!
use crate::cell::Cell;
use strum::Display;
use strum_macros::EnumIter;

/// The different types of ship in the game.
///
/// Use this type to create new ships.
///
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumIter)]
pub enum ShipKind {
    /// Aircraft Carrier: the longest ship in the game, occupying 5 consecutive cells.
    #[strum(serialize = "Aircraft Carrier")]
    AircraftCarrier,

    /// Battleship: a ship occupying 4 consecutive cells.
    Battleship,

    /// Cruiser: a medium-sized ship occupying 3 consecutive cells.
    Cruiser,

    /// Submarine: occupies 3 consecutive cells, like the Cruiser.
    ///
    /// Apart from the name, there is no structural difference between a Cruiser
    /// and a Submarine.
    Submarine,

    /// Destroyer: the shortest ship in the game, occupying 2 consecutive cells.
    Destroyer,
}

impl ShipKind {
    const AIRCRAFT_CARRIER_SIZE: u8 = 5;
    const BATTLESHIP_SIZE: u8 = 4;
    const CRUISER_SIZE: u8 = 3;
    const SUBMARINE_SIZE: u8 = 3;
    const DESTROYER_SIZE: u8 = 2;

    /// Creates a new [`Ship`] of this kind starting from the given cell.
    ///
    /// A ship is defined by its starting cell (`first`) and its [`ShipOrientation`],
    /// which determines the direction in which the ship occupies consecutive cells.
    ///
    /// Returns `None` if the ship would exceed the board boundaries.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::{ShipKind, Cell, ShipOrientation};
    /// let aircraft_carrier = ShipKind::AircraftCarrier
    ///     .ship(Cell::bounded(3, 3), ShipOrientation::Vertical)
    ///     .unwrap();
    /// ```
    ///
    pub fn ship(&self, first: Cell, orientation: ShipOrientation) -> Option<Ship> {
        Ship::new(self.size(), first, orientation)
    }

    /// Returns a randomly placed [`Ship`] of this kind.
    ///
    /// Both the starting cell and the orientation are chosen at random.
    /// The returned ship is guaranteed to fit within the game board.
    pub fn random(&self) -> Ship {
        loop {
            if let Some(ship) = self.ship(Cell::random(), ShipOrientation::random()) {
                break ship;
            }
        }
    }

    /// Returns the number of cells for this kind of ship.
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

/// Describes a ship as an item of the game.
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

    /// Returns all cells occupied by this ship.
    ///
    /// In a [crate::cell::Grid], all these cells will be set to [crate::cell::CellState::Occupied].
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

    /// Returns `true` if the ship is sunk.
    pub fn is_sunk(&self) -> bool {
        self.state == 0
    }

    /// Checks whether the given cell is a part of the ship and records the hit.
    pub fn hit_at(&mut self, cell: &Cell) -> bool {
        let bit = self.contains(cell);

        bit.map(|bit| {
            self.state &= !(1u8 << bit);
            true
        })
        .unwrap_or(false)
    }

    /// Returns `true` if the other ship is in the space of this ship.
    ///
    /// The space a ship occupies includes all the cells that define it, plus a one-cell border around them.
    /// If the second ship is on one or more of those cells, the ships are considered to be overlapping.
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
                if other.contains(&Cell::bounded(x, y)).is_some() {
                    return true;
                }
            }
        }

        false
    }

    /// Checks if the cell belongs to the ship and returns its relative index.
    ///
    /// If the cell is one of the occupied cells, returns the index (0..size),
    /// otherwise returns `None`.
    fn contains(&self, cell: &Cell) -> Option<u8> {
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
                Some(cell.y() - self.first_cell.y())
            }

            _ => None,
        }
    }
}

/// A fleet is a collection of ships, one for each [`ShipKind`].
///
/// A fleet is constructed using a builder function that, given a ship kind,
/// returns a ship of that kind. A generated ship may be rejected if it overlaps
/// with ships already present in the fleet, so the builder function must take
/// this possibility into account.
///
/// # Example
///
/// ```rust
/// let fleet = Fleet::new(|kind| { kind.random() });
/// let grid = Grid::new(fleet.ships());
/// println!("{:?}", grid);
/// ```
///
pub struct Fleet([Ship; 5]);

impl Fleet {
    // defines the order in which the ships are stored into internal array
    const COMPOSITION: [ShipKind; 5] = [
        ShipKind::AircraftCarrier,
        ShipKind::Battleship,
        ShipKind::Cruiser,
        ShipKind::Submarine,
        ShipKind::Destroyer,
    ];

    /// Builds a fleet using builder function to get a new ship of a given type.
    ///
    /// This method calls for each kind of ship the builder function given as an argument. If the
    /// ship built is overlapping with others in the fleet, the builder function is called again until
    /// it builds a valid one.
    ///
    /// # Example
    ///
    /// ```rust
    /// let fleet = Fleet::build(|kind| kind.random());
    /// ```
    ///
    pub fn build<Builder>(mut builder: Builder) -> Self
    where
        Builder: FnMut(&ShipKind) -> Ship,
    {
        let mut ships = Vec::<Ship>::with_capacity(Self::COMPOSITION.len());
        for kind in Self::COMPOSITION.iter() {
            loop {
                let ship = builder(kind);
                if ships.iter().any(|s| s.is_overlapping(&ship)) {
                    continue;
                }

                ships.push(ship);
                break;
            }
        }
        let tmp: [Ship; 5] = ships.try_into().unwrap();

        Self(tmp)
    }

    /// Evaluates if the attacked cell hits a ship
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ships = [
    ///      Cell::bounded(0, 0),
    ///      Cell::bounded(2, 0),
    ///      Cell::bounded(4, 0),
    ///      Cell::bounded(6, 0),
    ///      Cell::bounded(8, 0),
    ///  ];
    ///  let mut ships = ships.iter();
    ///
    ///  let mut fleet = Fleet::build(|kind| {
    ///      kind.ship(ships.next().unwrap().clone(), ShipOrientation::Vertical)
    ///          .unwrap()
    ///  });
    ///
    ///  let cell = Cell::new(0, 0).unwrap();
    ///  assert_eq!(fleet.hit_at(&cell), Some(ShipKind::AircraftCarrier));
    ///
    ///  let cell = Cell::new(1, 0).unwrap();
    ///  assert_eq!(fleet.hit_at(&cell), None);
    /// ```
    pub fn hit_at(&mut self, cell: &Cell) -> Option<ShipKind> {
        let index = self.0.iter_mut().position(|ship| ship.hit_at(cell));
        index.map(|index| Self::COMPOSITION[index].clone())
    }

    /// Checks whether all the ships are sunk
    pub fn is_sunk(&self) -> bool {
        self.0.iter().all(|ship| ship.is_sunk())
    }

    /// Returns the ship by its type
    pub fn get(&self, kind: &ShipKind) -> &Ship {
        Self::COMPOSITION
            .iter()
            .position(|ship| ship == kind)
            .map(|index| &self.0[index])
            .unwrap()
    }
}

impl AsRef<[Ship]> for Fleet {
    /// Returns a slice containing all the ships in the fleet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let fleet = Fleet::random();
    /// let grid = Grid::new(fleet.ships());
    /// println!("{:?}", grid);
    /// ```
    fn as_ref(&self) -> &[Ship] {
        &self.0
    }
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

/// Defines the orientation of a ship.
///
/// In this game, a ship can be placed either horizontally (the same Y coordinate shared by all cells)
/// or vertically (the same X coordinate shared by all cells)
///
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShipOrientation {
    Horizontal,
    Vertical,
}

impl ShipOrientation {
    /// Return a random orientation
    ///
    pub fn random() -> Self {
        match rand::random::<u8>() % 2 {
            0 => ShipOrientation::Horizontal,
            _ => ShipOrientation::Vertical,
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

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::ship::{Fleet, ShipOrientation};
    use crate::ship::{Ship, ShipKind};
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
        assert_eq!(ship.hit_at(&Cell::bounded(x, y)), expected);
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
        assert_eq!(ship.hit_at(&Cell::new(x, y).unwrap()), expected);
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
        assert_eq!(ship.hit_at(&Cell::bounded(x, y)), expected);
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
        assert_eq!(ship.hit_at(&Cell::new(x, y).unwrap()), expected);
    }

    #[test]
    fn test_check_hit_change_state() {
        let mut ship = ShipKind::AircraftCarrier
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        ship.hit_at(&Cell::bounded(0, 0));
        assert_eq!(ship.state, 0x1e);
        ship.hit_at(&Cell::bounded(5, 0));
        assert_eq!(ship.state, 0x1e);
        ship.hit_at(&Cell::bounded(4, 0));
        assert_eq!(ship.state, 0x0e);
    }

    #[rstest]
    #[case::at_0_0(0, 0)]
    #[case::at_4_0(4, 0)]
    #[case::at_5_0(5, 0)]
    #[case::at_5_5(5, 5)]
    #[case::at_5_4(5, 4)]
    #[case::at_4_5(4, 5)]
    #[trace]
    fn test_is_sunk(
        #[case] x_start: u8,
        #[case] y_start: u8,

        #[values(ShipKind::AircraftCarrier, ShipKind::Battleship)] kind: ShipKind,

        #[values(ShipOrientation::Horizontal, ShipOrientation::Vertical)]
        orientation: ShipOrientation,
    ) {
        let mut ship = kind
            .ship(Cell::new(x_start, y_start).unwrap(), orientation)
            .unwrap();

        let range = if orientation == ShipOrientation::Horizontal {
            x_start..(x_start + kind.size())
        } else {
            y_start..(y_start + kind.size())
        };

        for r in range {
            assert!(!ship.is_sunk());

            let cell = if orientation == ShipOrientation::Horizontal {
                Cell::new(r, y_start)
            } else {
                Cell::new(x_start, r)
            }
            .unwrap();
            ship.hit_at(&cell);
        }

        assert!(ship.is_sunk());
    }

    #[rstest]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(4, 4).unwrap(), ShipOrientation::Horizontal).unwrap()
    )]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(4, 4).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap()
    )]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(4, 4).unwrap(), ShipOrientation::Vertical).unwrap()
    )]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Horizontal).unwrap(),
        ShipKind::AircraftCarrier.ship(Cell::new(4, 0).unwrap(), ShipOrientation::Vertical).unwrap()
    )]
    #[case(
        ShipKind::AircraftCarrier.ship(Cell::new(3, 3).unwrap(), ShipOrientation::Vertical).unwrap(),
        ShipKind::Submarine.ship(Cell::new(0, 4).unwrap(), ShipOrientation::Horizontal).unwrap())]
    fn test_is_overlapping(#[case] ship1: Ship, #[case] ship2: Ship) {
        assert!(ship1.is_overlapping(&ship2));
    }

    #[rstest]
    fn test_random_ship() {
        let ship1 = ShipKind::AircraftCarrier.random();
        let mut counter = 0;
        loop {
            counter += 1;
            let tmp = ShipKind::AircraftCarrier.random();
            if ship1 != tmp {
                break;
            } else if counter > 10 {
                panic!("Random ship is always the same!");
            }
        }
    }

    #[rstest]
    fn test_build_fleet_with_overlapping_ships() {
        let mut data = vec![
            // Aircraft carrier
            (Cell::bounded(3, 3), ShipOrientation::Vertical),
            // Battleship
            (Cell::bounded(3, 0), ShipOrientation::Horizontal),
            // Cruiser (overlap)
            (Cell::bounded(5, 1), ShipOrientation::Vertical),
            // Cruiser (good)
            (Cell::bounded(5, 2), ShipOrientation::Vertical),
            // Submarine
            (Cell::bounded(7, 2), ShipOrientation::Horizontal),
            // Destroyer
            (Cell::bounded(5, 9), ShipOrientation::Horizontal),
        ];

        let fleet = Fleet::build(|kind| {
            let item = data.remove(0);
            kind.ship(item.0, item.1).unwrap()
        });

        assert_eq!(
            fleet.0[0],
            ShipKind::AircraftCarrier
                .ship(Cell::new(3, 3).unwrap(), ShipOrientation::Vertical)
                .unwrap()
        );

        assert_eq!(
            fleet.0[1],
            ShipKind::Battleship
                .ship(Cell::new(3, 0).unwrap(), ShipOrientation::Horizontal)
                .unwrap()
        );

        assert_eq!(
            fleet.0[2],
            ShipKind::Cruiser
                .ship(Cell::new(5, 2).unwrap(), ShipOrientation::Vertical)
                .unwrap()
        );

        assert_eq!(
            fleet.0[3],
            ShipKind::Submarine
                .ship(Cell::new(7, 2).unwrap(), ShipOrientation::Horizontal)
                .unwrap()
        );

        assert_eq!(
            fleet.0[4],
            ShipKind::Destroyer
                .ship(Cell::new(5, 9).unwrap(), ShipOrientation::Horizontal)
                .unwrap()
        );
    }

    #[test]
    fn test_new_fleet() {
        let aircraft_carrier = ShipKind::AircraftCarrier
            .ship(Cell::bounded(0, 0), ShipOrientation::Horizontal)
            .unwrap();
        let battleship = ShipKind::Battleship
            .ship(Cell::bounded(0, 2), ShipOrientation::Horizontal)
            .unwrap();
        let cruiser = ShipKind::Cruiser
            .ship(Cell::bounded(0, 4), ShipOrientation::Horizontal)
            .unwrap();
        let submarine = ShipKind::Submarine
            .ship(Cell::bounded(0, 6), ShipOrientation::Horizontal)
            .unwrap();
        let destroyer = ShipKind::Destroyer
            .ship(Cell::bounded(0, 8), ShipOrientation::Horizontal)
            .unwrap();
        let fleet = Fleet::build(|kind| match kind {
            ShipKind::AircraftCarrier => aircraft_carrier.clone(),
            ShipKind::Battleship => battleship.clone(),
            ShipKind::Cruiser => cruiser.clone(),
            ShipKind::Submarine => submarine.clone(),
            ShipKind::Destroyer => destroyer.clone(),
        });

        assert_eq!(fleet.0[0], aircraft_carrier);
        assert_eq!(fleet.0[1], battleship);
        assert_eq!(fleet.0[2], cruiser);
        assert_eq!(fleet.0[3], submarine);
        assert_eq!(fleet.0[4], destroyer);
    }

    #[rstest]
    fn test_hit_fleet_at() {
        let ships = [
            Cell::bounded(0, 0),
            Cell::bounded(2, 0),
            Cell::bounded(4, 0),
            Cell::bounded(6, 0),
            Cell::bounded(8, 0),
        ];
        let mut ships = ships.iter();

        let mut fleet = Fleet::build(|kind| {
            kind.ship(ships.next().unwrap().clone(), ShipOrientation::Vertical)
                .unwrap()
        });

        let cell = Cell::new(0, 0).unwrap();
        assert_eq!(fleet.hit_at(&cell), Some(ShipKind::AircraftCarrier));

        let cell = Cell::new(1, 0).unwrap();
        assert_eq!(fleet.hit_at(&cell), None);
    }

    #[rstest]
    pub fn test_fleet_is_sunk() {
        let mut fleet = Fleet::build(|kind| kind.random());
        let occupied_cells =
            fleet.as_ref()[1..]
                .iter()
                .fold(fleet.0[0].occupied_cells(), |acc, ship| {
                    let mut out = acc.clone();
                    out.append(&mut ship.occupied_cells().clone());
                    out
                });

        for cell in occupied_cells {
            assert!(!fleet.is_sunk());
            fleet.hit_at(&cell);
        }

        assert!(fleet.is_sunk());
    }
}
