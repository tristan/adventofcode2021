fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        line.chars().filter_map(|c| match c {
            '0'..='9' => Some(c as u8 - 48),
            _ => None
        }).collect()
    }).collect()
}

fn flash(input: &mut Vec<Vec<u8>>, y: usize, x: usize) {
    let offsets = [
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];
    for (dy, dx) in offsets {
        let py = match dy {
            -1 => y.wrapping_sub(1),
            0 => y,
            _ => y + 1
        };
        let px = match dx {
            -1 => x.wrapping_sub(1),
            0 => x,
            _ => x + 1
        };
        if py < input.len() && px < input[py].len() {
            input[py][px] += 1;
            if input[py][px] == 10 {
                flash(input, py, px);
            }
        }
    }
}

fn part1(mut input: Vec<Vec<u8>>) -> usize {
    let mut flashes = 0;
    let col_len = input.len();
    let row_len = input.get(0).unwrap().len();
    for _ in 0..100 {
        // increment and flash
        for y in 0..col_len {
            for x in 0..row_len {
                input[y][x] += 1;
                if input[y][x] == 10 {
                    flash(&mut input, y, x);
                }
            }
        }
        // count and reset
        for row in input.iter_mut() {
            for col in row {
                if *col > 9 {
                    flashes += 1;
                    *col = 0;
                }
            }
        }
    }
    flashes
}

fn part2(mut input: Vec<Vec<u8>>) -> usize {
    let col_len = input.len();
    let row_len = input.get(0).unwrap().len();
    (1..).find(|_| {
        // increment and flash
        for y in 0..col_len {
            for x in 0..row_len {
                input[y][x] += 1;
                if input[y][x] == 10 {
                    flash(&mut input, y, x);
                }
            }
        }
        // count and reset
        let mut flashes = 0;
        for row in input.iter_mut() {
            for col in row {
                if *col > 9 {
                    flashes += 1;
                    *col = 0;
                }
            }
        }
        flashes == col_len * row_len
    }).unwrap()
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_11_input.txt"));
        println!("part1: {}", part1(input.clone()));
        println!("part2: {}", part2(input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT)), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(TEST_INPUT)), 195);
    }
}
