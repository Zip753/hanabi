use std::io;

use hanabi::game::tui::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
