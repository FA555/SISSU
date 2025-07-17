use crate::rule::{Action, Card, Color, Place, SLOT_COUNT, TRAY_COUNT};
use crate::state::State;

use std::fmt;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Pop { src } => write!(f, "Pop from {src}"),
            Action::Move { src, dest, count } => {
                write!(f, "Move {count} cards from {src} to {dest}")
            }
            Action::CollapseDragon(color) => write!(f, "Collapse {color} Dragon"),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Number(color, number) => write!(f, "({color} {number})"),
            Card::Dragon(color) => write!(f, "({color} Dragon)"),
            Card::Flower => write!(f, "(Flower)"),
            Card::CollapsedDragon => write!(f, "(Full)"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Black => write!(f, "Black"),
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Place::Tray(index) => write!(f, "Tray {index}", index = index + 1),
            Place::Slot(index) => write!(f, "Slot {index}", index = index + 1),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str("[ ");
        for slot in self.slots.iter() {
            match slot {
                Some(card) => s.push_str(&format!("{card} ")),
                None => s.push_str("() "),
            }
        }
        s.push_str("]\n");
        for (i, tray) in self.trays.iter().enumerate() {
            s.push_str(&format!("Tray {i}: ", i = i + 1));
            for card in tray.iter() {
                s.push_str(&format!("{card} "));
            }
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

pub(crate) fn load_trays_and_slots(
    input: String,
) -> ([Vec<Card>; TRAY_COUNT], [Option<Card>; SLOT_COUNT]) {
    let mut trays = [const { Vec::new() }; TRAY_COUNT];
    let slots = [None; SLOT_COUNT];

    for (i, line) in input.lines().enumerate() {
        if i < TRAY_COUNT {
            trays[i] = line
                .split_whitespace()
                .map(|s| s.parse::<Card>().expect("Invalid card format"))
                .collect();
        }
    }

    (trays, slots)
}

pub(crate) fn print_solution(actions: &[Action], iteration_count: usize) {
    println!(
        "Found solution of {step} step(s) in {iteration_count} iterations",
        step = actions.len()
    );

    for (i, action) in actions.iter().enumerate() {
        println!("Step {i}: {action}", i = i + 1);
    }
}
