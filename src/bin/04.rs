use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn part_one(input: &str) -> Option<u64> {
    let mut hasher = Md5::new();
    let mut result = 0;
    for i in 0..std::u64::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            result = i;
            break;
        }

        hasher.reset();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut hasher = Md5::new();
    let mut result = 0;
    for i in 0..std::u64::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_six = output[0] as i32 + output[1] as i32 + output[2] as i32;
        if first_six == 0 {
            result = i;
            break;
        }

        hasher.reset();
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(609043));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(6742839));
    }
}
