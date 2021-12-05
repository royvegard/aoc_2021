use std::fs;

#[derive(Default)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let direction = (
            self.p2.x as i32 - self.p1.x as i32,
            self.p2.y as i32 - self.p1.y as i32,
        );
        let steps = i32::max(i32::abs(direction.0), i32::abs(direction.1));
        for i in 0..=steps {
            points.push(Point {
                x: (self.p1.x as i32 + i * direction.0.signum()) as usize,
                y: (self.p1.y as i32 + i * direction.1.signum()) as usize,
            });
        }
        points
    }
}

pub fn part_1(inp: String) -> usize {
    let segments = parse_input(inp);
    let mut max_x = 0;
    let mut max_y = 0;

    for ls in &segments {
        if ls.p1.x > max_x {
            max_x = ls.p1.x
        }
        if ls.p2.x > max_x {
            max_x = ls.p2.x
        }
        if ls.p1.y > max_y {
            max_y = ls.p1.y
        }
        if ls.p2.y > max_y {
            max_y = ls.p2.y
        }
    }

    let mut ocean_floor: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for ls in &segments {
        if ls.is_horizontal() || ls.is_vertical() {
            for p in ls.get_points() {
                ocean_floor[p.y][p.x] += 1;
            }
        }
    }

    let mut overlapping_points = 0;

    for r in &ocean_floor {
        for p in r {
            if *p > 1 {
                overlapping_points += 1;
            }
        }
    }

    overlapping_points
}

pub fn part_2(inp: String) -> usize {
    let segments = parse_input(inp);
    let mut max_x = 0;
    let mut max_y = 0;

    for ls in &segments {
        if ls.p1.x > max_x {
            max_x = ls.p1.x
        }
        if ls.p2.x > max_x {
            max_x = ls.p2.x
        }
        if ls.p1.y > max_y {
            max_y = ls.p1.y
        }
        if ls.p2.y > max_y {
            max_y = ls.p2.y
        }
    }

    let mut ocean_floor: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for ls in &segments {
        for p in ls.get_points() {
            ocean_floor[p.y][p.x] += 1;
        }
    }

    let mut overlapping_points = 0;

    for r in &ocean_floor {
        for p in r {
            if *p > 1 {
                overlapping_points += 1;
            }
        }
    }

    overlapping_points
}

fn parse_input(inp: String) -> Vec<LineSegment> {
    let input = fs::read_to_string(inp).expect("Something went wrong");
    let mut segments: Vec<LineSegment> = Vec::new();

    for l in input.lines() {
        let (p1, p2) = l.split_once("->").unwrap();
        let (x1, y1) = p1.trim().split_once(',').unwrap();
        let (x2, y2) = p2.trim().split_once(',').unwrap();

        let segment: LineSegment = LineSegment {
            p1: Point {
                x: x1.parse().unwrap(),
                y: y1.parse().unwrap(),
            },
            p2: Point {
                x: x2.parse().unwrap(),
                y: y2.parse().unwrap(),
            },
        };

        segments.push(segment);
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_05_test_1");
        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_05_test_1");
        assert_eq!(part_2(input), 12);
    }
}
