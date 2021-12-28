use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!(
        "The best path has risk {}",
        compute_cheapest_path("res/input.txt").unwrap()
    );
}

fn compute_cheapest_path(path: &str) -> Option<u32> {
    let mut field = read_input(path);
    let target_position = (field.len() - 1, field[0].len() - 1);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let start_state = State {
        cost: 0, // Best cost to direct neighbor
        x: 0,
        y: 0,
    };

    heap.push(start_state);
    while let Some(frontier) = heap.pop() {
        // Replace by heapify-up or similar?
        let mut node = &mut field[frontier.y][frontier.x];

        // There was a better route, note already finished.
        if node.finished {
            continue;
        } else {
            node.finished = true;
        }

        // Found target position
        let cost = match (frontier.y, frontier.x) {
            (0, 0) => 0,
            (x, y) if (x, y) == target_position => return Some(frontier.cost + node.risk),
            _ => frontier.cost + node.risk,
        };

        // Add non-finished neighbors to heap
        if frontier.x > 0 {
            heap.push(State {
                cost: cost,
                x: frontier.x - 1,
                ..frontier
            })
        }
        if frontier.x < target_position.1 {
            heap.push(State {
                cost: cost,
                x: frontier.x + 1,
                ..frontier
            })
        }
        if frontier.y > 0 {
            heap.push(State {
                cost: cost,
                y: frontier.y - 1,
                ..frontier
            })
        }
        if frontier.y < target_position.0 {
            heap.push(State {
                cost: cost,
                y: frontier.y + 1,
                ..frontier
            })
        }
    }
    None
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    cost: u32,
    x: usize,
    y: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.y.cmp(&self.y))
            .then_with(|| other.x.cmp(&self.x))
    }
}

struct Node {
    risk: u32,
    finished: bool,
}

fn read_input(path: &str) -> Vec<Vec<Node>> {
    let file = File::open(path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut field = Vec::new();
    for line in lines {
        field.push(
            line.expect("Error parsing line")
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .map(|x| Node {
                    risk: x,
                    finished: false,
                })
                .collect(),
        );
    }
    field
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let field = read_input("res/test-input.txt");
    }

    #[test]
    fn test_dijkstra() {
        assert_eq!(Some(40), compute_cheapest_path("res/test-input.txt"));
    }
}
