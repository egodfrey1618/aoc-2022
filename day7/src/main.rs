use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct DirPath(String);

impl DirPath {
    fn extend(&self, next: &str) -> DirPath {
        DirPath(self.0.clone() + next + "/")
    }

    fn root_path() -> Self {
        DirPath("/".to_string())
    }
}

// In OCaml, I'd probably want to have subdirs contain a reference to [Dir] instead, rather than just a path
// I couldn't work out how to get that in Rust, you get into borrowchecker hell, so I'm keeping just a reference
// to the name of the subdirectory, and have to keep swapping between that and the HashMap.

#[derive(Debug)]
struct Dir {
    // path is represented as e.g. /a/b/c
    files: HashMap<String, usize>,
    subdirs: HashSet<DirPath>,
    parent: Option<DirPath>,
}

#[derive(Debug)]
struct Filesystem {
    all_dirs: HashMap<DirPath, Dir>,
}

impl Filesystem {
    fn new() -> Self {
        let root_path = DirPath::root_path();

        let root_dir: Dir = Dir {
            files: HashMap::new(),
            subdirs: HashSet::new(),
            parent: None,
        };
        let mut all_dirs = HashMap::new();
        all_dirs.insert(root_path, root_dir);
        Filesystem { all_dirs }
    }

    fn add_subdir(&mut self, path: &DirPath, name: &str) {
        let dir = self
            .all_dirs
            .get_mut(path)
            .expect("BUG: [add_subdir] called with non-existent path");
        let subpath = path.extend(name);

        if !dir.subdirs.contains(&subpath) {
            dir.subdirs.insert(subpath.clone());

            let subdir: Dir = Dir {
                files: HashMap::new(),
                subdirs: HashSet::new(),
                parent: Some(path.clone()),
            };

            self.all_dirs.insert(subpath, subdir);
        }
    }

    fn add_file(&mut self, path: &DirPath, name: &str, size: usize) {
        let dir = self
            .all_dirs
            .get_mut(path)
            .expect("BUG: [add_file] called with non-existent path");

        match dir.files.get(name) {
            None => {
                let _ = dir.files.insert(name.to_string(), size);
            }
            Some(current_size) => {
                if size != *current_size {
                    panic!("File already added, with a different size")
                }
            }
        }
    }

    fn parent_dir(&self, path: &DirPath) -> Option<DirPath> {
        let dir = self
            .all_dirs
            .get(path)
            .expect("BUG: [parent_dir] called with non-existent path");

        dir.parent.clone()
    }

    fn total_dir_sizes(&self) -> HashMap<DirPath, usize> {
        fn populate_for_dir(
            filesystem: &Filesystem,
            result: &mut HashMap<DirPath, usize>,
            path: &DirPath,
        ) {
            let dir = filesystem
                .all_dirs
                .get(path)
                .expect("BUG: [populate_for_dir] called with non-existent path");

            let mut total_size = 0;

            // Populate the map recursively for each subdir, and include their weights.
            for subdir in dir.subdirs.iter() {
                populate_for_dir(filesystem, result, subdir);
                total_size += result.get(subdir).unwrap();
            }

            // Don't forget the files!
            for size in dir.files.values() {
                total_size += size;
            }
            let _ = result.insert(path.clone(), total_size);
        }

        let mut result = HashMap::new();
        populate_for_dir(self, &mut result, &DirPath::root_path());
        result
    }
}
struct ParserState {
    working_dir: DirPath,
    in_ls_command: bool,
}

fn parse_input_into_filesystem(s: Vec<&str>) -> Filesystem {
    let mut parser_state = ParserState {
        working_dir: DirPath::root_path(),
        in_ls_command: false,
    };

    let mut filesystem = Filesystem::new();

    for input in s.into_iter() {
        if input == "$ ls" {
            parser_state.in_ls_command = true;
        } else if input.starts_with("$ cd ") {
            parser_state.in_ls_command = false;
            let subdir = input.strip_prefix("$ cd ").unwrap();

            if subdir == ".." {
                parser_state.working_dir = filesystem
                    .parent_dir(&parser_state.working_dir)
                    .expect("Tried to cd .. from top-level");
            } else if subdir == "/" {
                parser_state.working_dir = DirPath::root_path();
            } else {
                parser_state.working_dir = parser_state.working_dir.extend(subdir);
            }
        } else if input.starts_with('$') {
            panic!("Unrecognised command: {}", input);
        } else {
            // We're not in a command. Better hope we were expecting this.
            if !parser_state.in_ls_command {
                panic!("BUG! Saw something that didn't look like a command, but I wasn't expecting output from ls")
            }

            if input.starts_with("dir") {
                // We're listing a directory
                filesystem.add_subdir(
                    &parser_state.working_dir,
                    input.strip_prefix("dir ").unwrap(),
                );
            } else {
                // We're listing a file
                let mut tokens = input.split(' ');
                let size = tokens
                    .next()
                    .expect("BUG: empty line in input?")
                    .parse::<usize>()
                    .expect("Couldn't parse size of file as an integer");
                let name = tokens.next().expect("BUG: No name of file?");
                match tokens.next() {
                    Some(_) => panic!("Wasn't expecting any more words in a line"),
                    None => (),
                };

                filesystem.add_file(&parser_state.working_dir, name, size);
            }
        }
    }
    filesystem
}

fn main() {
    let input: String = read_to_string("input").expect("Failed to read input");
    let lines = input.split('\n').filter(|s| !s.is_empty()).collect();

    let filesystem = parse_input_into_filesystem(lines);

    for (key, value) in filesystem.all_dirs.iter() {
        println!("{:?}: {:?}", key, value);
    }

    let total_dir_sizes = filesystem.total_dir_sizes();

    // Part 1: Total size of all dirs with cumulative weight <= 10^5
    let total: usize = total_dir_sizes
        .values()
        .copied()
        .filter(|x| *x <= 100000)
        .sum();

    println!("{}", total);

    // Part 2: Delete the smallest directory such that the total free size is at least 3e7
    let total_used_size = total_dir_sizes.get(&DirPath::root_path()).unwrap();
    let required_free_size = 30_000_000;
    let total_disk_size = 70_000_000;
    let current_free_size = total_disk_size - total_used_size;
    let need_to_free_up_at_least = required_free_size - current_free_size;

    let smallest_dir_that_frees_up_enough_space = total_dir_sizes
        .values()
        .copied()
        .filter(|x| *x >= need_to_free_up_at_least)
        .min()
        .expect("Oh no! No directory is big enough.");
    println!("{}", smallest_dir_that_frees_up_enough_space);
}
