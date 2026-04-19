mod app;
mod ui;

use crate::app::App;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    images_path: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let mut app = App::new(args.images_path);
    ratatui::run(|terminal| app.run(terminal))
}
