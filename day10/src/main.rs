use std::fs::read_to_string;

enum CPUInstruction {
    AddX(i64),
    Noop,
}

use CPUInstruction::*;

impl CPUInstruction {
    fn cost(&self) -> usize {
        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }

    fn parse_exn(s: &str) -> Self {
        if s == "noop" {
            Noop
        } else if s.starts_with("addx ") {
            let i = s.strip_prefix("addx ").unwrap().parse::<i64>().unwrap();
            AddX(i)
        } else {
            panic!("Couldn't parse instruction: {}", s)
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    reg_x: i64,
}

impl State {
    fn run(&mut self, instruction: &CPUInstruction) {
        match instruction {
            Noop => (),
            AddX(i) => self.reg_x += i,
        }
    }
}

// Returns the state DURING the given clock cycle. So the first state is 0.
fn run_instructions(state: &mut State, instructions: Vec<CPUInstruction>) -> Vec<State> {
    let mut result = vec![state.clone()];

    for instruction in instructions {
        for _ in 0..instruction.cost() {
            result.push(state.clone());
        }
        state.run(&instruction);
    }
    result
}

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

fn draw_output(output_states: &[State]) -> String {
    assert!(output_states.len() == SCREEN_WIDTH * SCREEN_HEIGHT);

    let mut s = String::new();
    for row in 0..SCREEN_HEIGHT {
        for pixel in 0..SCREEN_WIDTH {
            let sprite_centre: i64 = output_states[row * SCREEN_WIDTH + pixel].reg_x;
            let distance_from_sprite_centre = (pixel as i64) - sprite_centre;

            if distance_from_sprite_centre.abs() <= 1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    s
}

fn main() {
    let instructions: Vec<CPUInstruction> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(CPUInstruction::parse_exn)
        .collect();

    let mut state = State { reg_x: 1 };
    let output_states = run_instructions(&mut state, instructions);

    for (i, state) in output_states.iter().enumerate() {
        println!("{}, {:?}", i, state);
    }

    // Part 1
    let sum_of_signal_strengths_at_interesting_points: i64 = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .copied()
        .map(|x| (x as i64) * output_states[x].reg_x)
        .sum();

    println!("{}", sum_of_signal_strengths_at_interesting_points);

    // Part 2
    println!("{}", draw_output(&output_states[1..]));
}
