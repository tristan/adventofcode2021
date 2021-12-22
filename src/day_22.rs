struct Cube(bool, (i64, i64), (i64, i64), (i64, i64));

fn parse_input(input: &str) -> Vec<Cube> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (on, idx) = match &line[..3] {
                "on " => {
                    (true, 3)
                },
                "off" => {
                    (false, 4)
                },
                _ => panic!("invalid input")
            };
            let mut iter = line[idx..].split(',').map(|part| {
                let mut iter = part[2..].split("..")
                    .map(|num| num.parse::<i64>().unwrap());
                let s = iter.next().unwrap();
                let e = iter.next().unwrap();
                (s, e)
            });
            Cube(
                on,
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect()
}

fn part1(input: &[Cube]) -> usize {
    let mut cubes = vec![vec![vec![false; 101]; 101]; 101];
    for Cube(on, xr, yr, zr) in input {
        for x in xr.0..=xr.1 {
            if x < -50 || x > 50 {
                continue;
            }
            let x = (x + 50) as usize;
            for y in yr.0..=yr.1 {
                if y < -50 || y > 50 {
                    continue;
                }
                let y = (y + 50) as usize;
                for z in zr.0..=zr.1 {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    let z = (z + 50) as usize;
                    cubes[x][y][z] = *on;
                }
            }
        }
    }
    cubes.into_iter().flatten().flatten().filter(|&on| on).count()
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_22_input.txt"));
        println!("part1: {}", part1(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, Cube};

    const TEST_INPUT: &str = r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 590784);
    }
}
