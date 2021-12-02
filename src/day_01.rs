fn part1(input: &[usize]) -> usize {
    input.windows(2)
        .filter(|arr| arr[0] < arr[1])
        .count()
}

fn part2(input: &[usize]) -> usize {
    input.windows(3)
        .map(|arr| arr[0] + arr[1] + arr[2])
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|arr| arr[0] < arr[1])
        .count()
}

fn main() {
    let input = include_str!("../day_01_input.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<usize>())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod test {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = vec![
            199, 200, 208, 210, 200, 207, 240, 269, 260, 263
        ];
        assert_eq!(part1(&input), 7)
    }
    #[test]
    fn test_part2() {
        let input = vec![
            199, 200, 208, 210, 200, 207, 240, 269, 260, 263
        ];
        assert_eq!(part2(&input), 5)
    }

}
