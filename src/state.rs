use crate::rule::*;
// use crate::rule::{Action, Card, Color, Pile, Place, can_be_stacked};
use std::collections::HashMap;
use std::rc::Rc;

type Priority = f64;

pub(crate) struct State {
    trays: Vec<Vec<Card>>,
    slots: Vec<Option<Card>>,
    action: Option<Action>,
    prev_state: Option<Rc<State>>,
    step: usize,
    card_count: usize,
    priority: Priority,
}

impl State {
    fn new() -> Self {
        Self {
            trays: Vec::new(),
            slots: Vec::new(),
            action: None,
            prev_state: None,
            step: 0,
            card_count: 0,
            priority: 0.,
        }
    }

    fn with_trays(trays: &[Vec<Card>]) -> Self {
        let mut state = Self::new();
        state.trays = trays.iter().map(|_| Vec::new()).collect();
        state
    }

    fn transition(self: &Rc<Self>, action: &Action) -> Rc<State> {
        let mut state = State {
            trays: self.trays.clone(),
            slots: self.slots.clone(),
            step: self.step + 1,
            action: Some(*action),
            prev_state: Some(self.clone()),
            card_count: 0,
            priority: 0.,
        };

        match *action {
            Action::Pop { tray } => {
                state.trays[tray].pop().unwrap();
            }
            Action::Move { from, to, count } => {
                let mut cards_to_be_moved = Vec::<Card>::with_capacity(count);

                match from {
                    Place::Tray(tray) => {
                        for _ in 0..count {
                            cards_to_be_moved.push(state.trays[tray].pop().unwrap());
                        }
                    }
                    Place::Slot(slot) => {
                        cards_to_be_moved.push(state.slots[slot].unwrap());
                        state.slots[slot] = None;
                    }
                }

                match to {
                    Place::Tray(tray) => {
                        for card in cards_to_be_moved.into_iter().rev() {
                            state.trays[tray].push(card);
                        }
                    }
                    Place::Slot(slot) => {
                        state.slots[slot] = Some(cards_to_be_moved[0]);
                    }
                }
            }
            Action::Collapse(dragon) => {
                for tray in state.trays.iter_mut() {
                    if !tray.is_empty() && *tray.last().unwrap() == Card::Dragon(dragon) {
                        tray.pop();
                    }
                }

                for slot in state.slots.iter_mut() {
                    if let Some(card) = slot {
                        if *card == Card::Dragon(dragon) {
                            *slot = None;
                        }
                    }
                }

                for slot in state.slots.iter_mut() {
                    if slot.is_none() {
                        *slot = Some(Card::Full);
                        break;
                    }
                }
            }
        }

        state.auto_remove_cards();
        state.card_count = state.calc_card_count();
        state.priority = state.calc_priority();

        Rc::new(state)
    }

    fn calc_card_count(&self) -> usize {
        let mut count = 0;

        for tray in self.trays.iter() {
            count += tray.len();
        }

        for card in self.slots.iter().flatten() {
            if let Card::Full = card {
            } else {
                count += 1;
            }
        }

        count
    }

    fn calc_priority(&self) -> Priority {
        let mut stacked_cards = 0.;

        for tray in self.trays.iter() {
            if tray.is_empty() {
                continue;
            }

            let mut local_stacked_cards = 0;

            for i in ((tray.len() - 1)..0).rev() {
                if can_be_stacked(tray[i], tray[i - 1]) {
                    local_stacked_cards += 1;
                }
            }

            if local_stacked_cards == tray.len() - 1 {
                if let Card::Number(_, 9) = tray[0] {
                    stacked_cards += local_stacked_cards as f64 * 1.2;
                } else {
                    stacked_cards += local_stacked_cards as f64 * 1.1;
                }
            } else {
                stacked_cards += local_stacked_cards as f64;
            }
        }

        self.card_count as f64 + self.step as f64 * 0.1 - stacked_cards
    }

    fn auto_remove_cards(&mut self) {
        let mut lowest_per_suit = HashMap::<Color, i8>::new();
        lowest_per_suit.insert(Color::Red, 10);
        lowest_per_suit.insert(Color::Green, 10);
        lowest_per_suit.insert(Color::Black, 10);

        let mut call_again = false;

        loop {
            for tray in self.trays.iter_mut() {
                if tray.is_empty() {
                    continue;
                }

                if let Card::Flower = tray.last().unwrap() {
                    tray.pop();
                    call_again = true;
                } else if let Card::Number(_, 1) = tray.last().unwrap() {
                    tray.pop();
                    call_again = true;
                }

                for &card in tray.iter() {
                    if let Card::Number(color, number) = card {
                        if number < lowest_per_suit[&color] {
                            lowest_per_suit.insert(color, number);
                        }
                    }
                }
            }

            for &card in self.slots.iter() {
                if let Some(Card::Number(color, number)) = card {
                    if number < lowest_per_suit[&color] {
                        lowest_per_suit.insert(color, number);
                    }
                }
            }

            let mut try_remove_with_points_greater_than_one = |pile: &mut dyn Pile| {
                if let Card::Number(color, number) = pile.top_card().unwrap() {
                    if number > 2 {
                        if number <= lowest_per_suit[&Color::Red]
                            && number <= lowest_per_suit[&Color::Green]
                            && number <= lowest_per_suit[&Color::Black]
                        {
                            pile.remove_to_foundations();
                            call_again = true;
                        }
                    } else if number == 2 && number == lowest_per_suit[&color] {
                        pile.remove_to_foundations();
                        call_again = true;
                    }
                }
            };

            for tray in self.trays.iter_mut() {
                if !tray.is_empty() {
                    try_remove_with_points_greater_than_one(tray);
                }
            }

            for card in self.slots.iter_mut() {
                try_remove_with_points_greater_than_one(card);
            }

            if !call_again {
                break;
            }
        }
    }
}
