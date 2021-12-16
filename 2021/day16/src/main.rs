use malaire_aoc::run;

static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, "D2FE28", 6);
    run(0, solve_1, "38006F45291200", 9);
    run(0, solve_1, "EE00D40C823060", 14);
    run(0, solve_1, "8A004A801A8002F478", 16);
    run(0, solve_1, "620080001611562C8802118E34", 12);
    run(0, solve_1, "C0015000016115A2E0802F182340", 23);
    run(0, solve_1, "A0016C880162017C3686B18A3D4780", 31);
    run(1, solve_1, INPUT_X, 821);

    run(0, solve_2, "D2FE28", 2021);
    run(0, solve_2, "38006F45291200", 1);
    run(0, solve_2, "EE00D40C823060", 3);
    run(0, solve_2, "C200B40A82", 3);
    run(0, solve_2, "04005AC33890", 54);
    run(0, solve_2, "880086C3E88112", 7);
    run(0, solve_2, "CE00C43D881120", 9);
    run(0, solve_2, "D8005AC2A8F0", 1);
    run(0, solve_2, "F600BC2D8F", 0);
    run(0, solve_2, "9C005AC2F8F0", 0);
    run(0, solve_2, "9C0141080250320F1802104A08", 1);
    run(2, solve_2, INPUT_X, 2056021084691);
}

fn solve_1(input: &str) -> usize {
    parse_packet(&parse_input(input)).1
}

fn solve_2(input: &str) -> usize {
    parse_packet(&parse_input(input)).2
}

fn parse_input(input: &str) -> Vec<u8> {
    let mut bits = Vec::new();
    for hex in input.trim().as_bytes() {
        bits.extend_from_slice(match hex {
            b'0' => &[0, 0, 0, 0],
            b'1' => &[0, 0, 0, 1],
            b'2' => &[0, 0, 1, 0],
            b'3' => &[0, 0, 1, 1],
            b'4' => &[0, 1, 0, 0],
            b'5' => &[0, 1, 0, 1],
            b'6' => &[0, 1, 1, 0],
            b'7' => &[0, 1, 1, 1],
            b'8' => &[1, 0, 0, 0],
            b'9' => &[1, 0, 0, 1],
            b'A' => &[1, 0, 1, 0],
            b'B' => &[1, 0, 1, 1],
            b'C' => &[1, 1, 0, 0],
            b'D' => &[1, 1, 0, 1],
            b'E' => &[1, 1, 1, 0],
            b'F' => &[1, 1, 1, 1],
            _ => panic!(),
        });
    }
    bits
}

fn parse_number(bits: &[u8]) -> usize {
    let mut x = 0;
    for &bit in bits {
        x <<= 1;
        x += bit as usize;
    }
    x
}

// Returns `(packet_length, version_sum, eval_result)`.
fn parse_packet(bits: &[u8]) -> (usize, usize, usize) {
    let version = parse_number(&bits[..3]);
    let type_id = parse_number(&bits[3..6]);

    if type_id == 4 {
        let mut result = 0;
        let mut pos = 6;
        let mut is_last = false;

        while !is_last {
            result <<= 4;
            result += parse_number(&bits[pos + 1..pos + 5]);
            is_last = bits[pos] == 0;
            pos += 5;
        }

        (pos, version, result)
    } else {
        let mut version_sum = version;
        let mut sub_results = Vec::new();
        let mut pos;

        if bits[6] == 0 {
            pos = 22;
            let sub_bits = parse_number(&bits[7..pos]);
            while pos < 22 + sub_bits {
                let (sub_len, sub_sum, sub_result) = parse_packet(&bits[pos..]);
                pos += sub_len;
                version_sum += sub_sum;
                sub_results.push(sub_result);
            }
        } else {
            pos = 18;
            let sub_count = parse_number(&bits[7..pos]);
            for _ in 0..sub_count {
                let (sub_len, sub_sum, sub_result) = parse_packet(&bits[pos..]);
                pos += sub_len;
                version_sum += sub_sum;
                sub_results.push(sub_result);
            }
        }

        let result = match type_id {
            0 => sub_results.into_iter().sum(),
            1 => sub_results.into_iter().product(),
            2 => sub_results.into_iter().min().unwrap(),
            3 => sub_results.into_iter().max().unwrap(),
            5 => (sub_results[0] > sub_results[1]) as usize,
            6 => (sub_results[0] < sub_results[1]) as usize,
            7 => (sub_results[0] == sub_results[1]) as usize,
            _ => panic!(),
        };

        (pos, version_sum, result)
    }
}
