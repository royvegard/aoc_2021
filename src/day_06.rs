use std::collections::HashMap;
use std::fs;

pub fn part_1(inp: String) -> usize {
    let mut fish = parse_input(inp);
    let days = 80;

    for _ in 0..days {
        let mut new_fish = 0;
        for f in &mut fish {
            if *f == 0 {
                *f = 6;
                new_fish += 1;
            } else {
                *f -= 1;
            }
        }

        for _ in 0..new_fish {
            fish.push(8);
        }
    }
    fish.len()
}

pub fn part_2(inp: String) -> usize {
    let initial_fish = parse_input(inp);
    let mut fish: HashMap<usize, usize> = HashMap::new();

    for f in initial_fish {
        *(fish.entry(f as usize).or_default()) += 1;
    }

    let days = 256;

    for _ in 0..days {
        let mut new_fish: HashMap<usize, usize> = HashMap::new();

        for k in fish.keys() {
            if *k == 0 {
                *(new_fish.entry(6).or_default()) += fish[k];
                *(new_fish.entry(8).or_default()) = fish[k];
            } else {
                *(new_fish.entry(k - 1).or_default()) += fish[k];
            }
        }
        fish = new_fish
    }

    let mut sum = 0;
    for v in fish.values() {
        sum += *v;
    }

    sum as usize
}

fn parse_input(inp: String) -> Vec<usize> {
    let input = fs::read_to_string(inp).expect("Something went wrong");
    let fish = input
        .split(',')
        .map(|age| age.parse::<usize>().unwrap())
        .collect();

    fish
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_06_test_1");
        assert_eq!(part_1(input), 5934);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_06_test_1");
        assert_eq!(part_2(input), 26984457539);
    }
}
