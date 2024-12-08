mod input_loader;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn validate(report: &Vec<i32>) -> bool {
    let [a, b] = report.first_chunk::<2>().unwrap();
    let dir = if b > a { 1 } else { -1 };

    let mut prev = a;
    for n in report.iter().skip(1) {
        let d = (n - prev) * dir;
        if d < 1 || d > 3 {
            return false;
        }

        prev = n;
    }

    true
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().filter(validate).count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|report| {
            if validate(report) {
                return true;
            }

            (0..report.len()).any(|i| {
                let left = &report[..i];
                let right = &report[i + 1..];
                let merged = [left, right].concat();
                validate(&merged)
            })
        })
        .count()
}

fn main() {
    let input = input_loader::load(2);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[test]
fn test1() {
    let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    assert_eq!(part1(input), 2);
}

#[test]
fn test2() {
    let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    assert_eq!(part2(input), 4);
}
