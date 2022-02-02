pub fn parse_data(str: &str) -> Vec<(Direction, u32)> {
    let lines: Vec<(&str, &str)> = str.lines().map(|x| x.split_once(" ").unwrap()).collect();
    lines
        .iter()
        .map(|x| (x.0.into(), x.1.parse().unwrap()))
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Down,
    Forward,
    Up,
}

impl From<&str> for Direction {
    fn from(str: &str) -> Self {
        match str {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction."),
        }
    }
}

pub fn solve_part_one(str: &str) -> u32 {
    let course_data = parse_data(str);

    let mut horizontal_position = 0;
    let mut depth = 0;

    for instruction in course_data {
        if instruction.0 == Direction::Forward {
            horizontal_position += instruction.1;
        } else if instruction.0 == Direction::Up {
            depth -= instruction.1;
        } else {
            depth += instruction.1;
        }
    }

    depth * horizontal_position
}

pub fn solve_part_two(str: &str) -> u32 {
    let course_data = parse_data(str);

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in course_data {
        if instruction.0 == Direction::Forward {
            horizontal_position += instruction.1;
            depth += instruction.1 * aim;
        } else if instruction.0 == Direction::Up {
            aim -= instruction.1;
        } else {
            aim += instruction.1;
        }
    }

    depth * horizontal_position
}

fn main() {
    println!(
        "{}",
        solve_part_one(include_str!("./../inputs/puzzle_data.txt"))
    );
    println!(
        "{}",
        solve_part_two(include_str!("./../inputs/puzzle_data.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            150,
            solve_part_one(include_str!("./../inputs/test_data.txt"))
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            900,
            solve_part_two(include_str!("./../inputs/test_data.txt"))
        );
    }
}
