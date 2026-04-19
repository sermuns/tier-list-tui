use clap::Parser;
use color_eyre::Result;

mod app;
mod ui;

use crate::app::App;

#[derive(Parser)]
struct Args {}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))
}
