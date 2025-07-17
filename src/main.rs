mod algo;
mod constant;
mod io;
mod rule;
mod state;

use std::env::args;

use crate::algo::solve;
use crate::io::load_trays_and_slots;

fn main() {
    if args().len() != 2 {
        eprintln!("Usage: sissu <input_file>");
        std::process::exit(1);
    }

    let input_file = args().nth(1).unwrap();
    let input = std::fs::read_to_string(&input_file)
        .expect(format!("Failed to read input file: {input_file}").as_str());

    let (trays, slots) = load_trays_and_slots(input);
    solve(&trays, &slots);
}
