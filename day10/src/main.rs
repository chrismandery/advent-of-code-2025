use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct InputProblem {
    target_config: Vec<bool>,  // not using a bitmap for the sake of readability
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>
}

fn calc_button_presses(input: &InputProblem) -> usize {
    // This is the ugly brute-force solution for the first star
    for steps in 1..=input.buttons.len() {
        for buttons_pressed in input.buttons.iter().combinations(steps) {
            let end_config = get_config_after_button_presses(&buttons_pressed, input.target_config.len());
            assert!(end_config.len() == input.target_config.len());
            if end_config == input.target_config {
                return steps;
            }
        }
    }

    panic!("No solution found!");
}

fn get_config_after_button_presses(buttons_pressed: &Vec<&Vec<usize>>, n: usize) -> Vec<bool> {
    let mut res = vec![false; n];

    for bp in buttons_pressed {
        for i in *bp {
            res[*i] = !res[*i];
        }
    }

    res
}

fn main() {
    let inputs = read_input_file("../inputs/day10_input.txt");
    println!("Total number of button presses (first star methodology): {}", inputs.iter().map(|i| calc_button_presses(&i)).sum::<usize>());
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<InputProblem> {
    let input = read_to_string(input_path).expect("Could not read file!");
    let re_target_config = Regex::new(r"\[([\.#]+)\]").unwrap();
    let re_buttons = Regex::new(r"\(([\d,]+)\)").unwrap();
    let re_joltages = Regex::new(r"\{([\d,]+)\}").unwrap();
    
    input.lines().map(|l| {
        let c = re_target_config.captures(l).expect("Could not find button config when parsing input line!");
        let target_config: Vec<bool> = c[1].chars().map(|c| c == '#').collect();

        let mut buttons = vec!();
        for c in re_buttons.captures_iter(l) {
            let b: Vec<usize> = c[1].split(",").map(|s| s.parse().unwrap()).collect();
            buttons.push(b);
        }

        let c = re_joltages.captures(l).expect("Could not find joltages when parsing input line!");
        let joltages: Vec<usize> = c[1].split(",").map(|s| s.parse().unwrap()).collect();

        assert!(target_config.len() == joltages.len());

        InputProblem {
            target_config,
            buttons,
            joltages
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let inputs = read_input_file("../inputs/day10_example.txt");
        assert_eq!(inputs.iter().map(|i| calc_button_presses(&i)).sum::<usize>(), 7);
    }
}
