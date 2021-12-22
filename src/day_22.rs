#[derive(Clone)]
struct Cube(bool, (i64, i64), (i64, i64), (i64, i64));

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} x={}..{},y={}..{},z={}..{}",
            if self.0 {
                "on"
            } else {
                "off"
            },
            self.1.0, self.1.1,
            self.2.0, self.2.1,
            self.3.0, self.3.1,
        )
    }
}

impl Cube {
    fn on(&self) -> bool {
        self.0
    }
    fn xmin(&self) -> i64 {
        self.1.0
    }
    fn xmax(&self) -> i64 {
        self.1.1
    }
    fn ymin(&self) -> i64 {
        self.2.0
    }
    fn ymax(&self) -> i64 {
        self.2.1
    }
    fn zmin(&self) -> i64 {
        self.3.0
    }
    fn zmax(&self) -> i64 {
        self.3.1
    }
    fn intersect(&self, rhs: &Cube) -> Option<Cube> {
        intersecting_cube(self, rhs)
    }
    fn count(&self) -> i64 {
        let x = self.xmax() - self.xmin() + 1;
        let y = self.ymax() - self.ymin() + 1;
        let z = self.zmax() - self.zmin() + 1;
        let v = x * y * z;
        if self.on() {
            v
        } else {
            -v
        }
    }
    fn flip_on(mut self) -> Cube {
        self.0 = !self.0;
        self
    }
}

fn intersecting_cube(a: &Cube, b: &Cube) -> Option<Cube> {
    // if a is to the left of b
    let x_intersect = if a.xmin() <= b.xmin() {
        // if b starts inside a
        let xmin = if b.xmin() <= a.xmax() {
            b.xmin()
        } else {
            // no intersection
            return None;
        };
        let xmax = a.xmax().min(b.xmax());
        (xmin, xmax)
    } else {
        // if a starts inside b
        let xmin = if a.xmin() <= b.xmax() {
            a.xmin()
        } else {
            // no intersection
            return None;
        };
        let xmax = b.xmax().min(a.xmax());
        (xmin, xmax)
    };
    let y_intersect = if a.ymin() <= b.ymin() {
        // if b starts inside x
        let ymin = if b.ymin() <= a.ymax() {
            b.ymin()
        } else {
            // no intersection
            return None;
        };
        let ymax = a.ymax().min(b.ymax());
        (ymin, ymax)
    } else {
        // if a starts inside b
        let ymin = if a.ymin() <= b.ymax() {
            a.ymin()
        } else {
            // no intersection
            return None;
        };
        let ymax = b.ymax().min(a.ymax());
        (ymin, ymax)
    };
    let z_intersect = if a.zmin() <= b.zmin() {
        // if b starts inside x
        let zmin = if b.zmin() <= a.zmax() {
            b.zmin()
        } else {
            // no intersection
            return None;
        };
        let zmax = a.zmax().min(b.zmax());
        (zmin, zmax)
    } else {
        // if a starts inside b
        let zmin = if a.zmin() <= b.zmax() {
            a.zmin()
        } else {
            // no intersection
            return None;
        };
        let zmax = b.zmax().min(a.zmax());
        (zmin, zmax)
    };
    Some(Cube(b.on(), x_intersect, y_intersect, z_intersect))
}

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

fn part1(input: &[Cube]) -> i64 {
    let init_cube = Cube(false, (-50, 50), (-50, 50), (-50, 50));
    let input = input.iter().filter_map(|cube| {
        init_cube.intersect(cube)
    }).collect::<Vec<_>>();
    part2(&input)
}

fn part2(input: &[Cube]) -> i64 {
    let cubes = input.iter().fold(vec![], |mut cubes, a| {
        // for each cube we've already stored
        // figure out the intersections with this cube.

        // if the cube turns points on, we add a new
        // cube that turns off the intersecting points.
        let intersections = cubes.iter()
            .filter_map(|b| a.intersect(b).map(|c| c.flip_on()))
            .collect::<Vec<_>>();
        // if the cube turns on points, add it as well
        // this will overwrite all the intersecting negations
        // we possibly added before
        if a.on() { cubes.push(a.clone()); }
        cubes.extend(intersections);
        cubes
    });
    cubes.into_iter().map(|c| c.count())
        .sum::<i64>()
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_22_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2, Cube};

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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(include_str!("../day_22_test_input.txt"))), 2758514936282235);
    }

    #[test]
    fn test_intersecting_cube() {
        let a = Cube(true, (10, 12), (10, 12), (10, 12));
        let b = Cube(true, (11, 13), (11, 13), (11, 13));
        let Cube(_, x, y, z) = a.intersect(&b).unwrap();
        assert_eq!(x, (11, 12));
        assert_eq!(y, (11, 12));
        assert_eq!(z, (11, 12));
        let c = Cube(true, (10, 10), (10, 10), (10, 10));
        let Cube(_, x, y, z) = a.intersect(&c).unwrap();
        assert_eq!(x, (10, 10));
        assert_eq!(y, (10, 10));
        assert_eq!(z, (10, 10));
        let d = Cube(true, (15, 19), (11, 13), (11, 13));
        assert!(a.intersect(&d).is_none());
        let e = Cube(true, (6, 10), (11, 13), (11, 13));
        let Cube(_, x, y, z) = a.intersect(&e).unwrap();
        assert_eq!(x, (10, 10));
        assert_eq!(y, (11, 12));
        assert_eq!(z, (11, 12));
    }
}
