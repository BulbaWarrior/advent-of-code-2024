use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2024::All;
use anyhow::{Context, Ok};

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = File::open(filename)?;
    let input = BufReader::new(input);
    let reports = input.lines().map(|line| {
        let report = line?
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Report, _>>();
        anyhow::Ok(report?)
    });

    let All(safe_reports): All<u32, _> = reports
        .map(|report| {
            let safe = match report_safe(report?).context("empty report")? {
                false => 0,
                true => 1,
            };
            Ok(safe)
        })
        .map(All)
        .sum();

    let safe_reports = safe_reports?;
    println!("total safe reports: {safe_reports}");
    Ok(())
}

type Report = Vec<u32>;

fn report_safe(report: Report) -> Option<bool> {
    match &report[..] {
        [] => None,
        [_] => Some(true),
        [first, second, ..] => {
            let direction = match first.cmp(second) {
                // the difference has to be at least 1,
                // so two equals in a row is unsafe
                Ordering::Equal => return Some(false),
                other => other,
            };

            Some(
                report
                    .iter()
                    .zip(&report[1..])
                    .all(|(prev, next)| prev.cmp(next) == direction && prev.abs_diff(*next) <= 3),
            )
        }
    }
}
