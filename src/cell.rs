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
