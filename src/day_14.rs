use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let mut main = input.split("\n\n");
    (
        main.next().unwrap()
            .chars()
            .collect::<Vec<_>>(),
        main.next().unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let s = [line.chars().next().unwrap(), line.chars().nth(1).unwrap()];
                let c = line.chars().nth(6).unwrap();
                (s, c)
            })
            .collect()
    )
}

fn solve(
    input: &[char],
    mappings: &HashMap<[char; 2], char>,
    steps: usize,
) -> usize {
    let mut pairs = input.windows(2)
        .fold(HashMap::new(), |mut map, w| {
            *map.entry([w[0], w[1]]).or_insert(0) += 1;
            map
        });
    for _ in 0..steps {
        pairs = pairs.into_iter().fold(HashMap::new(), |mut map, (pair, count)| {
            let extra = *mappings.get(&pair).unwrap();
            let c = map.entry([pair[0], extra]).or_insert(0);
            *c += count;
            let c = map.entry([extra, pair[1]]).or_insert(0);
            *c += count;
            map
        });
    }
    let mut counts = pairs.into_iter().fold(HashMap::new(), |mut map, (pair, count)| {
        *map.entry(pair[0]).or_insert(0) += count;
        map
    });

    *counts.entry(*input.iter().last().unwrap()).or_insert(0) += 1;
    counts.values().max().unwrap() - counts.values().min().unwrap()
}


fn part1((input, mappings): &(Vec<char>, HashMap<[char; 2], char>)) -> usize {
    solve(input, mappings, 10)
}

fn part2((input, mappings): &(Vec<char>, HashMap<[char; 2], char>)) -> usize {
    solve(input, mappings, 40)
}


fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_14_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2188189693529);
    }
}
