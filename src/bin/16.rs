use std::collections::HashMap;

#[derive(PartialEq)]
enum MatchVal {
    Greater(u32),
    Equal(u32),
    Less(u32),
}

impl MatchVal {
    fn matches(&self, v: u32) -> bool {
        match self {
            MatchVal::Greater(u) => v > *u,
            MatchVal::Equal(u) => v == *u,
            MatchVal::Less(u) => v < *u,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut info: HashMap<&str, u32> = HashMap::new();

    info.insert("children", 3);
    info.insert("cats", 7);
    info.insert("samoyeds", 2);
    info.insert("pomeranians", 3);
    info.insert("akitas", 0);
    info.insert("vizslas", 0);
    info.insert("goldfish", 5);
    info.insert("trees", 3);
    info.insert("cars", 2);
    info.insert("perfumes", 1);

    let aunts: Vec<Vec<(&str, u32)>> = input
        .lines()
        .map(|l| {
            l.trim()
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|prop| {
                    let kv: Vec<&str> = prop.splitn(2, ": ").collect();
                    (kv[0], kv[1].parse::<u32>().unwrap())
                })
                .collect()
        })
        .collect();

    let (i, _) = aunts
        .iter()
        .enumerate()
        .find(|(_, a)| a.iter().all(|(k, v)| *(info.get(k).unwrap()) == *v))
        .unwrap();

    Some((i + 1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut info: HashMap<&str, MatchVal> = HashMap::new();

    info.insert("children", MatchVal::Greater(3));
    info.insert("cats", MatchVal::Equal(7));
    info.insert("samoyeds", MatchVal::Equal(2));
    info.insert("pomeranians", MatchVal::Less(3));
    info.insert("akitas", MatchVal::Equal(0));
    info.insert("vizslas", MatchVal::Equal(0));
    info.insert("goldfish", MatchVal::Less(5));
    info.insert("trees", MatchVal::Greater(3));
    info.insert("cars", MatchVal::Equal(2));
    info.insert("perfumes", MatchVal::Equal(1));

    let aunts: Vec<Vec<(&str, u32)>> = input
        .lines()
        .map(|l| {
            l.trim()
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|prop| {
                    let kv: Vec<&str> = prop.splitn(2, ": ").collect();
                    (kv[0], kv[1].parse::<u32>().unwrap())
                })
                .collect()
        })
        .collect();

    let (i, _) = aunts
        .iter()
        .enumerate()
        .find(|(_, a)| a.iter().all(|(k, v)| info.get(k).unwrap().matches(*v)))
        .unwrap();

    Some((i + 1) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(3));
    }
}
