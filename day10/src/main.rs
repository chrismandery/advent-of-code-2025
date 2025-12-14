use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct InputProblem {
    target_config: Vec<bool>,
    _buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
    reachable_config_patterns_with_steps: HashMap<Vec<bool>, usize>,
    reachable_joltage_patterns_with_steps: HashMap<Vec<usize>, usize>
}

fn build_patterns(buttons: &Vec<Vec<usize>>, config_len: usize) -> (HashMap<Vec<bool>, usize>, HashMap<Vec<usize>, usize>) {
    let mut res_config = HashMap::new();
    let mut res_joltage = HashMap::new();

    for steps in 0..=buttons.len() {
        for buttons_pressed in buttons.iter().combinations(steps) {
            let joltages = get_joltages_after_button_presses(&buttons_pressed, config_len);
            let config = joltages.iter().map(|j| j % 2 == 1).collect();
            assert!(joltages.len() == config_len);

            if !res_config.contains_key(&config) {
                res_config.insert(config, steps);
            }
            if !res_joltage.contains_key(&joltages) {
                res_joltage.insert(joltages, steps);
            }
        }
    }

    (res_config, res_joltage)
}

fn calc_button_presses_to_reach_joltage(ip: &InputProblem, joltages: &Vec<usize>,) -> Option<usize> {
    // Unfortunately, I didn't come up with this clever algorithm by myself, but took inspiration from
    // https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    if joltages.iter().all(|j| *j == 0) {
        return Some(0);
    }

    let mut min_steps: Option<usize> = None;

    for (pattern, pattern_steps) in &ip.reachable_joltage_patterns_with_steps {
        if joltages.iter().zip(pattern.iter()).any(| (j, p)| p > j || (p % 2) != (j % 2)) {
            continue;
        }

        let remainder: Vec<_> = joltages.iter().zip(pattern.iter()).map(|(j, p)| j - p).collect();
        let remainder_halfed: Vec<_> = remainder.iter().map(|j| j / 2).collect();

        if let Some(sub_solution_steps) = calc_button_presses_to_reach_joltage(ip, &remainder_halfed) {
            let total_steps = pattern_steps + 2 * sub_solution_steps;
            min_steps = Some(min_steps.map_or(total_steps, |m| m.min(total_steps)));
        }
    }

    min_steps
}

fn get_joltages_after_button_presses(buttons_pressed: &Vec<&Vec<usize>>, n: usize) -> Vec<usize> {
    let mut res = vec![0; n];

    for bp in buttons_pressed {
        for i in *bp {
            res[*i] += 1;
        }
    }

    res
}

fn main() {
    let inputs = read_input_file("../inputs/day10_input.txt");
    println!("Total number of button presses (first star methodology): {}",
        inputs.iter().map(|i| i.reachable_config_patterns_with_steps[&i.target_config]).sum::<usize>());
    println!("Total number of button presses (second star methodology): {}",
        inputs.iter().map(|i| calc_button_presses_to_reach_joltage(&i, &i.joltages).unwrap()).sum::<usize>());
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

        // Pre-calculate patterns for problem solving
        let (reachable_config_patterns_with_steps, reachable_joltage_patterns_with_steps) = build_patterns(&buttons, joltages.len());

        InputProblem {
            target_config,
            _buttons: buttons,
            joltages,
            reachable_config_patterns_with_steps,
            reachable_joltage_patterns_with_steps
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let inputs = read_input_file("../inputs/day10_example.txt");
        assert_eq!(inputs.iter().map(|i| i.reachable_config_patterns_with_steps[&i.target_config]).sum::<usize>(), 7);
    }

    #[test]
    fn example_second_star() {
        let inputs = read_input_file("../inputs/day10_example.txt");
        assert_eq!(inputs.iter().map(|i| calc_button_presses_to_reach_joltage(&i, &i.joltages).unwrap()).sum::<usize>(), 33);
    }
}
