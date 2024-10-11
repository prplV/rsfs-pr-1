mod cli;

use cli::Args;
use clap::Parser;
fn main() {
    let cli = Args::parse();
    println!("{:?}", cli);
}

