fn parse_input(input: &str) -> Vec<bool> {
    input.chars().filter_map(|c| {
        let num = match c {
            '0'..='9' => {
                Some(c as u8 - 48)
            },
            'A'..='F' => {
                Some(c as u8 - 55)
            },
            _ => None
        };
        num.map(|n| {
            vec![
                n & 0b00001000 > 0,
                n & 0b00000100 > 0,
                n & 0b00000010 > 0,
                n & 0b00000001 > 0,
            ]
        })
    }).flatten().collect()
}

#[derive(Debug)]
enum ParserState {
    Start,
    Header(usize, usize),
    Operator(usize),
}

#[derive(Debug)]
enum SubPacket {
    Length(usize),
    Packets(usize)
}

fn bits_to_num(bits: &[bool]) -> usize {
    bits.iter().fold(
        0,
        |n, &b| (n << 1) + if b { 1 } else { 0 }
    )
}

fn part1(input: &[bool]) -> usize {
    let mut parser_state = ParserState::Start;
    let mut pointer = 0;
    let mut version_sums = 0;
    let mut stack = vec![];
    loop {
        match parser_state {
            ParserState::Start => {
                // check end conditions
                if pointer + 6 > input.len() - 1 {
                    break;
                }
                // pop stack
                loop {
                    match stack.pop() {
                        Some(SubPacket::Packets(n)) => {
                            if n > 0 {
                                stack.push(SubPacket::Packets(n - 1));
                                break;
                            }
                        },
                        Some(SubPacket::Length(end)) => {
                            if pointer + 6 > end - 1 {
                                pointer = pointer.max(end);
                            } else {
                                stack.push(SubPacket::Length(end));
                                break;
                            }
                        },
                        None => { break; }
                    }
                }
                // read 3 bits and produce version
                let v = bits_to_num(&input[pointer..pointer + 3]);
                let t = bits_to_num(&input[pointer + 3..pointer + 6]);
                parser_state = ParserState::Header(v, t);
                pointer += 6;
            },
            ParserState::Header(v, t) => {
                version_sums += v;
                match t {
                    4 => {
                        // data
                        let mut data = 0usize;
                        loop {
                            let part = bits_to_num(&input[pointer + 1..pointer + 5]);
                            data = (data << 4) + part;
                            pointer += 5;
                            if !input[pointer - 5] {
                                break;
                            }
                        }
                        //dbg!(data);
                        parser_state = ParserState::Start;
                    },
                    _ => {
                        parser_state = ParserState::Operator(t)
                    }
                }
            },
            ParserState::Operator(_) => {
                if pointer > input.len() - 1 {
                    break;
                }
                if input[pointer] {
                    // 11 bit - number of packets
                    if pointer + 12 > input.len() - 1 {
                        break;
                    }
                    let num_packets = bits_to_num(&input[pointer + 1..pointer + 12]);
                    pointer += 12;
                    stack.push(SubPacket::Packets(num_packets));
                } else {
                    // 15 bit - total length of packets
                    if pointer + 16 > input.len() - 1 {
                        break;
                    }
                    let len = bits_to_num(&input[pointer + 1..pointer + 16]);
                    pointer += 16;
                    stack.push(SubPacket::Length(pointer + len));
                }
                parser_state = ParserState::Start;
            }
        }
    }
    version_sums
}

fn part2(_input: &[bool]) -> usize {
    0
}


fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_16_input.txt"));
        println!("part1: {}", part1(&input));
        println!("part2: {}", part2(&input));
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, part1, part2};

    const TEST_INPUT_1: &str = r#"8A004A801A8002F478
"#;
    const TEST_INPUT_2: &str = r#"620080001611562C8802118E34
"#;
    const TEST_INPUT_3: &str = r#"C0015000016115A2E0802F182340
"#;
    const TEST_INPUT_4: &str = r#"A0016C880162017C3686B18A3D4780
"#;

    const TEST_INPUT_5: &str = "EE00D40C823060";
    const TEST_INPUT_6: &str = "38006F45291200";

    #[test]
    fn test_part1() {
        use super::bits_to_num;
        assert_eq!(
            bits_to_num(&[false, false, false, false, false, false, false, false, false, false, true, true, false, true, true]),
            27
        );
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 16);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 12);
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 23);
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), 31);
        assert_eq!(part1(&parse_input(TEST_INPUT_5)), 14);
        assert_eq!(part1(&parse_input(TEST_INPUT_6)), 9);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse_input(TEST_INPUT_1)), 0);
    // }
}
