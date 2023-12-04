fn solve1(input: &str) -> u32 {
    let into_numbers = |s: &str| {
        s.trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>()
    };
    let count = |w: &[u8], p: &[u8]| {
        let c = p.iter().filter(|n| w.contains(n)).count();
        match c {
            0 => 0,
            _ => 2u32.pow(c as u32 - 1),
        }
    };

    input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .map(|l| l.split_once('|').unwrap())
        .map(|(w, p)| (into_numbers(w), into_numbers(p)))
        .map(|(w, p)| count(&w, &p))
        .sum()
}

fn solve2(input: &str) -> u32 {
    let into_numbers = |s: &str| {
        s.trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>()
    };
    let cards = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .map(|l| l.split_once('|').unwrap())
        .map(|(w, p)| (into_numbers(w), into_numbers(p)))
        .map(|(w, p)| p.iter().filter(|n| w.contains(n)).count() as u8)
        .collect::<Vec<_>>();

    fn pick(v: &[u8], i: u8) -> u32 {
        let w = v[i as usize];
        if w > 0 {
            (0..w).map(|j| pick(v, i + j + 1)).sum::<u32>() + 1
        } else {
            1
        }
    }

    (0..cards.len() as u8).map(|i| pick(&cards, i)).sum()
}

fn main() {
    let input = include_str!("../inputs/04.txt");
    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
}

#[test]
fn test1() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(solve1(input), 13);
}

#[test]
fn test2() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(solve2(input), 30);
}
