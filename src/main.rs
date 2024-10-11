mod cli;
mod core;

use cli::Args;
use clap::Parser;
// use core::{Diskmng, Filemng, Segment, Zipmng};

fn main() {
    let mut cli = Args::parse();
    let _ = cli.exec();
}

