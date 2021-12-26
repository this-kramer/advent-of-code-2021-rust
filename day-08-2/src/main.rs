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
    let mut measurements = read_inputs(path);

    let mut total = 0;
    while !measurements.is_empty() {
        let mut current_measurement = measurements.remove(0);
        let symbols = solve_instance(&mut current_measurement.unique_symbols);
        let result: u32 = current_measurement
            .challenges
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| {
                return u32::pow(10, i as u32)
                    * symbols
                        .iter()
                        .enumerate()
                        .filter(|(_, y)| x.len() == y.len() && x.iter().all(|s| y.contains(s))) // Compare vectors
                        .nth(0)
                        .unwrap()
                        .0 as u32;
            })
            .sum::<u32>();
        total += result;
    }

    return total;
}

fn solve_instance(unique_symbols: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut symbols: Vec<Vec<char>> = vec![Vec::new(); 10];

    // Trivial symbols
    let mut i = 0;
    loop {
        match unique_symbols[i].len() {
            2 => symbols[1] = unique_symbols.remove(i),
            3 => symbols[7] = unique_symbols.remove(i),
            4 => symbols[4] = unique_symbols.remove(i),
            7 => symbols[8] = unique_symbols.remove(i),
            _ => i += 1,
        }
        if i >= unique_symbols.len() {
            break;
        }
    }

    // Symbols with five segments
    let three_index = unique_symbols
        .iter()
        .enumerate()
        .filter(|(_, x)| x.len() == 5 && is_subset(&symbols[1], x))
        .nth(0)
        .unwrap()
        .0;
    symbols[3] = unique_symbols.remove(three_index);

    let five_index = unique_symbols
        .iter()
        .enumerate()
        .filter(|(_, x)| x.len() == 5 && is_subset(&setminus(&symbols[4], &symbols[1]), x))
        .nth(0)
        .unwrap()
        .0;
    symbols[5] = unique_symbols.remove(five_index);

    let two_index = unique_symbols
        .iter()
        .enumerate()
        .filter(|(_, x)| x.len() == 5)
        .nth(0)
        .unwrap()
        .0;
    symbols[2] = unique_symbols.remove(two_index);

    // Symbols with six segments
    let six_index = unique_symbols
        .iter()
        .enumerate()
        .filter(|(_, x)| x.len() == 6 && !is_subset(&symbols[1], x))
        .nth(0)
        .unwrap()
        .0;
    symbols[6] = unique_symbols.remove(six_index);

    let nine_index = unique_symbols
        .iter()
        .enumerate()
        .filter(|(_, x)| x.len() == 6 && is_subset(&symbols[3], x))
        .nth(0)
        .unwrap()
        .0;
    symbols[9] = unique_symbols.remove(nine_index);
    symbols[0] = unique_symbols.remove(0);

    // Sanity check
    assert_eq!(0, unique_symbols.len());
    return symbols;
}

fn setminus(left: &Vec<char>, right: &Vec<char>) -> Vec<char> {
    return left
        .iter()
        .filter(|x| !right.contains(x))
        .cloned()
        .collect();
}

fn is_subset(left: &Vec<char>, right: &Vec<char>) -> bool {
    return left.iter().all(|x| right.contains(x));
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
    fn simple_test_input() {
        assert_eq!(5353, solve("res/simple_test_input.txt"));
    }

    #[test]
    fn solve_with_test_data() {
        assert_eq!(61229, solve("res/test_input.txt"));
    }

    #[test]
    fn subset_test() {
        assert_eq!(
            true,
            is_subset(&vec!['b', 'd'], &vec!['a', 'b', 'c', 'd', 'e'])
        );
        assert_eq!(
            false,
            is_subset(&vec!['b', 'd', 'f'], &vec!['a', 'b', 'c', 'd', 'e'])
        );
        assert_eq!(true, is_subset(&Vec::<char>::new(), &vec!['a']));
        assert_eq!(true, is_subset(&Vec::<char>::new(), &Vec::<char>::new()));
    }

    #[test]
    fn setminus_test() {
        assert_eq!(
            vec!['a', 'c'],
            setminus(&vec!['a', 'b', 'c', 'd'], &vec!['b', 'd', 'e'])
        );
        assert_eq!(Vec::<char>::new(), setminus(&vec![], &vec!['b', 'd', 'e']));
    }

    #[test]
    fn integrate_set_math() {
        assert_eq!(
            true,
            is_subset(
                &setminus(&vec!['e', 'a', 'f', 'b'], &vec!['a', 'b']),
                &vec!['c', 'd', 'f', 'b', 'e']
            )
        );
    }
}
