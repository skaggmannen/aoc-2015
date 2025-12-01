fn main() {
    let input = util::read_input("day2/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let result = util::to_lines(input)
        .iter()
        .map(|l| Present::from_str(l))
        .fold(State::new(), |state, p| state.apply(p));

    format!("{}", result.total_area)
}

fn part2(input: &str) -> String {
    let result = util::to_lines(input)
        .iter()
        .map(|l| Present::from_str(l))
        .fold(State::new(), |state, p| state.apply(p));

    format!("{}", result.total_ribbon)
}

struct State {
    total_area: usize,
    total_ribbon: usize,
}

impl State {
    fn new() -> Self {
        Self {
            total_area: 0,
            total_ribbon: 0,
        }
    }

    fn apply(self, p: Present) -> Self {
        Self {
            total_area: self.total_area + p.area(),
            total_ribbon: self.total_ribbon + p.ribbon(),
        }
    }
}

struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn from_str(s: &str) -> Self {
        let parts = s
            .split("x")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        match &parts[..] {
            &[l, w, h, ..] => Self { l, w, h },
            _ => panic!("invalid input"),
        }
    }

    fn area(&self) -> usize {
        let Self { l, w, h } = self;
        let areas = [(l * w), (w * h), (h * l)];

        2 * areas.iter().sum::<usize>() + areas.iter().min().unwrap()
    }

    fn ribbon(&self) -> usize {
        let Self { l, w, h } = self;

        [(2 * l) + (2 * h), (2 * l) + (2 * w), (2 * h) + (2 * w)]
            .into_iter()
            .min()
            .unwrap()
            + l * w * h
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1("2x3x4"), "58");
        assert_eq!(super::part1("1x1x10"), "43");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2("2x3x4"), "34");
        assert_eq!(super::part2("1x1x10"), "14");
    }
}
