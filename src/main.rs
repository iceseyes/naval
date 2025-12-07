use crate::battlefield::Battlefield;

mod battlefield;
mod cell;
mod ship;

fn main() {
    let battlefield = Battlefield::random();
    dbg!(battlefield);
}
