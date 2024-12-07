use std::collections::HashSet;

use advent_of_code_2024::grid::{Grid, Shift};
use anyhow::Context;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl From<Direction> for Shift {
    fn from(value: Direction) -> Self {
        let (x, y) = match value {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };
        Shift { x, y }
    }
}

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = std::fs::read_to_string(filename)?;
    let grid = input.lines().map(|x| x.to_string()).collect::<Vec<_>>();
    let grid = Grid(grid);
    let start = grid.find('^').context("no starting position in input")?;
    let mut current = Some(start);
    let mut direction = Direction::Up;

    let path = std::iter::from_fn(|| {
        let visited = current?;
        let mut next = current? + direction.into();
        while let Some('#') = grid.get(next) {
            direction = direction.turn();
            next = current? + direction.into();
        }
        // have to check that next position is still within grid
        current = grid.get(next).map(|_| next);
        Some(visited)
    });
    let positions: HashSet<_> = path.collect();
    println!("total distinct positions: {}", positions.len());

    Ok(())
}
