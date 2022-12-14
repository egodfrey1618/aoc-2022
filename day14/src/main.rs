use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point(i64, i64);

#[derive(Clone)]
struct Grid(HashSet<Point>);

impl Grid {
    fn next_falling_point(&self, point: &Point) -> Option<Point> {
        vec![point.0, point.0 - 1, point.0 + 1]
            .into_iter()
            .map(|x| Point(x, point.1 + 1))
            .find(|p| !self.0.contains(p))
    }

    fn add_sand(&mut self, point: &Point) {
        let _: bool = self.0.insert(point.clone());
    }

    fn add_line_exn(&mut self, point0: &Point, point1: &Point) {
        let Point(x0, y0) = *point0;
        let Point(x1, y1) = *point1;

        let get_range = |i0: i64, i1: i64| {
            let i_start = i0.min(i1);
            let i_end = i0.max(i1);
            i_start..=i_end
        };

        if x0 == x1 {
            for y in get_range(y0, y1) {
                self.0.insert(Point(x0, y));
            }
        } else if y0 == y1 {
            for x in get_range(x0, x1) {
                self.0.insert(Point(x, y0));
            }
        } else {
            panic!("Can't call [add_line] unless x/y co-ordinates line up.")
        }
    }

    fn largest_y_coord(&self) -> i64 {
        self.0.iter().map(|p| p.1).max().unwrap_or(0)
    }
}

const SAND_START: Point = Point(500, 0);

fn main() {
    let s = read_to_string("input").expect("Failed to read input file");
    let mut grid = Grid(HashSet::new());

    for line in s.split('\n').filter(|s| !s.is_empty()) {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s| {
                let mut numbers = s.split(',').map(|x| x.parse::<i64>().unwrap());
                let x = numbers.next().unwrap();
                let y = numbers.next().unwrap();
                assert!(numbers.next() == None);
                Point(x, y)
            })
            .collect();

        for i in 1..points.len() {
            grid.add_line_exn(&points[i - 1], &points[i]);
        }
    }
    let abyss_level = grid.largest_y_coord() + 1;
    let original_grid = grid.clone();

    // Part 1
    let mut sand_count_rested = 0;
    loop {
        let mut sand_point = SAND_START;
        let mut sand_landed = false;

        while !sand_landed && sand_point.1 < abyss_level {
            match grid.next_falling_point(&sand_point) {
                None => {
                    sand_landed = true;
                }
                Some(new_point) => sand_point = new_point,
            }
        }

        if sand_point.1 == abyss_level {
            break;
        } else {
            sand_count_rested += 1;
            grid.add_sand(&sand_point);
        }
    }
    println!("{}", sand_count_rested);

    // Part 2
    // EG: I'm not thrilled with the code duplication here, I think I could probably
    // fold this all into a function that takes in some sort of stop condition.
    // (Also, it's a bit lazy just to add a really long line for the floor.)
    let mut grid = original_grid;
    let floor_level = grid.largest_y_coord() + 2;
    let floor = (Point(-100_000, floor_level), Point(100_000, floor_level));
    grid.add_line_exn(&floor.0, &floor.1);

    let mut sand_count_rested = 0;
    loop {
        let mut sand_point = SAND_START;
        let mut sand_landed = false;

        while !sand_landed {
            match grid.next_falling_point(&sand_point) {
                None => {
                    sand_landed = true;
                }
                Some(new_point) => sand_point = new_point,
            }
        }

        sand_count_rested += 1;
        grid.add_sand(&sand_point);
        if sand_point.0 == SAND_START.0 && sand_point.1 == SAND_START.1 {
            break;
        }
    }
    println!("{}", sand_count_rested);
}
