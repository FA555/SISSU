#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Color {
    Red,
    Green,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Dragon(pub(crate) Color);

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
pub(crate) enum Action {
    Pop {
        tray: usize,
    },
    Move {
        from: Place,
        to: Place,
        count: usize,
    },
    Collapse(Dragon),
}

pub(crate) fn can_be_stacked(from: Card, to: Card) -> bool {
    match (from, to) {
        (Card::Number(color_from, number_from), Card::Number(color_to, number_to)) => {
            color_from != color_to && number_from + 1 == number_to
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
