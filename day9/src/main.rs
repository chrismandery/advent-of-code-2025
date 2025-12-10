use itertools::Itertools;
use std::cmp::{max, min};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: u64,
    y: u64
}

#[derive(Debug, Eq, PartialEq)]
enum LineType {
    HORIZONTAL,
    VERTICAL
}

#[derive(Debug)]
struct Line {
    lt: LineType,
    coord_static: u64,
    coord_min: u64,
    coord_max: u64
}

impl Pos {
    fn line_between(self: &Self, other: &Pos) -> Line {
        if self.x == other.x {
            Line {
                lt: LineType::VERTICAL,
                coord_static: self.x,
                coord_min: min(self.y, other.y),
                coord_max: max(self.y, other.y)
            }
        } else if self.y == other.y {
            Line {
                lt: LineType::HORIZONTAL,
                coord_static: self.y,
                coord_min: min(self.x, other.x),
                coord_max: max(self.x, other.x)
            }
        } else {
            panic!("Not supported, positions for line_pos must have same X or Y coordinate!")
        }
    }

    fn rectangle_area(self: &Self, other: &Pos) -> u64 {
        let width = (self.x as i64 - other.x as i64).abs() as u64 + 1;
        let height = (self.y as i64 - other.y as i64).abs() as u64 + 1;
        width * height
    }

    fn rectangle_inner_lines(self: &Self, other: &Pos) -> Vec<Line> {
        // Returns the four lines describing a rectangle that is inside given the one described by the points.
        let mut top_left = Pos { x: min(self.x, other.x), y: min(self.y, other.y) };
        let mut top_right = Pos { x: max(self.x, other.x), y: min(self.y, other.y) };
        let mut bottom_left = Pos { x: min(self.x, other.x), y: max(self.y, other.y) };
        let mut bottom_right = Pos { x: max(self.x, other.x), y: max(self.y, other.y) };

        let mut res = Vec::new();

        if (bottom_right.x - top_left.x) > 1 && (bottom_right.y - top_left.y) > 1 {
            // Shrink by one
            top_left.x += 1; top_left.y += 1;
            top_right.x -= 1; top_right.y += 1;
            bottom_left.x += 1; bottom_left.y -= 1;
            bottom_right.x -= 1; bottom_right.y -= 1;

            res.push(top_left.line_between(&top_right));
            res.push(top_right.line_between(&bottom_right));
            res.push(bottom_right.line_between(&bottom_left));
            res.push(bottom_left.line_between(&top_left));
        }

        res
    }
}

impl Line {
    fn intersects(self: &Self, other: &Line) -> bool {
        if self.lt == other.lt {
            return false;
        } else {
            let (vertical, horizontal) = match self.lt {
                LineType::VERTICAL => (self, other),
                LineType::HORIZONTAL => (other, self),
            };

            let x_overlaps = vertical.coord_static >= horizontal.coord_min && vertical.coord_static <= horizontal.coord_max;
            let y_overlaps = horizontal.coord_static >= vertical.coord_min && horizontal.coord_static <= vertical.coord_max;
            x_overlaps && y_overlaps
        }
    }
}

fn calc_largest_rectangle_star1(tiles: &[Pos]) -> u64 {
    tiles
        .iter()
        .combinations(2)
        .map(|v| v[0].rectangle_area(v[1]))
        .max()
        .unwrap()
}

fn calc_largest_rectangle_star2(tiles: &[Pos]) -> u64 {
    // The idea here is that we are building a list of all the lines that lie between red tiles (= green tiles on the border) and then
    // check whether the interior for any potential rectangle intersects any of these.
    let mut border_lines = Vec::new();
    for idx in 0..tiles.len() {
        let pos_a = &tiles[idx];
        let pos_b = if idx < (tiles.len() - 1) { &tiles[idx+1] } else { &tiles[0] };
        border_lines.push(pos_a.line_between(&pos_b));
    }

    tiles
        .iter()
        .combinations(2)
        .filter_map(|v| {
            let inner_lines = v[0].rectangle_inner_lines(&v[1]);

            if inner_lines.iter().all(|l| !border_lines.iter().any(|bl| bl.intersects(&l))) {
                Some(v[0].rectangle_area(v[1]))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn main() {
    let tiles = read_input_file("../inputs/day9_input.txt");
    println!("Area of largest rectangle (first star methodology): {}", calc_largest_rectangle_star1(&tiles));
    println!("Area of largest rectangle (second star methodology): {}", calc_largest_rectangle_star2(&tiles));
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<Pos> {
    let input = read_to_string(input_path).expect("Could not read file!");
    input.lines().map(|l| {
        let (str_a, str_b) = l.split_once(",").unwrap();
        Pos {
            x: str_a.parse().expect("Could not parse number!"),
            y: str_b.parse().expect("Could not parse number!")
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let tiles = read_input_file("../inputs/day9_example.txt");
        assert_eq!(calc_largest_rectangle_star1(&tiles), 50);
    }

    #[test]
    fn example_second_star() {
        let tiles = read_input_file("../inputs/day9_example.txt");
        assert_eq!(calc_largest_rectangle_star2(&tiles), 24);
    }
}
