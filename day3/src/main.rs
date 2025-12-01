use std::collections::HashSet;

fn main() {
    let input = util::read_input("day3/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut houses = HashSet::new();

    input
        .chars()
        .into_iter()
        .filter(|c| !c.is_whitespace())
        .fold(State::new(&mut houses), |state, c| state.apply(c));

    format!("{}", houses.iter().count())
}

fn part2(input: &str) -> String {
    let mut houses = HashSet::new();

    // Santa takes all even instructions (0, 2, 4, ...).
    input
        .chars()
        .into_iter()
        .filter(|c| !c.is_whitespace())
        .step_by(2)
        .fold(State::new(&mut houses), |state, c| state.apply(c));

    // Robo-Santa takes all odd instructions (1, 3, 5, ...).
    input
        .chars()
        .into_iter()
        .filter(|c| !c.is_whitespace())
        .skip(1)
        .step_by(2)
        .fold(State::new(&mut houses), |state, c| state.apply(c));

    format!("{}", houses.iter().count())
}

struct State<'a> {
    pos: (i32, i32),
    houses: &'a mut HashSet<(i32, i32)>,
}

impl<'a> State<'a> {
    fn new(houses: &'a mut HashSet<(i32, i32)>) -> Self {
        houses.insert((0, 0));

        Self {
            pos: (0, 0),
            houses: houses,
        }
    }

    fn apply(self, c: char) -> Self {
        let (x, y) = self.pos;
        let pos = match c {
            '^' => (x, y + 1),
            '>' => (x + 1, y),
            'v' => (x, y - 1),
            '<' => (x - 1, y),
            _ => panic!("invalid input: {}", c),
        };

        self.houses.insert(pos);

        Self {
            pos,
            houses: self.houses,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(">"), "2");
        assert_eq!(super::part1("^>v<"), "4");
        assert_eq!(super::part1("^v^v^v^v^v"), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2("^v"), "3");
        assert_eq!(super::part2("^>v<"), "3");
        assert_eq!(super::part2("^v^v^v^v^v"), "11");
    }
}
