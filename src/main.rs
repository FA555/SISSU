mod constant;
mod rule;
mod state;

use crate::rule::{Action, Card, Color, Dragon, SLOT_COUNT, TRAY_COUNT};
use crate::state::State;

use std::rc::Rc;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let (trays, slots) = load_trays_and_slots();
    solve(&trays, &slots);
}

fn load_trays_and_slots() -> ([Vec<Card>; TRAY_COUNT], [Option<Card>; SLOT_COUNT]) {
    (
        [
            // b8 b9 g5 dr dg
            vec![
                Card::Number(Color::Black, 8),
                Card::Number(Color::Black, 9),
                Card::Number(Color::Green, 5),
                Card::Dragon(Dragon(Color::Red)),
                Card::Dragon(Dragon(Color::Green)),
            ],
            // r4 g6 r3 r7 dg
            vec![
                Card::Number(Color::Red, 4),
                Card::Number(Color::Green, 6),
                Card::Number(Color::Red, 3),
                Card::Number(Color::Red, 7),
                Card::Dragon(Dragon(Color::Green)),
            ],
            // r2 db dr g7 g8
            vec![
                Card::Number(Color::Red, 2),
                Card::Dragon(Dragon(Color::Black)),
                Card::Dragon(Dragon(Color::Red)),
                Card::Number(Color::Green, 7),
                Card::Number(Color::Green, 8),
            ],
            // r5 r8 b2 dg dg
            vec![
                Card::Number(Color::Red, 5),
                Card::Number(Color::Red, 8),
                Card::Number(Color::Black, 2),
                Card::Dragon(Dragon(Color::Green)),
                Card::Dragon(Dragon(Color::Green)),
            ],
            // b3 b5 db b6 dr
            vec![
                Card::Number(Color::Black, 3),
                Card::Number(Color::Black, 5),
                Card::Dragon(Dragon(Color::Black)),
                Card::Number(Color::Black, 6),
                Card::Dragon(Dragon(Color::Red)),
            ],
            // g4 g3 f db g2
            vec![
                Card::Number(Color::Green, 4),
                Card::Number(Color::Green, 3),
                Card::Flower,
                Card::Dragon(Dragon(Color::Black)),
                Card::Number(Color::Green, 2),
            ],
            // db g0 r1 dr b4
            vec![
                Card::Dragon(Dragon(Color::Black)),
                Card::Number(Color::Green, 0),
                Card::Number(Color::Red, 1),
                Card::Dragon(Dragon(Color::Red)),
                Card::Number(Color::Black, 4),
            ],
            // b7 g1 r9 r6
            vec![
                Card::Number(Color::Black, 7),
                Card::Number(Color::Green, 1),
                Card::Number(Color::Red, 9),
                Card::Number(Color::Red, 6),
            ],
        ],
        [None, None, None],
    )
}

fn solve(trays: &[Vec<Card>; TRAY_COUNT], slots: &[Option<Card>; SLOT_COUNT]) {
    if trays.iter().all(|tray| tray.is_empty()) {
        return;
    }

    let mut heap = BinaryHeap::new();
    let mut visited_states = HashSet::new();

    let current_state = Rc::new(State::with_trays_and_slots(trays, slots));
    heap.push(Reverse(current_state.clone()));
    visited_states.insert(current_state.clone());

    // for action in current_state.clone().valid_actions() {
    //     println!("[DEBUG] {action}");
    // }
    // for action in current_state.clone().valid_slot_actions() {
    //     println!("[DEBUG] {action}");
    // }
    // unimplemented!();

    let mut iteration_count = 0_usize; // aux
    while let Some(Reverse(current_state)) = heap.pop() {
        if current_state.card_count == 0 {
            print_solution(&get_solution(&current_state), iteration_count);
            return;
        }

        let mut state_transit_by_actions = |state: &Rc<State>, actions: &[Action]| -> usize {
            let mut valid_actions = 0;
            for action in actions {
                let new_state = state.transit(action);
                if visited_states.insert(new_state.clone()) {
                    heap.push(Reverse(new_state));
                    valid_actions += 1;
                }
            }
            valid_actions
        };

        if state_transit_by_actions(&current_state, &current_state.valid_actions()) == 0 {
            state_transit_by_actions(&current_state, &current_state.valid_slot_actions());
        }

        iteration_count += 1;
        if iteration_count % 10000 == 0 {
            println!("Iteration {iteration_count}");
        }
    }

    println!("No solution found");
}

fn print_solution(actions: &[Action], iteration_count: usize) {
    println!(
        "Found solution of {step} step(s) in {iteration_count} iterations",
        step = actions.len()
    );

    for (i, action) in actions.iter().enumerate() {
        println!("Step {i}: {action}", i = i + 1);
    }
}

fn get_solution(state: &Rc<State>) -> Vec<Action> {
    let mut solution = Vec::new();
    let mut current_state = state;
    while let Some(action) = current_state.action {
        solution.push(action);
        current_state = current_state.prev_state.as_ref().unwrap();
        println!("{current_state}");
    }
    solution.reverse();
    solution
}
