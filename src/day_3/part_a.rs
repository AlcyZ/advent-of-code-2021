use crate::utility::read_lines_as_vec;
use std::path::Path;

fn get_zero_or_one(list: &Vec<char>) -> &'static str {
    let mut zeros = 0;
    let mut ones = 0;

    for element in list.iter() {
        if *element == '0' {
            zeros += 1;
        }
        if *element == '1' {
            ones += 1;
        }
    }

    if zeros > ones {
        "0"
    } else {
        "1"
    }
}

fn gamma_rate_bin_string<P: AsRef<Path>>(input: P) -> String {
    let lines = read_lines_as_vec::<String, _>(input).unwrap();

    let mut collection: Vec<Vec<char>> = vec![];

    for line in lines.iter() {
        for (index, value) in line.chars().enumerate() {
            match collection.get_mut(index) {
                Some(inner_collection) => inner_collection.push(value),
                None => collection.push(vec![value]),
            }
        }
    }

    collection
        .iter()
        .map(|inner| get_zero_or_one(inner))
        .collect::<Vec<&str>>()
        .join("")
}

fn swap_bin_string(value: &str) -> String {
    let mut new_string = String::from("");

    for i in 0..value.len() {
        new_string += match &value[i..i + 1] {
            "0" => "1",
            "1" => "0",
            _ => panic!("should never happen"),
        };
    }

    new_string
}

fn part_a<P: AsRef<Path>>(input: P) -> i32 {
    let gamma_bin_string = gamma_rate_bin_string(input);
    let gamma_decimal = super::bin_str_to_decimal(&gamma_bin_string);

    let epsilon_bin_string = swap_bin_string(&gamma_bin_string);
    let epsilon_decimal = super::bin_str_to_decimal(&epsilon_bin_string);

    gamma_decimal * epsilon_decimal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_with_sample() {
        let result = part_a("./tests/inputs/day_3/sample.txt");

        assert_eq!(198, result);
    }

    #[test]
    fn part_a_with_real() {
        let result = part_a("./tests/inputs/day_3/input.txt");

        assert_eq!(1025636, result);
    }
}
