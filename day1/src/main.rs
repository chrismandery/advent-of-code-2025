use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day1_input.txt")?;
    println!("Number of times dial reaches zero (first star): {}", process_input(&input, false));
    println!("Number of times dial reaches or passes zero (second star): {}", process_input(&input, true));

    Ok(())
}

fn process_input(input: &[i32], count_zeroes_while_turning: bool) -> u32 {
    let mut zero_count = 0;
    let mut cur = 50;

    for rot in input {
        let old = cur;
        cur += rot;

        if count_zeroes_while_turning {
            let passed_zero = if cur > 100 {
                (cur - 1) as u32 / 100
            } else if cur < 0 {
                let v = ((-cur - 1) as u32 / 100) + 1;
                if old == 0 { v - 1 } else { v }  // if we started from zero already that doesn´t count
            } else {
                0
            };

            zero_count += passed_zero;
        }

        cur = cur.rem_euclid(100);

        if cur == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<i32>> {
    let input = read_to_string(input_path)?;
    let res = input
        .lines()
        .map(|l| {
            let (prefix, number_str) = l.split_at(1);
            let number: i32 = number_str.parse().expect("Could not parse!");
            match prefix {
                "L" => -number,
                "R" => number,
                _ => panic!("Unknown prefix: {}", l)
            }
        })
        .collect();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day1_example.txt").unwrap();
        assert_eq!(process_input(&input, false), 3);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day1_example.txt").unwrap();
        assert_eq!(process_input(&input, true), 6);
    }
}
