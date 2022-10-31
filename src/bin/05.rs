pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let three_vowels: bool = line
            .trim()
            .chars()
            .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count()
            >= 3;

        let twice_in_a_row: bool = line
            .trim()
            .chars()
            .zip(line.chars().skip(1))
            .any(|(a, b)| a == b);

        let no_forbidden_strings: bool = line
            .trim()
            .chars()
            .zip(line.chars().skip(1))
            .all(|ab| !matches!(ab, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')));

        if three_vowels && twice_in_a_row && no_forbidden_strings {
            count += 1;
        }
    }

    Some(count)
}

fn two_pairs(string: &str) -> bool {
    if string.len() < 4 {
        return false;
    }

    let pair = &string[0..2];
    let remain = &string[2..];

    remain.contains(pair) || two_pairs(&string[1..])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let repeat_separated = line
            .trim()
            .chars()
            .zip(line.chars().skip(2))
            .any(|(a, b)| a == b);
        if two_pairs(line.trim()) && repeat_separated {
            count += 1;
        }
    }

    Some(count)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(2));
    }
}
