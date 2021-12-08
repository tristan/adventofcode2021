fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut s = line.split(" | ")
                .map(|s| s.split_whitespace()
                     .map(|p| p.chars()
                          .fold(0u8, |prev, c| match c {
                              'a' => prev | 0b1,
                              'b' => prev | 0b10,
                              'c' => prev | 0b100,
                              'd' => prev | 0b1000,
                              'e' => prev | 0b10000,
                              'f' => prev | 0b100000,
                              'g' => prev | 0b1000000,
                              _ => panic!("invalid char: {}", c)
                          })
                     ).collect()
                );
            let pattens = s.next().unwrap();
            let output = s.next().unwrap();
            (pattens, output)
        })
        .collect()
}

fn part1(input: &[(Vec<u8>, Vec<u8>)]) -> usize {
    input.iter()
        .map(|(_, outputs)| outputs)
        .flatten()
        .filter(|output| matches!(output.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

fn part2(input: &[(Vec<u8>, Vec<u8>)]) -> usize {
    input.iter().map(|(pattens, outputs)| {
        let mut nums = [0u8; 10];
        let mut five_bits = Vec::with_capacity(3);
        pattens.iter().for_each(|&p| match p.count_ones() {
            2 => nums[1] = p,
            3 => nums[7] = p,
            4 => nums[4] = p,
            5 => five_bits.push(p),
            6 => {},
            7 => nums[8] = p,
            _ => panic!("invalid output")
        });
        five_bits.into_iter().for_each(|n| {
            if n & nums[1] == nums[1] { nums[3] = n }
            else if (n & nums[4]).count_ones() == 3 { nums[5] = n }
            else { nums[2] = n }
        });
        // a = one ^ seven
        let a = nums[1] ^ nums[7];
        // g = (four | seven) ^ three
        let g = (nums[4] | nums[7]) ^ nums[3];
        // nine = four | a | g
        nums[9] = nums[4] | a | g;
        // e = !nine (with the significant bit set as well, as it's not used)
        let e = !(nums[9] | 128);
        nums[6] = nums[5] | e;
        let b = !(nums[3] | e | 128);
        nums[0] = nums[7] | b | e | g;

        outputs.iter().map(|o| {
            nums.iter().enumerate().find_map(|(i, n)| if n == o {
                Some(i)
            } else {
                None
            }).unwrap()
        }).fold(0, |acc, n| {
            acc * 10 + n
        })
    }).sum()
}

fn main() {
    adventofcode2021::print_time!({
        let input = include_str!("../day_08_input.txt");
        let input = parse_input(input);
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 61229);
    }
}
