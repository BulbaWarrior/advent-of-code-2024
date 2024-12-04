use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;
use day4::{Grid, Position, Shift};

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = File::open(filename)?;
    let input = BufReader::new(input);
    let lines: Result<Vec<_>, _> = input.lines().collect();
    let grid = Grid(lines?);
    let target = "MAS";
    if target.len() % 2 == 0 {
        return Err(anyhow!(
            "don't know how to look for X-patterns of even-length words"
        ));
    }
    let center_letter = target
        .chars()
        .nth(target.len() / 2)
        .expect("No first letter in nonempty string");

    let positions = grid.0.iter().enumerate().flat_map(|(y, row)| {
        row.char_indices().flat_map(move |(x, char)| {
            (char == center_letter).then_some(Position {
                x: x as i32,
                y: y as i32,
            })
        })
    });

    let total_count: usize = positions.map(|pos| grid.check_position(target, pos)).sum();

    println!("total number of words '{target}': {total_count}");

    Ok(())
}

trait CheckPos {
    fn check_position(&self, target: &str, position: Position) -> usize;
}

impl CheckPos for Grid {
    fn check_position(&self, target: &str, position: Position) -> usize {
        if target.len() % 2 == 0 {
            panic!("Don't know how to check X-pattern for even-length words");
        }
        let half_len = target.len() as i8 / 2; //only valid for short words, but whatever
        let shifts = [
            Shift { x: 1, y: -1 },  // up-right
            Shift { x: 1, y: 1 },   // down-right
            Shift { x: -1, y: 1 },  // down-left
            Shift { x: -1, y: -1 }, // up-left
        ];

        let count = shifts
            .into_iter()
            .map(|shift| (-half_len..=half_len).map(move |n| position + shift * n))
            .flat_map(|word_index| {
                let word: Option<String> = word_index.map(|pos| self.get(pos)).collect();
                (word? == target).then_some(())
            })
            .count();

        match count {
            n if n < 2 => 0,
            n if n == 2 => 1,
            _ => unreachable!("it's impossible to get more than two words in an X-pattern"),
        }
    }
}
