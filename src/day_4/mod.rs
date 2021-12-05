use crate::day_4::board::BingoBoard;
use crate::utility::read_lines;
use std::path::Path;

mod board;

#[derive(Debug)]
struct Game {
    numbers: Vec<i32>,
    boards: Vec<BingoBoard>,
}

impl Game {
    fn from_input<P: AsRef<Path>>(input: P) -> Game {
        let mut lines = read_lines(input).unwrap();
        let numbers_string = lines.next().unwrap().unwrap();
        let numbers = numbers_string
            .split(",")
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut boards = vec![];

        let mut next_line = lines.next();
        while next_line.is_some() {
            let mut line = next_line.unwrap().unwrap();
            let mut fields = vec![];

            while !line.is_empty() {
                fields.append(&mut line_to_vec(line));

                line = match lines.next() {
                    Some(line) => line.unwrap(),
                    None => break,
                }
            }

            if fields.len() == 25 {
                boards.push(BingoBoard::new(fields));
            }

            next_line = lines.next();
        }

        Game { numbers, boards }
    }

    fn boards_in_game(&self) -> usize {
        self.boards
            .iter()
            .filter(|b| !b.has_won())
            .count()
    }

    fn play_bingo(&mut self) -> i32 {
        for number in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                board.bingo_move(*number);

                if board.has_won() {
                    return board.sum_open_fields() * *number;
                }
            }
        }

        panic!("Nobody won the bingo game");
    }

    fn last_winning_score(&mut self) -> i32 {
        for number in self.numbers.iter() {
            let boards_in_game = self.boards_in_game();
            for board in self.boards.iter_mut() {
                let has_won_before_move = board.has_won();
                board.bingo_move(*number);

                if !has_won_before_move && boards_in_game == 1 && board.has_won() {
                    return board.sum_open_fields() * *number;
                }
            }
        }

        panic!("Nobody won the bingo game");
    }
}

#[derive(Debug)]
enum Field {
    Hit,
    Open(i32),
}

impl Field {
    fn is_open(&self) -> bool {
        match self {
            Field::Hit => false,
            Field::Open(_) => true,
        }
    }
}

fn line_to_vec(line: String) -> Vec<Field> {
    line.split_whitespace()
        .map(|v| Field::Open(v.parse::<i32>().unwrap()))
        .collect()
}

fn part_a<P: AsRef<Path>>(input: P) -> i32 {
    let mut game = Game::from_input(input);

    game.play_bingo()
}

fn part_b<P: AsRef<Path>>(input: P) -> i32 {
    let mut game = Game::from_input(input);

    game.last_winning_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_with_sample() {
        let result = part_a("./tests/inputs/day_4/sample.txt");

        assert_eq!(4512, result);
    }

    #[test]
    fn part_a_with_real() {
        let result = part_a("./tests/inputs/day_4/input.txt");

        assert_eq!(89001, result);
    }

    #[test]
    fn part_b_with_sample() {
        let result = part_b("./tests/inputs/day_4/sample.txt");

        assert_eq!(1924, result);
    }

    #[test]
    fn part_b_with_real() {
        let result = part_b("./tests/inputs/day_4/input.txt");

        assert_eq!(7296, result);
    }
}
