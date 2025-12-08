use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone)]
struct JunctionBox {
    pos: (u32, u32, u32),
    circuit_id: usize
}

impl JunctionBox {
    fn dist(self: &Self, other: &JunctionBox) -> f64 {
        (
            (self.pos.0 as f64 - other.pos.0 as f64).powi(2) +
            (self.pos.1 as f64 - other.pos.1 as f64).powi(2) +
            (self.pos.2 as f64 - other.pos.2 as f64).powi(2)
        ).sqrt()
    }
}

fn build_ordered_dist_list(boxes: &[JunctionBox]) -> Vec<(usize, usize, f64)> {
    // Build list that contains all pair-wise distances (how ugly, but for 1k entries that's still just 1 million...)
    let mut all_dists = Vec::new();
    for idx1 in 0..boxes.len() {
        for idx2 in 0..idx1 {
            let dist = boxes[idx1].dist(&boxes[idx2]);
            all_dists.push((idx1, idx2, dist));
        }
    }
    all_dists.sort_by(|a, b| a.2.total_cmp(&b.2));
    all_dists
}

fn connect_boxes(boxes: &mut[JunctionBox], idx1: usize, idx2: usize) {
    let circuit1 = boxes[idx1].circuit_id;
    let circuit2 = boxes[idx2].circuit_id;

    if circuit1 != circuit2 {
        // Connect circuits: All boxes belonging to circuit 2 are relabeled to circuit 1 
        for b in boxes.iter_mut() {
            if b.circuit_id == circuit2 {
                b.circuit_id = circuit1;
            }
        }
    }
}

fn connect_n_shortest_boxes(boxes: &mut[JunctionBox], n: usize) {
    let all_dists = build_ordered_dist_list(boxes);

    // Connect n closest pairs
    for (idx1, idx2, _) in all_dists.into_iter().take(n) {
        connect_boxes(boxes, idx1, idx2);
    }
}

fn connect_until_one_circuit(boxes: &mut[JunctionBox]) -> (JunctionBox, JunctionBox) {
    // Returns the two boxes that were connected last before only one circuit was left
    let all_dists = build_ordered_dist_list(boxes);

    // Connect until one circuit if left (quite inefficiently, but sufficient for this problem size)
    for (idx1, idx2, _) in all_dists.into_iter() {
        connect_boxes(boxes, idx1, idx2);

        if boxes.iter().all(|b| b.circuit_id == boxes[0].circuit_id) {
            return (boxes[idx1].clone(), boxes[idx2].clone());
        }
    }

    panic!("Even after last connection, there's more than one circuit. This can never happen.");
}

fn get_top_n_counts_product(nums: &[usize], n: usize) -> usize {
    // Determine most frequent values
    let mut freq_map: HashMap<usize, usize> = HashMap::new();
    for num in nums {
        *freq_map.entry(*num).or_insert(0) += 1;
    }

    // Multiply n highest counts
    let mut freq_vec: Vec<_> = freq_map.into_values().collect();
    freq_vec.sort();
    freq_vec.reverse();
    freq_vec[0..n].iter().product()
}

fn main() {
    let input = read_input_file("../inputs/day8_input.txt");

    // First star
    let mut junction_boxes = input.clone();
    connect_n_shortest_boxes(&mut junction_boxes, 1000);
    let circuit_ids = junction_boxes.iter().map(|b| b.circuit_id).collect::<Vec<usize>>();
    println!("Product of sizes of largest 3 circuits (first star): {}", get_top_n_counts_product(&circuit_ids, 3));

    // Second star
    let mut junction_boxes = input.clone();
    let (box1, box2) = connect_until_one_circuit(&mut junction_boxes);
    println!("Product of X coordinates of boxes connected last (second star): {}", box1.pos.0 * box2.pos.0);
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<JunctionBox> {
    let input = read_to_string(input_path).expect("Could not read file!");
    input.lines().enumerate().map(|(i, l)| {
        let s: Vec<_> = l.split(",").collect();
        assert!(s.len() == 3);
        JunctionBox { pos: (s[0].parse().unwrap(), s[1].parse().unwrap(), s[2].parse().unwrap()), circuit_id: i }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let mut junction_boxes = read_input_file("../inputs/day8_example.txt");
        connect_n_shortest_boxes(&mut junction_boxes, 10);
        let circuit_ids = junction_boxes.iter().map(|b| b.circuit_id).collect::<Vec<usize>>();
        assert_eq!(get_top_n_counts_product(&circuit_ids, 3), 40);
    }


    #[test]
    fn example_second_star() {
        let mut junction_boxes = read_input_file("../inputs/day8_example.txt");
        let (box1, box2) = connect_until_one_circuit(&mut junction_boxes);
        assert_eq!(box1.pos.0 * box2.pos.0, 25272);
    }
}
