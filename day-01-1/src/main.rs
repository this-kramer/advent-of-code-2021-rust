use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let mut numbers: Vec<u32> = Vec::new();

    let path = Path::new("res/input.txt");
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let parsed_number: u32 = match line.expect("Error reading line").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        numbers.push(parsed_number);
    }

    let count = count_larger_measurements(numbers);
    println!(
        "There are {} numbers that are larger than their predecessor.",
        count
    );
}

// Function that given an array of integers counts how many numbers are strictly larger than their direct predecessor
fn count_larger_measurements(numbers: Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            count = count + 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_larger_measurements() {
        assert_eq!(count_larger_measurements(vec![1, 2, 1, 3]), 2);
    }

    #[test]
    fn test_empty_vec() {
        assert_eq!(count_larger_measurements(vec![]), 0);
    }
}
