use std::fs;

// TODO: should try to solve this in the "binary domain"
// instead of in the "String domain"

pub fn part_1(inp: String) -> i32 {
    let input = parse_input(inp);
    let bit_length = input[0].chars().count();
    let total_length = input.len();
    let mut bit_sums: Vec<usize> = Vec::new();
    bit_sums.resize_with(bit_length, || 0);

    for l in input {
        let bits = l
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for (i, b) in bits.iter().enumerate() {
            bit_sums[i] += b;
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for b in bit_sums {
        if b > (total_length / 2) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_rate = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma_rate * epsilon_rate
}

pub fn part_2(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut column = 0;
    let mut trimmed = input.clone();

    loop {
        let most_common = get_most_common(&trimmed, column);
        trimmed = trim(&trimmed, most_common as char, column);
        column += 1;

        if trimmed.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = i32::from_str_radix(&trimmed[0], 2).unwrap();

    let mut trimmed = input;
    column = 0;
    loop {
        let mut most_common = get_most_common(&trimmed, column);
        if most_common == '1' {
            most_common = '0';
        } else {
            most_common = '1';
        }
        trimmed = trim(&trimmed, most_common as char, column);
        column += 1;

        if trimmed.len() == 1 {
            break;
        }
    }

    let co2_generator_rating = i32::from_str_radix(&trimmed[0], 2).unwrap();

    oxygen_generator_rating * co2_generator_rating
}

fn parse_input(inp: String) -> Vec<String> {
    let output = fs::read_to_string(inp)
        .expect("Something went wrong")
        .lines()
        .map(|f| f.to_owned())
        .collect();

    output
}

fn get_most_common(inp: &[String], column: usize) -> char {
    let mut zeros = 0;
    let mut ones = 0;

    for l in inp {
        match l.as_bytes()[column] {
            48 => zeros += 1,
            49 => ones += 1,
            _ => panic!("Whyyyy!"),
        }
    }

    if zeros > ones {
        '0'
    } else {
        '1'
    }
}

fn trim(inp: &[String], bit: char, column: usize) -> Vec<String> {
    inp.iter()
        .filter(|l| l.as_bytes()[column] == bit as u8)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_03_test_1");
        assert_eq!(part_1(input), 198);
    }

    #[test]
    fn test_most_common_0() {
        let input = String::from("input/day_03_test_1");
        let input = parse_input(input);
        assert_eq!(get_most_common(&input, 0), '1');
    }

    #[test]
    fn test_most_common_1() {
        let input = String::from("input/day_03_test_1");
        let input = parse_input(input);
        assert_eq!(get_most_common(&input, 1), '0');
    }

    #[test]
    fn test_most_common_3() {
        let input = String::from("input/day_03_test_1");
        let input = parse_input(input);
        assert_eq!(get_most_common(&input, 3), '1');
    }

    #[test]
    fn test_trim_0() {
        let expected = vec![
            "11110", "10110", "10111", "10101", "11100", "10000", "11001",
        ];
        let input = String::from("input/day_03_test_1");
        let input = parse_input(input);
        assert_eq!(trim(&input, '1', 0), expected);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_03_test_1");
        assert_eq!(part_2(input), 230);
    }
}
