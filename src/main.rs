mod constant;
mod rule;
mod state;

use crate::rule::{Action, Card};
use crate::state::State;

use std::rc::Rc;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    format!("Hello, world!");
    let trays = load_trays();
    solve(&trays);
}

fn load_trays() -> Vec<Vec<Card>> {
    todo!()
}

fn solve(trays: &[Vec<Card>]) {
    if trays.iter().all(|tray| tray.is_empty()) {
        return;
    }

    let mut heap = BinaryHeap::new();
    let mut visited_states = HashSet::new();

    let current_state = Rc::new(State::with_trays(trays));
    heap.push(Reverse(current_state.clone()));
    visited_states.insert(current_state);

    let mut iteration_count = 0_usize; // aux
    while let Some(Reverse(current_state)) = heap.pop() {
        if current_state.card_count == 0 {
            println!("Found solution in {iteration_count} iterations.c");
            print_solution(&get_solution(&current_state));
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

        if state_transit_by_actions(&current_state, &current_state.valid_actions()) != 0 {
            state_transit_by_actions(&current_state, &current_state.valid_slot_actions());
        }

        if iteration_count % 1000 == 0 {
            println!("Iteration {iteration_count}");
        }
        iteration_count += 1;
    }

    todo!();
}

fn print_solution(actions: &[Action]) {
    for action in actions {
        match action {
            Action::Pop { src } => println!("Pop from {src}"),
            Action::Move { src, dest, count } => {
                println!("Move {count} cards from {src} to {dest}")
            }
            Action::Collapse(dragon) => println!("Collapse {dragon}"),
        }
    }
}

fn get_solution(state: &Rc<State>) -> Vec<Action> {
    let mut solution = Vec::new();
    let mut current_state = state;
    while let Some(action) = current_state.action {
        solution.push(action);
        current_state = current_state.prev_state.as_ref().unwrap();
    }
    solution.reverse();
    solution
}
