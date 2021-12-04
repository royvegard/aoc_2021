use std::fs;

#[derive(Default)]
struct BingoBoard {
    board: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
    has_bingo: bool,
}

impl BingoBoard {
    fn from_str(inp: &str) -> Self {
        let mut board = BingoBoard::default();
        for l in inp.lines() {
            board.board.push(
                l.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            );
            board.marked.push(vec![false; l.split_whitespace().count()]);
        }

        board
    }

    fn play_number(&mut self, number: usize) {
        for (row_number, row) in self.board.iter().enumerate() {
            for (column_number, value) in row.iter().enumerate() {
                if *value == number {
                    self.marked[row_number][column_number] = true;
                    return;
                }
            }
        }
    }

    fn check_board(&mut self) -> bool {
        for row in &self.marked {
            if row.iter().filter(|&m| *m).count() == row.len() {
                self.has_bingo = true;
                return true;
            }
        }

        for c in 0..self.marked.len() {
            let mut col_sum = 0;
            for row in &self.marked {
                if row[c] {
                    col_sum += 1;
                }
            }
            if col_sum == self.marked.len() {
                self.has_bingo = true;
                return true;
            }
        }

        false
    }

    fn get_score(&self) -> usize {
        let mut score = 0;

        for r in 0..self.marked.len() {
            for c in 0..self.marked[0].len() {
                if !&self.marked[r][c] {
                    score += &self.board[r][c];
                }
            }
        }
        score
    }
}

#[derive(Default)]
struct BingoGame {
    numbers: Vec<usize>,
    players: Vec<BingoBoard>,
}

pub fn part_1(inp: String) -> usize {
    let mut game = parse_input(inp);

    for number in game.numbers {
        for player in game.players.iter_mut() {
            player.play_number(number);
            if player.check_board() {
                return number * player.get_score();
            }
        }
    }

    1
}

pub fn part_2(inp: String) -> usize {
    let mut game = parse_input(inp);

    let mut last_score = 0;

    for number in game.numbers {
        for player in game.players.iter_mut() {
            if !player.has_bingo {
                player.play_number(number);
                if player.check_board() {
                    last_score = number * player.get_score();
                }
            }
        }
    }

    last_score
}

fn parse_input(inp: String) -> BingoGame {
    let input = fs::read_to_string(inp).expect("Something went wrong");
    let mut game = BingoGame::default();

    let (numbers, players) = input.split_once("\n\n").unwrap();
    game.numbers = numbers
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let players = players.split("\n\n").collect::<Vec<&str>>();

    for p in players {
        game.players.push(BingoBoard::from_str(p));
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_numbers() {
        let input = String::from("input/day_04_test_1");
        let game = parse_input(input);
        let expected_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        assert_eq!(game.numbers, expected_numbers);
    }

    #[test]
    fn test_part_1() {
        let input = String::from("input/day_04_test_1");
        assert_eq!(part_1(input), 4512);
    }

    #[test]
    fn test_part_2() {
        let input = String::from("input/day_04_test_1");
        assert_eq!(part_2(input), 1924);
    }
}
