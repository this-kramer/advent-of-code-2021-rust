use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::BufRead;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Move(Direction, i32);

fn main() {
    let path = Path::new("res/input.txt");
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut moves: Vec<Move> = Vec::new();

    for line in lines{
        let line : String = match line {
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
            words_in_line[1].parse().expect("Error parsing steps as a number!"),
        );
        moves.push(next_move);
    }

    let (x, y) = simulate_moves(moves);
    println!("The result is x: {}, y: {}, product: {}", x, y, x*y);
}

fn simulate_moves(moves: Vec<Move>) -> (i32, i32) {
    let mut aim: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for m in moves {
        match m {
            Move(Direction::Down, s) => aim += s,
            Move(Direction::Up, s) => aim -= s,
            Move(Direction::Forward, s) => {
                x += s;
                y += s * aim;
            },
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
            (10, 5),
            simulate_moves(vec![
                Move(Direction::Down, 2),
                Move(Direction::Forward, 7), 
                Move(Direction::Up, 5),
                Move(Direction::Forward, 3), 
            ])
        )
    }
}