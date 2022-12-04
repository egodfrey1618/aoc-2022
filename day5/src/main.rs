use std::fs::read_to_string;

#[derive(Debug)]
struct Configuration(Vec<Vec<char>>);
struct Instruction {
    count: usize,
    // 1-indexed
    from_index: usize,
    to_index: usize,
}

impl Configuration {
    fn apply_move(&mut self, instruction: &Instruction) {
        let from = instruction.from_index - 1;
        let to = instruction.to_index - 1;
        for _ in 0..instruction.count {
            let v = self.0[from].pop().expect("BUG: Move makes column empty");
            self.0[to].push(v)
        }
    }

    fn apply_move2(&mut self, instruction: &Instruction) {
        let from = instruction.from_index - 1;
        let to = instruction.to_index - 1;
        let mut crates_to_move: Vec<char> = vec![];
        for _ in 0..instruction.count {
            let v = self.0[from].pop().expect("BUG: Move makes column empty");
            crates_to_move.push(v);
        }
        // Reverse it. This means that we'll push on in reverse order.
        crates_to_move.reverse();
        for create in crates_to_move {
            self.0[to].push(create)
        }
    }
}

fn main() {
    // Laziness: I can't be bothered to parse out the configuration.
    let starting_configuration = Configuration(vec![
        vec!['F', 'D', 'B', 'Z', 'T', 'J', 'R', 'N'],
        vec!['R', 'S', 'N', 'J', 'H'],
        vec!['C', 'R', 'N', 'J', 'G', 'Z', 'F', 'Q'],
        vec!['F', 'V', 'N', 'G', 'R', 'T', 'Q'],
        vec!['L', 'T', 'Q', 'F'],
        vec!['Q', 'C', 'W', 'Z', 'B', 'R', 'G', 'N'],
        vec!['F', 'C', 'L', 'S', 'N', 'H', 'M'],
        vec!['D', 'N', 'Q', 'M', 'T', 'J'],
        vec!['P', 'G', 'S'],
    ]);

    let instructions: Vec<Instruction> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| s.starts_with("move"))
        .map(|s| {
            let words: Vec<&str> = s.split(' ').collect();
            let count: usize = words[1].parse().unwrap();
            let from_index: usize = words[3].parse().unwrap();
            let to_index: usize = words[5].parse().unwrap();
            Instruction {
                count,
                from_index,
                to_index,
            }
        })
        .collect();

    // Part 1
    let mut configuration = Configuration(starting_configuration.0.to_vec());
    for instruction in &instructions {
        configuration.apply_move(instruction)
    }
    let output: String = configuration.0.iter().map(|s| s[s.len() - 1]).collect();

    println!("{:?}", configuration);
    println!("{:?}", output);

    // Part 2
    let mut configuration = Configuration(starting_configuration.0.to_vec());
    for instruction in &instructions {
        configuration.apply_move2(instruction)
    }
    let output: String = configuration.0.iter().map(|s| s[s.len() - 1]).collect();

    println!("{:?}", configuration);
    println!("{:?}", output);
}
