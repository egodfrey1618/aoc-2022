use std::collections::HashSet;
use std::fs::read_to_string;

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

use Dir::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_dir(self, dir: &Dir) -> Self {
        let Point { x, y } = self;
        match dir {
            Left => Point { x: x - 1, y },
            Right => Point { x: x + 1, y },
            Up => Point { x, y: y - 1 },
            Down => Point { x, y: y + 1 },
        }
    }

    fn move_towards_point(self, other: &Point) -> Self {
        let other_x = other.x;
        let other_y = other.y;
        let tail_x = self.x;
        let tail_y = self.y;

        // We get to move at most 1 towards the other, but in both the x/y axes.
        let diff_x = other_x - tail_x;
        let diff_y = other_y - tail_y;

        if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
            // We're touching, so nothing to do.
            self
        } else {
            // We're not touching. Move towards!
            let cap_to_1 = |x: i64| x.clamp(-1, 1);
            let move_x = cap_to_1(diff_x);
            let move_y = cap_to_1(diff_y);
            Point {
                x: tail_x + move_x,
                y: tail_y + move_y,
            }
        }
    }
}

fn main() {
    let head_moves: Vec<(Dir, usize)> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut tokens = s.split(' ');
            let dir = match tokens.next() {
                None => panic!("Empty line?"),
                Some("U") => Up,
                Some("D") => Down,
                Some("L") => Left,
                Some("R") => Right,
                Some(_) => panic!("Unrecognised dir"),
            };
            let amount: usize = tokens
                .next()
                .expect("BUG: No distance in input?")
                .parse()
                .expect("Failed to parse distance as usize");
            assert!(tokens.next().is_none());
            (dir, amount)
        })
        .collect();

    let mut knot_positions: Vec<Point> = (0..10).map(|_| Point { x: 0, y: 0 }).collect();
    let mut all_knot1_positions = Vec::new();
    let mut all_knot9_positions = Vec::new();

    for (dir, moves) in head_moves {
        for _ in 0..moves {
            for i in 0..knot_positions.len() {
                if i == 0 {
                    // move the head
                    // EG: I'm not happy with these clones here - it feels like I should be able to
                    // say I'm giving up ownership because I'm immediately replacing that element in
                    // the vec, but the borrow checker doesn't know that. A better way might be to make
                    // [move_dir] and [move_towards_point] take &mut Point instead, and mutate rather
                    // than creating a copy.
                    knot_positions[0] = knot_positions[0].clone().move_dir(&dir);
                } else {
                    // move knot i towards i-1
                    knot_positions[i] = knot_positions[i]
                        .clone()
                        .move_towards_point(&knot_positions[i - 1]);
                }
            }
            all_knot1_positions.push(knot_positions[1].clone());
            all_knot9_positions.push(knot_positions[9].clone());
        }
    }

    let print = |positions: Vec<Point>| {
        let x = positions.iter().cloned().collect::<HashSet<Point>>().len();
        println!("{}", x);
    };
    print(all_knot1_positions);
    print(all_knot9_positions);
}
