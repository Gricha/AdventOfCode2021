use crate::utils::read_input;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct Node {
    id: String,
    small: bool,
    start: bool,
    end: bool,
    adjacent: BTreeSet<String>,
}

impl Node {
    fn create_from_id(id: &str) -> Self {
        let start = id == "start";
        let end = id == "end";
        let small = ('a'..='z').contains(&id.chars().nth(0).unwrap()) || start;
        Node {
            id: id.to_string(),
            small,
            start,
            end,
            adjacent: BTreeSet::new(),
        }
    }

    fn add_adjacent(&mut self, node: &str) {
        self.adjacent.insert(node.to_string());
    }

    fn step_path(&self, graph: &HashMap<String, Node>, mut path: Path) -> Vec<Path> {
        if self.end {
            return vec![path];
        }

        let mut possible_paths = Vec::new();

        if self.small && path.small_elements_used.contains(&self.id) && path.joker_used {
            // Early quit
            return possible_paths;
        }
        if self.start && path.small_elements_used.contains(&self.id) {
            // no ability to use joker, you cannot reenter start
            return vec![];
        }

        if self.small {
            if path.small_elements_used.contains(&self.id) {
                path.joker_used = true;
            } else {
                path.small_elements_used.insert(self.id.to_string());
            }
        }

        // Otherwise, add self to path and repeat for adjacents
        for node_id in self.adjacent.iter() {
            let paths = graph.get(node_id).unwrap().step_path(graph, path.clone());
            possible_paths.extend(paths.into_iter());
        }

        possible_paths
    }
}

#[derive(Debug, Clone)]
struct Path {
    small_elements_used: BTreeSet<String>,
    joker_used: bool,
}

impl Path {
    fn new() -> Self {
        Path {
            small_elements_used: BTreeSet::new(),
            joker_used: false,
        }
    }
}

pub fn day12() {
    let input = read_input("./day12/input");

    let mut nodes = HashMap::<String, Node>::new();

    input
        .into_iter()
        .filter_map(|l| l.split("-").map(|x| x.to_owned()).collect_tuple())
        .map(|(left, right)| {
            if !nodes.contains_key(&left) {
                nodes.insert(left.to_string(), Node::create_from_id(&left));
            }
            if !nodes.contains_key(&right) {
                nodes.insert(right.to_string(), Node::create_from_id(&right));
            }

            nodes.get_mut(&left).unwrap().add_adjacent(&right);
            nodes.get_mut(&right).unwrap().add_adjacent(&left);
        })
        .for_each(drop);

    let paths = nodes.get("start").unwrap().step_path(&nodes, Path::new());

    println!("{:?}", paths.len());
}
