use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Interval(i64, i64);

#[derive(Clone, Debug)]
struct Position(i64, i64);

impl Interval {
    fn overlaps_or_adjacent(&self, other: &Self) -> bool {
        let do_not_overlap = self.0 > other.1 + 1 || other.0 > self.1 + 1;

        !do_not_overlap
    }

    fn union(&self, other: &Self) -> Self {
        // only makes sense if they overlap or adjacent
        Interval(self.0.min(other.0), self.1.max(other.1))
    }

    fn len(&self) -> i64 {
        self.1 - self.0 + 1
    }
}

impl Position {
    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.1 - other.1).abs() + (self.0 - other.0).abs()
    }
}

#[derive(Debug)]
struct DisjointIntervals(Vec<Interval>);

impl DisjointIntervals {
    fn create(mut input: Vec<Interval>) -> Self {
        let mut result = Vec::new();

        for i in 0..input.len() {
            let overlapping_interval = (i + 1..input.len()).find_map(|j| {
                if input[j].overlaps_or_adjacent(&input[i]) {
                    Some(j)
                } else {
                    None
                }
            });

            match overlapping_interval {
                None => result.push(input[i].clone()),
                Some(j) => input[j] = input[j].union(&input[i]),
            }
        }
        DisjointIntervals(result)
    }

    fn len(&self) -> i64 {
        self.0.iter().map(Interval::len).sum()
    }
}

fn find_range_with_no_beacon(
    sensor: &Position,
    beacon: &Position,
    y_value: i64,
) -> Option<Interval> {
    let beacon_distance = sensor.manhattan_distance(beacon);
    let sensor_projected_to_y_axis = Position(sensor.0, y_value);
    let distance_to_y_axis = sensor.manhattan_distance(&sensor_projected_to_y_axis);

    if distance_to_y_axis > beacon_distance {
        // We learn nothing - the range around the sensor doesn't intersect the y-axis.
        None
    } else {
        // point1 and point2 are the two points on the y-axis, the same distance from the sensor
        // as the beacon. (We possibly have a degenerate case where they're equal.)
        let mut point1 = sensor.0 - (beacon_distance - distance_to_y_axis);
        let mut point2 = sensor.0 + (beacon_distance - distance_to_y_axis);

        // If either one is the beacon, we don't want to include those in our interval.
        if beacon.1 == y_value {
            if beacon.0 == point1 {
                point1 += 1
            }
            if beacon.0 == point2 {
                point2 -= 1
            }
        }

        // Then turn this into the interval!
        if point1 <= point2 {
            Some(Interval(point1, point2))
        } else {
            None
        }
    }
}

fn find_intervals_with_no_beacon(
    sensor_beacon_pairs: &[(Position, Position)],
    y_value: i64,
    include_known_beacons_in_result: bool,
) -> DisjointIntervals {
    let mut intervals: Vec<Interval> = sensor_beacon_pairs
        .iter()
        .filter_map(|(sensor, beacon)| find_range_with_no_beacon(sensor, beacon, y_value))
        .collect();

    if include_known_beacons_in_result {
        for (_sensor, beacon) in sensor_beacon_pairs {
            if beacon.1 == y_value {
                intervals.push(Interval(beacon.0, beacon.0));
            }
        }
    }
    DisjointIntervals::create(intervals)
}

fn main() {
    let lines: Vec<String> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let parse = |s: &str| -> (Position, Position) {
        let mut words = s.split(' ');
        assert!(words.next() == Some("Sensor"));
        assert!(words.next() == Some("at"));

        let parse_int_filtering_non_numeric = |s: &str| -> i64 {
            let t: String = s
                .chars()
                .filter(|c| *c == '-' || c.is_ascii_digit())
                .collect();
            t.parse().unwrap()
        };
        let sensor_x = parse_int_filtering_non_numeric(words.next().unwrap());
        let sensor_y = parse_int_filtering_non_numeric(words.next().unwrap());
        assert!(words.next() == Some("closest"));
        assert!(words.next() == Some("beacon"));
        assert!(words.next() == Some("is"));
        assert!(words.next() == Some("at"));
        let beacon_x = parse_int_filtering_non_numeric(words.next().unwrap());
        let beacon_y = parse_int_filtering_non_numeric(words.next().unwrap());

        let sensor = Position(sensor_x, sensor_y);
        let beacon = Position(beacon_x, beacon_y);
        (sensor, beacon)
    };

    let sensor_and_beacons: Vec<(Position, Position)> = lines.iter().map(|s| parse(s)).collect();

    // Part1
    let interesting_y_value = 2_000_000;

    let disjoint_intervals =
        find_intervals_with_no_beacon(&sensor_and_beacons, interesting_y_value, false);

    println!("{:?}", disjoint_intervals);
    println!("{:?}", disjoint_intervals.len());

    // Part2.
    // I'm just running my solution for part 1 over all possible y-values, and then seeing which one doesn't cover the whole interval.
    // I suspect there's a faster way to do this, but I'm not seeing it.
    let bound = 4_000_000;
    for y in 0..=bound {
        let disjoint_intervals = find_intervals_with_no_beacon(&sensor_and_beacons, y, true);
        if disjoint_intervals.0.len() > 1 {
            // I then just worked out the actual co-ordinates by hand from this output.
            println!("{}, {:?}", y, disjoint_intervals);
        }
    }
}
