use std::fs;

pub fn part_1(inp: String) -> i32 {
    let crab_positions = parse_input(inp);
    let pos = median(&crab_positions);

    i32::min(
        fuel_1(&crab_positions, pos),
        fuel_1(&crab_positions, pos + 1),
    )
}

pub fn part_2(inp: String) -> i32 {
    let crab_positions = parse_input(inp);
    let pos = mean(&crab_positions);

    i32::min(
        fuel_2(&crab_positions, pos),
        fuel_2(&crab_positions, pos + 1),
    )
}

fn fuel_1(pos: &[i32], point: i32) -> i32 {
    let mut fuel_sum = 0;

    for c in pos {
        fuel_sum += i32::abs(c - point);
    }

    fuel_sum
}

fn fuel_2(pos: &[i32], point: i32) -> i32 {
    let mut fuel_sum = 0;

    for c in pos {
        let dist = i32::abs(c - point);
        fuel_sum += dist * (dist + 1) / 2;
    }

    fuel_sum
}

fn median(inp: &[i32]) -> i32 {
    let mut pos = inp.to_owned();
    pos.sort_unstable();
    pos[pos.len() / 2]
}

fn mean(inp: &[i32]) -> i32 {
    inp.iter().sum::<i32>() / inp.len() as i32
}

fn parse_input(inp: String) -> Vec<i32> {
    let input = fs::read_to_string(inp).expect("Something went wrong");
    let crab = input
        .split(',')
        .map(|pos| pos.parse::<i32>().unwrap())
        .collect();

    crab
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_07_test_1");
        assert_eq!(part_1(input), 37);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_07_test_1");
        assert_eq!(part_2(input), 168);
    }

    #[test]
    fn test_part_1_test_2() {
        let input = String::from("input/day_07_test_2");
        assert_eq!(part_1(input), 24);
    }
}
