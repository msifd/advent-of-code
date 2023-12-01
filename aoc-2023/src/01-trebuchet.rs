use regex::Regex;

fn calibrate1(input: &str) -> u32 {
    input
        .lines()
        .map(|l: &str| {
            let mut digits = l
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap());

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn calibrate2(input: &str) -> u32 {
    let match_alpha = |s| match s {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    };

    let first_re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last_re = Regex::new(r"^.*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    input
        .lines()
        .map(|line| {
            let first = first_re
                .captures(line)
                .map(|c| match_alpha(c.get(1).unwrap().as_str()))
                .unwrap();
            let last = last_re
                .captures(line)
                .map(|c| match_alpha(c.get(1).unwrap().as_str()))
                .unwrap();
            first * 10 + last
        })
        .sum()
}

fn main() {
    let input = include_str!("../inputs/01.txt");
    println!("part 1: {}", calibrate1(input));
    println!("part 2: {}", calibrate2(input));
}

#[test]
fn test1() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    assert_eq!(calibrate1(input), 142);
}

#[test]
fn test2() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    assert_eq!(calibrate2(input), 281);
}
