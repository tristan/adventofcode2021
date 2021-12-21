fn part1(mut p1pos: usize, mut p2pos: usize) -> usize {
    p1pos -= 1;
    p2pos -= 1;
    let mut dice = (1..=100).cycle();
    let mut roll_dice = || { dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap() };
    let mut p1score = 0;
    let mut p2score = 0;
    let mut rolls = 0;
    loop {
        let roll = roll_dice();
        rolls += 3;
        p1pos = (p1pos + roll) % 10;
        p1score += p1pos + 1;
        if p1score >= 1000 {
            return p2score * rolls;
        }
        let roll = roll_dice();
        rolls += 3;
        p2pos = (p2pos + roll) % 10;
        p2score += p2pos + 1;
        if p2score >= 1000 {
            return p1score * rolls;
        }
    }
}

const ROLLS_UNIVERSES: [(usize, usize); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1)
];

fn part2_turn(
    p1pos: usize,
    p2pos: usize,
    p1score: usize,
    p2score: usize,
    universes: usize,
    turn: bool
) -> (usize, usize) {
    let mut totals = (0, 0);
    if turn {
        if p2score >= 21 {
            return (0, universes)
        }
        for (roll, new_universes) in ROLLS_UNIVERSES {
            let res = part2_turn(
                (p1pos + roll) % 10,
                p2pos,
                p1score + ((p1pos + roll) % 10) + 1,
                p2score,
                universes * new_universes,
                false,
            );
            totals = (
                totals.0 + res.0,
                totals.1 + res.1
            );
        }
    } else {
        if p1score >= 21 {
            return (universes, 0)
        }
        for (roll, new_universes) in ROLLS_UNIVERSES {
            let res = part2_turn(
                p1pos,
                (p2pos + roll) % 10,
                p1score,
                p2score + ((p2pos + roll) % 10) + 1,
                universes * new_universes,
                true,
            );
            totals = (
                totals.0 + res.0,
                totals.1 + res.1
            );
        }
    }
    totals

}

fn part2(p1pos: usize, p2pos: usize) -> usize {
    let (p1u, p2u) = part2_turn(p1pos - 1, p2pos - 1, 0, 0, 1, true);
    p1u.max(p2u)
}


fn main() {
    adventofcode2021::print_time!({
        println!("part1: {}", part1(8, 3));
        println!("part2: {}", part2(8, 3));
    });
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(4, 8), 739785);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(4, 8), 444356092776315);
    }
}
