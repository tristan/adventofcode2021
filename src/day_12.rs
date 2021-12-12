use std::collections::{HashMap, HashSet};

enum CaveSize {
    Large,
    Small
}

struct Cave {
    size: CaveSize,
    links: HashSet<String>,
}

impl Cave {
    fn new(name: &str) -> Cave {
        let size = if name.chars().any(|c| c.is_uppercase()) {
            CaveSize::Large
        } else {
            CaveSize::Small
        };
        Cave {
            size,
            links: HashSet::new()
        }
    }

    fn is_small(&self) -> bool {
        matches!(&self.size, CaveSize::Small)
    }
}

fn parse_input(input: &str) -> HashMap<String, Cave> {
    let mut caves = HashMap::new();
    input.lines().filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut s = line.split('-');
            let start = s.next().unwrap().to_owned();
            let end = s.next().unwrap().to_owned();
            caves.entry(start.clone()).or_insert_with(|| Cave::new(&start))
                .links.insert(end.clone());
            caves.entry(end.clone()).or_insert_with(|| Cave::new(&end))
                .links.insert(start);
        });
    caves
}

fn search(
    input: &HashMap<String, Cave>,
    paths: &mut Vec<Vec<String>>,
    current: String,
    mut path: Vec<String>,
    visited_small_twice: bool,
) {
    path.push(current.clone());
    let cave = input.get(&current).unwrap();
    for next_cave_name in &cave.links {
        if next_cave_name == "end" {
            let mut final_path = path.clone();
            final_path.push("end".to_owned());
            paths.push(final_path);
        } else if next_cave_name == "start" {
            continue;
        } else {
            let next_cave = input.get(next_cave_name).unwrap();
            if next_cave.is_small() && path.contains(next_cave_name) {
                if visited_small_twice {
                    // dead end
                    continue;
                } else {
                    search(input, paths, next_cave_name.clone(), path.clone(), true);
                }
            } else {
                search(input, paths, next_cave_name.clone(), path.clone(), visited_small_twice);
            }
        }
    }
}

fn part1(input: &HashMap<String, Cave>) -> usize {
    let mut paths = Vec::new();
    search(input, &mut paths, "start".to_owned(), vec![], true);
    paths.len()
}

fn part2(input: &HashMap<String, Cave>) -> usize {
    let mut paths = Vec::new();
    search(input, &mut paths, "start".to_owned(), vec![], false);
    paths.len()
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_12_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT_1: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    const TEST_INPUT_2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 10);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 36);
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 103);
    }
}
