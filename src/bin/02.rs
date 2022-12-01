use std::cmp;

pub type Gift = (u32, u32, u32);

pub fn part_one(input: &str) -> Option<u32> {
    let gifts: Vec<Gift> = input
        .lines()
        .map(|i| {
            let mut gift = i.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        })
        .collect();

    let mut total = 0;

    for gift in gifts {
        let top = 2 * gift.0 * gift.1;
        let side = 2 * gift.1 * gift.2;
        let front = 2 * gift.2 * gift.0;

        let cmp = cmp::min(top, side);
        let min = cmp::min(cmp, front);

        total += top + side + front + min / 2;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let gifts: Vec<Gift> = input
        .lines()
        .map(|i| {
            let mut gift = i.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        })
        .collect();

    let mut total = 0;

    for gift in gifts {
        let smallest_side: (u32, u32) = {
            let mut vec = vec![gift.0, gift.1, gift.2];
            vec.sort();

            (vec[0], vec[1])
        };

        let volume = gift.0 * gift.1 * gift.2;

        total += 2 * (smallest_side.0 + smallest_side.1) + volume;
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(168));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(138));
    }
}
