use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    println!("The result is {}", solve(read_file("res/input.txt")));
}

fn solve(field: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    let rows = field.len();
    for row in 0..rows {
        let columns = field[row].len();
        for column in 0..columns {
            if row > 0 && field[row - 1][column] <= field[row][column] {
                continue;
            }
            if row < rows - 1 && field[row + 1][column] <= field[row][column] {
                continue;
            }
            if column > 0 && field[row][column - 1] <= field[row][column] {
                continue;
            }
            if column < columns - 1 && field[row][column + 1] <= field[row][column] {
                continue;
            }
            result += field[row][column] + 1;
        }
    }
    return result;
}

fn read_file(path: &str) -> Vec<Vec<u32>> {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut field: Vec<Vec<u32>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        field.push(
            line.chars()
                .map(|x| x.to_digit(10).expect("Error parsing digit"))
                .collect(),
        );
    }

    return field;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_test_input() {
        assert_eq!(15, solve(read_file("res/test_input.txt")));
    }
}
