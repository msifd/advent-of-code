fn solve1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .enumerate()
        .filter(|(_i, l)| {
            l.split_terminator(';').all(|subset| {
                subset.split_terminator(',').all(|pick| {
                    let mut it = pick.trim().split_terminator(' ');
                    let number = it.next().unwrap().parse::<u32>().unwrap();
                    let color = it.next().unwrap();
                    match color {
                        "red" if number <= 12 => true,
                        "green" if number <= 13 => true,
                        "blue" if number <= 14 => true,
                        _ => false,
                    }
                })
            })
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .map(|l| {
            l.split_terminator(';')
                .flat_map(|subset| {
                    subset.split_terminator(',').map(|pick| {
                        let mut it = pick.trim().split_terminator(' ');
                        let number = it.next().unwrap().parse::<u32>().unwrap();
                        let color = it.next().unwrap();
                        (number, color)
                    })
                })
                .fold([0; 3], |mut acc, (n, c)| {
                    match c {
                        "red" => acc[0] = acc[0].max(n),
                        "green" => acc[1] = acc[1].max(n),
                        "blue" => acc[2] = acc[2].max(n),
                        _ => unreachable!(),
                    }
                    acc
                })
                .into_iter()
                .reduce(|a, b| a * b)
                .unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../inputs/02.txt");
    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
}

#[test]
fn test1() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(solve1(input), 8);
}

#[test]
fn test2() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(solve2(input), 2286);
}
