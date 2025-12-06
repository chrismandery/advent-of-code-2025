use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum PosType {
    Start,
    End
}

#[derive(Debug, Eq, PartialEq)]
struct PosWithType {
    p: u64,
    t: PosType
}

impl PartialOrd for PosWithType { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl Ord for PosWithType {
    // It is important that start positions come before stop positions (position being the same).
    // Otherwise, we would be counting the position twice in case one range ends where another starts.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.p.cmp(&other.p).then(self.t.cmp(&other.t)) }
}

fn calc_considered_fresh_count(fresh_ranges: &[(u64, u64)]) -> u64 {
    // Ranges can overlap arbitrarily. We build a list of all start/end positions of all the ranges and iterate it to count positions that are in at least one list.
    let mut positions: Vec<PosWithType> = fresh_ranges
        .iter()
        .flat_map(|(start, end)| [
            PosWithType { p: *start, t: PosType::Start },
            PosWithType { p: *end, t: PosType::End }
        ])
        .collect();
    positions.sort();  // We use a list instead of set because there can be duplicate elements (and I'm too lazy to use a multiset)

    let mut active_ranges = 0;
    let mut fresh_count = 0;
    let mut start_active_range = 0;

    for cur in positions {
        match cur.t {
            PosType::Start => {
                if active_ranges == 0 {
                    start_active_range = cur.p;
                }
                active_ranges += 1;
            }
            PosType::End => {
                assert!(active_ranges > 0);
                active_ranges -= 1;
                if active_ranges == 0 {
                    fresh_count += cur.p - start_active_range + 1;
                }
            }
        }
    }

    fresh_count
}

fn count_fresh_ingredients(fresh_ranges: &[(u64, u64)], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|i| fresh_ranges.iter().any(|(start, end)| *i >= start && *i <= end))
        .count()
}

fn main() {
    let (fresh_ranges, ingredients) = read_input_file("../inputs/day5_input.txt");
    println!("Fresh ingredients (first star): {}", count_fresh_ingredients(&fresh_ranges, &ingredients));
    println!("Total number of IDs that are considered fresh (second star): {}", calc_considered_fresh_count(&fresh_ranges));
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> (Vec<(u64, u64)>, Vec<u64>) {
    let input = read_to_string(input_path).expect("Could not read file!");
    let mut fresh_ranges = vec!();
    let mut ingredients = vec!();

    for line in input.lines() {
        if line.contains("-") {
            let s = line.split_once("-").unwrap();
            fresh_ranges.push((s.0.parse().unwrap(), s.1.parse().unwrap()));
        } else if !line.is_empty() {
            ingredients.push(line.parse().unwrap());
        }
    }
    
    (fresh_ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let (fresh_ranges, ingredients) = read_input_file("../inputs/day5_example.txt");
        assert_eq!(count_fresh_ingredients(&fresh_ranges, &ingredients), 3);
    }

    #[test]
    fn example_second_star() {
        let (fresh_ranges, _) = read_input_file("../inputs/day5_example.txt");
        assert_eq!(calc_considered_fresh_count(&fresh_ranges), 14);
    }
}
