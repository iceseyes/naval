use crate::cell::Cell;
use crate::ship::{display_ships, validate_ships, Ship, ShipDirection};
use std::fmt;
use std::fmt::{Display, Formatter};

macro_rules! random_ship_placement {
    ($ship: ident) => {
        loop {
            let ship = Ship::$ship(
                Cell::new(rand::random::<u8>(), rand::random::<u8>()),
                ShipDirection::random(),
            );

            if let Some(ship) = ship {
                break BattlefieldCell::new(ship.cell(), ship.direction()).unwrap();
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShootState {
    None,
    Hit,
    Miss,
    Sunk,
}

pub struct BattlefieldCell(Cell, ShipDirection);

impl BattlefieldCell {
    pub fn new(cell: Cell, direction: ShipDirection) -> Option<Self> {
        if cell.x > 9 || cell.y > 9 {
            None
        } else {
            Some(Self(cell, direction))
        }
    }
}

pub struct Battlefield {
    ships: [Ship; 5],
    battle_shoots: [[ShootState; 10]; 10],
}

impl Battlefield {
    pub fn new(
        aircraft_carrier: BattlefieldCell,
        battleship: BattlefieldCell,
        submarine: BattlefieldCell,
        cruiser: BattlefieldCell,
        destroyer: BattlefieldCell,
    ) -> Result<Self, String> {
        let ships = [
            Ship::aircraft_carrier(aircraft_carrier.0, aircraft_carrier.1)
                .ok_or_else(|| "Aircraft carrier not placed".to_string())?,
            Ship::battleship(battleship.0, battleship.1)
                .ok_or_else(|| "Battleship not placed".to_string())?,
            Ship::submarine(submarine.0, submarine.1)
                .ok_or_else(|| "Submarine not placed".to_string())?,
            Ship::cruiser(cruiser.0, cruiser.1).ok_or_else(|| "Cruiser not placed".to_string())?,
            Ship::destroyer(destroyer.0, destroyer.1)
                .ok_or_else(|| "Destroyer not placed".to_string())?,
        ];

        // Check for overlapping ships
        validate_ships(&ships[..])?;

        Ok(Battlefield {
            ships,
            battle_shoots: [[ShootState::None; 10]; 10],
        })
    }

    pub fn random() -> Self {
        loop {
            if let Ok(bf) = Self::new(
                random_ship_placement!(aircraft_carrier),
                random_ship_placement!(battleship),
                random_ship_placement!(submarine),
                random_ship_placement!(cruiser),
                random_ship_placement!(destroyer),
            ) {
                break bf;
            }
        }
    }
}

impl fmt::Debug for Battlefield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = [['.'; 10]; 10];
        let labels = ['A', 'B', 'S', 'C', 'D'];

        for (idx, ship) in self.ships.iter().enumerate() {
            for cell in ship.occupied_cells() {
                let x = cell.x as usize;
                let y = cell.y as usize;
                grid[y][x] = labels[idx];
            }
        }

        writeln!(f, "Battlefield:")?;
        for y in grid {
            let row: String = y.iter().collect();
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

impl Display for Battlefield {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_ships(&self.ships[..]))
    }
}
