use std::clone::Clone;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

struct Measurement {
    unique_symbols: Vec<Vec<char>>,
    challenges: Vec<Vec<char>>,
}

fn main() {
    let result = solve("res/input.txt");
    println!("The result is {}", result);
}

fn solve(path: &str) -> u32 {
    let measurements = read_inputs(path);

    let mut count = 0;
    for measurement in measurements {
        count += solve_instance(measurement);
    }

    return count;
}

fn solve_instance(measurement: Measurement) -> u32 {
    for m in &measurement.challenges {
        println!("{}", m.iter().collect::<String>());
    }
    return measurement
        .challenges
        .iter()
        .filter(|s| !(5..=6).contains(&s.len()))
        .count() as u32;
}

fn read_inputs(path: &str) -> Vec<Measurement> {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut measurements = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let split_line = line.split(" | ").collect::<Vec<&str>>();
        let measurement = Measurement {
            unique_symbols: split_line[0]
                .split(" ")
                .map(|x| x.chars().collect())
                .collect(),
            challenges: split_line[1]
                .split(" ")
                .map(|x| x.chars().collect())
                .collect(),
        };
        measurements.push(measurement);
    }

    return measurements;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_test_data() {
        assert_eq!(26, solve("res/test_input.txt"))
    }
}
