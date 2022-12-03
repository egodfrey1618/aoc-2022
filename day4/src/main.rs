use std::fs::read_to_string;

#[derive(Copy, Clone, Debug)]
struct Interval(usize, usize);

impl Interval {
    fn of_string(s: &str) -> Self {
        let numbers: Vec<usize> = s
            .split('-')
            .map(|s| s.parse::<usize>().expect("expected integer in interval"))
            .collect();

        if numbers.len() != 2 {
            panic!(
                "BUG: Interval.of_string passed not exactly 2 numbers: {}",
                s
            )
        }

        Interval(numbers[0], numbers[1])
    }

    fn contains(&self, other: &Interval) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Interval) -> bool {
        let do_not_overlap = self.0 > other.1 || other.0 > self.1;

        !do_not_overlap
    }
}

#[derive(Debug)]
struct TestCase(Interval, Interval);

impl TestCase {
    // Like in some of the others, I probably should be impl'ing FromStr, and using parse
    fn of_string(s: &str) -> Self {
        let intervals: Vec<Interval> = s.split(',').map(Interval::of_string).collect();

        if intervals.len() != 2 {
            panic!(
                "BUG: TestCase.of_string passed not exactly 2 intervals: {}",
                s
            )
        }

        TestCase(intervals[0], intervals[1])
    }

    fn one_contains_other(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn one_overlaps_other(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

fn main() {
    let cases: Vec<TestCase> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(TestCase::of_string)
        .collect();

    // Part 1
    let answer: usize = cases
        .iter()
        .filter(|t| t.one_contains_other())
        .fold(0, |acc, _| acc + 1);

    println!("{:?}", answer);

    // Part 2
    let answer: usize = cases
        .iter()
        .filter(|t| t.one_overlaps_other())
        .fold(0, |acc, _| acc + 1);

    println!("{:?}", answer);
}
