use std::fs::read_to_string;

fn main() {
    let s: String = read_to_string("input").expect("Failed to read input file");

    let lines: Vec<&str> = s.split('\n').collect();
    let number_of_lines = lines.len();

    let mut chunks: Vec<Vec<usize>> = vec![vec![]];

    for (i, line) in lines.into_iter().enumerate() {
        if line.is_empty() {
            if i != number_of_lines - 1 {
                chunks.push(Vec::new())
            }
        } else {
            let number_of_chunks = chunks.len();
            chunks[number_of_chunks - 1].push(line.parse().expect("Couldn't parse line as string"))
        }
    }

    let mut chunk_sizes: Vec<usize> = chunks.iter().map(|chunk| chunk.iter().sum()).collect();
    chunk_sizes.sort_unstable();
    let number_of_chunks = chunks.len();

    println!("Top elf");
    println!("{}", chunk_sizes[number_of_chunks - 1]);
    println!("Top 3 elves");
    println!(
        "{}",
        chunk_sizes[number_of_chunks - 1]
            + chunk_sizes[number_of_chunks - 2]
            + chunk_sizes[number_of_chunks - 3]
    );
}
