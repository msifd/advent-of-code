fn simulate(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("../res/25-input.txt");
    println!("{}", simulate(input));
}

#[test]
fn example() {
    let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    assert_eq!(simulate(input), 58);
}
