use crate::cell::Cell;
use crate::ship::{Ship, ShipDirection};

macro_rules! random_ship_placement {
    ($ship: ident) => {
        loop {
            let ship = Ship::$ship(
                Cell::new(rand::random::<u8>(), rand::random::<u8>()),
                ShipDirection::random(),
            );

            if let Some(ship) = ship {
                break ship;
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
    pub fn new(x: u8, y: u8, direction: ShipDirection) -> Option<Self> {
        if x > 9 || y > 9 {
            None
        } else {
            Some(Self(Cell::new(x, y), direction))
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
        let aircraft_carrier = Ship::aircraft_carrier(aircraft_carrier.0, &aircraft_carrier.1)
            .ok_or_else(|| "Aircraft carrier not placed".to_string())?;
        let battleship = Ship::battleship(battleship.0, &battleship.1)
            .ok_or_else(|| "Battleship not placed".to_string())?;
        let submarine = Ship::submarine(submarine.0, &submarine.1)
            .ok_or_else(|| "Submarine not placed".to_string())?;
        let cruiser =
            Ship::cruiser(cruiser.0, &cruiser.1).ok_or_else(|| "Cruiser not placed".to_string())?;
        let destroyer = Ship::destroyer(destroyer.0, &destroyer.1)
            .ok_or_else(|| "Destroyer not placed".to_string())?;

        Ok(Battlefield {
            ships: [aircraft_carrier, battleship, submarine, cruiser, destroyer],
            battle_shoots: [[ShootState::None; 10]; 10],
        })
    }

    pub fn random() -> Self {
        unimplemented!()
    }
}
