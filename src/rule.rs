pub(crate) use std::fmt::{Display, Formatter, Result as FmtResult};
pub(crate) use crate::constant::{SLOT_COUNT, TRAY_COUNT};

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

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Black => write!(f, "Blue"),
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

impl Display for Dragon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

#[derive(Clone, Copy)]
pub(crate) enum Place {
    Tray(usize),
    Slot(usize),
}

impl Display for Place {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Place::Tray(index) => write!(f, "Tray {index}"),
            Place::Slot(index) => write!(f, "Slot {index}"),
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
