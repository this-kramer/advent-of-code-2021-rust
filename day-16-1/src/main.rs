use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("The result is {}", analyze_packet("res/input.txt"));
}

fn analyze_packet(path: &str) -> u32 {
    parse_packet(&hex_to_binary_str(read_input(path))).1
}

fn parse_packet(packet: &str) -> (usize, u32) {
    match (&packet[..3], &packet[3..6]) {
        (version, "100") => parse_literal(packet),
        (version, operator_id) => parse_operator(packet),
    }
}

fn version_number(packet: &str) -> u32 {
    u32::from_str_radix(packet.chars().take(3).collect::<String>().as_str(), 2).unwrap()
}

fn parse_operator(packet: &str) -> (usize, u32) {
    const HEADER_SIZE: usize = 7;
    const LEN_TYPE_INDEX: usize = 6;
    const BIT_LEN_FIELD_SIZE: usize = 15;
    const CHILD_COUNT_FIELD_SIZE: usize = 11;

    let mut sum: u32 = version_number(packet);
    let mut size = HEADER_SIZE;
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
                let (child_size, child_sum) = parse_packet(&packet[size..]);
                size += child_size;
                sum += child_sum;
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
                let (child_size, child_sum) = parse_packet(&packet[size..]);
                size += child_size;
                sum += child_sum;
            }
        }
        _ => panic!("Error parsing length type ID"),
    }
    println!("Operator => size: {}, sum: {}", size, sum);
    (size, sum)
}

fn parse_literal(packet: &str) -> (usize, u32) {
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
    (j + 5, version_number(packet)) // TODO change this
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
    #[case("110100101111111000101000", 6)]
    #[case("00111000000000000110111101000101001010010001001000000000", 1+6+2)]
    #[case("11101110000000001101010000001100100000100011000001100000", 7+4+2+1)]
    #[case("11101110000000001101010000001100100000100011000001100000", 7+4+2+1)]
    fn test_simple_cases(#[case] packet: String, #[case] sum: u32) {
        assert_eq!(sum, parse_packet(&packet).1)
    }

    #[rstest]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_complex_cases(#[case] packet_hex: String, #[case] sum: u32) {
        assert_eq!(sum, parse_packet(&hex_to_binary_str(packet_hex)).1)
    }
}
