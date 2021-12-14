use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let mut main = input.split("\n\n");
    (
        main.next().unwrap()
            .chars()
            .collect(),
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

fn part1(input: &(Vec<char>, HashMap<[char; 2], char>)) -> usize {
    let (orig, inputs) = input;
    let mut template = orig.clone();
    let last = *orig.last().unwrap();
    for _ in 0..10 {
        template = template.windows(2).fold(Vec::new(), |mut acc, chunk| {
            acc.push(chunk[0]);
            if let Some(mid) = inputs.get(chunk) {
                acc.push(*mid);
            }
            acc
        });
        template.push(last);
    }
    let mut counts = HashMap::new();
    for c in template {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    let v = counts.into_values().collect::<Vec<_>>();
    v.iter().max().unwrap() - v.iter().min().unwrap()
}


fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_14_input.txt"));
        println!("part1: {}", part1(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1};

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
}
