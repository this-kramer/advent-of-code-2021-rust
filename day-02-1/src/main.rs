use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Move(Direction, u32);

fn main() {
    let path = Path::new("res/input.txt");
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut moves: Vec<Move> = Vec::new();

    for line in lines {
        let line: String = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };

        let words_in_line: Vec<&str> = line.split(" ").collect();

        assert_eq!(words_in_line.len(), 2);

        let next_move = Move(
            match words_in_line[0] {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                &_ => continue,
            },
            words_in_line[1]
                .parse()
                .expect("Error parsing steps as a number!"),
        );
        moves.push(next_move);
    }

    let (x, y) = simulate_moves(moves);
    println!("The result is x: {}, y: {}, product: {}", x, y, x * y);
}

fn simulate_moves(moves: Vec<Move>) -> (u32, u32) {
    let mut x = 0;
    let mut y = 0;
    for m in moves {
        match m {
            Move(Direction::Down, s) => y += s,
            Move(Direction::Up, s) => y -= s,
            Move(Direction::Forward, s) => x += s,
        }
    }
    return (x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_larger_measurements() {
        assert_eq!(
            (7, 3),
            simulate_moves(vec![
                Move(Direction::Down, 5),
                Move(Direction::Forward, 7),
                Move(Direction::Up, 2)
            ])
        )
    }
}
