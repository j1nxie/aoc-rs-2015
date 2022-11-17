use std::collections::HashSet;

fn split_replacement(line: &str) -> (&str, &str) {
    let parts: Vec<_> = line.splitn(2, " => ").collect();

    (parts[0], parts[1])
}

fn replace(s: &str, i: usize, from: &str, to: &str) -> String {
    let before: String = s.chars().take(i).collect();
    let after: String = s.chars().skip(i + from.len()).collect();

    before + to + &after
}
pub fn part_one(input: &str) -> Option<u32> {
    let split_input: Vec<&str> = input.splitn(2, "\n\n").collect();
    let replacements = split_input[0];
    let molecule = split_input[1];

    let set: HashSet<String> = replacements
        .trim()
        .lines()
        .map(|l| split_replacement(l))
        .flat_map(|(from, to)| {
            molecule
                .match_indices(from)
                .map(move |(i, _)| replace(molecule, i, from, to))
        })
        .collect();

    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(6));
    }
}
