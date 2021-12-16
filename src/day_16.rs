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

fn bits_to_num(bits: &[bool]) -> usize {
    bits.iter().fold(
        0,
        |n, &b| (n << 1) + if b { 1 } else { 0 }
    )
}

fn process_packet(input: &[bool], mut pointer: usize) -> Option<(usize, usize, usize)> {
    if pointer + 6 > input.len() - 1 {
        None
    } else {
        // read 3 bits and produce version
        let v = bits_to_num(&input[pointer..pointer + 3]);
        let t = bits_to_num(&input[pointer + 3..pointer + 6]);
        pointer += 6;
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
                Some((pointer, v, data))
            },
            _ => {
                // read sub packets
                if pointer > input.len() - 1 {
                    return None;
                }
                let mut version_sums = v;
                let mut results = vec![];
                if input[pointer] {
                    // 11 bit - number of packets
                    if pointer + 12 > input.len() - 1 {
                        return None;
                    }
                    let num_packets = bits_to_num(&input[pointer + 1..pointer + 12]);
                    pointer += 12;
                    for _ in 0..num_packets {
                        match process_packet(input, pointer) {
                            Some((npointer, vsums, result)) => {
                                pointer = npointer;
                                version_sums += vsums;
                                results.push(result);
                            },
                            None => panic!("unexpected eol in op")
                        }
                    }
                } else {
                    // 15 bit - total length of packets
                    if pointer + 16 > input.len() - 1 {
                        return None;
                    }
                    let len = bits_to_num(&input[pointer + 1..pointer + 16]);
                    pointer += 16;
                    let end = pointer + len;
                    loop {
                        if pointer + 6 > end {
                            pointer = pointer.max(end);
                            break;
                        }
                        match process_packet(input, pointer) {
                            Some((npointer, vsums, result)) => {
                                pointer = npointer;
                                version_sums += vsums;
                                results.push(result);
                            },
                            None => panic!("unexpected eol in op")
                        }
                    }
                }
                Some((pointer, version_sums, match t {
                    0 => results.into_iter().sum(),
                    1 => results.into_iter().product(),
                    2 => results.into_iter().min().unwrap(),
                    3 => results.into_iter().max().unwrap(),
                    5 => if results[0] > results[1] { 1 } else { 0 },
                    6 => if results[0] < results[1] { 1 } else { 0 },
                    7 => if results[0] == results[1] { 1 } else { 0 },
                    _ => panic!("unknown op")
                }))
            }
        }
    }
}

fn main() {
    adventofcode2021::print_time!({
        let input = parse_input(include_str!("../day_16_input.txt"));
        let res = process_packet(&input, 0).unwrap();
        println!("part1: {}", res.1);
        println!("part2: {}", res.2);
    });
}

#[cfg(test)]
mod test {

    use super::{parse_input, process_packet};

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

    fn part1(input: &[bool]) -> usize {
        process_packet(input, 0).unwrap().1
    }

    fn part2(input: &[bool]) -> usize {
        process_packet(input, 0).unwrap().2
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 16);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 12);
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 23);
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), 31);
        assert_eq!(part1(&parse_input(TEST_INPUT_5)), 14);
        assert_eq!(part1(&parse_input(TEST_INPUT_6)), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input("C200B40A82")), 3);
        assert_eq!(part2(&parse_input("04005AC33890")), 54);
        assert_eq!(part2(&parse_input("880086C3E88112")), 7);
        assert_eq!(part2(&parse_input("CE00C43D881120")), 9);
        assert_eq!(part2(&parse_input("D8005AC2A8F0")), 1);
        assert_eq!(part2(&parse_input("F600BC2D8F")), 0);
        assert_eq!(part2(&parse_input("9C005AC2F8F0")), 0);
        assert_eq!(part2(&parse_input("9C0141080250320F1802104A08")), 1);
    }
}
