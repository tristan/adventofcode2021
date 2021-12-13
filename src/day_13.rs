use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Fold {
    X(i32),
    Y(i32)
}

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Fold>) {
    let mut main = input.split("\n\n");
    (
        main.next().unwrap()
            .split('\n')
            .map(|xy| {
                let mut xy = xy.split(',').map(|i| i.parse::<i32>().unwrap());
                let x = xy.next().unwrap();
                let y = xy.next().unwrap();
                (x ,y)
            })
            .collect(),
        main.next().unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|folds| {
                let val = folds[13..].parse::<i32>().unwrap();
                match folds.chars().nth(11) {
                    Some('x') => Fold::X(val),
                    Some('y') => Fold::Y(val),
                    _ => panic!("unexpected input")
                }
            })
            .collect()
    )
}

fn part1(input: &(HashSet<(i32, i32)>, Vec<Fold>)) -> usize {
    let (dots, folds) = input;
    let newdots = match folds.get(0) {
        Some(&Fold::X(xf)) => {
            dots.iter()
                .map(|&(x, y)| {
                    if x < xf {
                        (x, y)
                    } else {
                        (xf - (x - xf), y)
                    }
                })
                .collect::<HashSet<_>>()
        }
        Some(&Fold::Y(yf)) => {
            dots.iter()
                .map(|&(x, y)| {
                    if y < yf {
                        (x, y)
                    } else {
                        (x, yf - (y - yf))
                    }
                })
                .collect::<HashSet<_>>()
        },
        None => panic!("missing folds")
    };
    newdots.len()
}

fn part2(input: &(HashSet<(i32, i32)>, Vec<Fold>)) -> String {
    let (dots, folds) = input;
    let mut dots = folds.iter().fold(dots.clone(), |dots, &fold| {
        match fold {
            Fold::X(xf) => {
                dots.iter()
                    .map(|&(x, y)| {
                        if x < xf {
                            (x, y)
                        } else {
                            (xf - (x - xf), y)
                        }
                    })
                    .collect::<HashSet<_>>()
            }
            Fold::Y(yf) => {
                dots.iter()
                    .map(|&(x, y)| {
                        if y < yf {
                            (x, y)
                        } else {
                            (x, yf - (y - yf))
                        }
                    })
                    .collect::<HashSet<_>>()
            }
        }
    });
    dots.insert((0, -1));
    let mut dots = dots.into_iter().collect::<Vec<_>>();
    dots.sort_unstable_by_key(|&(x, y)| (y, x));
    let mut result = String::with_capacity(256);
    dots.windows(2).for_each(|parts| {
        let (mut px, py) = parts[0];
        let (x, y) = parts[1];
        if py != y {
            result.push('\n');
            px = 0
        }
        result.push_str(&" ".repeat((x - px - 1).max(0) as usize));
        result.push('*');
    });
    result
}


fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_13_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}
