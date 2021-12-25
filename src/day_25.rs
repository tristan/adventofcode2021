use std::collections::HashSet;

type InputType = (HashSet<(u8, u8)>, HashSet<(u8, u8)>, u8, u8);

fn parse_input(input: &str) -> InputType {
    let mut east = HashSet::new();
    let mut south = HashSet::new();
    let mut x_size = 0;
    let mut y_size = 0;
    input.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            let y = y as u8;
            y_size = y;
            line.chars().enumerate().for_each(|(x, c)| {
                let x = x as u8;
                x_size = x;
                match c {
                    '>' => { east.insert((x, y)); },
                    'v' => { south.insert((x, y)); },
                    _ => {}
                };
            });
        });
    (east, south, x_size + 1, y_size + 1)
}

fn part1(input: InputType) -> usize {
    // move east
    let (mut east, mut south, x_size, y_size) = input;
    let mut steps = 0;
    loop {
        steps += 1;
        let mut movement = false;
        let mut new_east = HashSet::with_capacity(east.len());
        for &(x, y) in &east {
            let nx = (x + 1) % x_size;
            if !east.contains(&(nx, y)) && !south.contains(&(nx, y)) {
                movement = true;
                new_east.insert((nx, y));
            } else {
                new_east.insert((x, y));
            }
        }
        east = new_east;
        let mut new_south = HashSet::with_capacity(south.len());
        for &(x, y) in &south {
            let ny = (y + 1) % y_size;
            if !east.contains(&(x, ny)) && !south.contains(&(x, ny)) {
                movement = true;
                new_south.insert((x, ny));
            } else {
                new_south.insert((x, y));
            }
        }
        south = new_south;
        if !movement {
            break;
        }
    }
    steps
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_25_input.txt"));
        println!("part1: {}", part1(input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1};

    const TEST_INPUT: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT)), 58);
    }

}
