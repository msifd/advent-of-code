use std::fmt::Display;

struct Map {
    cols: usize,
    data: Vec<char>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap().len();
        let data = input
            .chars()
            .filter(|c| match c {
                '>' | 'v' | '.' => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        Self { cols, data }
    }

    fn pos_to_east(&self, pos: usize) -> usize {
        let x = pos % self.cols;
        if x + 1 < self.cols {
            pos + 1
        } else {
            pos - x
        }
    }

    fn pos_to_south(&self, pos: usize) -> usize {
        let to_south = pos + self.cols;
        if to_south < self.data.len() {
            to_south
        } else {
            pos % self.cols
        }
    }

    fn simulate(&mut self, filter: char) -> bool {
        let mut next = self.data.clone();
        let mut moved = false;

        for pos in 0..self.data.len() {
            if self.data[pos] != filter {
                continue;
            }

            let move_pos = match filter {
                '>' => self.pos_to_east(pos),
                'v' => self.pos_to_south(pos),
                _ => unreachable!(),
            };

            if self.data[move_pos] == '.' {
                next[pos] = '.';
                next[move_pos] = filter;
                moved = true;
            }
        }

        self.data = next;
        moved
    }

    pub fn step(&mut self) -> bool {
        let mut moved = false;
        moved |= self.simulate('>');
        moved |= self.simulate('v');
        moved
    }

    pub fn run(&mut self) -> u32 {
        let mut step = 1;
        while self.step() {
            step += 1;
        }
        step
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pos in 0..self.data.len() {
            write!(f, "{}", self.data[pos])?;
            if (pos + 1) % self.cols == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../res/25-input.txt");
    println!("{}", Map::new(input).run());
}

#[test]
fn test_example() {
    let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    assert_eq!(Map::new(input).run(), 58);
}
