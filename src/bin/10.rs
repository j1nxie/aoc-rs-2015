fn process(s: String) -> String {
    let mut curr_char: char = ' ';
    let mut count: u32 = 0;
    let mut new_string = String::new();

    for c in s.chars() {
        if c != curr_char {
            if count > 0 {
                new_string.push_str(&count.to_string());
                new_string.push(curr_char);
            }
            count = 1;
            curr_char = c;
        } else {
            count += 1;
        }
    }

    if count > 0 {
        new_string.push_str(&count.to_string());
        new_string.push(curr_char);
    }

    new_string
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut result_string = input.to_string();

    for _ in 0..40 {
        result_string = process(result_string);
    }

    Some(result_string.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result_string = input.to_string();

    for _ in 0..50 {
        result_string = process(result_string);
    }

    Some(result_string.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(82350));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(1166642));
    }
}
