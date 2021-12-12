use std::collections::{HashMap, HashSet};

struct Cave<'a> {
    is_small: bool,
    links: HashSet<&'a str>,
}

impl<'a> Cave<'a> {
    fn new(name: &str) -> Cave {
        Cave {
            is_small: name.chars().any(|c| c.is_ascii_lowercase()),
            links: HashSet::new()
        }
    }
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Cave> {
    let mut caves = HashMap::new();
    input.lines().filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut s = line.split('-');
            let start = s.next().unwrap();
            let end = s.next().unwrap();
            caves.entry(start).or_insert_with(|| Cave::new(start))
                .links.insert(end);
            caves.entry(end).or_insert_with(|| Cave::new(end))
                .links.insert(start);
        });
    caves
}

fn search<'a>(
    input: &'a HashMap<&'a str, Cave>,
    current: &'a str,
    mut path: HashSet<&'a str>,
    visited_small_twice: bool,
) -> usize {
    path.insert(current);
    let cave = input.get(current).unwrap();
    let mut paths = 0;
    for next_cave_name in &cave.links {
        if next_cave_name == &"end" {
            paths += 1;
        } else if next_cave_name == &"start" {
            continue;
        } else {
            let next_cave = input.get(next_cave_name).unwrap();
            if next_cave.is_small && path.contains(next_cave_name) {
                if visited_small_twice {
                    // dead end
                    continue;
                } else {
                    paths += search(input, next_cave_name, path.clone(), true);
                }
            } else {
                paths += search(input, next_cave_name, path.clone(), visited_small_twice);
            }
        }
    }
    paths
}

fn part1(input: &HashMap<&str, Cave>) -> usize {
    search(input, "start", HashSet::new(), true)
}

fn part2(input: &HashMap<&str, Cave>) -> usize {
    search(input, "start", HashSet::new(), false)
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
