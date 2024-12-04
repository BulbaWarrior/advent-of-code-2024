use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = File::open(filename)?;
    let input = BufReader::new(input);
    let lines: Result<Vec<_>, _> = input.lines().collect();
    let grid = Grid(lines?);
    let target = "XMAS";
    let first_letter = target
        .chars()
        .next()
        .expect("No first letter in nonempty string");

    let positions = grid.0.iter().enumerate().flat_map(|(y, row)| {
        row.char_indices().flat_map(move |(x, char)| {
            (char == first_letter).then_some(Position {
                x: x as i32,
                y: y as i32,
            })
        })
    });

    let total_count: usize = positions.map(|pos| grid.check_position(target, pos)).sum();

    println!("total number of words '{target}': {total_count}");

    Ok(())
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Shift {
    x: i8,
    y: i8,
}

impl std::ops::Add<Shift> for Position {
    type Output = Position;

    fn add(self, rhs: Shift) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x + rhs.x as i32,
            y: y + rhs.y as i32,
        }
    }
}

impl std::ops::Mul<i8> for Shift {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

struct Grid(Vec<String>);
impl Grid {
    fn get(&self, Position { x, y }: Position) -> Option<char> {
        let y: usize = y.try_into().ok()?;
        let x: usize = x.try_into().ok()?;
        let row = self.0.get(y)?;
        row.chars().nth(x)
    }

    fn check_position(&self, target: &str, position: Position) -> usize {
        let len = target.len();
        let shifts = [
            Shift { x: 0, y: -1 },  // up
            Shift { x: 1, y: -1 },  // up-right
            Shift { x: 1, y: 0 },   // right
            Shift { x: 1, y: 1 },   // down-right
            Shift { x: 0, y: 1 },   // down
            Shift { x: -1, y: 1 },  // down-left
            Shift { x: -1, y: 0 },  // left
            Shift { x: -1, y: -1 }, // up-left
        ];

        shifts
            .into_iter()
            .map(|shift| (0..len as i8).map(move |n| position + shift * n))
            .flat_map(|word_index| {
                let word: Option<String> = word_index.map(|pos| self.get(pos)).collect();
                (word? == target).then_some(())
            })
            .count()
    }
}
