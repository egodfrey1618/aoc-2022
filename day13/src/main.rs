use std::cmp::Ordering::*;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Sexp {
    Atom(usize),
    List(Vec<Sexp>),
}
use Sexp::*;

impl Sexp {
    fn parse_exn(s: &str) -> Self {
        match s.parse::<usize>() {
            Ok(i) => Atom(i),
            Err(_) => {
                // This must be a list. Split into list elements.
                let mut chars = s.chars();
                assert!(chars.next() == Some('['));
                let mut i = 1;

                let mut split = vec![];
                while i + 1 < s.len() {
                    let mut temp_string = String::new();
                    let mut nesting_count = 0;
                    loop {
                        let c = chars.next().unwrap();
                        i += 1;

                        if i == s.len() {
                            // Closing bracket
                            assert!(c == ']');
                            break;
                        } else if nesting_count == 0 && c == ',' {
                            // At the end of an element.
                            break;
                        } else {
                            match c {
                                '[' => nesting_count += 1,
                                ']' => nesting_count -= 1,
                                _ => (),
                            };
                            temp_string.push(c);
                        }
                    }
                    split.push(temp_string);
                }
                List(split.iter().map(|s| Sexp::parse_exn(s)).collect())
            }
        }
    }

    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        fn compare_slices(x: &[Sexp], y: &[Sexp]) -> std::cmp::Ordering {
            match (x.is_empty(), y.is_empty()) {
                (true, true) => Equal,
                (true, false) => Less,
                (false, true) => Greater,
                (false, false) => match x[0].compare(&y[0]) {
                    Less => Less,
                    Greater => Greater,
                    Equal => compare_slices(&x[1..], &y[1..]),
                },
            }
        }

        match (self, other) {
            (Atom(x), Atom(y)) => x.cmp(y),
            (Atom(x), List(_)) => List(vec![Atom(*x)]).compare(other),
            (List(_), Atom(y)) => self.compare(&List(vec![Atom(*y)])),
            (List(x), List(y)) => compare_slices(x, y),
        }
    }
}

fn main() {
    let s: String = read_to_string("input").expect("Failed to read input file");

    let lines = s
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let sexps: Vec<(Sexp, Sexp)> = lines
        .chunks(2)
        .into_iter()
        .map(|lines| (Sexp::parse_exn(lines[0]), Sexp::parse_exn(lines[1])))
        .collect();

    // Part 1
    let result: usize = sexps
        .iter()
        .enumerate()
        .flat_map(|(i, pair)| {
            if pair.0.compare(&pair.1) == Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();
    println!("{}", result);

    // Part 2
    let mut all_sexps: Vec<Sexp> = sexps.into_iter().flat_map(|s| vec![s.0, s.1]).collect();
    let divider0 = List(vec![List(vec![Atom(2)])]);
    let divider1 = List(vec![List(vec![Atom(6)])]);
    all_sexps.push(divider0.clone());
    all_sexps.push(divider1.clone());

    all_sexps.sort_by(Sexp::compare);

    let index0 = all_sexps
        .binary_search_by(|s| s.compare(&divider0))
        .unwrap()
        + 1;
    let index1 = all_sexps
        .binary_search_by(|s| s.compare(&divider1))
        .unwrap()
        + 1;
    println!("{}", index0 * index1);
}
