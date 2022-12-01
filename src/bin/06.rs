#[derive(Clone, Copy)]
struct Light {
    brightness: i8,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lights: Vec<Vec<Light>> = vec![vec![Light { brightness: 0 }; 1000]; 1000];
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();

        if words.len() == 5 {
            match words[1] {
                "on" => {
                    let start: Vec<&str> = words[2].split(',').collect();
                    let end: Vec<&str> = words[4].split(',').collect();

                    let start_x = start[0].parse::<usize>().unwrap();
                    let start_y = start[1].parse::<usize>().unwrap();
                    let end_x = end[0].parse::<usize>().unwrap();
                    let end_y = end[1].parse::<usize>().unwrap();

                    for row in lights.iter_mut().take(end_x + 1).skip(start_x) {
                        for light in row.iter_mut().take(end_y + 1).skip(start_y) {
                            light.brightness = 1;
                        }
                    }
                }
                "off" => {
                    let start: Vec<&str> = words[2].split(',').collect();
                    let end: Vec<&str> = words[4].split(',').collect();

                    let start_x = start[0].parse::<usize>().unwrap();
                    let start_y = start[1].parse::<usize>().unwrap();
                    let end_x = end[0].parse::<usize>().unwrap();
                    let end_y = end[1].parse::<usize>().unwrap();

                    for row in lights.iter_mut().take(end_x + 1).skip(start_x) {
                        for light in row.iter_mut().take(end_y + 1).skip(start_y) {
                            light.brightness = 0;
                        }
                    }
                }

                _ => (),
            }
        }
        if words.len() == 4 {
            let start: Vec<&str> = words[1].split(',').collect();
            let end: Vec<&str> = words[3].split(',').collect();

            let start_x = start[0].parse::<usize>().unwrap();
            let start_y = start[1].parse::<usize>().unwrap();
            let end_x = end[0].parse::<usize>().unwrap();
            let end_y = end[1].parse::<usize>().unwrap();

            for row in lights.iter_mut().take(end_x + 1).skip(start_x) {
                for light in row.iter_mut().take(end_y + 1).skip(start_y) {
                    match light.brightness {
                        0 => light.brightness = 1,
                        1 => light.brightness = 0,
                        _ => continue,
                    }
                }
            }
        }
    }

    Some(
        lights
            .iter()
            .flatten()
            .filter(|x| x.brightness == 1)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lights: Vec<Vec<Light>> = vec![vec![Light { brightness: 0 }; 1000]; 1000];
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();

        if words.len() == 5 {
            match words[1] {
                "on" => {
                    let start: Vec<&str> = words[2].split(',').collect();
                    let end: Vec<&str> = words[4].split(',').collect();

                    let start_x = start[0].parse::<usize>().unwrap();
                    let start_y = start[1].parse::<usize>().unwrap();
                    let end_x = end[0].parse::<usize>().unwrap();
                    let end_y = end[1].parse::<usize>().unwrap();

                    for row in lights.iter_mut().take(end_x + 1).skip(start_x) {
                        for light in row.iter_mut().take(end_y + 1).skip(start_y) {
                            light.brightness += 1;
                        }
                    }
                }
                "off" => {
                    let start: Vec<&str> = words[2].split(',').collect();
                    let end: Vec<&str> = words[4].split(',').collect();

                    let start_x = start[0].parse::<usize>().unwrap();
                    let start_y = start[1].parse::<usize>().unwrap();
                    let end_x = end[0].parse::<usize>().unwrap();
                    let end_y = end[1].parse::<usize>().unwrap();

                    for row in lights.iter_mut().take(end_x + 1).skip(start_x) {
                        for light in row.iter_mut().take(end_y + 1).skip(start_y) {
                            light.brightness -= 1;

                            if light.brightness < 0 {
                                light.brightness = 0;
                            }
                        }
                    }
                }

                _ => (),
            }
        }
        if words.len() == 4 {
            let start: Vec<&str> = words[1].split(',').collect();
            let end: Vec<&str> = words[3].split(',').collect();

            let start_x = start[0].parse::<usize>().unwrap();
            let start_y = start[1].parse::<usize>().unwrap();
            let end_x = end[0].parse::<usize>().unwrap();
            let end_y = end[1].parse::<usize>().unwrap();

            for row in lights.iter_mut().take(end_x + 1).skip(start_x) {
                for light in row.iter_mut().take(end_y + 1).skip(start_y) {
                    light.brightness += 2;
                }
            }
        }
    }

    let mut sum: u32 = 0;

    lights
        .iter()
        .flatten()
        .filter(|x| x.brightness > 0)
        .for_each(|x| sum += x.brightness as u32);

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(999996));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(999996));
    }
}
