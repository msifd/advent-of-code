mod input_loader;

fn is_xmas_arr(s: &[char; 4]) -> bool {
    *s == ['X', 'M', 'A', 'S'] || *s == ['S', 'A', 'M', 'X']
}

fn part1(input: &str) -> usize {
    let chars = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    let side = input.lines().next().unwrap().len();
    let to_chars = |p: [usize; 4]| p.map(|pos| chars[pos]);

    let up = (0..side)
        .flat_map(|y| (0..side - 3).map(move |x| [0, 1, 2, 3].map(|o| (x + o) + y * side)));

    let right = (0..side)
        .flat_map(|y| (0..side - 3).map(move |x| [0, 1, 2, 3].map(|o| (x + o) * side + y)));

    let left_diag = (3..(side + side - 4)).flat_map(|r| {
        let is_bottom = r / side;
        let y = r - (r % side) * is_bottom - 1 * is_bottom;
        let w = (r) + (side - r % side - 1) * is_bottom - (r + 1) * is_bottom - 2;
        let s = r / side + (r % side) * is_bottom;

        (0..w).map(move |c| {
            [0, 1, 2, 3].map(|o| {
                let x = s + c + o;
                let y = y - c - o;
                x + y * side
            })
        })
    });

    let right_diag = (3..(side + side - 4)).flat_map(|r| {
        let is_bottom = r / side;
        let y = r - (r % side) * is_bottom - 1 * is_bottom;
        let w = (r) + (side - r % side - 1) * is_bottom - (r + 1) * is_bottom - 2;
        let s = r / side + (r % side) * is_bottom;

        (0..w).map(move |c| {
            [0, 1, 2, 3].map(|o| {
                let x = side - s - c - o - 1;
                let y = y - c - o;
                x + y * side
            })
        })
    });

    up.chain(right)
        .chain(left_diag)
        .chain(right_diag)
        .map(to_chars)
        // .inspect(|a| println!("-- {a:?}"))
        .filter(is_xmas_arr)
        .count()
}

fn is_x_mas_arr(s: &[char; 5]) -> bool {
    *s == ['M', 'S', 'A', 'M', 'S']
        || *s == ['M', 'M', 'A', 'S', 'S']
        || *s == ['S', 'M', 'A', 'S', 'M']
        || *s == ['S', 'S', 'A', 'M', 'M']
}

fn part2(input: &str) -> usize {
    let chars = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    let side = input.lines().next().unwrap().len() as isize;
    let to_chars = |p: [isize; 5]| p.map(|pos| chars[pos as usize]);

    let x_mas = (1..side - 1).flat_map(|y| {
        (1..side - 1)
            .map(move |x| [-side - 1, -side + 1, 0, side - 1, side + 1].map(|o| x + y * side + o))
    });

    x_mas
        .map(to_chars)
        // .inspect(|a| println!("-- {a:?}"))
        .filter(is_x_mas_arr)
        .count()
}

fn main() {
    let input = input_loader::load(4);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[test]
fn test1() {
    assert_eq!(part1(TEST_INPUT), 18);
}

#[test]
fn test2() {
    assert_eq!(part2(TEST_INPUT), 9);
}
