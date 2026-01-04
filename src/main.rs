mod grid;
mod player;
mod ship;
mod state;
mod tui;
mod widgets;

use std::io;
use tui::NavalBattleTui;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| NavalBattleTui::new().run(terminal))
}
