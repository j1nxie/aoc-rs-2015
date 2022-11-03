use std::collections::HashMap;

fn parser(cmd: &str, values: &mut HashMap<String, u32>) -> Option<()> {
    fn get(values: &HashMap<String, u32>, idx: &str) -> Option<u32> {
        if let Some(x) = values.get(idx) {
            Some(*x)
        } else {
            None
        }
    }

    let get_value = |cmd: &str| -> Option<u32> {
        if is_int(cmd) {
            Some(cmd.parse::<u32>().unwrap())
        } else {
            get(values, cmd)
        }
    };

    fn is_int(s: &str) -> bool {
        s.parse::<usize>().is_ok()
    }

    let cmd = cmd.split(' ').collect::<Vec<_>>();

    match cmd.len() {
        3 => {
            let value = get_value(cmd[0])?;
            values.insert(cmd[2].to_string(), value);
        }

        4 => {
            let value = get_value(cmd[1])?;
            values.insert(cmd[3].to_string(), !value);
        }

        5 => {
            let value = get_value(cmd[2])?;
            let idx = get_value(cmd[0])?;
            values.insert(
                cmd[4].to_string(),
                match cmd[1] {
                    "AND" => idx & value,
                    "OR" => idx | value,
                    "RSHIFT" => idx >> value,
                    "LSHIFT" => idx << value,
                    _ => panic!("um this isn't supposed to happen."),
                },
            );
        }

        _ => panic!("um this isn't supposed to happen."),
    }

    Some(())
}

pub fn part_one(input: &str) -> Option<u32> {
    let hashes: &mut HashMap<String, u32> = &mut HashMap::new();

    let mut lines = input.lines().collect::<Vec<_>>();

    let mut i = 0;
    while !lines.is_empty() {
        parser(lines[i], hashes);
        match parser(lines[i], hashes) {
            Some(_) => {
                lines.remove(i);
            }
            None => {
                i += 1;
            }
        }
        if i >= lines.len() {
            i = 0;
        }
    }

    Some(*hashes.get("a").unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let hashes: &mut HashMap<String, u32> = &mut HashMap::new();

    let mut lines = input.lines().collect::<Vec<_>>();

    let mut i = 0;
    while !lines.is_empty() {
        hashes.insert("b".to_string(), 956);
        parser(lines[i], hashes);
        match parser(lines[i], hashes) {
            Some(_) => {
                lines.remove(i);
            }
            None => {
                i += 1;
            }
        }
        if i >= lines.len() {
            i = 0;
        }
    }

    Some(*hashes.get("a").unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(123));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(123));
    }
}
