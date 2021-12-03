fn part1(input: &[&str]) -> u64 {
    let counts = input.iter()
        .fold(vec![], |acc, line| {
            acc.iter()
                .chain(std::iter::repeat(&(0u64, 0u64)))
                .zip(line.chars())
                .map(|(&(zeros, ones), chr)| match chr {
                    '0' => (zeros + 1, ones),
                    _ => (zeros, ones + 1),
                })
                .collect::<Vec<_>>()
        });
    let (gamma, epsilon) = counts.iter().fold((0, 0), |(gamma, epsilon), (zeros, ones)| {
        if zeros > ones {
            (gamma << 1, (epsilon << 1) + 1)
        } else {
            ((gamma << 1) + 1, epsilon << 1)
        }
    });
    gamma * epsilon
}

fn split_up<'a>(mut zeros: Vec<&'a str>, mut ones: Vec<&'a str>, line: &'a str, index: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    match line.chars().nth(index) {
        Some('0') => zeros.push(line),
        _ => ones.push(line)
    };
    (zeros, ones)
}

fn part2_dive(input: &[&str], index: usize, least: bool) -> u64 {
    let (zeros, ones) = input.iter()
        .fold((vec![], vec![]), |acc, line| split_up(acc.0, acc.1, line, index));
    let parts = if !least {
        if zeros.len() > ones.len() {
            zeros
        } else {
            ones
        }
    } else if ones.len() < zeros.len() {
        ones
    } else {
        zeros
    };
    if parts.len() == 1 {
        u64::from_str_radix(parts.get(0).unwrap(), 2).unwrap()
    } else {
        part2_dive(&parts, index + 1, least)
    }
}

fn part2(input: &[&str]) -> u64 {
    let (zeros, ones) = input.iter()
        .fold((vec![], vec![]), |acc, line| split_up(acc.0, acc.1, line, 0));
    let zeros_dive = part2_dive(&zeros, 1, zeros.len() < ones.len());
    let ones_dive = part2_dive(&ones, 1, zeros.len() > ones.len());
    zeros_dive * ones_dive
}

fn main() {
    let input = include_str!("../day_03_input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod test {

    use super::{part1, part2};

    const TEST_INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    fn get_test_input() -> Vec<&'static str> {
        TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 198)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 230)
    }

}
