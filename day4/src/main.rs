use array2d::Array2D;
use std::cmp::{min, max};
use std::fs::read_to_string;
use std::path::Path;

type Field = Array2D<bool>;

fn count_and_remove_accessible_rolls(field: &Field) -> (usize, Field) {
    // Returns the count of accessible rolls and an updated field with them being removed
    let mut accessible_rolls = 0;
    let mut updated_field = field.clone();

    let num_rows = field.num_rows();
    let num_cols = field.num_columns();
    
    for row in 0..num_rows {
        for col in 0..num_cols {
            if !*field.get(row, col).unwrap() {
                continue;
            }

            // Count neighbor rolls
            let mut neighbor_count= 0;
            for neighbor_row in (max(row, 1) - 1)..min(row + 2, num_rows) {
                for neighbor_col in (max(col, 1) - 1)..min(col + 2, num_cols) {
                    if (neighbor_row != row || neighbor_col != col) && *field.get(neighbor_row, neighbor_col).unwrap() {
                        neighbor_count += 1;
                    }
                }
            }

            if neighbor_count < 4 {
                accessible_rolls += 1;
                updated_field.set(row, col, false).unwrap();
            }
        }
    }

    (accessible_rolls, updated_field)
}

fn main() {
    let field = read_input_file("../inputs/day4_input.txt");
    println!("Immediately accessible rolls of paper (first star): {}", count_and_remove_accessible_rolls(&field).0);
    println!("Iteratively accessible rolls of paper (first star): {}", count_iteratively_accessible_rolls(&field));
}

#[allow(dead_code)]
fn print_field(field: &Field) {
    // For debugging
    for row in 0..field.num_rows() {
        for col in 0..field.num_columns() {
            if *field.get(row, col).unwrap() {
                print!("x")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Field {
    let input = read_to_string(input_path).expect("Could not read file!");
    let rows: Vec<Vec<bool>> = input.lines().map(|l| l.chars().map(|c| c == '@').collect()).collect();
    Field::from_rows(&rows).unwrap()
}

fn count_iteratively_accessible_rolls(initial_field: &Field) -> usize {
    let mut total_accessible = 0;
    let mut cur_field = initial_field.clone();
    let mut cur_removed;

    loop {
        (cur_removed, cur_field) = count_and_remove_accessible_rolls(&cur_field);
        if cur_removed == 0 {
            return total_accessible;
        }
        total_accessible += cur_removed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let field = read_input_file("../inputs/day4_example.txt");
        assert_eq!(count_and_remove_accessible_rolls(&field).0, 13);
    }

    #[test]
    fn example_second_star() {
        let field = read_input_file("../inputs/day4_example.txt");
        assert_eq!(count_iteratively_accessible_rolls(&field), 43);
    }
}
