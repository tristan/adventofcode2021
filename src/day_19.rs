use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<(i32, i32, i32)>> {
    input.split("\n\n")
        .map(|scanners| {
            scanners.split('\n')
                .skip(1)
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let mut i = line.split(',');
                    (
                        i.next().unwrap().parse::<i32>().unwrap(),
                        i.next().unwrap().parse::<i32>().unwrap(),
                        i.next().unwrap().parse::<i32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn vec_diff(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

struct Rotation {
    x: usize,
    y: usize,
    z: usize,
    step: usize,
    list: Vec<(i32, i32, i32)>,
}

impl Rotation {
    fn new(list: Vec<(i32, i32, i32)>) -> Rotation {
        Rotation {
            x: 0,
            y: 0,
            z: 0,
            step: 0,
            list
        }
    }

    fn rotate_z(&mut self) {
        self.z = (self.z + 1) % 4;
        self.list.iter_mut().for_each(|p| {
            *p = (p.1, -p.0, p.2);
        });
    }

    fn rotate_y(&mut self) {
        self.y = (self.y + 1) % 4;
        self.list.iter_mut().for_each(|p| {
            *p = (-p.2, p.1, p.0);
        });
    }

    fn rotate_x(&mut self) {
        self.x = (self.x + 1) % 4;
        self.list.iter_mut().for_each(|p| {
            *p = (p.0, -p.2, p.1);
        });
    }
}

impl Iterator for Rotation {
    // We can refer to this type using Self::Item
    type Item = Vec<(i32, i32, i32)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.step += 1;
        if self.step > 1 {
            self.rotate_z();
            if self.z == 0 {
                self.rotate_y();
                if self.y == 0 {
                    self.rotate_x();
                    if self.x == 0 {
                        return None;
                    }
                }
            }
        }
        Some(self.list.clone())
    }
}

fn solve(input: &[Vec<(i32, i32, i32)>]) -> (usize, i32) {
    let mut result = input[0].clone();

    let mut scanner_positions = vec![];

    let mut remaining = input.iter().cloned().enumerate().skip(1)
        .collect::<Vec<_>>();

    'outer: while !remaining.is_empty() {
        println!("outer");
        let scanner_0_hs = result.iter()
            .cloned().collect::<HashSet<_>>();
        for idx in 0..remaining.len() {
            println!("scanner {}", remaining[idx].0);
            let s1_rots = Rotation::new(remaining[idx].1.clone());
            for scanner_1 in s1_rots {
                for i in 0..scanner_1.len() {
                    let s0i = result.iter();
                    let s1i = scanner_1.iter().cycle().skip(i);
                    for (p0, p1) in s0i.zip(s1i) {
                        let diff = vec_diff(p1, p0);
                        let scanner_1_hs = scanner_1.iter()
                            .map(|p| vec_diff(p, &diff))
                            .collect::<HashSet<_>>();
                        let int = scanner_0_hs
                            .intersection(&scanner_1_hs)
                            .collect::<Vec<_>>();
                        if int.len() >= 12 {
                            println!("YES!");
                            println!("scanner: {},{},{}", diff.0, diff.1, diff.2);
                            scanner_positions.push(diff);
                            for p in int {
                                println!("{},{},{}", p.0, p.1, p.2);
                            }
                            result = scanner_0_hs
                                .union(&scanner_1_hs)
                                .cloned()
                                .collect::<Vec<_>>();
                            remaining.remove(idx);
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    (
        result.len(),
        scanner_positions.iter().enumerate().map(|(i, a)| {
            scanner_positions.iter().enumerate().filter_map(|(j, b)| {
                if i == j {
                    None
                } else {
                    Some(
                        (a.0 - b.0) + (a.1 - b.1) + (a.2 - b.2)
                    )
                }
            }).collect::<Vec<_>>()
        }).flatten().max().unwrap()
    )
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_19_input.txt"));
        let res = solve(&input);
        println!("part1: {}", res.0);
        println!("part2: {}", res.1);
    });
}


#[cfg(test)]
mod test {

    use super::{parse_input, Rotation, solve};

    #[test]
    fn test_rotate() {
        let mut r = Rotation::new(vec![(1, 1, 1)]);

        assert_eq!(r.next(), Some(vec![(1, 1, 1)]));
        assert_eq!(r.next(), Some(vec![(1, -1, 1)]));
        assert_eq!(r.next(), Some(vec![(-1, -1, 1)]));
        assert_eq!(r.next(), Some(vec![(-1, 1, 1)]));
        // y
        assert_eq!(r.next(), Some(vec![(-1, 1, 1)]));
        assert_eq!(r.next(), Some(vec![(1, 1, 1)]));
        assert_eq!(r.next(), Some(vec![(1, -1, 1)]));
        assert_eq!(r.next(), Some(vec![(-1, -1, 1)]));
        // y
        assert_eq!(r.next(), Some(vec![(-1, 1, -1)]));
        assert_eq!(r.next(), Some(vec![(1, 1, -1)]));
        assert_eq!(r.next(), Some(vec![(1, -1, -1)]));
        assert_eq!(r.next(), Some(vec![(-1, -1, -1)]));
        // y
        assert_eq!(r.next(), Some(vec![(1, 1, -1)]));
        assert_eq!(r.next(), Some(vec![(1, -1, -1)]));
        assert_eq!(r.next(), Some(vec![(-1, -1, -1)]));
        assert_eq!(r.next(), Some(vec![(-1, 1, -1)]));
        // x
        assert_eq!(r.next(), Some(vec![(1, -1, 1)]));
        assert_eq!(r.next(), Some(vec![(-1, -1, 1)]));
        assert_eq!(r.next(), Some(vec![(-1, 1, 1)]));
        assert_eq!(r.next(), Some(vec![(1, 1, 1)]));
    }

    #[test]
    fn test_solve() {
        let input = parse_input(include_str!("../day_19_test_input.txt"));
        assert_eq!(dbg!(solve(&input)), (79, 3621));
    }
}
