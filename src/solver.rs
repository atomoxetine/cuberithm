use std::cmp;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

use crate::{cube::CubeState, rotation::Rotation};

pub struct Solver {
    pub middle_states: HashMap<CubeState, Vec<Rotation>>,
    pub found_solutions: HashSet<Vec<Rotation>>,
    pub initial_state: CubeState,
    pub desired_state: CubeState,
    pub min_moves: u8,
    pub max_moves: u8,
}

impl Solver {
    pub fn new(
        initial_state: &CubeState,
        desired_state: &CubeState,
        min_moves: u8,
        max_moves: u8,
    ) -> Solver {
        Solver {
            middle_states: HashMap::new(),
            found_solutions: HashSet::new(),
            initial_state: *initial_state,
            desired_state: *desired_state,
            min_moves,
            max_moves,
        }
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner,
     * stops when reaching a solution or max(1, floor(max_moves/2)) (meet in the middle)
     */
    fn first_pass_(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        dbg!(state);
        if state == self.desired_state
            && path.len() as u8 >= self.min_moves
            && path.len() as u8 <= self.max_moves
        {
            self.found_solutions.insert(path.clone());
            return;
        }

        if state == self.desired_state {
            return;
        }

        if path.len() as u8 > cmp::max(1, self.max_moves / 2) {
            self.middle_states.insert(state, path.clone());
            return;
        }

        prev_states.push(state);

        for rotation in Rotation::iter() {
            let new_state = state.rotate(rotation);
            if prev_states.contains(&new_state) {
                continue;
            }
            path.push(rotation);
            self.first_pass_(new_state, prev_states, path);
            path.pop();
        }

        prev_states.pop();
    }

    pub fn first_pass(&mut self) {
        self.first_pass_(self.initial_state, &mut Vec::new(), &mut Vec::new());
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner
     * stops when reaching a solution (doesnt save this time) or
     * when reaching a previously reached state or ceil(max_moves/2) (meet in the middle)
     */
    fn second_pass_(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        dbg!(state);
        if state == self.initial_state {
            return;
        }

        if path.len() as u8 >= (self.max_moves + 1) / 2 {
            return;
        }

        let optional_path = self.middle_states.get(&state);

        match optional_path {
            Some(found_path) => {
                if ((found_path.len() as u8 + path.len() as u8) < self.min_moves)
                    || ((found_path.len() as u8 + path.len() as u8) > self.max_moves)
                {
                    return;
                }
                let mut complete_path = found_path.clone();
                for idx in (0..path.len()).rev() {
                    complete_path.push(path[idx]);
                }
                self.found_solutions.insert(complete_path);
                return;
            }
            None => {}
        }

        prev_states.push(state);

        for rotation in Rotation::iter() {
            let new_state = state.rotate(rotation);
            if prev_states.contains(&new_state) {
                continue;
            };
            path.push(rotation.reverse());
            self.second_pass_(new_state, prev_states, path);
            path.pop();
        }

        prev_states.pop();
    }

    pub fn second_pass(&mut self) {
        self.second_pass_(self.desired_state, &mut Vec::new(), &mut Vec::new());
    }

    pub fn solve(&mut self) {
        self.first_pass();
        self.second_pass();
    }

    pub fn collect_solutions(&self) -> Vec<Vec<Rotation>> {
        let mut solutions: Vec<Vec<Rotation>> = Vec::new();

        for solution in self.found_solutions.iter() {
            solutions.push(solution.clone());
        }

        solutions
    }
}