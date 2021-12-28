use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!(
        "The result is {}",
        compute_polymerization_stages("res/input.txt")
    )
}

fn compute_polymerization_stages(path: &str) -> u32 {
    let (start_polymer, rules) = read_input(path);

    let mut current = start_polymer.clone();
    for _ in 0..10 {
        let mut next = String::new();
        for (left, right) in current.chars().zip(current.chars().skip(1)) {
            next.push(left);
            for rule in &rules {
                if left == rule.left && right == rule.right {
                    next.push(rule.insertion);
                }
            }
        }
        next.push(current.chars().rev().nth(0).unwrap());
        current = next;
    }

    let buckets = current
        .chars()
        .fold(HashMap::<char, u32>::new(), |mut m, c| {
            *m.entry(c).or_default() += 1;
            m
        });
    let min = buckets.iter().map(|(_, &x)| x).min().unwrap();
    let max = buckets.iter().map(|(_, &x)| x).max().unwrap();

    max - min
}

struct PairInsertionRule {
    left: char,
    right: char,
    insertion: char,
}

impl From<&PairInsertionRule> for String {
    fn from(rule: &PairInsertionRule) -> Self {
        format!("{}{} -> {}", rule.left, rule.right, rule.insertion)
    }
}
impl fmt::Display for PairInsertionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<String> for PairInsertionRule {
    fn from(string: String) -> Self {
        let split_string: Vec<&str> = string.split(" -> ").collect();
        assert_eq!(2, split_string.len());
        assert_eq!(2, split_string[0].len());
        assert_eq!(1, split_string[1].len());
        PairInsertionRule {
            left: split_string[0].chars().nth(0).expect("Error parsing rule"),
            right: split_string[0].chars().nth(1).expect("Error parsing rule"),
            insertion: split_string[1].chars().nth(0).expect("Error parsing rule"),
        }
    }
}

fn read_input(path: &str) -> (String, Vec<PairInsertionRule>) {
    let file = File::open(path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Get starting polymer
    let starting_polymer = lines.next().unwrap().unwrap();
    lines.next();

    let mut rules = Vec::new();
    for line in lines {
        rules.push(PairInsertionRule::from(line.expect("Error parsing line")));
    }

    (starting_polymer, rules)
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_folds() {
        assert_eq!(1588, compute_polymerization_stages("res/test-input.txt"))
    }
}
