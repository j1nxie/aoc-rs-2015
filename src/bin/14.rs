use regex::{Captures, Regex};

enum State {
    Resting,
    Flying,
}

struct Deer {
    speed: u32,
    time: u32,
    rest: u32,

    state: State,
    remaining: u32,
    distance: u32,
    points: u32,
}

impl Deer {
    fn new(cap: Captures) -> Deer {
        Deer {
            speed: cap[2].parse().unwrap(),
            time: cap[3].parse().unwrap(),
            rest: cap[4].parse().unwrap(),

            state: State::Flying,
            remaining: cap[3].parse().unwrap(),
            distance: 0,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if let State::Flying = self.state {
            self.distance += self.speed;
        }

        self.remaining -= 1;
        if self.remaining == 0 {
            match self.state {
                State::Flying => {
                    self.state = State::Resting;
                    self.remaining = self.rest;
                }

                State::Resting => {
                    self.state = State::Flying;
                    self.remaining = self.time;
                }
            }
        }
    }

    fn give_point(&mut self) {
        self.points += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    const TIME: u32 = 2503;
    let mut answer: Vec<_> = vec![];
    for cap in re.captures_iter(input) {
        let speed = &cap[1].parse::<u32>().unwrap();
        let time = &cap[2].parse::<u32>().unwrap();
        let rest = &cap[3].parse::<u32>().unwrap();

        let total_time: f64 = TIME as f64 / (*time as f64 + *rest as f64);

        answer.push(total_time.ceil() as u32 * (speed * time))
    }

    Some(answer.into_iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    const TIME: u32 = 2503;

    let mut deer: Vec<Deer> = input
        .lines()
        .map(|l| Deer::new(re.captures(l).unwrap()))
        .collect();

    let mut timer: u32 = 0;
    while timer < TIME {
        for d in deer.iter_mut() {
            d.tick();
        }

        let lead_dist = deer.iter().max_by_key(|d| d.distance).unwrap().distance;
        deer.iter_mut()
            .filter(|d| d.distance.eq(&lead_dist))
            .for_each(|d| d.give_point());

        timer += 1;
    }

    deer.sort_by(|a, b| b.points.cmp(&a.points));
    Some(deer.first().unwrap().points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(2660));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(1564));
    }
}
