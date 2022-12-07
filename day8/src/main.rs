use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point((usize, usize));

struct Trees {
    grid: Vec<Vec<usize>>,
}

impl Trees {
    fn get(&self, p: &Point) -> usize {
        self.grid[p.0 .0][p.0 .1]
    }

    fn points(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                result.push(Point((x, y)));
            }
        }
        result
    }
}

struct Line {
    points: Vec<Point>,
}

impl Line {
    fn all_for_grid(size: usize) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();

        // Left-to-right and right-to-left
        for y in 0..size {
            let points: Vec<Point> = (0..size).map(|x| Point((x, y))).collect();

            let mut points_rev = points.clone();
            points_rev.reverse();

            lines.push(Line { points });
            lines.push(Line { points: points_rev });
        }

        // Top-to-bottom and bottom-to-top
        for x in 0..size {
            let points: Vec<Point> = (0..size).map(|y| Point((x, y))).collect();

            let mut points_rev = points.clone();
            points_rev.reverse();

            lines.push(Line { points });
            lines.push(Line { points: points_rev });
        }

        lines
    }
}

fn trees_to_visible_dirs(t: &Trees) -> HashMap<Point, usize> {
    let lines = Line::all_for_grid(t.grid.len());
    let mut result = HashMap::new();

    for p in t.points() {
        result.insert(p, 0);
    }

    for line in lines {
        // The first point in this direction is visible.
        let first_point = &line.points[0];

        // I'd love to define this as a closure / separate function instead - it's reused
        // in the fold below.
        // But if I do that, I seem to land in borrow-checker hell, or need to define a [fn]
        // and pass in anything I want to capture explicitly, which is a bit gross.
        result.insert(first_point.clone(), result.get(first_point).unwrap() + 1);

        let _ = &line.points[1..]
            .iter()
            .fold(t.get(first_point), |tallest_tree_so_far, point| {
                let height_of_this_tree = t.get(point);
                if height_of_this_tree > tallest_tree_so_far {
                    result.insert(point.clone(), result.get(point).unwrap() + 1);
                }
                height_of_this_tree.max(tallest_tree_so_far)
            });
    }
    result
}

fn scenic_score(t: &Trees, p: &Point) -> usize {
    fn scenic_score_along_line(t: &Trees, p: &Point, line: &Vec<Point>) -> usize {
        let starting_size = t.get(p);
        let mut count = 0;
        for point in line {
            count += 1;
            let this_tree_size = t.get(point);
            if this_tree_size >= starting_size {
                return count;
            }
        }
        count
    }

    let grid_size = t.grid.len();
    let (x, y) = p.0;

    let to_right = (x + 1..grid_size).map(|x| Point((x, y))).collect();
    let to_left = (0..x).rev().map(|x| Point((x, y))).collect();
    let to_bottom = (y + 1..grid_size).map(|y| Point((x, y))).collect();
    let to_up = (0..y).rev().map(|y| Point((x, y))).collect();
    let lines = vec![to_right, to_left, to_bottom, to_up];

    lines
        .iter()
        .map(|line| scenic_score_along_line(t, p, line))
        .reduce(|x, y| x * y)
        .unwrap()
}

fn main() {
    let s: String = read_to_string("input").expect("failed to read input file");
    let grid: Vec<Vec<usize>> = s
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().expect("non-integer in grid"))
                .collect()
        })
        .collect();
    let trees = Trees { grid };
    let visible_count = trees_to_visible_dirs(&trees);

    // Part 1
    let total_visible = visible_count
        .values()
        .copied()
        .filter(|c| *c > 0)
        .fold(0, |x, _| x + 1);

    // Part 2
    let best_scenic_score = trees
        .points()
        .iter()
        .map(|p| scenic_score(&trees, p))
        .max()
        .unwrap();

    println!("{}", total_visible);
    println!("{}", best_scenic_score);
}
