use crate::utility::read_lines_as_vec;
use std::path::Path;

#[derive(Debug)]
enum Bit {
    ZERO,
    ONE,
}

enum Rating {
    Oxygen,
    Scrubber,
}

fn bits_at_pos(pos: usize, lines: &Vec<String>) -> (i32, i32) {
    let mut zeros = 0;
    let mut ones = 0;

    for line in lines.iter() {
        let nth = &line[pos..pos + 1];

        match nth {
            "0" => zeros += 1,
            "1" => ones += 1,
            _ => {}
        }
    }

    (zeros, ones)
}

fn nth_bit_most_common(pos: usize, lines: &Vec<String>) -> Bit {
    let (zeros, ones) = bits_at_pos(pos, &lines);

    if ones >= zeros {
        Bit::ONE
    } else {
        Bit::ZERO
    }
}

fn nth_bit_least_common(pos: usize, lines: &Vec<String>) -> Bit {
    let (zeros, ones) = bits_at_pos(pos, &lines);

    if ones < zeros {
        Bit::ONE
    } else {
        Bit::ZERO
    }
}

fn rating_string(kind: Rating, lines: &Vec<String>) -> String {
    let mut pos = 0;
    let mut copy = lines.iter().map(|l| l.clone()).collect::<Vec<String>>();

    while copy.len() > 1 {
        let nth_bit = match kind {
            Rating::Oxygen => nth_bit_most_common(pos, &copy),
            Rating::Scrubber => nth_bit_least_common(pos, &copy),
        };

        copy = copy
            .iter()
            .filter(|l| match l.chars().nth(pos) {
                Some(char) => match nth_bit {
                    Bit::ZERO => char == '0',
                    Bit::ONE => char == '1',
                },
                None => false,
            })
            .map(|l| l.clone())
            .collect::<Vec<String>>();

        pos += 1;
    }

    copy.first().unwrap().clone()
}

fn rating(kind: Rating, lines: &Vec<String>) -> i32 {
    super::bin_str_to_decimal(&rating_string(kind, &lines))
}

fn part_b<P: AsRef<Path>>(input: P) -> i32 {
    let lines = read_lines_as_vec::<String, _>(input).unwrap();

    let oxygen_rating = rating(Rating::Oxygen, &lines);
    let scrubber_rating = rating(Rating::Scrubber, &lines);

    oxygen_rating * scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_b_with_sample() {
        let result = part_b("./tests/inputs/day_3/sample.txt");

        assert_eq!(230, result);
    }

    #[test]
    fn part_b_with_real() {
        let result = part_b("./tests/inputs/day_3/input.txt");

        assert_eq!(793873, result);
    }
}
