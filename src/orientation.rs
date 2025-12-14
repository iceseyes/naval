#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShipOrientation {
    Horizontal,
    Vertical,
}

impl ShipOrientation {
    pub fn random() -> Self {
        match rand::random::<u8>() % 2 {
            0 => ShipOrientation::Horizontal,
            _ => ShipOrientation::Vertical,
        }
    }
}
