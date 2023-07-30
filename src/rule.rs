pub(crate) use crate::constant::{DRAGON_COUNT, SLOT_COUNT, TRAY_COUNT};
pub(crate) use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Color {
    Red,
    Green,
    Black,
}

impl Color {
    pub(crate) fn values() -> impl Iterator<Item = Color> {
        [Color::Red, Color::Green, Color::Black].iter().copied()
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Dragon(pub(crate) Color);

impl Dragon {
    pub(crate) fn values() -> impl Iterator<Item = Dragon> {
        Color::values().map(Dragon)
    }
}

impl fmt::Display for Dragon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Dragon", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub(crate) enum Card {
    Number(Color, i8),
    Dragon(Dragon),
    Flower,
    Full,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Number(color, number) => write!(f, "({} {})", color, number),
            Card::Dragon(dragon) => write!(f, "({})", dragon),
            Card::Flower => write!(f, "(Flower)"),
            Card::Full => write!(f, "(Full)"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Place {
    Tray(usize),
    Slot(usize),
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Place::Tray(index) => write!(f, "Tray {index}", index = index + 1),
            Place::Slot(index) => write!(f, "Slot {index}", index = index + 1),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Action {
    Pop {
        src: Place,
    },
    Move {
        src: Place,
        dest: Place,
        count: usize,
    },
    Collapse(Dragon),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Pop { src } => write!(f, "Pop from {src}"),
            Action::Move { src, dest, count } => {
                write!(f, "Move {count} cards from {src} to {dest}")
            }
            Action::Collapse(dragon) => write!(f, "Collapse {dragon}"),
        }
    }
}

pub(crate) fn can_be_stacked(src: Card, dest: Card) -> bool {
    match (src, dest) {
        (Card::Number(color_src, number_src), Card::Number(color_dest, number_dest)) => {
            color_src != color_dest && number_src + 1 == number_dest
        }
        _ => false,
    }
}

pub(crate) trait Pile {
    fn remove_to_foundations(&mut self);

    fn top_card(&self) -> Option<Card>;
}

impl Pile for Vec<Card> {
    fn remove_to_foundations(&mut self) {
        self.pop();
    }

    fn top_card(&self) -> Option<Card> {
        self.last().copied()
    }
}

impl Pile for Option<Card> {
    fn remove_to_foundations(&mut self) {
        self.take();
    }

    fn top_card(&self) -> Option<Card> {
        *self
    }
}
