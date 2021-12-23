use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let result = solve("res/input.txt");
    println!("The result is {}", result);
}

fn solve(path: &str) -> u32 {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut colony: Vec<u32> = vec![0; 9];

    if let Some(Ok(line)) = lines.next() {
        line.split(",")
            .map(|x| x.parse().expect("Error parsing inputs as number"))
            .for_each(|x: usize| colony[x] += 1);
    };

    let mut new_colony: Vec<u32> = vec![0; 9];
    for _ in 0..80 {
        new_colony = vec![0; 9];
        for (i, &count) in colony.iter().enumerate() {
            match i {
                0 => {
                    new_colony[8] += count;
                    new_colony[6] += count;
                }
                _ => new_colony[i - 1] += count,
            }
        }
        colony = new_colony;
    }

    return colony.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_test_data() {
        assert_eq!(5934, solve("res/test_input.txt"))
    }
}
