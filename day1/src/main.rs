fn main() {
    let input = util::read_input("day1/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut state = State::new();

    let result = util::to_lines(input)
        .into_iter()
        .map(|l| Rotation::from_str(&l))
        .map(|r| {
            state = state.apply(r);
            state.dial
        })
        .filter(|&v| v == 0)
        .count();

    format!("{}", result)
}

fn part2(input: &str) -> String {
    let state = util::to_lines(input)
        .into_iter()
        .map(|l| Rotation::from_str(&l))
        .fold(State::new(), |state, r| state.apply(r));

    format!("{}", state.clicks)
}

#[derive(Clone, Copy, Debug)]
struct State {
    dial: usize,
    clicks: usize,
}

impl State {
    fn new() -> Self {
        Self {
            dial: 50,
            clicks: 0,
        }
    }

    fn apply(self, r: Rotation) -> Self {
        return if r.direction == 'R' {
            // This is just normal modulo math.
            let clicks = self.clicks + (self.dial + r.distance) / 100;
            let next = (self.dial + r.distance) % 100;

            Self {
                dial: next,
                clicks: clicks,
            }
        } else {
            // Going left is a bit more complicated, since it involves negative modulo...
            let clicks = self.clicks + r.distance / 100;
            let remainder = r.distance % 100;

            if remainder < self.dial {
                Self {
                    dial: self.dial - remainder,
                    clicks: clicks,
                }
            } else if remainder == self.dial {
                // Landing on 0 generates another click.
                Self {
                    dial: 0,
                    clicks: clicks + 1,
                }
            } else if self.dial == 0 {
                // Turning left from 0 does not generate a click.
                Self {
                    dial: 100 - (remainder - self.dial),
                    clicks: clicks,
                }
            } else {
                // Turning past 0 generates another click.
                Self {
                    dial: 100 - (remainder - self.dial),
                    clicks: clicks + 1,
                }
            }
        };
    }
}

#[derive(Debug)]
struct Rotation {
    distance: usize,
    direction: char,
}

impl Rotation {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let distance = chars.as_str().parse().unwrap();
        Self {
            distance,
            direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "6");
    }
}
