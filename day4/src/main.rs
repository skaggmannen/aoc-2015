fn main() {
    let input = util::read_input("day4/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    for i in 0.. {
        let digest = format!(
            "{:X?}",
            md5::compute(format!("{}{}", input.trim(), i).as_bytes())
        );
        if digest.starts_with("00000") {
            return format!("{}", i);
        }
    }

    panic!("oh no!")
}

fn part2(input: &str) -> String {
    for i in 0.. {
        let digest = format!(
            "{:X?}",
            md5::compute(format!("{}{}", input.trim(), i).as_bytes())
        );
        if digest.starts_with("000000") {
            return format!("{}", i);
        }
    }

    panic!("oh no!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1("abcdef"), "609043");
        assert_eq!(super::part1("pqrstuv"), "1048970");
    }

    #[test]
    fn test_part2() {}
}
