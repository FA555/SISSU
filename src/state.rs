use crate::rule::*;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type Priority = f64;

pub(crate) struct State {
    lowest_each_suit: HashMap<Color, i8>,
    pub(crate) trays: [Vec<Card>; TRAY_COUNT],
    pub(crate) slots: [Option<Card>; SLOT_COUNT],
    pub(crate) action: Option<Action>,
    pub(crate) prev_state: Option<Rc<State>>,
    step: usize,
    pub(crate) card_count: usize,
    priority: Priority,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.partial_cmp(&other.priority).unwrap()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.trays == other.trays && self.slots == other.slots
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trays.hash(state);
        self.slots.hash(state);
    }
}

impl State {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        const EMPTY_VEC_CARD: Vec<Card> = vec![];
        Self {
            lowest_each_suit: HashMap::new(),
            trays: [EMPTY_VEC_CARD; TRAY_COUNT],
            slots: [None; SLOT_COUNT],
            action: None,
            prev_state: None,
            step: 0,
            card_count: 0,
            priority: 0.,
        }
    }

    pub(crate) fn with_trays_and_slots(
        trays: &[Vec<Card>; TRAY_COUNT],
        slots: &[Option<Card>; SLOT_COUNT],
    ) -> Self {
        let mut state = Self {
            lowest_each_suit: HashMap::new(),
            trays: trays.to_owned(),
            slots: slots.to_owned(),
            action: None,
            prev_state: None,
            step: 0,
            card_count: 0,
            priority: 0.,
        };
        state.auto_remove_cards(); // lowest_per_suit is updated here
        state.card_count = state.calc_card_count();
        state.priority = state.calc_priority();
        state
    }

    pub(crate) fn transit(self: &Rc<Self>, action: &Action) -> Rc<State> {
        let mut state = State {
            lowest_each_suit: HashMap::new(),
            trays: self.trays.clone(),
            slots: self.slots,
            step: self.step + 1,
            action: Some(*action),
            prev_state: Some(self.clone()),
            card_count: 0,
            priority: 0.,
        };

        match *action {
            Action::Pop { src } => match src {
                Place::Tray(tray) => {
                    state.trays[tray].pop();
                }
                Place::Slot(_) => {
                    panic!("Impossible to pop from slot.");
                }
            },
            Action::Move {
                src: from,
                dest: to,
                count,
            } => {
                let mut cards_to_be_moved = Vec::with_capacity(count);

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
                        *slot = Some(Card::CollapsedDragon);
                        break;
                    }
                }
            }
        }

        state.auto_remove_cards(); // lowest_per_suit is updated here
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
            if let Card::CollapsedDragon = card {
            } else {
                count += 1;
            }
        }

        count
    }

    fn calc_priority(&self) -> Priority {
        let mut priority_of_cards: Priority = 0.;

        for tray in self.trays.iter() {
            if tray.is_empty() {
                continue;
            }

            let mut cards_stacked_cur_tray = 0;

            for i in ((tray.len() - 1)..0).rev() {
                if can_be_stacked(tray[i], tray[i - 1]) {
                    cards_stacked_cur_tray += 1;
                }
            }

            if cards_stacked_cur_tray == tray.len() - 1 {
                if let Card::Number(_, 9) = tray[0] {
                    priority_of_cards += cards_stacked_cur_tray as f64 * 1.2;
                } else {
                    priority_of_cards += cards_stacked_cur_tray as f64 * 1.1;
                }
            } else {
                priority_of_cards += cards_stacked_cur_tray as f64;
            }
        }

        self.card_count as f64 + self.step as f64 * 0.1 - priority_of_cards
    }

    fn auto_remove_cards(&mut self) {
        loop {
            let mut call_again = false;
            self.lowest_each_suit =
                HashMap::from([(Color::Red, 10), (Color::Green, 10), (Color::Black, 10)]);

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
                        if number < self.lowest_each_suit[&color] {
                            self.lowest_each_suit.insert(color, number);
                        }
                    }
                }
            }

            for &card in self.slots.iter() {
                if let Some(Card::Number(color, number)) = card {
                    if number < self.lowest_each_suit[&color] {
                        self.lowest_each_suit.insert(color, number);
                    }
                }
            }

            let mut try_remove_points_gt_one = |pile: &mut dyn Pile| {
                if let Some(Card::Number(color, number)) = pile.top_card() {
                    if number > 2 {
                        if number <= self.lowest_each_suit[&Color::Red]
                            && number <= self.lowest_each_suit[&Color::Green]
                            && number <= self.lowest_each_suit[&Color::Black]
                        {
                            pile.remove_to_foundations();
                            call_again = true;
                        }
                    } else if number == 2 && number == self.lowest_each_suit[&color] {
                        pile.remove_to_foundations();
                        call_again = true;
                    }
                }
            };

            for tray in self.trays.iter_mut() {
                if !tray.is_empty() {
                    try_remove_points_gt_one(tray);
                }
            }

            for card in self.slots.iter_mut() {
                try_remove_points_gt_one(card);
            }

            if !call_again {
                break;
            }
        }
    }

    pub(crate) fn valid_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut exposed_dragon_count = HashMap::from([
            (Dragon(Color::Red), 0),
            (Dragon(Color::Green), 0),
            (Dragon(Color::Black), 0),
        ]);

        for (i, tray) in self.trays.iter().enumerate() {
            if tray.is_empty() {
                continue;
            }

            if let &Card::Dragon(dragon) = tray.last().unwrap() {
                *exposed_dragon_count.get_mut(&dragon).unwrap() += 1;
            } else if let &Card::Number(color, number) = tray.last().unwrap() {
                if self.lowest_each_suit[&color] == number {
                    actions.push(Action::Pop {
                        src: Place::Tray(i),
                    });
                }
            }

            for (j, &card) in tray.iter().enumerate().rev() {
                for (k, other_tray) in self.trays.iter().enumerate() {
                    if i == k {
                        continue;
                    }

                    if !other_tray.is_empty() {
                        let &target_card = other_tray.last().unwrap();
                        if can_be_stacked(card, target_card) {
                            actions.push(Action::Move {
                                src: Place::Tray(i),
                                dest: Place::Tray(k),
                                count: tray.len() - j,
                            });
                        }
                    } else if !tray.is_empty() && j != 0 {
                        // (k, other_tray) is empty
                        actions.push(Action::Move {
                            src: Place::Tray(i),
                            dest: Place::Tray(k),
                            count: tray.len() - j,
                        });
                    }
                }

                if j > 0 && !can_be_stacked(card, tray[j - 1]) {
                    break;
                }
            }
        }

        let mut has_empty_slot = false;
        let mut has_empty_slot_for_specicific_dragon = HashMap::from([
            (Dragon(Color::Red), false),
            (Dragon(Color::Green), false),
            (Dragon(Color::Black), false),
        ]);

        for (i, &slot) in self.slots.iter().enumerate() {
            match slot {
                None => {
                    has_empty_slot = true;
                }
                Some(Card::CollapsedDragon) => {
                    continue;
                }
                Some(card) => {
                    if let Card::Dragon(dragon) = card {
                        *exposed_dragon_count.get_mut(&dragon).unwrap() += 1;
                        has_empty_slot_for_specicific_dragon.insert(dragon, true);
                    }
                    for (j, tray) in self.trays.iter().enumerate() {
                        if tray.is_empty() || can_be_stacked(card, *tray.last().unwrap()) {
                            actions.push(Action::Move {
                                src: Place::Slot(i),
                                dest: Place::Tray(j),
                                count: 1,
                            });
                        }
                    }
                }
            }
        }

        for dragon in Dragon::values() {
            if exposed_dragon_count[&dragon] == DRAGON_COUNT
                && (has_empty_slot || has_empty_slot_for_specicific_dragon[&dragon])
            {
                actions.push(Action::Collapse(dragon));
            }
        }

        actions
    }

    pub(crate) fn valid_slot_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();

        for (i, tray) in self.trays.iter().enumerate() {
            if tray.is_empty() {
                continue;
            }

            for (j, &slot) in self.slots.iter().enumerate() {
                if slot.is_none() {
                    actions.push(Action::Move {
                        src: Place::Tray(i),
                        dest: Place::Slot(j),
                        count: 1,
                    });
                }
            }
        }

        actions
    }
}
