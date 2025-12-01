fn main() {
    let input = util::read_input("day1/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let state = input
        .chars()
        .into_iter()
        .fold(State::new(), |state, c| state.apply(c));

    format!("{}", state.floor)
}

fn part2(input: &str) -> String {
    let mut state = State::new();
    for (i, c) in input.chars().enumerate() {
        state = state.apply(c);
        if state.floor < 0 {
            return format!("{}", i + 1);
        }
    }

    panic!("santa never went to the basement!")
}

#[derive(Clone, Copy, Debug)]
struct State {
    floor: i32,
}

impl State {
    fn new() -> Self {
        Self { floor: 0 }
    }

    fn apply(self, c: char) -> Self {
        match c {
            '(' => Self {
                floor: self.floor + 1,
            },
            ')' => Self {
                floor: self.floor - 1,
            },
            _ => Self { floor: self.floor },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        // (()) and ()() both result in floor 0.
        assert_eq!(part1("(())"), "0");
        assert_eq!(part1("()()"), "0");
        // ((( and (()(()( both result in floor 3.
        assert_eq!(part1("((("), "3");
        assert_eq!(part1("(()(()("), "3");
        // ))((((( also results in floor 3.
        assert_eq!(part1("))((((("), "3");
        // ()) and ))( both result in floor -1 (the first basement level).
        assert_eq!(part1("())"), "-1");
        assert_eq!(part1("))("), "-1");
        // ))) and )())()) both result in floor -3.
        assert_eq!(part1(")))"), "-3");
        assert_eq!(part1(")())())"), "-3");
    }

    #[test]
    fn test_part2() {
        // ) causes him to enter the basement at character position 1.
        assert_eq!(part2(")"), "1");
        // ()()) causes him to enter the basement at character position 5.
        assert_eq!(part2("()())"), "5");
    }
}
