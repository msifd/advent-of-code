use std::collections::HashMap;

fn calibrate1(input: &str) -> u32 {
    input
        .lines()
        .map(|l: &str| {
            let mut digits = l
                .chars()
                .filter(|c| char::is_numeric(*c))
                .map(|c| c.to_digit(10).unwrap());

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn calibrate2(input: &str) -> u32 {
    let match_alpha = |s| match s {
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };

    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .fold(None, |n| );

        })
        .for_each(|v| println!("- {v:?}"));

    0
    // input
    //     .lines()
    //     .map(|l: &str| {
    //         let mut digits = l
    //             .chars()
    //             .filter(|c| char::is_numeric(*c))
    //             .map(|c| c.to_digit(10).unwrap());

    //         let first = digits.next().unwrap();
    //         let last = digits.last().unwrap_or(first);
    //         first * 10 + last
    //     })
    //     // .inspect(|n| println!("- {n}"))
    //     .sum()
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
