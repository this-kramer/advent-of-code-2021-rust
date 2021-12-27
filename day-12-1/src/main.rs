use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone)]
struct Cave {
    title: String,
    neighbors: Vec<String>,
}

fn main() {
    println!("There are {} paths!", count_paths(read_cave_from_file("res/input.txt")));
}

fn count_paths(caves: Vec<Cave>) -> u32 {
    return count_paths_rec("start".to_string(), &Vec::new(), &caves);
}

fn count_paths_rec(current_cave_title: String, current_path: &Vec<&str>, caves: &Vec<Cave>) -> u32 {
    // This cave is already in path and small
    if current_path.iter().any(|&c| c == current_cave_title)
        && !current_cave_title.chars().all(|x| x.is_uppercase())
    {
        return 0;
    }
    // Found the exit, end recursion and add one found path to count
    if current_cave_title == "end" {
        return 1;
    }

    let current_cave = caves
        .iter()
        .filter(|x| x.title == current_cave_title)
        .nth(0)
        .unwrap();

    let mut new_path = current_path.clone();
    new_path.push(&current_cave_title);

    let mut count = 0;
    for neighbor in &current_cave.neighbors {
        count += count_paths_rec(neighbor.clone(), &new_path, caves);
    }
    return count;
}

fn read_cave_from_file(path: &str) -> Vec<Cave> {
    let path = Path::new(path);
    let file = File::open(&path).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut caves: Vec<Cave> = Vec::new();
    let mut edges = Vec::new();
    for line in lines {
        edges.push(
            line.unwrap()
                .split("-")
                .take(2)
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    for edge in edges.iter() {
        match caves.iter_mut().find(|c| c.title == edge[0]) {
            Some(cave) => cave.neighbors.push(edge[1].to_string()),
            None => caves.push(Cave {
                title: edge[0].to_string(),
                neighbors: vec![edge[1].to_string()],
            }),
        }
        match caves.iter_mut().find(|c| c.title == edge[1]) {
            Some(cave) => cave.neighbors.push(edge[0].to_string()),
            None => caves.push(Cave {
                title: edge[1].to_string(),
                neighbors: vec![edge[0].to_string()],
            }),
        }
    }
    return caves;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            10,
            count_paths(read_cave_from_file("res/small-test-input.txt"))
        );
    }

    #[test]
    fn medium_test() {
        assert_eq!(
            19,
            count_paths(read_cave_from_file("res/medium-test-input.txt"))
        );
    }
    #[test]
    fn large_test() {
        assert_eq!(
            226,
            count_paths(read_cave_from_file("res/large-test-input.txt"))
        );
    }
}
