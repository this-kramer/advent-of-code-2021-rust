use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    println!("The result is {}", solve("res/input.txt"));
}

fn solve(path: &str) -> u32 {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut result = 0;
    for line in lines {
        result += parse_brackets(line.unwrap());
    }

    return result;
}

fn parse_brackets(line: String) -> u32 {
    let mut bracket_stack = Vec::<char>::new();
    for bracket in line.chars() {
        match (bracket, bracket_stack.last()) {
            ('{', _) | ('[', _) | ('(', _) | ('<', _) => bracket_stack.push(bracket),
            ('}', Some('{')) | (']', Some('[')) | (')', Some('(')) | ('>', Some('<')) => {
                drop(bracket_stack.pop())
            }
            (')', _) => return 3,
            (']', _) => return 57,
            ('}', _) => return 1197,
            ('>', _) => return 25137,
            _ => (),
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(26397, solve("res/test-input.txt"));
    }
}
