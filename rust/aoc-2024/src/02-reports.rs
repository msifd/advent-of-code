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

fn part1(input: &str) -> usize {
    let reports = parse(input);

    reports
        .into_iter()
        .filter(|report| {
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
        })
        .count()
}

fn main() {
    let input = input_loader::load(2);
    println!("part 1: {}", part1(&input));
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
