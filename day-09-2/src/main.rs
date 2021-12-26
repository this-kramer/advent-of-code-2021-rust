use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    println!("The result is {}", solve(&read_file("res/input.txt")));
}

struct Point {
    height: u32,
    visited: bool,
}

fn solve(field: &Vec<Vec<u32>>) -> u32 {
    let low_points = find_low_points(field);
    let rows = field.len();
    let columns = field[0].len();
    let mut sizes = Vec::new();

    for low_point in low_points {
        let basin_count = count_basin_points(
            &mut to_field_with_bool(&field),
            low_point.0,
            low_point.1,
            rows,
            columns,
        );
        sizes.push(basin_count);
    }
    sizes.sort();
    return sizes.iter().rev().take(3).product();
}

fn to_field_with_bool(field: &Vec<Vec<u32>>) -> Vec<Vec<Point>> {
    let mut new_field = Vec::new();
    for row in field {
        let mut new_row = Vec::new();
        for cell in row {
            new_row.push(Point {
                height: *cell,
                visited: false,
            });
        }
        new_field.push(new_row);
    }
    return new_field;
}

fn count_basin_points(
    field: &mut Vec<Vec<Point>>,
    row: usize,
    column: usize,
    rows: usize,
    columns: usize,
) -> u32 {
    if field[row][column].visited || field[row][column].height == 9 {
        return 0;
    }
    field[row][column].visited = true;

    let mut count = 1;
    if row > 0 && field[row - 1][column].height > field[row][column].height {
        count += count_basin_points(field, row - 1, column, rows, columns);
    }
    if row < rows - 1 && field[row + 1][column].height > field[row][column].height {
        count += count_basin_points(field, row + 1, column, rows, columns);
    }
    if column > 0 && field[row][column - 1].height > field[row][column].height {
        count += count_basin_points(field, row, column - 1, rows, columns);
    }
    if column < columns - 1 && field[row][column + 1].height > field[row][column].height {
        count += count_basin_points(field, row, column + 1, rows, columns);
    }
    return count;
}

fn find_low_points(field: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
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
            low_points.push((row, column));
        }
    }
    return low_points;
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
    fn test_low_points() {
        let all_low_points = vec![(0, 1), (0, 9), (2, 2), (4, 6)];
        let computed_low_points = find_low_points(&read_file("res/test_input.txt"));
        assert_eq!(all_low_points.len(), computed_low_points.len());
        assert_eq!(
            true,
            all_low_points
                .iter()
                .all(|x| computed_low_points.contains(x))
        );
    }

    #[test]
    fn test_with_test_data() {
        assert_eq!(1134, solve(&read_file("res/test_input.txt")));
    }
}
