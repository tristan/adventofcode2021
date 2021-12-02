fn part1(input: &[&str]) -> isize {
    let (depth, position) = input.iter().fold((0, 0), |(depth, position), s| {
        let mut i = s.splitn(2, ' ');
        let c = i.next().unwrap();
        let x = i.next().unwrap().parse::<isize>().unwrap();
        match c {
            "forward" => (depth, position + x),
            "up" => (depth - x, position),
            "down" => (depth + x, position),
            _ => panic!("invalid command!")
        }
    });
    depth * position
}

fn part2(input: &[&str]) -> isize {
    let (depth, position, _) = input.iter().fold(
        (0, 0, 0),
        |(depth, position, aim), s| {
            let mut i = s.splitn(2, ' ');
            let c = i.next().unwrap();
            let x = i.next().unwrap().parse::<isize>().unwrap();
            match c {
                "forward" => (depth + aim * x, position + x, aim),
                "up" => (depth, position, aim - x),
                "down" => (depth, position, aim + x),
                _ => panic!("invalid command!")
            }
        }
    );
    depth * position
}

fn main() {
    let input = include_str!("../day_02_input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    const TEST_INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    fn get_input() -> Vec<&'static str> {
        TEST_INPUT.split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()

    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_input()), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_input()), 900);
    }
}
