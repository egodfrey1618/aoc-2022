use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;

const STARTING_LABEL: &str = "AA";

// Weighted graph, stored as an adjacency matrix (which will be useful for the APSP representation)
struct Graph {
    label_to_indices: HashMap<String, usize>,
    flow_rates: Vec<usize>,
    edges: Vec<Vec<Option<usize>>>,
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (s, i) in &self.label_to_indices {
            writeln!(f, "{}: {}", s, i)?;
        }
        writeln!(f, "Edges:")?;
        for (i, edge_row) in self.edges.iter().enumerate() {
            writeln!(f, "{}: {:?}", i, edge_row)?;
        }
        Ok(())
    }
}

impl Graph {
    // Run Floyd-Marshall on a graph
    fn floyd_marshall(&self) -> Vec<Vec<Option<usize>>> {
        let mut matrix: Vec<Vec<Option<usize>>> = self.edges.clone();

        for i in 0..self.edges.len() {
            // Allow considering paths that go through vertex i.
            // So up to this point, we'll have paths that allow vertices 0, ..., i-1.
            for j in 0..self.edges.len() {
                for k in 0..self.edges.len() {
                    // Is there a path from j -> k, going through vertex i?
                    match (matrix[j][i], matrix[i][k]) {
                        (None, _) | (_, None) => (),
                        (Some(x1), Some(x2)) => {
                            let best_path_going_through_vertex = x1 + x2;

                            matrix[j][k] = {
                                match matrix[j][k] {
                                    None => Some(best_path_going_through_vertex),
                                    Some(current) => {
                                        Some(current.min(best_path_going_through_vertex))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        matrix
    }

    fn compress_empty_flow_rates(&self) -> Graph {
        let matrix_using_old_indices = self.floyd_marshall();

        let keep = |i: usize| self.flow_rates[i] != 0 || self.label_to_indices[STARTING_LABEL] == i;

        let new_to_old_index: HashMap<usize, usize> = (0..self.flow_rates.len())
            .filter(|i| keep(*i))
            .enumerate()
            .collect();
        let old_to_new_index: HashMap<usize, usize> =
            new_to_old_index.iter().map(|(x, y)| (*y, *x)).collect();

        let label_to_indices: HashMap<String, usize> = self
            .label_to_indices
            .iter()
            .filter_map(|(label, old_index)| {
                old_to_new_index
                    .get(old_index)
                    .map(|i| (label.to_string(), *i))
            })
            .collect();

        let flow_rates: Vec<usize> = (0..new_to_old_index.len())
            .map(|new_index| {
                let old_index = new_to_old_index.get(&new_index).unwrap();
                self.flow_rates[*old_index]
            })
            .collect();

        let mut edges: Vec<Vec<Option<usize>>> = vec![];
        for i in 0..label_to_indices.len() {
            let mut row: Vec<Option<usize>> = Vec::new();
            for j in 0..label_to_indices.len() {
                let old_i = new_to_old_index.get(&i).unwrap();
                let old_j = new_to_old_index.get(&j).unwrap();
                row.push(matrix_using_old_indices[*old_i][*old_j]);
            }
            edges.push(row);
        }
        Graph {
            label_to_indices,
            flow_rates,
            edges,
        }
    }
}

fn main() {
    let input: Vec<String> = read_to_string("input")
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    // Parsing junk. I'm so bad at this in Rust!
    let parsed: Vec<(String, usize, Vec<String>)> = input
        .iter()
        .map(|s| {
            println!("{}", s);
            let s = s.strip_prefix("Valve ").unwrap();
            let (valve_name, s) = {
                let (before, after) = s.split_once(' ').unwrap();
                (before.to_string(), after)
            };
            let s = s.strip_prefix("has flow rate=").unwrap();
            let (flow_rate, s) = {
                let (before, after) = s.split_once(';').unwrap();
                (before.parse::<usize>().unwrap(), after)
            };
            let s = {
                vec![" tunnel leads to valve ", " tunnels lead to valves "]
                    .iter()
                    .find_map(|prefix| s.strip_prefix(prefix))
                    .unwrap()
            };
            let valves: Vec<String> = s.split(", ").map(|s| s.to_string()).collect();
            (valve_name, flow_rate, valves)
        })
        .collect();

    let label_to_indices: HashMap<String, usize> = parsed
        .iter()
        .enumerate()
        .map(|(i, (name, _, _))| (name.to_string(), i))
        .collect();

    let flow_rates: Vec<usize> = parsed
        .iter()
        .map(|(_name, flow_rate, _edges)| *flow_rate)
        .collect();

    let mut edges: Vec<Vec<Option<usize>>> = vec![];
    for _ in 0..label_to_indices.len() {
        edges.push(vec![None; label_to_indices.len()]);
    }
    for (name, _, valves) in &parsed {
        let i = label_to_indices.get(name).unwrap();
        for valve in valves {
            let j = label_to_indices.get(valve).unwrap();
            edges[*i][*j] = Some(1);
        }
    }

    let graph: Graph = Graph {
        label_to_indices,
        flow_rates,
        edges,
    };

    // Step 1: "Compress" nodes in the graph which have empty flow rates.
    println!("{:?}", graph.compress_empty_flow_rates());

    // Step 2: Combinatorial brute force (basically a DFS), but with some intelligence.
}
