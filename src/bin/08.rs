pub fn part_one(input: &str) -> Option<u32> {
    let mut total_literal: u32 = 0;
    let mut total_memory: u32 = 0;
    for line in input.lines() {
        let literal_size = line.len() as u32;

        let mut memory_size = 0;

        let mut chars = line.chars().peekable();
        while chars.peek().is_some() {
            let c = chars.next().unwrap();

            if c == '"' {
                continue;
            } else if c == '\\' {
                let next = chars.next().unwrap();
                match next {
                    '\\' => {}
                    '"' => {}
                    'x' => {
                        assert!(chars.next().unwrap().is_ascii_hexdigit());
                        assert!(chars.next().unwrap().is_ascii_hexdigit());
                    }
                    _ => panic!("um this isn't supposed to happen."),
                }
            }
            memory_size += 1;
        }

        total_literal += literal_size;
        total_memory += memory_size;
    }

    Some(total_literal - total_memory)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_encoded: u32 = 0;
    let mut total_literal: u32 = 0;
    for line in input.lines() {
        total_literal += line.len() as u32;
        let mut encoded = line.chars().fold('"'.to_string(), |mut encoded, c| {
            match c {
                '"' => encoded.push_str(r#"\""#),
                '\\' => encoded.push_str(r#"\\"#),
                c => encoded.push(c),
            }
            encoded
        });

        encoded.push('"');

        total_encoded += encoded.len() as u32;
    }

    Some(total_encoded - total_literal)
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(19));
    }
}
