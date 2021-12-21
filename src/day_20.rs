fn parse_input(input: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut parts = input.split("\n\n");
    (
        parts.next().unwrap()
            .chars().filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None
            })
            .collect(),
        parts.next().unwrap()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars().filter_map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    _ => None
                }).collect()
            })
            .collect()
    )
}

fn expand_image((img, fill): (Vec<Vec<bool>>, bool)) -> Vec<Vec<bool>> {
    let mut res = Vec::with_capacity(img.len() + 4);
    let empty = vec![fill; img[0].len() + 4];
    res.push(empty.clone());
    res.push(empty.clone());
    for mut row in img {
        row.insert(0, fill);
        row.insert(0, fill);
        row.push(fill);
        row.push(fill);
        res.push(row);
    }
    res.push(empty.clone());
    res.push(empty);
    res
}

fn enhance_image(enhancement: &[bool], image: (Vec<Vec<bool>>, bool)) -> (Vec<Vec<bool>>, bool) {
    let fill = image.1;
    let image = expand_image(image);
    (
        image.windows(3).map(|arr| {
            let r0 = &arr[0];
            let r1 = &arr[1];
            let r2 = &arr[2];
            r0.windows(3).zip(r1.windows(3)).zip(r2.windows(3)).map(|((g0, g1), g2)| {
                let mut res = 0usize;
                for i in g0.iter().chain(g1.iter()).chain(g2.iter()) {
                    res <<= 1;
                    if *i {
                        res += 1;
                    }
                }
                enhancement[res]
            }).collect::<Vec<bool>>()
        }).collect::<Vec<Vec<bool>>>(),
        if fill {
            enhancement[511]
        } else {
            enhancement[0]
        }
    )
}

fn part1((enhancement, mut image): (Vec<bool>, Vec<Vec<bool>>)) -> usize {
    let mut fill = false;
    for _ in 0..2 {
        let res = enhance_image(&enhancement, (image, fill));
        image = res.0;
        fill = res.1;
    }
    image.iter().flatten().filter(|&&v| v).count()
}

fn part2((enhancement, mut image): (Vec<bool>, Vec<Vec<bool>>)) -> usize {
    let mut fill = false;
    for _ in 0..50 {
        let res = enhance_image(&enhancement, (image, fill));
        image = res.0;
        fill = res.1;
    }
    image.iter().flatten().filter(|&&v| v).count()
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_20_input.txt"));
        println!("part1: {}", part1(input.clone()));
        println!("part1: {}", part2(input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT)), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(TEST_INPUT)), 3351);
    }
}
