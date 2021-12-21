use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::BufRead;

fn main() {
    let mut numbers: Vec<u32> = Vec::new();

    let path = Path::new("res/input.txt");
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines{
        let parsed_number: u32 = match line.expect("Error reading line").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        numbers.push(parsed_number);
    }

    let count = count_larger_sliding_window(numbers);
    println!("There are {} sliding windows that are larger than their predecessor.", count);

}

// Function that given an array of integers counts how many 3-element sliding windows numbers are strictly larger than the previous window
fn count_larger_sliding_window(numbers: Vec<u32>) -> u32 {
    if numbers.len() <= 3 { 
        return 0; 
    }

    let mut count: u32 = 0;
    let mut window = numbers[0] + numbers[1] + numbers[2];

    for i in 3..numbers.len() {
        let new_window = window - numbers[i-3] + numbers[i];
        if new_window > window {
            count = count + 1;
        }
        window = new_window;
    }
    return count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_larger_measurements() {
        assert_eq!(count_larger_sliding_window(vec![1,2,1,3]), 1);
    }

    #[test]
    fn test_small_vec() {
        assert_eq!(count_larger_sliding_window(vec![]), 0);
        assert_eq!(count_larger_sliding_window(vec![1]), 0);
        assert_eq!(count_larger_sliding_window(vec![1,2]), 0);
        assert_eq!(count_larger_sliding_window(vec![1,2,3]), 0);
    }
}