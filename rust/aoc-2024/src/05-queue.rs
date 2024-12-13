mod input_loader;

// #[derive(Debug)]
// struct Rule {
//     pub n: i32,
//     pub bound: i32,
// }

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = input.lines().collect::<Vec<_>>();
    let split_pos = lines.iter().position(|s| s.is_empty()).unwrap();
    let data_lines = lines.split_off(split_pos);
    let rule_lines = lines;

    let rules = rule_lines
        .into_iter()
        .map(|s| {
            let mut nums = s.split('|').map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect::<Vec<_>>();

    let data = data_lines
        .into_iter()
        .skip(1)
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .collect();

    (rules, data)
}

fn is_ordered(rules: &Vec<(i32, i32)>, update: &Vec<i32>, (i, n): (usize, &i32)) -> bool {
    let rules = rules
        .iter()
        .filter(|r| r.0 == *n)
        .map(|r| r.1)
        .collect::<Vec<_>>();
    !update.iter().take(i).any(|p| rules.contains(p))
}

fn part1(input: &str) -> i32 {
    let (rules, data) = parse(input);

    data.into_iter()
        .filter(|update| {
            update
                .iter()
                .enumerate()
                .skip(1)
                .all(|t| is_ordered(&rules, update, t))
        })
        .map(|v| v[v.len() / 2])
        .sum()
}

fn part2(input: &str) -> i32 {
    let (rules, data) = parse(input);

    data.into_iter()
        .filter(|update| {
            update
                .iter()
                .enumerate()
                .skip(1)
                .any(|t| !is_ordered(&rules, update, t))
        })
        .inspect(|val| println!("uns {val:?}"))
        .map(|update| {
            let mut rules = rules
                .iter()
                .filter(|r| update.contains(&r.0) && update.contains(&r.1))
                .collect::<Vec<_>>();

            rules.sort_by(|l, r| l.0.cmp(&r.1));
            rules.into_iter().map(|r| r.0).collect::<Vec<_>>()
        })
        .inspect(|val| println!("sor {val:?}"))
        .map(|v| v[v.len() / 2])
        .sum()
}

fn main() {
    let input = input_loader::load(5);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

#[test]
fn test1() {
    assert_eq!(part1(TEST_INPUT), 143);
}

#[test]
fn test2() {
    assert_eq!(part2(TEST_INPUT), 123);
}
