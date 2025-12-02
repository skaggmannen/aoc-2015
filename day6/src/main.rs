use std::collections::{HashMap, HashSet};

fn main() {
    let input = util::read_input("day6/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut lit_lights = HashSet::<Coords>::new();

    util::to_lines(input)
        .iter()
        .map(|s| Instruction::from_str(s))
        .for_each(|i| i.execute(&mut lit_lights));

    format!("{}", lit_lights.iter().count())
}

fn part2(input: &str) -> String {
    let mut lit_lights = HashMap::<Coords, u32>::new();

    util::to_lines(input)
        .iter()
        .map(|s| Instruction::from_str(s))
        .for_each(|i| i.execute_with_dim(&mut lit_lights));

    format!("{}", lit_lights.values().sum::<u32>())
}

struct Instruction {
    op: Op,
    first: Coords,
    last: Coords,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let op = Op::from_str(s);

        let parts = s
            .strip_prefix("turn on")
            .or(s.strip_prefix("turn off"))
            .or(s.strip_prefix("toggle"))
            .unwrap()
            .trim()
            .split(" ")
            .collect::<Vec<_>>();

        let [first, _, last] = &parts[..] else {
            panic!("invalid input: {}", s);
        };

        Self {
            op,
            first: Coords::from_str(first),
            last: Coords::from_str(last),
        }
    }

    fn execute(&self, lights: &mut HashSet<Coords>) {
        let Coords(x1, y1) = self.first;
        let Coords(x2, y2) = self.last;

        for x in x1..=x2 {
            for y in y1..=y2 {
                let c = Coords(x, y);
                match self.op {
                    Op::TurnOn => {
                        lights.insert(c);
                    }
                    Op::TurnOff => {
                        lights.remove(&c);
                    }
                    Op::Toggle => {
                        if lights.contains(&c) {
                            lights.remove(&c);
                        } else {
                            lights.insert(c);
                        }
                    }
                }
            }
        }
    }

    fn execute_with_dim(&self, lights: &mut HashMap<Coords, u32>) {
        let Coords(x1, y1) = self.first;
        let Coords(x2, y2) = self.last;

        for x in x1..=x2 {
            for y in y1..=y2 {
                let c = Coords(x, y);
                match self.op {
                    Op::TurnOn => {
                        *lights.entry(c).or_insert(0) += 1;
                    }
                    Op::TurnOff => {
                        let v = lights.entry(c).or_insert(1);
                        if *v > 0 {
                            *v -= 1
                        }
                    }
                    Op::Toggle => {
                        *lights.entry(c).or_insert(0) += 2;
                    }
                }
            }
        }
    }
}

enum Op {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(PartialEq, Eq, Hash)]
struct Coords(u32, u32);

impl Coords {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();

        Self(x, y)
    }
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s {
            s if s.starts_with("turn on") => Op::TurnOn,
            s if s.starts_with("turn off") => Op::TurnOff,
            s if s.starts_with("toggle") => Op::Toggle,
            _ => panic!("invalid input: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1("turn on 0,0 through 999,999"), "1000000");
        assert_eq!(super::part1("toggle 0,0 through 999,0"), "1000");
        assert_eq!(super::part1("turn off 499,499 through 500,500"), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2("turn on 0,0 through 999,999"), "1000000");
        assert_eq!(super::part2("toggle 0,0 through 999,0"), "2000");
    }
}
