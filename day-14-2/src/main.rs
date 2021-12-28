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

fn compute_polymerization_stages(path: &str) -> u64 {
    let (start_polymer, rules) = read_input(path);

    // Store consecutive neighbors in polymer string in a hashmap to reduce size
    let mut pairs = HashMap::<(char, char), u64>::new();
    for (left, right) in start_polymer.chars().zip(start_polymer.chars().skip(1)) {
        *pairs.entry((left, right)).or_insert(0) += 1;
    }

    // Run 40 round of polymerization
    for _ in 0..40 {
        let mut new_pairs = HashMap::<(char, char), u64>::new();
        'outer: for (&(left, right), &c) in pairs.iter() {
            for rule in &rules {
                if left == rule.left && right == rule.right {
                    *new_pairs.entry((left, rule.insertion)).or_insert(0) += c;
                    *new_pairs.entry((rule.insertion, right)).or_insert(0) += c;
                    continue 'outer;
                }
            }
            *new_pairs.entry((left, right)).or_insert(0) += c;
        }
        pairs = new_pairs;
    }
    let mut buckets = pairs.iter().fold(
        HashMap::<char, u64>::new(),
        |mut m, (&(left, right), &count)| {
            *m.entry(left).or_default() += count;
            *m.entry(right).or_default() += count;
            m
        },
    );

    // First and last element does not appear twice in sum of pairs item's
    *buckets
        .entry(
            start_polymer
                .chars()
                .nth(0)
                .expect("Error getting first character"),
        )
        .or_default() += 1;
    *buckets
        .entry(
            start_polymer
                .chars()
                .rev()
                .nth(0)
                .expect("Error getting the last character"),
        )
        .or_default() += 1;

    // Find min and max
    let min = buckets.iter().map(|(_, &x)| x).min().unwrap() / 2;
    let max = buckets.iter().map(|(_, &x)| x).max().unwrap() / 2;

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
        assert_eq!(
            2188189693529,
            compute_polymerization_stages("res/test-input.txt")
        )
    }
}
