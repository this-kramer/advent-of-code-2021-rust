use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    simulate_folds("res/input.txt")
}

fn simulate_folds(input_path: &str) {
    let (points, folds) = read_field(input_path);

    let mut points = points.clone();

    for fold in folds {
        for i in 0..points.len() {
            points[i] = points[i].flip(&fold);
        }
    }

    let mut max_x = 0;
    let mut max_y = 0;

    for point in &points {
        max_x = max(point.0 as usize, max_x);
        max_y = max(point.1 as usize, max_y);
    }

    let mut field = vec![vec![Cell::Empty; max_x + 1]; max_y + 1];
    for point in &points {
        field[point.1 as usize][point.0 as usize] = Cell::Point;
    }

    for row in field {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Cell::Empty => ' ',
                    Cell::Point => '#',
                }
            )
        }
        println!()
    }
}

#[derive(Clone)]
enum Cell {
    Empty,
    Point,
}

#[derive(PartialEq, Debug)]
enum Fold {
    Left(i32),
    Up(i32),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Point(i32, i32);

impl Point {
    fn flip(&self, flip: &Fold) -> Point {
        match flip {
            Fold::Left(x) => Point(if self.0 > *x { 2 * x - self.0 } else { self.0 }, self.1),
            Fold::Up(y) => Point(self.0, if self.1 > *y { 2 * y - self.1 } else { self.1 }),
        }
    }
}

impl From<String> for Point {
    fn from(string: String) -> Self {
        let coords: Vec<i32> = string
            .split(',')
            .map(|x| x.parse::<i32>().expect("Error parsing input as number"))
            .collect();
        Point(coords[0], coords[1])
    }
}

impl From<String> for Fold {
    fn from(string: String) -> Self {
        match (&string[11..12], &string[13..]) {
            ("x", number) => Fold::Left(number.parse().expect("Error parsing fold number")),
            ("y", number) => Fold::Up(number.parse().expect("Error parsing fold number")),
            (x, y) => panic!(
                "Cannot build fold from input: {} parsed as '{}', '{}'",
                string, x, y
            ),
        }
    }
}

fn read_field(path: &str) -> (Vec<Point>, Vec<Fold>) {
    let file = File::open(path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut points = Vec::new();
    let mut folds = Vec::new();
    for line in lines {
        match line {
            Ok(content) if content.starts_with("fold along") => folds.push(Fold::from(content)),
            Ok(content) if content.contains(',') => points.push(Point::from(content)),
            _ => continue,
        }
    }

    (points, folds)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_count_folds() {
        simulate_folds("res/test-input.txt")
    }

    #[rstest]
    #[case(Point(1, 2), Point(1, 4), Fold::Up(3))]
    #[case(Point(1, 2), Point(1, 2), Fold::Up(3))]
    #[case(Point(2, 1), Point(4, 1), Fold::Left(3))]
    #[case(Point(2, 1), Point(2, 1), Fold::Left(3))]
    fn test_fold(#[case] expected: Point, #[case] start: Point, #[case] flip: Fold) {
        assert_eq!(expected, start.flip(&flip))
    }

    #[rstest]
    #[case(Fold::Up(5), String::from("fold along y=5"))]
    #[case(Fold::Up(5), String::from("fold along y=5"))]
    fn test_fold_from_string(#[case] expected: Fold, #[case] input_string: String) {
        assert_eq!(expected, Fold::from(input_string))
    }

    #[rstest]
    #[case(Point(1, 4), String::from("1,4"))]
    #[case(Point(113, 432), String::from("113,432"))]
    fn test_point_from_string(#[case] expected: Point, #[case] input_string: String) {
        assert_eq!(expected, Point::from(input_string))
    }
}
