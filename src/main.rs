use crate::battlefield::{Battlefield, ShootState};
use crate::cell::{Cell, Grid};
use crate::ship::{validate_ships, Ship, ShipKind};
use ship::ShipOrientation;
use std::str::Chars;
use strum::IntoEnumIterator;

mod battlefield;
mod cell;
mod ship;

struct Game {
    computer: Battlefield,
    player: Battlefield,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut player_fleet = Vec::new();

    println!("Welcome to Battleship!");
    println!("I'm ready, please set up your fleet.");

    println!("Where do you want to place your ships?");

    ShipKind::iter().for_each(|ship_kind| {
        add_ship(&mut player_fleet, ship_kind);
    });

    println!("Fleet set up successfully!");

    let fleet: [Ship; 5] = std::mem::take(&mut player_fleet).try_into().unwrap();
    let mut computer = Battlefield::random();
    let mut player = Battlefield::new(fleet)?;

    loop {
        println!("Player's turn. Where do you want to attack?");
        let (x, y) = loop {
            let mut input_string = String::new();
            std::io::stdin().read_line(&mut input_string).unwrap();
            if let Ok((x, y)) = parse_coordinates(&mut input_string.trim().chars()) {
                break (x, y);
            } else {
                println!("Invalid coordinates: {input_string}. try again.");
            }
        };

        match computer.check(Cell::bounded(x, y)) {
            ShootState::None => {
                eprintln!("Invalid coordinates: {x},{y}. try again.");
            }
            ShootState::Hit => {
                println!("Hit!");
            }
            ShootState::Miss => {
                println!("Miss!");
            }
            ShootState::Sunk => {
                println!("Sunk!");
            }
        }

        println!("{}", computer.display());
        if computer.is_defeated() {
            println!("Congratulations! You have defeated the computer's fleet!");
            break;
        }

        let p = computer.attack();
        let s = player.check(p);
        println!("Computer attacked: ({}, {}): {s:?}", p.x(), p.y());
        println!("{}", player.display());

        if player.is_defeated() {
            println!("You have lost!");
            break;
        }
    }

    Ok(())
}

fn ask_for_coordinates(kind: &ShipKind) -> (Cell, ShipOrientation) {
    println!("=> {kind} ({}) <=", kind.size());

    loop {
        println!(
            "Enter the coordinates of the top left cell and the orientation (H: Horizontal, V: Vertical). for example: A5v or l9H"
        );
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        let coordinates = input_string.trim();
        match parse_ship_position(coordinates) {
            Ok((x, y, direction)) => {
                break (Cell::bounded(x, y), direction);
            }
            Err(e) => {
                println!("Invalid coordinates: {coordinates}. {e}");
            }
        }
    }
}

fn add_ship(player_fleet: &mut Vec<Ship>, kind: ShipKind) {
    loop {
        let (first_cell, direction) = ask_for_coordinates(&kind);
        player_fleet.push(kind.ship(first_cell, direction).unwrap());
        if validate_ships(player_fleet.as_slice()).is_ok() {
            break;
        } else {
            player_fleet.pop();
        }
    }

    let grid = Grid::from_ships(player_fleet.as_slice());
    println!("{}", grid);
}

fn parse_ship_position(coordinates: &str) -> Result<(u8, u8, ShipOrientation), String> {
    let mut chars = coordinates.chars();

    let (x, y) = parse_coordinates(&mut chars)?;

    let orientation = match chars
        .next()
        .ok_or("Missing orientation: expected H or V".to_string())?
        .to_ascii_lowercase()
    {
        'h' => ShipOrientation::Horizontal,
        'v' => ShipOrientation::Vertical,
        _ => Err("Invalid orientation".to_string())?,
    };

    Ok((x, y, orientation))
}

fn parse_coordinates(coordinates: &mut Chars) -> Result<(u8, u8), String> {
    let x = coordinates
        .next()
        .ok_or("Missing X coordinate: expected A,B,C,D,E,F,G,H,I,J".to_string())?
        .to_ascii_lowercase();
    let x = if x.is_ascii_alphabetic() && x <= 'j' {
        x as u8 - b'a'
    } else {
        Err("Invalid X coordinate: expected A,B,C,D,E,F,G,H,I,J".to_string())?
    };

    let y = coordinates
        .next()
        .ok_or("Missing Y coordinate: expected 0,1,2,3,4,5,6,7,8,9".to_string())?
        .to_digit(10)
        .ok_or("Invalid Y coordinate: expected 0,1,2,3,4,5,6,7,8,9".to_string())? as u8;

    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A5h", (0, 5, ShipOrientation::Horizontal))]
    #[case("j9H", (9, 9, ShipOrientation::Horizontal))]
    #[case("b0v", (1, 0, ShipOrientation::Vertical))]
    #[case("J0V", (9, 0, ShipOrientation::Vertical))]
    #[should_panic(expected = "Missing X coordinate: expected A,B,C,D,E,F,G,H,I,J")]
    #[case("", (0, 0, ShipOrientation::Horizontal))]
    #[should_panic(expected = "Invalid X coordinate: expected A,B,C,D,E,F,G,H,I,J")]
    #[case("p", (0, 0, ShipOrientation::Horizontal))]
    #[should_panic(expected = "Invalid X coordinate: expected A,B,C,D,E,F,G,H,I,J")]
    #[case("p0T", (0, 0, ShipOrientation::Horizontal))]
    fn test_parse_coordinates(#[case] input: &str, #[case] expected: (u8, u8, ShipOrientation)) {
        assert_eq!(parse_ship_position(input).unwrap(), expected);
    }
}
