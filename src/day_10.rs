enum Chunk {
    Parentheses, // ( )
    Brackets, // [ ]
    Braces, // { }
    Chevrons, // < >
}

fn part1(input: &str) -> u32 {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .fold(0, |mut score, line| {
            let mut stack = vec![];
            for c in line.chars() {
                match c {
                    '(' => stack.push(Chunk::Parentheses),
                    '[' => stack.push(Chunk::Brackets),
                    '{' => stack.push(Chunk::Braces),
                    '<' => stack.push(Chunk::Chevrons),
                    ')' => if !matches!(stack.pop(), Some(Chunk::Parentheses)) {
                        score += 3;
                        break;
                    },
                    ']' => if !matches!(stack.pop(), Some(Chunk::Brackets)) {
                        score += 57;
                        break;
                    },
                    '}' => if !matches!(stack.pop(), Some(Chunk::Braces)) {
                        score += 1197;
                        break;
                    },
                    '>' => if !matches!(stack.pop(), Some(Chunk::Chevrons)) {
                        score += 25137;
                        break;
                    },
                    _ => panic!("invalid character!")
                }
            }
            score
        })
}

fn part2(input: &str) -> u64 {
    let mut scores = input.split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut stack = vec![];
            for c in line.chars() {
                match c {
                    '(' => stack.push(Chunk::Parentheses),
                    '[' => stack.push(Chunk::Brackets),
                    '{' => stack.push(Chunk::Braces),
                    '<' => stack.push(Chunk::Chevrons),
                    ')' => if !matches!(stack.pop(), Some(Chunk::Parentheses)) {
                        return None;
                    },
                    ']' => if !matches!(stack.pop(), Some(Chunk::Brackets)) {
                        return None;
                    },
                    '}' => if !matches!(stack.pop(), Some(Chunk::Braces)) {
                        return None;
                    },
                    '>' => if !matches!(stack.pop(), Some(Chunk::Chevrons)) {
                        return None;
                    },
                    _ => panic!("invalid character!")
                }
            }
            let score = stack.into_iter().rev().fold(0, |score, c| {
                score * 5 + match c {
                    Chunk::Parentheses => 1,
                    Chunk::Brackets => 2,
                    Chunk::Braces => 3,
                    Chunk::Chevrons => 4
                }
            });
            Some(score)
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    adventofcode2021::print_time!({
        let input = include_str!("../day_10_input.txt");
        println!("part1: {}", part1(input));
        println!("part2: {}", part2(input));
    });
}

#[cfg(test)]
mod test {

    use super::{part1, part2};

    const TEST_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 288957);
    }

}
