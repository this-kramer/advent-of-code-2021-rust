use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn main() {
    let result = solve("res/input.txt");
    println!("The result is {}", result);
}

fn solve(path: &str) -> u32 {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut vents: Vec<Vent> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        vents.push(line_to_vent(line));
    }

    return count_blocked_fields(vents);
}

fn line_to_vent(line: String) -> Vent {
    let a: Vec<usize> = line
        .split("->")
        .flat_map(|x| x.trim().split(','))
        .map(|x| x.parse().expect("Error parsing coordinate as number"))
        .collect();
    return Vent {
        x1: a[0],
        y1: a[1],
        x2: a[2],
        y2: a[3],
    };
}

fn count_blocked_fields(vents: Vec<Vent>) -> u32 {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for vent in &vents {
        max_x = cmp::max(max_x, cmp::max(vent.x1, vent.x2) as usize);
        max_y = cmp::max(max_y, cmp::max(vent.y1, vent.y2) as usize);
    }
    let width: usize = max_x + 1;
    let height: usize = max_y + 1;

    let mut field: Vec<u32> = vec![0; width * height];
    for vent in vents {
        if vent.y1 == vent.y2 {
            for x in cmp::min(vent.x1, vent.x2)..=cmp::max(vent.x1, vent.x2) {
                field[width * vent.y1 + x] += 1;
            }
        }
        if vent.x1 == vent.x2 {
            for y in cmp::min(vent.y1, vent.y2)..=cmp::max(vent.y1, vent.y2) {
                field[width * y + vent.x1] += 1;
            }
        }
    }

    let mut blocked = 0;
    for f in field {
        if f > 1 {
            blocked += 1;
        }
    }
    return blocked;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_test_data() {
        assert_eq!(5, solve("res/test_input.txt"))
    }
}
