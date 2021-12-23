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

    let mut submarines: Vec<u32> = Vec::new();
    if let Some(Ok(line)) = lines.next() {
        line.split(",")
            .map(|x| x.parse().expect("Error parsing inputs as number"))
            .for_each(|x: u32| submarines.push(x));
    };

    let total = submarines.len();
    let mut best_pos: u32 = 0;
    let mut best_score: u32 = u32::MAX;
    for i in 0..total {
        let score = submarines
            .iter()
            .map(|&x| {
                if x > i as u32 {
                    x - i as u32
                } else {
                    i as u32 - x
                }
            })
            .sum::<u32>();
        if score < best_score {
            best_score = score;
            best_pos = i as u32;
        }
    }
    println!("Best position: {} with score {}", best_pos, best_score);

    return best_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_test_data() {
        assert_eq!(37, solve("res/test_input.txt"))
    }
}
