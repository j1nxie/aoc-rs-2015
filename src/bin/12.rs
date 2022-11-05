use json::JsonValue;
use regex::Regex;

fn has_red(json: &JsonValue) -> bool {
    json.entries()
        .find(|(_, v)| v.is_string() && v.as_str().unwrap().eq_ignore_ascii_case("red"))
        .is_some()
}

fn count_objects(json: &JsonValue, ignore_red: bool) -> i32 {
    match json {
        JsonValue::Object(_) => {
            if ignore_red && has_red(json) {
                return 0;
            }

            json.entries()
                .map(|(_, v)| count_objects(v, ignore_red))
                .sum::<i32>()
        }

        JsonValue::Array(_) => json
            .members()
            .map(|v| count_objects(v, ignore_red))
            .sum::<i32>(),

        JsonValue::Number(_) => json.as_i32().unwrap(),

        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let re = Regex::new(r"\-*\d+").unwrap();
    let mut result = 0;
    for caps in re.captures_iter(input) {
        result += caps.get(0).unwrap().as_str().parse::<i32>().unwrap();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let parsed_json = json::parse(input).unwrap();
    let result = count_objects(&parsed_json, true);

    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(6));
    }
}
