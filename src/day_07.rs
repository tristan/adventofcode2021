fn parse_input(input: &str) -> Vec<i32> {
    input.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

fn part1(positions: &[i32]) -> i32 {
    let mut prev = i32::MAX;
    for pos in 0.. {
        let next = positions
            .iter()
            .map(|i| (pos - i).abs())
            .sum::<i32>();
        if next > prev {
            break;
        }
        prev = next
    }
    prev
}

fn part2(positions: &[i32]) -> i32 {
    let mut prev = i32::MAX;
    for pos in 0.. {
        let next = positions
            .iter()
            .map(|i| {
                let n = (pos - i).abs();
                n * (n + 1) / 2
            })
            .sum::<i32>();
        if next > prev {
            break;
        }
        prev = next
    }
    prev
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_07_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}


#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    fn get_test_input() -> Vec<i32> {
        parse_input(TEST_INPUT)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 168);
    }
}
