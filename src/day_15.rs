use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c as usize - 48).collect())
        .collect()
}

fn solve(
    input: &[Vec<usize>],
) -> usize {
    let mut scores = vec![vec![usize::MAX; input.len()]; input.len()];
    let mut visited = HashSet::new();
    let offsets = [
        (-1, 0),
        (0, -1),
        (1, 0),
        (0, 1)
    ];
    *scores.get_mut(0).unwrap().get_mut(0).unwrap() = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0usize, 0usize)));
    while let Some(Reverse((score, x, y))) = queue.pop() {
        visited.insert((x, y));

        for (dx, dy) in offsets {
            let nx = match dx.cmp(&0) {
                Ordering::Less => x.wrapping_sub(1),
                Ordering::Greater => x + 1,
                Ordering::Equal => x,
            };
            let ny = match dy.cmp(&0) {
                Ordering::Less => y.wrapping_sub(1),
                Ordering::Greater => y + 1,
                Ordering::Equal => y,
            };
            if nx < input.len() && ny < input.len() && !visited.contains(&(nx, ny)) {
                let nrisk = *input.get(ny).unwrap().get(nx).unwrap();
                let nscore = scores.get_mut(ny).unwrap().get_mut(nx).unwrap();
                let new_score = score + nrisk;
                if new_score < *nscore {
                    *nscore = new_score;
                    queue.push(Reverse((new_score, nx, ny)));
                }
            }
        }
    }
    *scores.iter().last().unwrap().iter().last().unwrap()
}


fn part1(input: &[Vec<usize>]) -> usize {
    solve(input)
}

fn part2(input: &[Vec<usize>]) -> usize {
    let input = (0..input.len() * 5).map(|y| {
        (0..input.len() * 5).map(|x| {
            let xinc = x / input.len();
            let yinc = y / input.len();
            let risk = input.get(y % input.len()).unwrap().get(x % input.len()).unwrap()
                + xinc + yinc;
            if risk > 9 {
                risk - 9
            } else {
                risk
            }
        }).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();
    solve(&input)
}


fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_15_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 315);
    }
}
