use std::fs;

pub fn part_1(inp: String) -> i32 {
    let height_map = parse_input(inp);

    let map_width = height_map[0].len();
    let map_height = height_map.len();
    let mut low_point: Vec<i32> = Vec::new();
    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            let mut adjacent = Vec::new();
            if x > 0 {
                adjacent.push(height_map[y][x - 1]);
            }
            if x < map_width - 1 {
                adjacent.push(height_map[y][x + 1]);
            }
            if y > 0 {
                adjacent.push(height_map[y - 1][x]);
            }
            if y < map_height - 1 {
                adjacent.push(height_map[y + 1][x]);
            }

            adjacent.sort_unstable();
            if height_map[y][x] < adjacent[0] {
                low_point.push(height_map[y][x]);
            }
        }
    }

    low_point.iter().map(|n| n + 1).sum::<i32>()
}

pub fn part_2(inp: String) -> i32 {
    let cave_map = Map {
        height_map: parse_input(inp),
    };

    let low_points = cave_map.get_low_points();
    let mut basin_size: Vec<i32> = Vec::new();
    let mut id = 0;

    for lp in low_points {
        let mut stack = vec![lp];
        let mut visited: Vec<Point> = Vec::new();
        basin_size.push(0);

        while !stack.is_empty() {
            let current_point = stack.pop().unwrap();

            if visited.contains(&current_point) {
                continue;
            }

            let adjacent = cave_map.get_adjacent(&current_point);

            for p in adjacent {
                if cave_map.get_height(&p) < 9 {
                    stack.push(p);
                }
            }
            visited.push(current_point);
            basin_size[id] += 1;
        }

        id += 1;
    }

    basin_size.sort_unstable();

    let mut product = 1;
    for s in &basin_size[(basin_size.len() - 3)..basin_size.len()] {
        product *= s;
    }

    product
}

#[derive(PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    height_map: Vec<Vec<i32>>,
}

impl Map {
    fn get_adjacent(&self, point: &Point) -> Vec<Point> {
        let map_width = self.height_map[0].len();
        let map_height = self.height_map.len();
        let mut adjacent = Vec::new();

        if point.x > 0 {
            adjacent.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.x < map_width - 1 {
            adjacent.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y > 0 {
            adjacent.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.y < map_height - 1 {
            adjacent.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }
        adjacent
    }

    fn get_low_points(&self) -> Vec<Point> {
        let mut low_point: Vec<Point> = Vec::new();

        for y in 0..self.height_map.len() {
            for x in 0..self.height_map[y].len() {
                let mut heights = self
                    .get_adjacent(&Point { x, y })
                    .iter()
                    .map(|p| self.get_height(p))
                    .collect::<Vec<i32>>();

                heights.sort_unstable();
                if self.get_height(&Point { x, y }) < heights[0] {
                    low_point.push(Point { x, y });
                }
            }
        }

        low_point
    }

    fn get_height(&self, point: &Point) -> i32 {
        self.height_map[point.y][point.x]
    }
}

fn parse_input(inp: String) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(inp)
        .expect("Something went wrong")
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();

    let mut output: Vec<Vec<i32>> = Vec::new();
    for l in input {
        output.push(
            l.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_09_test_1");
        assert_eq!(part_1(input), 15);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_09_test_1");
        assert_eq!(part_2(input), 1134);
    }
}
