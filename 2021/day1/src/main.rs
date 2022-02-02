pub fn parse_data(str: &str) -> Vec<u32> {
    str.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

pub fn solve_part_one(str: &str) -> u32 {
    let numbers = parse_data(str);

    let mut increase_count = 0;

    for window in numbers.windows(2) {
        if window[1] > window[0] {
            increase_count += 1;
        }
    }

    increase_count
}

pub fn solve_part_two(str: &str) -> u32 {
    let numbers = parse_data(str);

    let mut increase_count = 0;

    for window in numbers.windows(4) {
        if window[3] + window[2] + window[1] > window[2] + window[1] + window[0] {
            increase_count += 1;
        }
    }

    increase_count
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
        assert_eq!(7, solve_part_one(include_str!("./../inputs/test_data.txt")));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(5, solve_part_two(include_str!("./../inputs/test_data.txt")));
    }
}
