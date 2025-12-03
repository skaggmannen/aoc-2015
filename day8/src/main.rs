fn main() {
    let input = util::read_input("day8/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let result: usize = util::to_lines(input)
        .iter()
        .map(|s| {
            let unescaped = unescape(s);

            s.len() - unescaped
        })
        .sum();

    format!("{}", result)
}

fn unescape(s: &str) -> usize {
    let mut unescaped = 0;
    let mut chars = s.chars();

    loop {
        let Some(c) = chars.next() else {
            break;
        };

        match c {
            '\\' => {
                let c2 = chars.next().unwrap();

                match c2 {
                    'x' => {
                        _ = chars.by_ref().take(2).collect::<String>();

                        unescaped += 1;
                    }
                    _ => unescaped += 1,
                }
            }
            '"' => {}
            _ => unescaped += 1,
        }
    }

    unescaped
}

fn part2(input: &str) -> String {
    let result: usize = util::to_lines(input)
        .iter()
        .map(|s| {
            let escaped = escape(s);

            escaped - s.len()
        })
        .sum();

    format!("{}", result)
}

fn escape(s: &str) -> usize {
    let mut escaped = 0;

    // Add opening "
    escaped += 1;

    for c in s.chars() {
        match c {
            '"' => escaped += 2,  // Add a \
            '\\' => escaped += 2, // Add a \
            _ => escaped += 1,
        };
    }

    // Add closing "
    escaped += 1;

    escaped
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(r#""""#), "2");
        assert_eq!(super::part1(r#""abc""#), "2");
        assert_eq!(super::part1(r#""aaa\"aaa""#), "3");
        assert_eq!(super::part1(r#""\x27""#), "5");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(r#""""#), "4");
        assert_eq!(super::part2(r#""abc""#), "4");
        assert_eq!(super::part2(r#""aaa\"aaa""#), "6");
        assert_eq!(super::part2(r#""\x27""#), "5");
    }
}
