use anyhow::{anyhow, bail, Result};
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day2_input.txt")?;
    println!("First star (sum of invalid IDs): {}", process_input(&input, false));
    println!("Second star (sum of invalid IDs): {}", process_input(&input, true));

    Ok(())
}

fn process_input(input: &[(u64, u64)], accept_arbitrary_prefix_duplication: bool) -> u64 {
    let mut invalid_id_sum = 0;

    for (start, end) in input {
        for cur in *start..=*end {
            let s = cur.to_string();
            let len = s.len();

            if accept_arbitrary_prefix_duplication {
                // Second star
                for len_prefix in 1..=(len/2) {
                    if len_prefix > 1 && len % len_prefix != 0 { continue };
                    if s == s[0..len_prefix].repeat(len / len_prefix) {
                        invalid_id_sum += cur;
                        break;
                    }
                } 
            } else {
                // First star
                if len % 2 == 0 {
                    if s[0..(len/2)] == s[(len/2)..len] {
                        invalid_id_sum += cur;
                    }
                }
            }
        }
    }

    invalid_id_sum
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<(u64, u64)>> {
    let input = read_to_string(input_path)?;
    let line = input.lines().next().ok_or_else(|| anyhow!("Could not read line!"))?;
    let mut res: Vec<(u64, u64)> = vec!();
    
    for pair_str in line.split(",") {
        let components: Vec<&str> = pair_str.split("-").collect();
        match components.as_slice() {
            [start, end] => { res.push((start.parse()?, end.parse()?)); }
            _ => { bail!("Could not parse: {}", pair_str); }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day2_example.txt").unwrap();
        assert_eq!(process_input(&input, false), 1227775554);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day2_example.txt").unwrap();
        assert_eq!(process_input(&input, true), 4174379265);
    }
}
