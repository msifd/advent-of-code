fn get_checksum(input: &str, pt2: bool) -> u32 {
    let input = input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn get_max_dist(row: &Vec<u32>) -> u32 {
        let mut min = u32::max_value();
        let mut max = 0;
        for &v in row {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        max - min
    }

    fn get_division(row: &Vec<u32>) -> u32 {
        for &i in row {
            for &j in row {
                if i == j {
                    continue;
                }
                if i % j == 0 {
                    return i / j;
                }
            }
        }
        0
    }

    let mapper = if !pt2 { get_max_dist } else { get_division };
    input.iter().map(mapper).sum()
}

pub fn day2() {
    let input = include_str!("inputs/day2.txt");

    println!(
        "Day 2 - {} - {}",
        get_checksum(input, false),
        get_checksum(input, true)
    );
}

#[test]
fn test_pt1() {
    let input = "
        5 1 9 5
        7 5 3
        2 4 6 8
        ";
    assert_eq!(get_checksum(input, false), 18);
}

#[test]
fn test_pt2() {
    let input = "
        5 9 2 8
        9 4 7 3
        3 8 6 5
        ";
    assert_eq!(get_checksum(input, true), 9);
}
