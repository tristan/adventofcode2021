use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Node {
    Number(u64),
    Open,
    Close,
}

impl Node {
    fn unwrap(&self) -> u64 {
        match self {
            Node::Number(i) => *i,
            _ => panic!("cannot unwrap non number node")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Num(Vec<Node>);

impl std::ops::Add for Num {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.0.is_empty() {
            rhs
        } else if rhs.0.is_empty() {
            self
        } else {
            self.0.insert(0, Node::Open);
            self.0.extend(rhs.0);
            self.0.push(Node::Close);
            self.normalise();
            self
        }
    }
}

impl std::ops::Add for &Num {
    type Output = Num;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.is_empty() {
            rhs.clone()
        } else if rhs.0.is_empty() {
            self.clone()
        } else {
            self.clone() + rhs.clone()
        }
    }
}


impl Num {
    fn parse_str(s: &str) -> Num {
        Num(s.chars().filter_map(|c| match c {
            '[' => Some(Node::Open),
            ']' => Some(Node::Close),
            '0'..='9' => Some(Node::Number(c as u64 - 48)),
            _ => None,
        }).collect())
    }

    fn split(&mut self) -> bool {
        let mut idx = 0;
        while idx < self.0.len() {
            match self.0.get(idx) {
                Some(&Node::Number(n)) if n > 9 => {
                    let left = n / 2;
                    let right = n - left;
                    self.0.splice(idx..=idx, [Node::Open, Node::Number(left), Node::Number(right), Node::Close]);
                    return true;
                },
                _ => {
                    idx += 1;
                }
            }
        }
        false
    }

    fn explode(&mut self) -> bool {
        let mut idx = 0;
        let mut depth = 0;
        let mut has_exploded = false;
        while idx < self.0.len() {
            if depth == 5 {
                let left = self.0.get(idx).unwrap().unwrap();
                let right = self.0.get(idx + 1).unwrap().unwrap();
                self.0.splice(idx - 1..idx + 3, [Node::Number(0)]);
                if let Some(Node::Number(n)) = self.0[..idx-1].iter_mut().rfind(|x| matches!(x, Node::Number(_))) {
                    *n += left
                }
                if let Some(Node::Number(n)) = self.0[idx..].iter_mut().find(|x| matches!(x, Node::Number(_))) {
                    *n += right
                }
                idx = 0;
                depth = 0;
                has_exploded = true;
            }
            match self.0.get(idx) {
                Some(Node::Open) => {
                    depth += 1;
                    idx += 1;
                },
                Some(Node::Close) => {
                    depth -= 1;
                    idx += 1;
                },
                _ => {
                    idx += 1;
                }
            }
        }
        has_exploded
    }

    fn normalise(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn magnitude(&self) -> u64 {
        let mut mutliplier = 1;
        let mut output = 0;
        let last = self.0.len() - 1;
        for (i, sym) in self.0.iter().enumerate() {
            match sym {
                Node::Open => mutliplier *= 3,
                Node::Close => {
                    mutliplier /= 2;
                    if i < last && matches!(self.0.get(i + 1), Some(Node::Open | Node::Number(_))) {
                        mutliplier = (mutliplier / 3) * 2;
                    }
                },
                Node::Number(num) => {
                    output += mutliplier * *num as u64;
                    if i < last && matches!(self.0.get(i + 1), Some(Node::Open | Node::Number(_))) {
                        mutliplier = (mutliplier / 3) * 2;
                    }
                }
            }

        }
        output
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last = self.0.len() - 1;
        for (i, n) in self.0.iter().enumerate() {
            match n {
                Node::Open => {
                    write!(f, "[")?;
                },
                Node::Close => {
                    write!(f, "]")?;
                    if i < last && matches!(self.0.get(i + 1), Some(Node::Open | Node::Number(_))) {
                        write!(f, ",")?;
                    }
                },
                Node::Number(n) => {
                    write!(f, "{}", n)?;
                    if i < last && matches!(self.0.get(i + 1), Some(Node::Open | Node::Number(_))) {
                        write!(f, ",")?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn part1(list: &str) -> u64 {
    list.lines()
        .filter(|line| !line.is_empty())
        .map(Num::parse_str)
        .fold(Num(vec![]), |acc, n| acc + n)
        .magnitude()
}

fn part2(list: &str) -> u64 {
    let nums = list.lines()
        .filter(|line| !line.is_empty())
        .map(Num::parse_str)
        .collect::<Vec<_>>();
    nums.iter().enumerate().map(|(i, a)| {
        nums.iter().enumerate().filter_map(|(j, b)| {
            if i == j {
                None
            } else {
                Some((a.add(b)).magnitude())
            }
        }).collect::<Vec<u64>>()
    }).flatten()
        .max().unwrap()
}

fn main() {
    adventofcode2021::print_time!({
        let input = include_str!("../day_18_input.txt");
        println!("part1: {}", part1(input));
        println!("part2: {}", part2(input));
    });
}

#[cfg(test)]
mod test {

    use super::{Num, part1, part2};

    const TEST_INPUT_1: &str = r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
"#;

    const TEST_INPUT_2: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;

    #[test]
    fn test_explode() {
        let mut n = Num::parse_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        n.explode();
        assert_eq!(n, Num::parse_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_add_0() {
        let n = Num::parse_str("[[[[4,3],4],4],[7,[[8,4],9]]]") + Num::parse_str("[1,1]");
        assert_eq!(n, Num::parse_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_add_n() {
        assert_eq!(
            Num::parse_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]") + Num::parse_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
            Num::parse_str("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        );
    }

    #[test]
    fn test_add_1() {
        let res = TEST_INPUT_1.lines()
            .filter(|line| !line.is_empty())
            .map(Num::parse_str)
            .fold(Num(vec![]), |acc, n| acc + n)
            ;
        assert_eq!(res, Num::parse_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    }

    #[test]
    fn test_add_2() {
        let res = TEST_INPUT_2.lines()
            .filter(|line| !line.is_empty())
            .map(Num::parse_str)
            .fold(Num(vec![]), |acc, n| acc + n)
            ;
        assert_eq!(res, Num::parse_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_2), 4140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 3993);
    }
}
