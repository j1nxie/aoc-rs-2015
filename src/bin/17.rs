fn combinations(
    containers: &Vec<u32>,
    max: u32,
    initial: u32,
    count: usize,
    skip: usize,
    groups: &mut Vec<usize>,
) {
    if initial == max {
        groups.push(count);
        return;
    }

    if initial > max || skip >= containers.len() {
        return;
    }

    containers
        .iter()
        .enumerate()
        .skip(skip)
        .for_each(|(i, c)| combinations(containers, max, c + initial, count + 1, i + 1, groups));
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result: Vec<usize> = vec![];
    let containers: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    combinations(&containers, 150, 0, 0, 0, &mut result);
    Some(result.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: Vec<usize> = vec![];
    let containers: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    combinations(&containers, 150, 0, 0, 0, &mut result);

    let min = result.iter().min().unwrap();
    Some(result.iter().filter(|c| *c == min).count())
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(5));
    }
}
