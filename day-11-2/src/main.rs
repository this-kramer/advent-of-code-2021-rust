use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    println!("The result is {}", compute_synchronized_round("res/input.txt"));
}

struct Octupus {
    energy: u32,
    flashed: bool, // Did this octupus flash this round?
}

fn compute_synchronized_round(path: &str) -> u32 {
    let mut field = parse_field_from_file(path);
    let rows = field.len();
    let columns = field[0].len();

    let mut round = 0;
    loop {
        round += 1;
        for row in 0..rows {
            for column in 0..columns {
                field[row][column].energy += 1;
            }
        }
        loop {
            let mut flashed = false;
            for row in 0..rows {
                for column in 0..columns {
                    if field[row][column].energy > 9 && field[row][column].flashed == false {
                        field[row][column].flashed = true;
                        flashed = true;
                        if row > 0 && column > 0 {
                            field[row - 1][column - 1].energy += 1;
                        }
                        if row > 0 {
                            field[row - 1][column].energy += 1;
                        }
                        if row > 0 && column < columns - 1 {
                            field[row - 1][column + 1].energy += 1;
                        }
                        if column > 0 {
                            field[row][column - 1].energy += 1;
                        }
                        if column < columns - 1 {
                            field[row][column + 1].energy += 1;
                        }
                        if row < rows - 1 && column > 0 {
                            field[row + 1][column - 1].energy += 1;
                        }
                        if row < rows - 1 {
                            field[row + 1][column].energy += 1;
                        }
                        if row < rows - 1 && column < columns - 1 {
                            field[row + 1][column + 1].energy += 1;
                        }
                    }
                }
            }
            if !flashed {
                break;
            }
        }
        // Collect flashes and reset
        let mut all_flashed = true;
        for row in 0..rows {
            for column in 0..columns {
                if field[row][column].flashed {
                    field[row][column] = Octupus {
                        energy: 0,
                        flashed: false,
                    };
                } else {
                    all_flashed = false;
                }
            }
        }
        if all_flashed {
            return round;
        }
    }
}

fn parse_field_from_file(path: &str) -> Vec<Vec<Octupus>> {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut field = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for number in line.unwrap().chars() {
            row.push(Octupus {
                energy: number.to_digit(10).expect("Error parsing number"),
                flashed: false,
            });
        }
        field.push(row);
    }
    return field;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(195, compute_synchronized_round("res/test-input.txt"));
    }
}
