use regex::Regex;

mod input_loader;

fn part1(input: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let (_, [l, r]) = c.extract();
            l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)")
        .unwrap()
        .captures_iter(input)
        .scan(true, |todo, c| {
            Some(match c.get(1).unwrap().as_str() {
                "mul" if *todo => {
                    let l = c.get(2).unwrap().as_str();
                    let r = c.get(3).unwrap().as_str();
                    l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap()
                }
                "do" => {
                    *todo = true;
                    0
                }
                "don't" => {
                    *todo = false;
                    0
                }
                _ => 0,
            })
        })
        .sum()
}

fn main() {
    let input = input_loader::load(3);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[test]
fn test1() {
    let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    assert_eq!(part1(input), 161);
}

#[test]
fn test2() {
    let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(part2(input), 48);
}
