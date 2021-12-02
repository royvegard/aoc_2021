use std::fs;

pub fn part_1(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut horizontal = 0;
    let mut depth = 0;

    for l in input {
        let mut instruction = l.split_whitespace();
        let direction = instruction.next().unwrap();
        let length = instruction.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal += length,
            "down" => depth += length,
            "up" => depth -= length,
            _ => {}
        }
    }
    horizontal * depth
}

pub fn part_2(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for l in input {
        let mut instruction = l.split_whitespace();
        let direction = instruction.next().unwrap();
        let length = instruction.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                horizontal += length;
                depth += aim * length;
            }
            "down" => aim += length,
            "up" => aim -= length,
            _ => {}
        }
    }
    horizontal * depth
}

fn parse_input(inp: String) -> Vec<String> {
    let output = fs::read_to_string(inp)
        .expect("Something went wrong")
        .lines()
        .map(|f| f.to_owned())
        .collect();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_02_test_1");
        assert_eq!(part_1(input), 150);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_02_test_1");
        assert_eq!(part_2(input), 900);
    }
}
