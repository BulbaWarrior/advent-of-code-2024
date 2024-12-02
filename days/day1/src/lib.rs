use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

pub fn read_numbers(filename: &str) -> anyhow::Result<(Vec<u32>, Vec<u32>)> {
    let input = File::open(filename)?;
    let input = BufReader::new(input);

    let pairs = input.lines().map(|line| {
        let line = line?;
        let mut values = line
            .split_whitespace()
            .map(|x| anyhow::Ok(x.parse::<u32>()?));
        let format_message = "badly formated line: each line should contain two numbers";
        let left = values.next().context(format_message)??;
        let right = values.next().context(format_message)??;
        anyhow::Ok((left, right))
    });

    let mut list1 = vec![];
    let mut list2 = vec![];

    for pair in pairs {
        let (left, right) = pair?;
        list1.push(left);
        list2.push(right);
    }
    Ok((list1, list2))
}
