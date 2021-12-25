fn sweep_1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn sweep_3(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn main() {
    let input = include_str!("../res/01-input.txt");
    println!("{}", sweep_1(input));
    println!("{}", sweep_3(input));
}

#[test]
fn test_example() {
    let input = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(sweep_1(input), 7);
    assert_eq!(sweep_3(input), 5);
}
