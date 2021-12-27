use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    println!("The result is {}", solve("res/input.txt"));
}

fn solve(path: &str) -> u64 {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut scores = Vec::new();
    for line in lines {
        if let Some(score) = parse_brackets(line.unwrap()) {
            scores.push(score);
        }
    }

    scores.sort();

    return scores[scores.len() / 2];
}

fn parse_brackets(line: String) -> Option<u64> {
    let mut bracket_stack = Vec::<char>::new();
    for bracket in line.chars() {
        match (bracket, bracket_stack.last()) {
            ('{', _) | ('[', _) | ('(', _) | ('<', _) => bracket_stack.push(bracket),
            ('}', Some('{')) | (']', Some('[')) | (')', Some('(')) | ('>', Some('<')) => {
                drop(bracket_stack.pop())
            }
            (')', _) | (']', _) | ('}', _) | ('>', _) => return None, // Corrupted lines
            _ => (),
        }
    }

    let mut score = 0;
    for unclosed_bracket in bracket_stack.iter().rev() {
        score *= 5;
        match unclosed_bracket {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => (),
        }
    }

    return Some(score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(288957, solve("res/test-input.txt"));
    }
}
