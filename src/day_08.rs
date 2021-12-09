use std::collections::HashMap;
use std::fs;

#[derive(Default)]
struct SegmentNumber {
    number: i32,
    segments: String,
}
pub fn part_1(inp: String) -> i32 {
    let inp = parse_input(inp);
    let mut instances = 0;

    for l in inp {
        let (_, output) = l.split_once('|').unwrap();
        let output = output.split_whitespace().collect::<Vec<&str>>();

        for digit in output {
            match digit.len() {
                2 | 3 | 4 | 7 => instances += 1,
                _ => (),
            }
        }
    }
    instances
}

fn parse_input(inp: String) -> Vec<String> {
    let input = fs::read_to_string(inp)
        .expect("Something went wrong")
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();

    input
}

fn get_out_digit(inp: &str) -> i32 {
    let (input, output) = inp.split_once('|').unwrap();
    let mut segment_numbers: Vec<SegmentNumber> = Vec::new();
    let mut number_map: [&str; 10] = [""; 10];

    for digit in input.split_whitespace().collect::<Vec<&str>>() {
        match digit.len() {
            2 => {
                segment_numbers.push(SegmentNumber {
                    number: 1,
                    segments: digit.trim().to_owned(),
                });
                number_map[1] = digit.trim();
            }
            3 => segment_numbers.push(SegmentNumber {
                number: 7,
                segments: digit.trim().to_owned(),
            }),
            4 => segment_numbers.push(SegmentNumber {
                number: 4,
                segments: digit.trim().to_owned(),
            }),
            7 => segment_numbers.push(SegmentNumber {
                number: 8,
                segments: digit.trim().to_owned(),
            }),
            _ => (),
        }
    }
    dbg!(input);
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_08_test_1");
        assert_eq!(part_1(input), 26);
    }

    #[test]
    fn test_line_1() {
        let input = String::from("input/day_08_test_1");
        let inp = parse_input(input);
        assert_eq!(get_out_digit(&inp[0]), 8394);
    }
}
