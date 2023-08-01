mod algo;
mod constant;
mod io;
mod rule;
mod state;

use crate::algo::solve;
use crate::io::load_trays_and_slots;

fn main() {
    let (trays, slots) = load_trays_and_slots();
    solve(&trays, &slots);
}
