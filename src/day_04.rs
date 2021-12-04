use std::collections::HashMap;

#[derive(Clone)]
struct Board {
    numbers: HashMap<u8, (usize, usize)>,
    marked_row_counts: [u8; 5],
    marked_col_counts: [u8; 5],
}

impl Board {
    fn new(numbers: HashMap<u8, (usize, usize)>) -> Board {
        Board {
            numbers,
            marked_row_counts: [0u8; 5],
            marked_col_counts: [0u8; 5],
        }
    }
    fn mark(&mut self, number: u8) -> bool {
        if let Some((x,y)) = self.numbers.remove(&number) {
            self.marked_row_counts[x] += 1;
            self.marked_col_counts[y] += 1;
            self.marked_row_counts[x] == 5 || self.marked_col_counts[y] == 5
        } else {
            false
        }
    }

    fn sum_remaining_numbers(&self) -> u64 {
        self.numbers.keys().map(|&k| k as u64).sum()
    }
}

fn parse_input(input: &'static str) -> (Vec<u8>, Vec<Board>) {
    let mut blocks = input.split("\n\n");
    let numbers = blocks.next().unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u8>>();

    let boards = blocks.map(|block| {
        let numbers = block.split('\n')
            .enumerate()
            .map(|(y, row)| {
                row.split_whitespace()
                    .enumerate()
                    .map(move |(x, v)| (v.parse::<u8>().unwrap(), (x, y)))
            })
            .flatten()
            .collect::<HashMap<u8, (usize, usize)>>();
        Board::new(numbers)
    }).collect::<Vec<_>>();

    (numbers, boards)
}

fn part1(numbers: Vec<u8>, mut boards: Vec<Board>) -> u64 {
    for number in numbers {
        for board in &mut boards {
            if board.mark(number) {
                return board.sum_remaining_numbers() * number as u64
            }
        }
    }
    panic!("No boards won!")
}

fn part2(numbers: Vec<u8>, mut boards: Vec<Board>) -> u64 {
    let mut complete_boards: Vec<(u8, Board)> = vec![];
    for number in numbers {
        boards = boards.into_iter()
            .filter_map(|mut board| {
                if board.mark(number) {
                    complete_boards.push((number, board));
                    None
                } else {
                    Some(board)
                }
            }).collect::<Vec<_>>();
        if boards.is_empty() {
            break;
        }
    }
    complete_boards.into_iter().last()
        .map(|(n, b)| n as u64 * b.sum_remaining_numbers())
        .unwrap()
}

fn main() {
    let input = include_str!("../day_04_input.txt");
    let (numbers, boards) = parse_input(input);
    println!("part1: {}", part1(numbers.clone(), boards.clone()));
    println!("part2: {}", part2(numbers, boards));
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    fn get_test_input() -> (Vec<u8>, Vec<super::Board>) {
        super::parse_input(TEST_INPUT)
    }

    #[test]
    fn test_part1() {
        let (n, b) = get_test_input();
        assert_eq!(super::part1(n, b), 4512);
    }

    #[test]
    fn test_part2() {
        let (n, b) = get_test_input();
        assert_eq!(super::part2(n, b), 1924);
    }
}
