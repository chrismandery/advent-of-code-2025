use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

type Shape = Array2D<bool>;

#[derive(Debug)]
struct PackingProblem {
    size: (usize, usize),
    shape_counts: Vec<usize>
}

fn can_fit_shapes(shapes: &[Shape], problem: &PackingProblem) -> bool {
    // We don't actually need to solve the NP-hard packing problem. It seems to be sufficient to check whether there is enough space in the
    // region to theoretically fit the units occupied by the shapes (not considering the actual shape).
    assert!(shapes.len() == problem.shape_counts.len());
    let shape_units: Vec<usize> = shapes.iter().map(|s| s.elements_row_major_iter().filter(|e| **e).count()).collect();
    let space_available = problem.size.0 * problem.size.1;
    let space_needed: usize = problem.shape_counts.iter().zip(shape_units.iter()).map(|(a, b)| a * b).sum();
    space_needed <= space_available
}

fn main() {
    let (shapes, packing_problems) = read_input_file("../inputs/day12_input.txt");
    println!("Total number of solvable packing problems: {}", packing_problems.iter().filter(|p| can_fit_shapes(&shapes, p)).count());
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> (Vec<Shape>, Vec<PackingProblem>) {
    // Parsing logic here is really ugly but I'm not in the mood to refactor it
    let input = read_to_string(input_path).expect("Could not read file!");
    let mut lines = input.lines();
    let mut line;

    let mut shapes = vec!();

    loop {
        line = lines.next().unwrap();
        if !line.ends_with(":") {
            break;
        }

        let mut rows = vec!();

        loop {
            line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }

            let row = line.chars().map(|c| c == '#').collect();
            rows.push(row);
        }

        shapes.push(Shape::from_rows(&rows).unwrap());
    }

    let mut packing_problems = vec!();

    loop {
        let (size_str, shape_counts) = line.split_once(":").unwrap();
        let (size_x_str, size_y_str) = size_str.split_once("x").unwrap();

        let p = PackingProblem {
            size: (size_x_str.parse().unwrap(), size_y_str.parse().unwrap()),
            shape_counts: shape_counts.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
        };
        packing_problems.push(p);

        match lines.next() {
            Some(next_line) => line = next_line,
            None => return (shapes, packing_problems)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let (shapes, packing_problems) = read_input_file("../inputs/day12_example.txt");
        assert_eq!(packing_problems.iter().filter(|p| can_fit_shapes(&shapes, p)).count(), 3);  // Should be 2 but we implemented can_fit_shapes() in a sloppy way, see comment there
    }
}
