use std::{fs::File, io::Read};

use advent_of_code_2024::All;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let mut input = File::open(filename)?;
    let mut haystack = String::new();
    input.read_to_string(&mut haystack)?;
    let re = Regex::new(r"mul\((\d{1, 3}),(\d{1, 3})\)").unwrap();
    let All(res): All<u32, anyhow::Error> = re
        .captures_iter(&haystack)
        .map(|capture| {
            let first: u32 = capture
                .get(1)
                .expect("no first group in a capture with two capture groups")
                .as_str()
                .parse()?;
            let second: u32 = capture
                .get(2)
                .expect("no second group in a capture with two capture groups")
                .as_str()
                .parse()?;
            anyhow::Ok(first * second)
        })
        .map(All)
        .sum();

    let total_sum = res?;
    println!("sum of multiplications: {total_sum}");
    Ok(())
}
