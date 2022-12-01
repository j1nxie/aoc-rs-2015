pub fn part_one(input: &str) -> Option<i32> {
    let mut floors = 0;
    for c in input.chars() {
        if String::from(c) == "(" {
            floors += 1;
        } else if String::from(c) == ")" {
            floors -= 1;
        }
    }
    Some(floors)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    let mut floors: i64 = 0;
    for c in input.chars() {
        if String::from(c) == "(" {
            floors += 1;
            count += 1;
        } else if String::from(c) == ")" {
            floors -= 1;
            count += 1;
        }
        if floors == -1 {
            return Some(count);
        }
    }
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(0));
    }
}
