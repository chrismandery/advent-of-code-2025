use array2d::Array2D;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

type Field = Array2D<bool>;

fn calc_beam_splits_and_paths(splitter_pos: Field, start_col: usize) -> (usize, usize) {
    // Calculate the number of splits (first star) and the total number of possible paths (second star).
    let mut split_count = 0;
    let mut active_x_pos = BTreeMap::new();
    active_x_pos.insert(start_col, 1);

    for cur_row in 1..splitter_pos.num_rows() {
        let mut new_active_x_pos = BTreeMap::new();

        for (x_pos, count) in active_x_pos {
            if *splitter_pos.get(cur_row, x_pos).unwrap() {
                assert!(x_pos > 0 && x_pos < splitter_pos.num_columns());

                let new_count = new_active_x_pos.entry(x_pos - 1).or_insert(0);
                *new_count += count;

                let new_count = new_active_x_pos.entry(x_pos + 1).or_insert(0);
                *new_count += count;

                split_count += 1;
            } else {
                let new_count = new_active_x_pos.entry(x_pos).or_insert(0);
                *new_count += count;
            }
        }

        active_x_pos = new_active_x_pos;
    }

    (split_count, active_x_pos.values().sum())
}

fn main() {
    let (splitter_pos, start_col) = read_input_file("../inputs/day7_input.txt");
    let (beam_splits, possible_paths) = calc_beam_splits_and_paths(splitter_pos, start_col);
    println!("Total number of beam splits (first star): {}", beam_splits);
    println!("Total number of possible paths (second star): {}", possible_paths);
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> (Field, usize) {
    // Returns the field (positions with a splitter) and the start X position
    let input = read_to_string(input_path).expect("Could not read file!");
    let rows: Vec<Vec<bool>> = input.lines().map(|l| l.chars().map(|c| c == '^').collect()).collect();
    let field = Field::from_rows(&rows).unwrap();
    let start_col = input.lines().next().unwrap().chars().position(|c| c == 'S').expect("Could not find start position!");
    (field, start_col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let (splitter_pos, start_col) = read_input_file("../inputs/day7_example.txt");
        assert_eq!(calc_beam_splits_and_paths(splitter_pos, start_col).0, 21);
    }

    #[test]
    fn example_second_star() {
        let (splitter_pos, start_col) = read_input_file("../inputs/day7_example.txt");
        assert_eq!(calc_beam_splits_and_paths(splitter_pos, start_col).1, 40);
    }
}
