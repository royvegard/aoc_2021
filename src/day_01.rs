use std::fs;

pub fn part_1(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut increased_depth = 0;

    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            increased_depth += 1;
        }
    }
    increased_depth
}

pub fn part_2(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut increased_depth = 0;

    for i in 3..input.len() {
        if input[i - 3] < input[i] {
            increased_depth += 1;
        }
    }
    increased_depth
}

fn parse_input(inp: String) -> Vec<u32> {
    fs::read_to_string(inp)
        .expect("Something went wrong")
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_01_test_1");
        assert_eq!(part_1(input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_01_test_1");
        assert_eq!(part_2(input), 5);
    }
}
