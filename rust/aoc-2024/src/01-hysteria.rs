mod input_loader;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();
            (a, b)
        })
        .unzip()
}

fn part1(input: &str) -> u32 {
    let (mut va, mut vb) = parse(input);

    va.sort();
    vb.sort();

    va.into_iter()
        .zip(vb.into_iter())
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum()
}

fn main() {
    let input = input_loader::load();
    println!("part 1: {}", part1(&input));
}

#[test]
fn test1() {
    let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    assert_eq!(part1(input), 11);
}