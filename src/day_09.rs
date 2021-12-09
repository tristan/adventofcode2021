use std::collections::HashSet;

fn parse_input(input: &str) -> (usize, usize, Vec<u32>) {
    let w = input.chars().position(|c| matches!(c, '\n')).unwrap();
    let i = input.chars().filter_map(|c| match c {
        '0'..='9' => Some(c as u32 - 48),
        _ => None
    }).collect::<Vec<u32>>();
    let h = i.len() / w;
    (w, h, i)
}

fn find_low_points(w: usize, h: usize, input: &[u32]) -> impl Iterator<Item=usize> + '_ {
    (0..(w * h)).filter(move |&i| {
        if i == 0 { // top left
            if input[i] < input[i + 1] && input[i] < input[i + w] {
                return true;
            }
        } else if i == w - 1 { // top right
            if input[i] < input[i - 1] && input[i] < input[i + w] {
                return true;
            }
        } else if i == w * h - 1 { // bottom right
            if input[i] < input[i - 1] && input[i] < input[i - w] {
                return true;
            }
        } else if i == w * (h - 1) { // bottom left
            if input[i] < input[i + 1] && input[i] < input[i - w] {
                return true;
            }
        } else if i < w { // top row
            if input[i] < input[i - 1] && input[i] < input[i + 1] && input[i] < input[i + w] {
                return true;
            }
        } else if i > w * (h - 1) { // bottom row
            if input[i] < input[i - 1] && input[i] < input[i + 1] && input[i] < input[i - w] {
                return true;
            }
        } else if i % w == 0 { // left row
            if input[i] < input[i + 1] && input[i] < input[i + w] && input[i] < input[i - w] {
                return true;
            }
        } else if i % w == (w - 1) { // right row
            if input[i] < input[i - 1] && input[i] < input[i + w] && input[i] < input[i - w] {
                return true;
            }
        } else { // others
            if input[i] < input[i - 1] && input[i] < input[i + 1] && input[i] < input[i + w] && input[i] < input[i - w] {
                return true;
            }
        }
        false
    })
}

fn climb_basin(w: usize, h: usize, input: &[u32], pos: usize) -> usize {
    let mut queue = vec![pos];
    let mut cache: HashSet<usize> = HashSet::new();
    while let Some(i) = queue.pop() {
        if cache.contains(&i) {
            continue;
        }
        cache.insert(i);
        if i == 0 { // top left
            if input[i] < input[i + 1] && input[i + 1] != 9 {
                queue.push(i + 1);
            }
            if input[i] < input[i + w] && input[i + w] != 9 {
                queue.push(i + w);
            }
        } else if i == w - 1 { // top right
            if input[i] < input[i - 1] && input[i - 1] != 9 {
                queue.push(i - 1);
            }
            if input[i] < input[i + w] && input[i - 1] != 9 {
                queue.push(i + w);
            }
        } else if i == w * h - 1 { // bottom right
            if input[i] < input[i - 1] && input[i - 1] != 9 {
                queue.push(i - 1);
            }
            if input[i] < input[i - w] && input[i - w] != 9 {
                queue.push(i - w);
            }
        } else if i == w * (h - 1) { // bottom left
            if input[i] < input[i + 1] && input[i + 1] != 9 {
                queue.push(i + 1);
            }
            if input[i] < input[i - w] && input[i - w] != 9 {
                queue.push(i - w);
            }
        } else if i < w { // top row
            if input[i] < input[i - 1] && input[i - 1] != 9 {
                queue.push(i - 1);
            }
            if input[i] < input[i + 1] && input[i + 1] != 9 {
                queue.push(i + 1);
            }
            if input[i] < input[i + w] && input[i + w] != 9 {
                queue.push(i + w);
            }
        } else if i > w * (h - 1) { // bottom row
            if input[i] < input[i - 1] && input[i - 1] != 9 {
                queue.push(i - 1);
            }
            if input[i] < input[i + 1] && input[i + 1] != 9 {
                queue.push(i + 1);
            }
            if input[i] < input[i - w] && input[i - w] != 9 {
                queue.push(i - w);
            }
        } else if i % w == 0 { // left row
            if input[i] < input[i + 1] && input[i + 1] != 9 {
                queue.push(i + 1);
            }
            if input[i] < input[i + w] && input[i + w] != 9 {
                queue.push(i + w);
            }
            if input[i] < input[i - w] && input[i - w] != 9 {
                queue.push(i - w);
            }
        } else if i % w == (w - 1) { // right row
            if input[i] < input[i - 1] && input[i - 1] != 9 {
                queue.push(i - 1);
            }
            if input[i] < input[i + w] && input[i + w] != 9 {
                queue.push(i + w);
            }
            if input[i] < input[i - w] && input[i - w] != 9 {
                queue.push(i - w);
            }
        } else { // others
            if input[i] < input[i - 1] && input[i - 1] != 9 {
                queue.push(i - 1);
            }
            if input[i] < input[i + 1] && input[i + 1] != 9 {
                queue.push(i + 1);
            }
            if input[i] < input[i + w] && input[i + w] != 9 {
                queue.push(i + w);
            }
            if input[i] < input[i - w] && input[i - w] != 9 {
                queue.push(i - w);
            }
        }
    }
    cache.len()
}

fn part1(input: &(usize, usize, Vec<u32>)) -> u32 {
    let (w, h, input) = input;
    find_low_points(*w, *h, input)
        .map(|i| input[i] + 1)
        .sum()
}

fn part2(input: &(usize, usize, Vec<u32>)) -> u32 {
    let (w, h, input) = input;
    let mut basin_sizes = find_low_points(*w, *h, input)
        .map(|i| climb_basin(*w, *h, input, i))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).fold(1, |a, &x| a * x as u32)
}

fn main() {
    adventofcode2021::print_time!({
        let input = include_str!("../day_09_input.txt");
        let input = parse_input(input);
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {
    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 15);
    }


    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 1134);
    }
}
