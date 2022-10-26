use rand::Rng;
use std::convert::TryFrom;

use super::{count_state_handler::CountStateHandler, pattern_state_handler::PatternStateHandler};
use crate::config::Config;

#[derive(Clone)]
pub struct WolframsUniverse {
    pub current_age: usize,
    pub current_state: Vec<Vec<bool>>,
    pub last_state: Vec<bool>,
    pub state_handler: PatternStateHandler,
    pub x_len: usize,
}

impl From<&Config> for WolframsUniverse {
    fn from(config: &Config) -> Self {
        Self::new(config.x_len, config.probably_full, config.rule)
    }
}

impl Default for WolframsUniverse {
    fn default() -> Self {
        Self {
            current_age: 0,
            current_state: Vec::default(),
            last_state: Vec::default(),
            state_handler: PatternStateHandler::new(182),
            x_len: 50,
        }
    }
}

impl WolframsUniverse {
    pub fn new(x_len: usize, probably_full: f32, rule: u8) -> Self {
        let init_state = Self::init_universe(x_len, probably_full);

        Self {
            x_len,
            current_state: vec![init_state.clone()],
            last_state: init_state,
            state_handler: PatternStateHandler::new(rule),
            ..Default::default()
        }
    }

    pub fn _new_empty(x_len: usize) -> Self {
        let init_state = vec![false; x_len];

        Self {
            x_len,
            current_state: vec![init_state.clone()],
            last_state: init_state,
            ..Default::default()
        }
    }

    pub fn _new_full(x_len: usize) -> Self {
        let init_state = vec![true; x_len];

        Self {
            x_len,
            current_state: vec![init_state.clone()],
            last_state: init_state,
            ..Default::default()
        }
    }

    pub fn _new_with_full_middle(x_len: usize) -> Self {
        let init_state = Self::init_with_full_middle(x_len);

        Self {
            x_len,
            current_state: vec![init_state.clone()],
            last_state: init_state,
            ..Default::default()
        }
    }

    pub fn get_next_universe_state(
        current_states: &Vec<bool>,
        state_handler: &impl GetNextState,
    ) -> Vec<bool> {
        let mut result = Vec::with_capacity(current_states.len());

        for position in 0..current_states.len() {
            let state = state_handler.get_next_element_state(current_states, position);
            result.push(state);
        }

        result
    }

    fn init_universe(size: usize, probably_full: f32) -> Vec<bool> {
        let mut universe = Vec::with_capacity(size);

        for _ in 0..size {
            let state = get_random_state(probably_full);
            universe.push(state);
        }

        universe
    }

    pub fn iterate(&mut self) {
        self.current_age += 1;

        let last_state = self.current_state.last().unwrap();
        let next_state = Self::get_next_universe_state(last_state, &self.state_handler);

        self.current_state.push(next_state);
    }

    fn init_with_full_middle(x_len: usize) -> Vec<bool> {
        let mut state = vec![false; x_len];
        state[x_len / 2] = true;

        state
    }

    pub fn recreate(&mut self, x_len: usize, probably_full: f32, rule: u8) {
        let init_state = Self::init_universe(x_len, probably_full);

        self.x_len = x_len;
        self.current_state = vec![init_state.clone()];
        self.last_state = init_state;
        self.current_age = 0;
        self.state_handler = PatternStateHandler::new(rule);
    }

    pub fn recreate_with_full_middle(&mut self, x_len: usize, rule: u8) {
        let init_state = Self::init_with_full_middle(x_len);

        self.x_len = x_len;
        self.current_state = vec![init_state.clone()];
        self.last_state = init_state;
        self.current_age = 0;
        self.state_handler = PatternStateHandler::new(rule);
    }
}

pub trait GetNextState {
    fn get_next_element_state(&self, current_states: &Vec<bool>, index: usize) -> bool;
}

impl GetNextState for CountStateHandler {
    fn get_next_element_state(&self, current_states: &Vec<bool>, index: usize) -> bool {
        let full_neighbour_count = get_full_neighbour_count(
            current_states,
            index,
            self.range_offset,
            self.valid_state,
            self.include_self,
        );

        self.valid_counts.contains(&full_neighbour_count)
    }
}

impl GetNextState for PatternStateHandler {
    fn get_next_element_state(&self, current_states: &Vec<bool>, index: usize) -> bool {
        let range_offset = (self.patterns.first().unwrap().len() / 2) as isize;
        let mut current_pattern = Vec::new();

        for offset in -range_offset..=range_offset {
            let current_index = get_index(current_states.len(), index, offset);
            current_pattern.push(current_states[current_index])
        }

        self.patterns.contains(&current_pattern)
    }
}

fn get_full_neighbour_count(
    current_state: &Vec<bool>,
    index: usize,
    range_offset: isize,
    valid_state: bool,
    include_self: bool,
) -> usize {
    let mut result = 0;

    for offset in -range_offset..=range_offset {
        let current_index = get_index(current_state.len(), index, offset);

        if valid_state == current_state[current_index] {
            if (index == current_index && include_self) || (index != current_index) {
                result += 1;
            }
        }
    }

    result
}

fn get_index(max: usize, index: usize, offset: isize) -> usize {
    let new_position = isize::try_from(index).unwrap() + offset;
    let max = isize::try_from(max).unwrap();

    if new_position >= isize::try_from(max).unwrap() {
        return usize::try_from(new_position - max).unwrap();
    }

    if new_position < 0 {
        return usize::try_from(new_position + max).unwrap();
    }

    usize::try_from(new_position).unwrap()
}

fn get_random_state(probably_full: f32) -> bool {
    let mut random_generator = rand::thread_rng();
    let random_number = random_generator.gen_range(0.0..1000.0);

    random_number < 1000.0 * probably_full
}
