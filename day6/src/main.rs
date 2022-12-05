use std::collections::HashSet;
use std::fs::read_to_string;

fn all_unique(window: &str) -> bool {
    let mut seen = HashSet::new();

    for c in window.chars() {
        if seen.contains(&c) {
            return false;
        }
        seen.insert(c);
    }
    true
}

fn find_first_index_with_unique_window(s: &str, window_size: usize) -> usize {
    let index: usize = (window_size - 1..s.len() - 1)
        .find(|&ending_index| {
            let window = &s[ending_index - (window_size - 1)..=ending_index];
            all_unique(window)
        })
        .expect("BUG: No window had all unique chars");

    // The problem wants 1-indexing, so add 1
    index + 1
}

fn main() {
    let s: String = read_to_string("input").expect("Failed to read input file");

    println!("{}", find_first_index_with_unique_window(&s, 4));
    println!("{}", find_first_index_with_unique_window(&s, 14));
}
