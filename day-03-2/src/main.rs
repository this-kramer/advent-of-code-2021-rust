use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("res/input.txt");
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut data: Vec<String> = Vec::new();

    for line in lines {
        let line: String = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };
        data.push(line);
    }

    let oxygen = to_number(find_best_string(&data, true, true));
    let co2 = to_number(find_best_string(&data, false, false));
    println!(
        "Oxygen: {}, CO2: {}, product: {}",
        oxygen,
        co2,
        oxygen * co2
    );
}

fn find_best_string(input: &Vec<String>, most_common: bool, one_tie: bool) -> String {
    let size = input[0].len();

    // oxygen generator rating
    let mut filter: String = String::new();

    for digit in 0..size {
        // Counters
        let mut ones = 0;
        let mut zeros = 0;

        for entry in input {
            if entry.starts_with(&filter) {
                match entry.chars().nth(digit) {
                    Some('1') => ones += 1,
                    Some('0') => zeros += 1,
                    _ => panic!("Unexpected character or index out of bounds"),
                }
            }
        }

        // The actual logic
        filter += match (zeros.cmp(&0), ones.cmp(&0), ones.cmp(&zeros)) {
            (Ordering::Greater, Ordering::Equal, _) => "0",
            (Ordering::Equal, Ordering::Greater, _) => "1",
            (Ordering::Equal, Ordering::Equal, _) => panic!("Something went wrong!"),
            (_, _, Ordering::Equal) => {
                if one_tie {
                    "1"
                } else {
                    "0"
                }
            }
            (_, _, Ordering::Greater) => {
                if most_common {
                    "1"
                } else {
                    "0"
                }
            }
            (_, _, Ordering::Less) => {
                if most_common {
                    "0"
                } else {
                    "1"
                }
            }
        };
    }

    // Return result
    for entry in input {
        if entry.starts_with(&filter) {
            return entry.to_string();
        }
    }
    panic!("Result not found, that's weird!");
}

fn to_number(bitstring: String) -> u32 {
    let mut result = 0;
    for c in bitstring.chars() {
        result <<= 1;
        if c == '1' {
            result += 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_test() {
        assert_eq!(22, to_number("10110".to_string()));
        assert_eq!(7, to_number("111".to_string()));
    }

    #[test]
    fn full_test() {
        let test_data = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        assert_eq!(23, to_number(find_best_string(&test_data, true, true)));
        assert_eq!(10, to_number(find_best_string(&test_data, false, false)));
    }
}
