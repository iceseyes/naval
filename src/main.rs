use crate::battlefield::Battlefield;
use crate::cell::Cell;
use crate::ship::{display_ships, validate_ships, Ship, ShipKind};
use orientation::ShipOrientation;
use strum::IntoEnumIterator;

mod battlefield;
mod cell;
mod orientation;
mod ship;

struct Game {
    computer: Battlefield,
    player: Battlefield,
}

fn main() {
    let battlefield = Battlefield::random();
    let mut player_fleet = Vec::new();

    println!("Welcome to Battleship!");
    println!("I'm ready, please set up your fleet.");

    println!("Where do you want to place your ships?");

    ShipKind::iter().for_each(|ship_kind| {
        add_ship(&mut player_fleet, ship_kind);
    });

    println!("Fleet set up successfully!");
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
        match parse_coordinates(coordinates) {
            Ok((x, y, direction)) => {
                break (Cell::new(x, y), direction);
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

    println!("{}", display_ships(player_fleet.as_slice()));
}

fn parse_coordinates(coordinates: &str) -> Result<(u8, u8, ShipOrientation), String> {
    let mut chars = coordinates.chars();

    let x = chars
        .next()
        .ok_or("Missing X coordinate: expected A,B,C,D,E,F,G,H,I,J".to_string())?
        .to_ascii_lowercase();
    let x = if x.is_ascii_alphabetic() && x <= 'j' {
        x as u8 - b'a'
    } else {
        Err("Invalid X coordinate: expected A,B,C,D,E,F,G,H,I,J".to_string())?
    };

    let y = chars
        .next()
        .ok_or("Missing Y coordinate: expected 0,1,2,3,4,5,6,7,8,9".to_string())?
        .to_digit(10)
        .ok_or("Invalid Y coordinate: expected 0,1,2,3,4,5,6,7,8,9".to_string())? as u8;

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
        assert_eq!(parse_coordinates(input).unwrap(), expected);
    }
}
