struct BrokenShip {
    horizontal: i32,
    depth: i32,
}

impl BrokenShip {
    fn new() -> BrokenShip {
        BrokenShip {
            horizontal: 0,
            depth: 0,
        }
    }

    fn execute(&mut self, command: NavigationCommand) {
        match command {
            NavigationCommand::Forward(value) => self.horizontal += value,
            NavigationCommand::Up(value) => self.depth -= value,
            NavigationCommand::Down(value) => self.depth += value,
        }
    }
}

struct Ship {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn execute(&mut self, command: NavigationCommand) {
        match command {
            NavigationCommand::Forward(value) => {
                self.horizontal += value;
                self.depth += self.aim * value;
            }
            NavigationCommand::Up(value) => self.aim -= value,
            NavigationCommand::Down(value) => self.aim += value,
        }
    }
}

#[derive(Debug)]
enum NavigationCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl NavigationCommand {
    fn from_line(line: String) -> Result<NavigationCommand, FooError> {
        let mut iter = line.split_whitespace();
        let command = iter.next().ok_or(FooError)?;
        let value = iter.next().unwrap().parse::<i32>().map_err(|_| FooError)?;

        match command {
            "forward" => Ok(NavigationCommand::Forward(value)),
            "down" => Ok(NavigationCommand::Down(value)),
            "up" => Ok(NavigationCommand::Up(value)),
            _ => Err(FooError),
        }
    }
}

#[derive(Debug)]
struct FooError;

fn get_broken_navigation_result(commands: Vec<NavigationCommand>) -> i32 {
    let mut ship = BrokenShip::new();

    for command in commands {
        ship.execute(command);
    }

    ship.horizontal * ship.depth
}

fn get_navigation_result(commands: Vec<NavigationCommand>) -> i32 {
    let mut ship = Ship::new();

    for command in commands {
        ship.execute(command);
    }

    ship.horizontal * ship.depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utility::read_lines;
    use std::path::Path;

    fn map_input_to_commands<P: AsRef<Path>>(input: P) -> Vec<NavigationCommand> {
        let lines = read_lines(input).unwrap();

        lines
            .filter_map(|l| {
                if let Ok(line) = l {
                    return NavigationCommand::from_line(line).ok();
                }

                None
            })
            .collect()
    }

    #[test]
    fn broken_navigation_with_sample_input() {
        let commands = map_input_to_commands("./tests/inputs/day_2/sample.txt");

        assert_eq!(150, get_broken_navigation_result(commands));
    }

    #[test]
    fn broken_navigation_with_real_input() {
        let commands = map_input_to_commands("./tests/inputs/day_2/input.txt");

        assert_eq!(1804520, get_broken_navigation_result(commands));
    }

    #[test]
    fn navigation_with_sample_input() {
        let commands = map_input_to_commands("./tests/inputs/day_2/sample.txt");

        assert_eq!(900, get_navigation_result(commands));
    }

    #[test]
    fn navigation_with_real_input() {
        let commands = map_input_to_commands("./tests/inputs/day_2/input.txt");

        assert_eq!(1971095320, get_navigation_result(commands));
    }
}
