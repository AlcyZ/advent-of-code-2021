mod measure;

fn increased_previous_element_count(input: &Vec<i32>) -> i32 {
    let mut increased_counter = 0;
    let mut previous = None;

    for current in input {
        if let Some(previous) = previous {
            if current > previous {
                increased_counter += 1;
            }
        }

        previous = Some(current);
    }

    increased_counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_1::measure::three_measurement;
    use crate::utility::read_lines_as_vec;

    #[test]
    fn larger_than_previous_with_sample() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, increased_previous_element_count(&input));
    }

    #[test]
    fn fetch_and_parse_input_file() {
        let input = read_lines_as_vec::<i32, _>("./tests/inputs/day_1/input.txt").unwrap();

        assert_eq!(1692, increased_previous_element_count(&input));
    }

    #[test]
    fn three_measurement_with_input() {
        let input = read_lines_as_vec::<i32, _>("./tests/inputs/day_1/input.txt").unwrap();

        assert_eq!(1724, three_measurement(&input));
    }
}
