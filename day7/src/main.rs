use std::collections::HashMap;

fn main() {
    let input = util::read_input("day7/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let gates = Gates::from_str(input);

    format!("{}", gates.resolve("a", &mut HashMap::new()))
}

fn part2(input: &str) -> String {
    let mut gates = Gates::from_str(input);

    let a = gates.resolve("a", &mut HashMap::new());

    gates.set("b", Gate::Load(format!("{}", a)));

    let a = gates.resolve("a", &mut HashMap::new());

    format!("{}", a)
}

struct Gates {
    wires: HashMap<String, Gate>,
}

impl Gates {
    fn from_str(input: &str) -> Self {
        let mut wires = HashMap::new();

        util::to_lines(input).iter().for_each(|s| {
            let parts: Vec<_> = s.split(" ").collect();
            match &parts[..] {
                [x, "->", y] => wires.insert(y.to_string(), Gate::Load(x.to_string())),
                [x, "AND", y, "->", z] => {
                    wires.insert(z.to_string(), Gate::And(x.to_string(), y.to_string()))
                }
                [x, "OR", y, "->", z] => {
                    wires.insert(z.to_string(), Gate::Or(x.to_string(), y.to_string()))
                }
                [p, "LSHIFT", v, "->", q] => wires.insert(
                    q.to_string(),
                    Gate::LShift(p.to_string(), v.parse().unwrap()),
                ),
                [p, "RSHIFT", v, "->", q] => wires.insert(
                    q.to_string(),
                    Gate::RShift(p.to_string(), v.parse().unwrap()),
                ),
                ["NOT", e, "->", f] => wires.insert(f.to_string(), Gate::Not(e.to_string())),
                _ => panic!("invalid input: {}", s),
            };
        });

        Self { wires }
    }

    fn resolve(&self, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
        if let Some(v) = cache.get(wire) {
            return *v;
        }

        if let Ok(v) = wire.parse() {
            return v;
        }

        let Some(gate) = self.wires.get(wire) else {
            panic!("wire not found: {}", wire);
        };

        let v = match gate {
            Gate::Load(x) => self.resolve(x, cache),
            Gate::And(x, y) => self.resolve(x, cache) & self.resolve(y, cache),
            Gate::Or(x, y) => self.resolve(x, cache) | self.resolve(y, cache),
            Gate::Not(e) => !self.resolve(e, cache),
            Gate::LShift(p, v) => self.resolve(p, cache) << *v,
            Gate::RShift(p, v) => self.resolve(p, cache) >> *v,
        };

        cache.insert(wire.to_string(), v);

        v
    }

    fn set(&mut self, wire: &str, g: Gate) {
        self.wires.insert(wire.to_string(), g);
    }
}

#[derive(Debug)]
enum Gate {
    Load(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    const INPUT: &str = "123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> i";

    #[test]
    fn test_part1() {
        let mut cache = HashMap::new();
        let gates = super::Gates::from_str(INPUT);

        assert_eq!(gates.resolve("d", &mut cache), 72);
        assert_eq!(gates.resolve("e", &mut cache), 507);
        assert_eq!(gates.resolve("f", &mut cache), 492);
        assert_eq!(gates.resolve("g", &mut cache), 114);
        assert_eq!(gates.resolve("h", &mut cache), 65412);
        assert_eq!(gates.resolve("i", &mut cache), 65079);
        assert_eq!(gates.resolve("x", &mut cache), 123);
        assert_eq!(gates.resolve("y", &mut cache), 456);
    }

    #[test]
    fn test_part2() {}
}
