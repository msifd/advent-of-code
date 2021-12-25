use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    East,
    South,
    None,
}

struct Map {
    cols: usize,
    data: Vec<Cell>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap().len();
        let data = input
            .chars()
            .map(|c| match c {
                '>' => Some(Cell::East),
                'v' => Some(Cell::South),
                '.' => Some(Cell::None),
                _ => None,
            })
            .filter_map(|o| o)
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

    fn simulate(&mut self, filter: Cell) -> bool {
        let mut next = self.data.clone();
        let mut moved = false;

        for pos in 0..self.data.len() {
            if self.data[pos] != filter {
                continue;
            }

            let move_pos = match filter {
                Cell::East => self.pos_to_east(pos),
                Cell::South => self.pos_to_south(pos),
                _ => unreachable!(),
            };

            if self.data[move_pos] == Cell::None {
                next[pos] = Cell::None;
                next[move_pos] = filter;
                moved = true;
            }
        }

        self.data = next;
        moved
    }

    pub fn step(&mut self) -> bool {
        let mut moved = false;
        moved |= self.simulate(Cell::East);
        moved |= self.simulate(Cell::South);
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
            write!(f, "{}", match self.data[pos] {
                Cell::East => ">",
                Cell::South => "v",
                Cell::None => ".",
            })?;
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
