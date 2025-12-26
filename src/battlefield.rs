use crate::cell::Cell;
use crate::orientation::ShipOrientation;
use crate::ship::{display_ships, validate_ships, Ship, ShipKind};
use std::fmt;
use std::fmt::{Display, Formatter};

macro_rules! random_ship_placement {
    ($ship: ident) => {
        loop {
            let ship = ShipKind::$ship.ship(
                Cell::bounded(rand::random::<u8>(), rand::random::<u8>()),
                ShipOrientation::random(),
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

pub struct Battlefield {
    ships: [Ship; 5],
    battle_shoots: [[ShootState; 10]; 10],
}

impl Battlefield {
    pub fn new(ships: [Ship; 5]) -> Result<Self, String> {
        // Check for overlapping ships
        validate_ships(&ships[..])?;

        Ok(Battlefield {
            ships,
            battle_shoots: [[ShootState::None; 10]; 10],
        })
    }

    pub fn random() -> Self {
        loop {
            if let Ok(bf) = Self::new([
                random_ship_placement!(AircraftCarrier),
                random_ship_placement!(Battleship),
                random_ship_placement!(Submarine),
                random_ship_placement!(Cruiser),
                random_ship_placement!(Destroyer),
            ]) {
                break bf;
            }
        }
    }

    pub fn check(&mut self, cell: Cell) -> ShootState {
        let mut hit = None;

        for ship in &mut self.ships {
            if ship.check_hit(&cell) {
                hit = Some(ship);
                break;
            }
        }

        if let Some(ship) = hit {
            if ship.is_sunk() {
                self.battle_shoots[cell.y() as usize][cell.x() as usize] = ShootState::Sunk;
            } else {
                self.battle_shoots[cell.y() as usize][cell.x() as usize] = ShootState::Hit;
            }
        } else {
            self.battle_shoots[cell.y() as usize][cell.x() as usize] = ShootState::Miss;
        }

        self.battle_shoots[cell.y() as usize][cell.x() as usize]
    }

    pub fn is_defeated(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunk())
    }

    pub fn attack(&mut self) -> Cell {
        loop {
            let x = rand::random::<u8>() % 10;
            let y = rand::random::<u8>() % 10;
            let cell = Cell::bounded(x, y);

            if self.battle_shoots[y as usize][x as usize] == ShootState::None {
                return cell;
            }
        }
    }

    pub fn display(&self) -> String {
        let mut out = "  A B C D E F G H I J \n".to_string();
        for (index, y) in self.battle_shoots.iter().enumerate() {
            out.push(char::from(b'0' + index as u8));
            out.push(' ');
            y.iter().for_each(|o| {
                let ch = match o {
                    ShootState::None => ' ',
                    ShootState::Hit | ShootState::Sunk => 'X',
                    ShootState::Miss => '.',
                };
                out.push(ch);
                out.push(' ')
            });
            out.push('\n');
        }

        out
    }
}

impl fmt::Debug for Battlefield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = [['.'; 10]; 10];
        let labels = ['A', 'B', 'S', 'C', 'D'];

        for (idx, ship) in self.ships.iter().enumerate() {
            for cell in ship.occupied_cells() {
                let x = cell.x() as usize;
                let y = cell.y() as usize;
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
