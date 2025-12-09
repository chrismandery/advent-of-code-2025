use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Eq, Hash, PartialEq)]
struct Pos{
    x: u64,
    y: u64
}

impl Pos {
    fn line_pos(self: &Self, other: &Pos) -> Vec<Pos> {
        if self.x == other.x {
            ((min(self.y, other.y)..=max(self.y, other.y))).map(|y| Pos { x: self.x, y } ).collect()
        } else if self.y == other.y {
            ((min(self.x, other.x)..=max(self.x, other.x))).map(|x| Pos { x, y: self.y } ).collect()
        } else {
            panic!("Not supported, positions for line_pos must have same X or Y coordinate!")
        }
    }

    fn rectangle_area(self: &Self, other: &Pos) -> u64 {
        let width = (self.x as i64 - other.x as i64).abs() as u64 + 1;
        let height = (self.y as i64 - other.y as i64).abs() as u64 + 1;
        width * height
    }

    fn rectangle_inner_points(self: &Self, other: &Pos) -> HashSet<Pos> {
        let mut res = HashSet::new();
        for x in (min(self.x, other.x) + 1)..=(max(self.x, other.x) - 1) {
            for y in (min(self.y, other.y) + 1)..=(max(self.y, other.y) - 1) {
                res.insert(Pos { x, y});
            }
        }
        res
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
    // The idea here is that we are building a set of all the points that lie between red tiles (= green tiles on the border) and then
    // check whether the interior for any potential rectangle contains any of these points.
    let mut border_tiles = HashSet::new();
    for idx in 0..tiles.len() {
        let pos_a = &tiles[idx];
        let pos_b = if idx < (tiles.len() - 1) { &tiles[idx+1] } else { &tiles[0] };
        for pos in pos_a.line_pos(&pos_b) {
            border_tiles.insert(pos);
        }
    }

    dbg!(&border_tiles.len());

    tiles
        .iter()
        .combinations(2)
        .enumerate()
        .filter_map(|(i, v)| {
            println!("Testing {}...", i);

            let inner_points = v[0].rectangle_inner_points(&v[1]);
            dbg!(&inner_points.len());

            if inner_points.is_disjoint(&border_tiles) {
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
