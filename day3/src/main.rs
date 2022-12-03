use std::collections::HashSet;
use std::fs::read_to_string;

fn char_priority(c: char) -> usize {
    if c.is_ascii_uppercase() {
        (c as usize) - 0x41 + 27
    } else if c.is_ascii_lowercase() {
        (c as usize) - 0x61 + 1
    } else {
        panic!("can't parse priority of char {}, not alphabet", c)
    }
}

fn unique_from_set<T: std::fmt::Debug>(s: HashSet<T>) -> T {
    if s.len() != 1 {
        panic!("[unique_from_set] called on set of size > 1: {:?}", s)
    }

    // TODO: It feels like there should be a much simpler way of doing this.
    let mut as_vec = s.into_iter().collect::<Vec<T>>();
    as_vec.remove(0)
}

#[derive(Debug)]
struct Rucksack {
    first_half: HashSet<char>,
    second_half: HashSet<char>,
}

impl Rucksack {
    fn of_string(line: &str) -> Self {
        assert!(line.len() % 2 == 0);

        let first_half = line[0..line.len() / 2].chars().collect::<HashSet<char>>();
        let second_half = line[line.len() / 2..].chars().collect::<HashSet<char>>();
        Rucksack {
            first_half,
            second_half,
        }
    }

    fn unique_intersection(&self) -> char {
        let both = &self.first_half & &self.second_half;

        unique_from_set(both)
    }

    fn all_chars(&self) -> HashSet<char> {
        &self.first_half | &self.second_half
    }
}

const CHUNK_SIZE: usize = 3;

fn main() {
    let rucksacks: Vec<Rucksack> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(Rucksack::of_string)
        .collect();

    let total_priority_of_unique_intersections: usize = rucksacks
        .iter()
        .map(|r| r.unique_intersection())
        .map(char_priority)
        .sum();
    println!("{:?}", total_priority_of_unique_intersections);

    assert!(rucksacks.len() % CHUNK_SIZE == 0);
    let total_priority_of_elf_groups: usize = rucksacks
        .chunks(CHUNK_SIZE)
        .map(|rucksack_slice| {
            let chars_in_each_rucksack_in_this_chunk: HashSet<char> = rucksack_slice
                .iter()
                .map(|r| r.all_chars())
                .reduce(|r1, r2| &r1 & &r2)
                .expect("BUG: reduce shouldn't have received chunk of size 0");
            unique_from_set(chars_in_each_rucksack_in_this_chunk)
        })
        .map(char_priority)
        .sum();

    println!("{:?}", total_priority_of_elf_groups);
}
