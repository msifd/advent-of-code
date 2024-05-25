fn dive_1(input: &str) -> u32 {
    let mut dist = 0;
    let mut depth = 0;

    input
        .lines()
        .map(|line| line.split_once(' '))
        .filter_map(std::convert::identity)
        .map(|(cmd, num)| (cmd, num.parse::<u32>().unwrap()))
        .for_each(|(cmd, num)| match cmd {
            "forward" => dist += num,
            "up" => depth -= num,
            "down" => depth += num,
            _ => unimplemented!(),
        });

    dist * depth
}

fn dive_2(input: &str) -> u32 {
    let mut dist = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .lines()
        .map(|line| line.split_once(' '))
        .filter_map(std::convert::identity)
        .map(|(cmd, num)| (cmd, num.parse::<u32>().unwrap()))
        .for_each(|(cmd, num)| match cmd {
            "forward" => {
                dist += num;
                depth += num * aim;
            }
            "up" => aim -= num,
            "down" => aim += num,
            _ => unimplemented!(),
        });

    dist * depth
}

fn main() {
    let input = include_str!("../res/02-input.txt");
    println!("{}", dive_1(input));
    println!("{}", dive_2(input));
}

#[test]
fn test_example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(dive_1(input), 150);
    assert_eq!(dive_2(input), 900);
}
