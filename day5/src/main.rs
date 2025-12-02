fn main() {
    let input = util::read_input("day5/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let nice_strings = util::to_lines(input)
        .into_iter()
        .filter(|s| has_vowels(s) && has_double_letters(s) && !has_naughty_strings(s));

    format!("{}", nice_strings.count())
}

fn part2(input: &str) -> String {
    let nice_strings = util::to_lines(input)
        .into_iter()
        .filter(|s| has_pair(s) && has_repeating_letter(s));

    format!("{}", nice_strings.count())
}

fn has_vowels(s: &str) -> bool {
    "aeiou".chars().fold(0, |acc, vowel| {
        acc + s.chars().filter(|&c| c == vowel).count()
    }) >= 3
}

fn has_double_letters(s: &str) -> bool {
    let bytes = s.bytes().collect::<Vec<_>>();

    bytes.windows(2).any(|pair| pair[0] == pair[1])
}

fn has_naughty_strings(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn has_pair(s: &str) -> bool {
    let bytes = s.bytes().collect::<Vec<_>>();

    for i in 0..bytes.len() {
        match &bytes[i..] {
            [a, b, tail @ ..] => {
                if tail.windows(2).any(|pair| pair[0] == *a && pair[1] == *b) {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

fn has_repeating_letter(s: &str) -> bool {
    let bytes = s.bytes().collect::<Vec<_>>();

    bytes.windows(3).any(|triplet| triplet[0] == triplet[2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1("ugknbfddgicrmopn"), "1");
        assert_eq!(super::part1("aaa"), "1");
        assert_eq!(super::part1("jchzalrnumimnmhp"), "0");
        assert_eq!(super::part1("haegwjzuvuyypxyu"), "0");
        assert_eq!(super::part1("dvszwmarrgswjxmb"), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::has_pair("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(super::has_repeating_letter("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(super::has_pair("xxyxx"), true);
        assert_eq!(super::has_repeating_letter("xxyxx"), true);
        assert_eq!(super::has_pair("uurcxstgmygtbstg"), true);
        assert_eq!(super::has_repeating_letter("uurcxstgmygtbstg"), false);
        assert_eq!(super::has_pair("ieodomkazucvgmuy"), false);
        assert_eq!(super::has_repeating_letter("ieodomkazucvgmuy"), true);
    }
}
