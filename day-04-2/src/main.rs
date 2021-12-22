use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

const N: usize = 5;

struct Field(u32, bool);

fn main() {
    let result = giant_squid("res/input.txt");
    println!("The result is {}", result);
}

fn giant_squid(path: &str) -> u32 {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse numbers
    let numbers: Vec<u32> = line_to_vec(lines.next().unwrap().unwrap());

    let mut boards: Vec<Vec<Field>> = Vec::new();

    while lines.next().is_some() {
        let mut board: Vec<Field> = Vec::new();
        for _ in 0..N {
            board.append(&mut line_to_field_vec(lines.next().unwrap().unwrap()));
        }
        boards.push(board);
    }

    return compute_winning_board(&mut boards, numbers);
}

fn line_to_field_vec(line: String) -> Vec<Field> {
    return line
        .split(' ')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| Field(x.parse().unwrap(), false))
        .collect();
}

fn line_to_vec(line: String) -> Vec<u32> {
    return line
        .split(',')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
}

fn compute_winning_board(boards: &mut Vec<Vec<Field>>, drawn_numbers: Vec<u32>) -> u32 {
    // Need these indices instead of iterators to mutate values
    for number in drawn_numbers {
        for i in 0..boards.len() {
            for j in 0..boards[i].len() {
                match boards[i][j] {
                    Field(z, false) if z == number => boards[i][j] = Field(number, true),
                    _ => {}
                }
            }
        }

        // Carefully remove all won boards
        let mut finished = false;
        'outer: while !finished {
            for i in 0..boards.len() {
                if is_winning(&boards[i]) {
                    if boards.len() == 1 {
                        let mut score = 0;
                        for field in &boards[0] {
                            score += if !field.1 { field.0 } else { 0 };
                        }
                        return number * score;
                    }
                    boards.remove(i);
                    continue 'outer;
                }
            }
            finished = true;
        }
    }
    panic!("No solution found!");
}

fn is_winning(board: &Vec<Field>) -> bool {
    for row in 0..N {
        let mut row_winning = true;
        for column in 0..N {
            row_winning &= board[N * row + column].1;
        }
        if row_winning {
            return true;
        };
    }
    for column in 0..N {
        let mut column_winning = true;
        for row in 0..N {
            column_winning &= board[N * row + column].1;
        }
        if column_winning {
            return true;
        };
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_test_data() {
        assert_eq!(1924, giant_squid("res/test_input.txt"))
    }
}
