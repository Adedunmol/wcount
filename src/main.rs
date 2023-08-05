use std::process;
use wcount::{Cli, run};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    if let Err(err) = run(&args) {
        eprintln!("{}", err);
        process::exit(1);
    }

}