use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("res/input.txt");
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut data: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let mut column: Vec<u32> = Vec::new();
        let line: String = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };
        for bit in line.chars() {
            match bit {
                '0' => column.push(0),
                '1' => column.push(1),
                _ => panic!("Illegal character in file!"),
            }
        }
        data.push(column);
    }
    let result = compute_result(data);
    println!("The result is {}", result);
}

fn compute_result(input: Vec<Vec<u32>>) -> u32 {
    let total = input.len();
    assert_ne!(0, total);
    let size = input[0].len();

    let mut gamma_count: Vec<usize> = vec![0; size];
    for row in input {
        for (i, &column) in row.iter().rev().enumerate() {
            gamma_count[i] += column as usize;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..size {
        if gamma_count[i] > total / 2 {
            println!("Position {} is has more 1s", i);
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }
    return gamma * epsilon;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        assert_eq!(
            1,
            compute_result(vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 1]])
        );
    }
}
