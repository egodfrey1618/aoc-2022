use std::fs::read_to_string;

struct Grid {
    heights: Vec<Vec<usize>>,
}

impl Grid {
    fn neighbours(&self, v: (usize, usize)) -> Vec<(usize, usize)> {
        let x_size = self.heights.len();
        let y_size = self.heights[0].len();

        let left = if v.0 != 0 { Some((v.0 - 1, v.1)) } else { None };
        let right = if v.0 != x_size - 1 {
            Some((v.0 + 1, v.1))
        } else {
            None
        };
        let up = if v.1 != 0 { Some((v.0, v.1 - 1)) } else { None };
        let down = if v.1 != y_size - 1 {
            Some((v.0, v.1 + 1))
        } else {
            None
        };

        vec![left, right, up, down]
            .into_iter()
            .flatten()
            .filter(|p| self.heights[p.0][p.1] <= self.heights[v.0][v.1] + 1)
            .collect()
    }

    fn bfs(&self, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
        let x_size = self.heights.len();
        let y_size = self.heights[0].len();

        let mut distances = Vec::new();
        for _ in 0..x_size {
            let mut v = Vec::new();
            for _ in 0..y_size {
                v.push(None)
            }
            distances.push(v)
        }
        distances[start.0][start.1] = Some(0);

        let mut frontier = vec![start];
        while !frontier.is_empty() && distances[end.0][end.1] == None {
            let mut new_frontier = Vec::new();
            for v in frontier {
                for n in self.neighbours(v) {
                    if distances[n.0][n.1] == None {
                        new_frontier.push(n);
                        distances[n.0][n.1] = Some(distances[v.0][v.1].unwrap() + 1);
                    }
                }
            }
            frontier = new_frontier;
        }
        distances[end.0][end.1]
    }
}

fn main() {
    let lines: Vec<Vec<char>> = read_to_string("input")
        .expect("Failed to read input!")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let find_all_points_with_char = |target_char| {
        let points: Vec<(usize, usize)> = lines
            .iter()
            .enumerate()
            .flat_map(|(x, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(y, c)| {
                        if *c == target_char {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();
        points
    };

    let find_char = |target_char| {
        let v = find_all_points_with_char(target_char);
        if v.len() == 1 {
            v[0]
        } else {
            panic!("Did not find unique point equal to char")
        }
    };

    let start_char = 'S';
    let end_char = 'E';

    let start = find_char(start_char);
    let end = find_char(end_char);

    let char_to_score = |char: &char| {
        if *char == start_char {
            'a' as usize
        } else if *char == end_char {
            'z' as usize
        } else {
            *char as usize
        }
    };

    let heights: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| line.iter().map(char_to_score).collect())
        .collect();
    let grid = Grid { heights };
    println!("Best path from point S: {:?}", grid.bfs(start, end));

    // Dumb solution for part 2: Just try starting from every point labelled 'a' separately.
    // I think a much better solution would be to work backwards from the end-point, and seeing
    // the first 'a' point I hit - which is going to be asymptotically a lot quicker. But this
    // is easily fast enough.
    let a_points = find_all_points_with_char('a');
    let best_path_from_any_a_point = a_points
        .into_iter()
        .flat_map(|start| grid.bfs(start, end))
        .reduce(usize::min);
    println!(
        "Best path from any a point: {:?}",
        best_path_from_any_a_point
    );
}
