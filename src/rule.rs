pub(crate) use crate::constant::{DRAGON_COUNT, SLOT_COUNT, TRAY_COUNT};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(Color::Red),
            "g" => Ok(Color::Green),
            "b" => Ok(Color::Black),
            _ => Err(format!("Invalid color: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate) enum Card {
    CollapsedDragon,
    Dragon(Color),
    Flower,
    Number(Color, i8),
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Check for special cards
        match s {
            "f" | "ff" => return Ok(Card::Flower),
            "rd" | "rr" => return Ok(Card::Dragon(Color::Red)),
            "gd" | "gg" => return Ok(Card::Dragon(Color::Green)),
            "bd" | "bb" => return Ok(Card::Dragon(Color::Black)),
            _ => {}
        }

        // Try to parse compact format like "r1", "g2", "b3"
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 2 {
            if let Some(last_char) = chars.last() {
                if last_char.is_ascii_digit() {
                    let color_part = &s[..1];
                    let number_part = last_char.to_string();

                    if let (Ok(color), Ok(number)) =
                        (Color::from_str(color_part), number_part.parse::<i8>())
                    {
                        if number >= 1 && number <= 9 {
                            return Ok(Card::Number(color, number));
                        }
                    }
                }
            }
        }

        Err(format!("Invalid card format: {}. Expected formats: 'f', 'dr/dg/db', or '<color><number>' (e.g., r1, g2, b3)", s))
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Place {
    Tray(usize),
    Slot(usize),
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
    CollapseDragon(Color),
}

pub(crate) fn can_be_stacked(src: Card, dest: Card) -> bool {
    match (src, dest) {
        (Card::Number(color_src, number_src), Card::Number(color_dest, number_dest)) => {
            color_src != color_dest && number_src + 1 == number_dest
        }
        _ => false,
    }
}

pub(crate) fn validate_game(
    trays: &[Vec<Card>; TRAY_COUNT],
    slots: &[Option<Card>; SLOT_COUNT],
) -> Result<(), String> {
    let mut bucket = HashMap::<Card, usize>::new();
    for tray in trays.iter() {
        for card in tray.iter() {
            *bucket.entry(*card).or_insert(0) += 1;
        }
    }
    for slot in slots.iter() {
        if let Some(card) = slot {
            *bucket.entry(*card).or_insert(0) += 1;
        }
    }

    let get = |card: Card| -> usize { bucket.get(&card).copied().unwrap_or(0) };

    for color in Color::values() {
        let dragon_count = get(Card::Dragon(color));
        if dragon_count > DRAGON_COUNT {
            return Err(format!("Too many {} dragons: {}", color, dragon_count).into());
        }

        {
            let mut missing = false;
            for number in 9..=1 {
                let card_count = get(Card::Number(color, number));
                match card_count {
                    0 => missing = true,
                    1 => {
                        if missing {
                            return Err(format!("Missing {} card: {}", color, number + 1).into());
                        }
                    }
                    _ => return Err(format!("Too many {} cards: {}", color, number).into()),
                }
            }
        }
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_str() {
        // Test special cards
        assert_eq!("f".parse::<Card>().unwrap(), Card::Flower);

        // Test short dragon names
        assert_eq!("dr".parse::<Card>().unwrap(), Card::Dragon(Color::Red));
        assert_eq!("dg".parse::<Card>().unwrap(), Card::Dragon(Color::Green));
        assert_eq!("db".parse::<Card>().unwrap(), Card::Dragon(Color::Black));

        // Test number cards (compact format only)
        assert_eq!("r1".parse::<Card>().unwrap(), Card::Number(Color::Red, 1));
        assert_eq!("g5".parse::<Card>().unwrap(), Card::Number(Color::Green, 5));
        assert_eq!("b9".parse::<Card>().unwrap(), Card::Number(Color::Black, 9));

        // Test error cases
        assert!("invalid".parse::<Card>().is_err());
        assert!("flower".parse::<Card>().is_err());
        assert!("red_1".parse::<Card>().is_err());
        assert!("dragon_red".parse::<Card>().is_err());
        assert!("red1".parse::<Card>().is_err()); // Too long
        assert!("r0".parse::<Card>().is_err()); // Invalid number
        assert!("x1".parse::<Card>().is_err()); // Invalid color

        // CollapsedDragon should not be parseable from string
        assert!("collapsed_dragon".parse::<Card>().is_err());
        assert!("cd".parse::<Card>().is_err());
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!("r".parse::<Color>().unwrap(), Color::Red);
        assert_eq!("g".parse::<Color>().unwrap(), Color::Green);
        assert_eq!("b".parse::<Color>().unwrap(), Color::Black);

        // Error cases
        assert!("invalid".parse::<Color>().is_err());

        // Should not accept full names
        assert!("red".parse::<Color>().is_err());
        assert!("green".parse::<Color>().is_err());
        assert!("black".parse::<Color>().is_err());

        // Should not accept uppercase single letters
        assert!("R".parse::<Color>().is_err());
        assert!("G".parse::<Color>().is_err());
        assert!("B".parse::<Color>().is_err());
    }
}
