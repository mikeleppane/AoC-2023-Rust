use std::collections::HashMap;

use crate::Selector;
use aoc_2023_rust::{run_solution, Runner};
use day01::Day01;

mod day01;

pub fn run(which: Selector) {
    let mut day01 = Day01::new();

    let mut days: HashMap<u8, &mut dyn Runner> = HashMap::new();
    days.insert(1, &mut day01);

    match which {
        Selector::Last => {
            let last = *days.keys().max().unwrap();
            if let Some(d) = days.get_mut(&last) {
                run_solution(*d);
            }
        }
        Selector::All => {
            for d in days.values_mut() {
                run_solution(*d);
            }
        }
        Selector::One(num) => {
            if let Some(d) = days.get_mut(&(num as u8)) {
                run_solution(*d);
            }
        }
    }
}
