mod engine;
mod tui;

use std::io;
use tui::NavalBattleTui;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| NavalBattleTui::new().run(terminal))
}
