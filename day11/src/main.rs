use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs::read_to_string;
use std::path::Path;

type Graph = HashMap<String, Vec<String>>;

fn calc_path_count(graph: &Graph, start: &str, target: &str, must_visit: &[&str]) -> usize {
    // We cannot use HashSet because it does not implement Hash and we want to use this set as a key in a HashMap below
    let must_visit_set: BTreeSet<String> = must_visit.iter().map(|s| s.to_string()).collect();

    // For each source/destination, we store how many path are incoming and with each count store how many of the "must-visit" nodes were already visited 
    type IncomingPathTracker = HashMap<BTreeSet<String>, usize>;

    // This function assumes the graph contains no cycles (otherwise there could be infinite paths anyway)
    let mut nodes_to_expand: VecDeque<String> = VecDeque::new();
    nodes_to_expand.push_back(start.to_string());

    let mut incoming_path_counts: HashMap<String, HashMap<String, IncomingPathTracker>> = HashMap::new();  // For each node, stores how many paths are coming from each source node
    let mut tmp: IncomingPathTracker = HashMap::new(); tmp.insert(BTreeSet::new(), 1);
    let mut tmp2 = HashMap::new(); tmp2.insert("dummy".to_string(), tmp);
    incoming_path_counts.insert(start.to_string(), tmp2);
    
    while let Some(cur_node) = nodes_to_expand.pop_front() {
        if cur_node == target {
            continue;
        }

        // Aggregate incoming paths but keep information how many of them include which must-visit nodes (written as explicit loop for readability)
        let mut incoming_to_this_node: IncomingPathTracker = HashMap::new();
        for (_, tracker) in incoming_path_counts.get(&cur_node).unwrap() {
            for (visited_must_visit_nodes, count) in tracker {
                let mut visited_must_visit_nodes_copy = visited_must_visit_nodes.clone();
                if must_visit_set.contains(&cur_node) {
                    visited_must_visit_nodes_copy.insert(cur_node.to_string());
                }
                *incoming_to_this_node.entry(visited_must_visit_nodes_copy).or_default() += count;
            }
        }

        for dest_node in graph.get(&cur_node).unwrap() {
            let entry = incoming_path_counts.entry(dest_node.clone()).or_default();
            entry.insert(cur_node.clone(), incoming_to_this_node.clone());

            if !nodes_to_expand.contains(dest_node) {
                nodes_to_expand.push_back(dest_node.to_string());
            }
        }
    }

    let out_incoming_path_tracker = incoming_path_counts.get(target).unwrap_or_else(|| panic!("No path to target!"));
    out_incoming_path_tracker.values().map(|c| c.get(&must_visit_set).unwrap_or(&0)).sum()
}

fn main() {
    let graph = read_input_file("../inputs/day11_input.txt");
    println!("Total number of paths from you to out (first star): {}", calc_path_count(&graph, "you", "out", &[]));
    println!("Total number of paths from svr to oud that contain dac/fft (second star): {}", calc_path_count(&graph, "svr", "out", &["dac", "fft"]));
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Graph {
    let input = read_to_string(input_path).expect("Could not read file!");
    input.lines().map(|l| {
        let (source, target_list) = l.split_once(":").unwrap();
        let targets = target_list.trim().split_whitespace().map(String::from).collect();

        (source.to_owned(), targets)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let graph = read_input_file("../inputs/day11_example1.txt");
        assert_eq!(calc_path_count(&graph, "you", "out", &[]), 5);
    }

    #[test]
    fn example_second_star() {
        let graph = read_input_file("../inputs/day11_example2.txt");
        assert_eq!(calc_path_count(&graph, "svr", "out", &["dac", "fft"]), 2);
    }
}
