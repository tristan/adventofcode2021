use std::collections::HashMap;
use std::cmp::Ordering;

use adventofcode2021::time;

fn main() {
    let input = parse_input(include_str!("../day_05_input.txt"));
    println!("part1: {}", time!(part1(&input)));
    println!("part2: {}", time!(part2(&input)));
}

fn parse_input(input: &str) -> Vec<(u64, u64, u64, u64)> {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line
                .split(" -> ")
                .map(|vert| vert
                     .split(',')
                     .map(|part| part.parse::<u64>().unwrap())
                )
                .flatten();
            let x1 = parts.next().unwrap();
            let y1 = parts.next().unwrap();
            let x2 = parts.next().unwrap();
            let y2 = parts.next().unwrap();
            (x1, y1, x2, y2)
        })
        .collect()
}

fn part1(input: &[(u64, u64, u64, u64)]) -> usize {
    let map = input.iter().fold(
        HashMap::new(),
        |mut map, &(x1, y1, x2, y2)| {
            if x1 == x2 || y1 == y2 {
                let (x1, x2) = (x1.min(x2), x1.max(x2));
                let (y1, y2) = (y1.min(y2), y1.max(y2));
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        let current = map.entry((x, y))
                            .or_insert(0);
                        *current += 1;
                    }
                }
            }
            map
        });
    map.into_values()
        .filter(|&v| v > 1)
        .count()
}

fn part2(input: &[(u64, u64, u64, u64)]) -> usize {
    let map = input.iter().fold(
        HashMap::new(),
        |mut map, &(x1, y1, x2, y2)| {
            let xiter: Box<dyn Iterator<Item=u64>> = match x1.cmp(&x2) {
                Ordering::Equal => Box::new(std::iter::repeat(x1)),
                Ordering::Greater => Box::new((x2..=x1).rev()),
                Ordering::Less => Box::new(x1..=x2)
            };
            let yiter: Box<dyn Iterator<Item=u64>> = match y1.cmp(&y2) {
                Ordering::Equal => Box::new(std::iter::repeat(y1)),
                Ordering::Greater => Box::new((y2..=y1).rev()),
                Ordering::Less => Box::new(y1..=y2)
            };
            for (x, y) in xiter.zip(yiter) {
                let current = map.entry((x, y))
                    .or_insert(0);
                *current += 1;
            }
            map
        });
    map.into_values()
        .filter(|&v| v > 1)
        .count()
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    fn get_test_input() -> Vec<(u64, u64, u64, u64)> {
        parse_input(TEST_INPUT)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 12);
    }
}
