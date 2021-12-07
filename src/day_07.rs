use std::fs;

pub fn part_1(inp: String) -> i32 {
    let crab_positions = parse_input(inp);
    let med = median(&crab_positions);
    let mut fuel_sum = 0;

    for c in crab_positions {
        fuel_sum += i32::abs(c - med);
    }

    fuel_sum
}

pub fn part_2(inp: String) -> i32 {
    let crab_positions = parse_input(inp);
    let men = mean(&crab_positions) - 1;
    let mut fuel_sum = 0;

    for c in crab_positions {
        let dist = i32::abs(c - men);
        //        for d in 1..=i32::abs(c - men) {
        //            fuel_sum += d;
        //        }
        fuel_sum += dist * (dist + 1) / 2;
    }

    fuel_sum
}

fn median(inp: &Vec<i32>) -> i32 {
    let mut pos = inp.clone();
    pos.sort_unstable();
    pos[pos.len() / 2]
}

fn mean(inp: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in inp {
        sum += *i;
    }
    (sum as f32 / inp.len() as f32).round() as i32
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
}
