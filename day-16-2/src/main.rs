use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("The result is {}", analyze_packet("res/input.txt"));
}

fn analyze_packet(path: &str) -> u64 {
    parse_packet(&hex_to_binary_str(read_input(path))).1
}

fn parse_packet(packet: &str) -> (usize, u64) {
    match &packet[3..6] {
        "100" => parse_literal(packet),
        _ => parse_operator(packet),
    }
}

fn version_number(packet: &str) -> u64 {
    u64::from_str_radix(packet.chars().take(3).collect::<String>().as_str(), 2).unwrap()
}

fn parse_operator(packet: &str) -> (usize, u64) {
    const HEADER_SIZE: usize = 7;
    const LEN_TYPE_INDEX: usize = 6;
    const BIT_LEN_FIELD_SIZE: usize = 15;
    const CHILD_COUNT_FIELD_SIZE: usize = 11;

    let mut size = HEADER_SIZE;
    let mut children: Vec<u64> = Vec::new();

    match packet.chars().nth(LEN_TYPE_INDEX) {
        Some('0') => {
            println!(
                "{}",
                packet
                    .chars()
                    .skip(HEADER_SIZE)
                    .take(BIT_LEN_FIELD_SIZE)
                    .collect::<String>()
            );
            let payload_size: usize = usize::from_str_radix(
                packet
                    .chars()
                    .skip(HEADER_SIZE)
                    .take(BIT_LEN_FIELD_SIZE)
                    .collect::<String>()
                    .as_str(),
                2,
            )
            .unwrap();
            println!("length type id: '0', payload bits: {}", payload_size);
            size += 15;

            while size < payload_size + 22 {
                let (child_size, result) = parse_packet(&packet[size..]);
                size += child_size;
                children.push(result)
            }
        }
        Some('1') => {
            println!(
                "{}",
                packet
                    .chars()
                    .skip(HEADER_SIZE)
                    .take(CHILD_COUNT_FIELD_SIZE)
                    .collect::<String>()
            );
            let child_count: usize = usize::from_str_radix(
                packet
                    .chars()
                    .skip(HEADER_SIZE)
                    .take(CHILD_COUNT_FIELD_SIZE)
                    .collect::<String>()
                    .as_str(),
                2,
            )
            .unwrap();
            println!("length type id: '1', child packets: {}", child_count);
            size += 11;

            for _ in 0..child_count {
                let (child_size, result) = parse_packet(&packet[size..]);
                size += child_size;
                children.push(result);
            }
        }
        _ => panic!("Error parsing length type ID"),
    }

    let result: u64 = match &packet[3..6] {
        "000" => children.iter().sum(),
        "001" => children.iter().product(),
        "010" => *children.iter().min().unwrap(),
        "011" => *children.iter().max().unwrap(),
        "101" => {
            if children[0] > children[1] {
                1
            } else {
                0
            }
        }
        "110" => {
            if children[0] < children[1] {
                1
            } else {
                0
            }
        }
        "111" => {
            if children[0] == children[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Invalid type ID"),
    };
    (size, result)
}

fn parse_literal(packet: &str) -> (usize, u64) {
    let mut payload = String::new();
    let mut j = 6;
    loop {
        payload.push_str(
            packet
                .chars()
                .skip(j + 1)
                .take(4)
                .collect::<String>()
                .as_str(),
        );
        match packet.chars().nth(j) {
            Some('0') => break,
            Some('1') => j += 5,
            _ => panic!("Invalid character, payload should be terminated with 0 bit"),
        }
    }
    println!(
        "Literal => size: {}, sum: {}",
        j + 5,
        version_number(packet)
    );
    (j + 5, u64::from_str_radix(&payload, 2).unwrap()) // TODO change this
}

fn hex_to_binary_str(hex_string: String) -> String {
    hex_string
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("illegal character in input sequence"),
        })
        .collect::<String>()
}

fn read_input(path: &str) -> String {
    let file = File::open(path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    lines.next().unwrap().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse() {
        assert_eq!(
            String::from("110100101111111000101000"),
            hex_to_binary_str(read_input("res/test-input.txt"))
        )
    }

    #[rstest]
    #[case("C200B40A82", 3)]
    #[case("04005AC33890", 54)]
    #[case("880086C3E88112", 7)]
    #[case("CE00C43D881120", 9)]
    #[case("D8005AC2A8F0", 1)]
    #[case("F600BC2D8F", 0)]
    #[case("9C005AC2F8F0", 0)]
    #[case("9C0141080250320F1802104A08", 1)]
    fn test_complex_cases(#[case] packet_hex: String, #[case] sum: u32) {
        assert_eq!(sum, parse_packet(&hex_to_binary_str(packet_hex)).1)
    }
}
