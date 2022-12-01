use std::collections::HashSet;

pub type Position = (i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut current_pos = (0, 0);

    houses.insert(current_pos);

    for &c in input.as_bytes() {
        match c {
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            _ => continue,
        }

        houses.insert(current_pos);
    }

    Some(houses.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);

    houses.insert(santa);

    for (i, &c) in input.as_bytes().iter().enumerate() {
        let current_pos = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };

        match c {
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            _ => continue,
        }

        houses.insert(*current_pos);
    }

    Some(houses.len().try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(3));
    }
}
