use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply
}

impl Operation {
    fn apply(self: &Self, numbers: &[u64]) -> u64 {
        match self {
            Operation::Add => numbers.iter().sum(),
            Operation::Multiply => numbers.iter().product()
        }
    }
}

#[derive(Debug)]
struct CalcBlock {
    field: Array2D<char>,
    op: Operation
}

fn calc_first_star(block: &CalcBlock) -> u64 {
    let numbers: Vec<u64> = block.field
        .as_rows()
        .iter()
        .map(|row| {
            let s: String = row.iter().collect();
            s.trim().parse().expect("Could not parse as number!")
        })
        .collect();
    block.op.apply(&numbers)
}

fn calc_second_star(block: &CalcBlock) -> u64 {
    let numbers: Vec<u64> = block.field
        .as_columns()
        .iter()
        .map(|row| {
            let s: String = row.iter().collect();
            s.trim().parse().expect("Could not parse as number!")
        })
        .collect();
    block.op.apply(&numbers)
}

fn main() {
    let input = read_input_file("../inputs/day6_input.txt");
    println!("Sum of all solutions (first star methodology): {}", input.iter().map(|i| calc_first_star(&i)).sum::<u64>());
    println!("Sum of all solutions (second star methodology): {}", input.iter().map(|i| calc_second_star(&i)).sum::<u64>());
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<CalcBlock> {
    let input = read_to_string(input_path).expect("Could not read file!");

    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let op_line = lines.pop().unwrap();
    let length = lines[0].len();
    assert!(lines.iter().all(|l| l.len() == length));

    let mut res = vec!();
    let mut start_index = 0;

    for col in 0..=length {
        if col == length || lines.iter().all(|l| l[col] == ' ') {
            assert!(start_index != col);

            // We have reached an empty column or the end -> flush block from start_index to col-1
            let rows: Vec<Vec<char>> = lines.iter().map(|l| l[start_index..col].to_vec()).collect();
            let field = Array2D::from_rows(&rows).unwrap();

            let op = if op_line[start_index..col].contains(&'+') { Operation::Add }
            else if op_line[start_index..col].contains(&'*') { Operation::Multiply }
            else { panic!("Could not extract operation!"); };
            
            res.push(CalcBlock { field, op });

            start_index = col + 1
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day6_example.txt");
        assert_eq!(input.iter().map(|i| calc_first_star(&i)).sum::<u64>(), 4277556);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day6_example.txt");
        assert_eq!(input.iter().map(|i| calc_second_star(&i)).sum::<u64>(), 3263827);
    }
}
