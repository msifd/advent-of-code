fn solve_captcha(input: &str, part2: bool) -> u32 {
    if input.is_empty() {
        return 0;
    }

    let input = input.chars().collect::<Vec<_>>();
    let distance = if part2 { input.len() / 2 } else { 1 };

    let mut sum = 0;
    for i in 0..input.len() {
        let second_index = if i + distance < input.len() {
            i + distance
        } else {
            (i + distance) % input.len()
        };

        if input[i] == input[second_index] {
            sum += input[i].to_digit(10).unwrap();
        }
    }

    sum
}

pub fn day1() {
    let input = include_str!("inputs/day1.txt");
    println!(
        "Day 1 - {} - {}",
        solve_captcha(input, false),
        solve_captcha(input, true)
    );
}

#[test]
fn test_pt1() {
    assert_eq!(solve_captcha("1122", false), 3);
    assert_eq!(solve_captcha("1111", false), 4);
    assert_eq!(solve_captcha("1234", false), 0);
    assert_eq!(solve_captcha("91212129", false), 9);
}

#[test]
fn test_pt2() {
    assert_eq!(solve_captcha("1212", true), 6);
    assert_eq!(solve_captcha("1221", true), 0);
    assert_eq!(solve_captcha("123425", true), 4);
    assert_eq!(solve_captcha("123123", true), 12);
    assert_eq!(solve_captcha("12131415", true), 4);
}
