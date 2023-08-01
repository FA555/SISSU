use crate::io::print_solution;
use crate::rule::{Action, Card, SLOT_COUNT, TRAY_COUNT};
use crate::state::State;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    rc::Rc,
};

pub(crate) fn solve(trays: &[Vec<Card>; TRAY_COUNT], slots: &[Option<Card>; SLOT_COUNT]) {
    if trays.iter().all(|tray| tray.is_empty()) {
        return;
    }

    let mut heap = BinaryHeap::new();
    let mut visited_states = HashSet::new();

    let current_state = Rc::new(State::with_trays_and_slots(trays, slots));
    heap.push(Reverse(current_state.clone()));
    visited_states.insert(current_state);

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

fn get_solution(state: &Rc<State>) -> Vec<Action> {
    let mut solution = Vec::new();
    let mut current_state = state;
    while let Some(action) = current_state.action {
        solution.push(action);
        current_state = current_state.prev_state.as_ref().unwrap();
        // println!("{current_state}");
    }
    solution.reverse();
    solution
}
