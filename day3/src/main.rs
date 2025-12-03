use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

fn calculate_joltage_two_digits(batteries: &[u8]) -> u8 {
    let first_max_index = batteries[0..(batteries.len()-1)].iter().enumerate().rev().max_by_key(|&(_idx, &val)| val).unwrap().0;
    let second_max = batteries[(first_max_index+1)..].iter().max().unwrap();
    batteries[first_max_index] * 10 + second_max
}

fn calculate_joltage_multi_digit_greedy(mut batteries: &[u8], mut remaining_digits: usize) -> u64 {
    // Greedily choose maximum in available slice that always makes sure that there are enough digits left
    let mut res = String::new();

    while remaining_digits > 0 {
        remaining_digits -= 1;
        let last_choosable_index = batteries.len() - remaining_digits;  // e.g., len is 10, 3 digits need to remain -> we can choose max index 7
        let first_max_index = batteries[0..last_choosable_index].iter().enumerate().rev().max_by_key(|&(_idx, &val)| val).unwrap().0;
        res += &batteries[first_max_index].to_string();
        batteries = &batteries[(first_max_index+1)..];
    }

    res.parse().unwrap()
}

fn calculate_total_joltage_two_digits(input: &Vec<Vec<u8>>) -> u64 {
    input.iter().map(|b| calculate_joltage_two_digits(b) as u64).sum::<u64>()
}

fn calculate_total_joltage_multi_digit_greedy(input: &Vec<Vec<u8>>, digit_count: usize) -> u64 {
    input.iter().map(|b| calculate_joltage_multi_digit_greedy(b, digit_count) as u64).sum::<u64>()
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day3_input.txt")?;
    println!("Sum of two-digit joltages (first star): {}", calculate_total_joltage_two_digits(&input));
    println!("Sum of twelve-digit joltages (second star): {}", calculate_total_joltage_multi_digit_greedy(&input, 12));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Vec<u8>>> {
    let input = read_to_string(input_path)?;
    let res = input
        .lines()
        .map(|l| {
            l.chars().map(|c| c.to_digit(10).expect("Input contains non-digit!") as u8).collect()
        })
        .collect();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day3_example.txt").unwrap();
        assert_eq!(calculate_total_joltage_two_digits(&input), 357);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day3_example.txt").unwrap();
        assert_eq!(calculate_total_joltage_multi_digit_greedy(&input, 12), 3121910778619);
    }
}
