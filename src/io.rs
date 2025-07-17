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


pub(crate) fn load_trays_and_slots() -> ([Vec<Card>; TRAY_COUNT], [Option<Card>; SLOT_COUNT]) {
    (
        [
            // b3 dg db db b6
            vec![
                Card::Number(Color::Black, 3),
                Card::Dragon(Color::Green),
                Card::Dragon(Color::Black),
                Card::Dragon(Color::Black),
                Card::Number(Color::Black, 6),
            ],
            // r3 dr r7 b8 dr
            vec![
                Card::Number(Color::Red, 3),
                Card::Dragon(Color::Red),
                Card::Number(Color::Red, 7),
                Card::Number(Color::Black, 8),
                Card::Dragon(Color::Red),
            ],
            // dr g9 r9 g1 db
            vec![
                Card::Dragon(Color::Red),
                Card::Number(Color::Green, 9),
                Card::Number(Color::Red, 9),
                Card::Number(Color::Green, 1),
                Card::Dragon(Color::Black),
            ],
            // b4 g7 g2 r2 dr
            vec![
                Card::Number(Color::Black, 4),
                Card::Number(Color::Green, 7),
                Card::Number(Color::Green, 2),
                Card::Number(Color::Red, 2),
                Card::Dragon(Color::Red),
            ],
            // r8 g4 g3 b7
            vec![
                Card::Number(Color::Red, 8),
                Card::Number(Color::Green, 4),
                Card::Number(Color::Green, 3),
                Card::Number(Color::Black, 7),
            ],
            // b2 r5 g5 dg b5
            vec![
                Card::Number(Color::Black, 2),
                Card::Number(Color::Red, 5),
                Card::Number(Color::Green, 5),
                Card::Dragon(Color::Green),
                Card::Number(Color::Black, 5),
            ],
            // g6 g8 dg dg
            vec![
                Card::Number(Color::Green, 6),
                Card::Number(Color::Green, 8),
                Card::Dragon(Color::Green),
                Card::Dragon(Color::Green),
            ],
            // f r6 db r4 b9
            vec![
                Card::Flower,
                Card::Number(Color::Red, 6),
                Card::Dragon(Color::Black),
                Card::Number(Color::Red, 4),
                Card::Number(Color::Black, 9),
            ],
        ],
        [None, None, None],
    )
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
