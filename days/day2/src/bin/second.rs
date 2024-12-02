use advent_of_code_2024::All;
use anyhow::Context;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = File::open(filename)?;
    let input = BufReader::new(input);
    let reports = input.lines().map(|line| {
        let res = line?
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Report, _>>();
        anyhow::Ok(res?)
    });

    let All(safe_reports): All<u32, _> = reports
        .map(|report| -> anyhow::Result<u32> {
            let report = report?;
            match report_safe(&report).context("found empty report")? {
                false => {
                    for i in 0..report.len() {
                        let mut tolerated = report.clone();
                        tolerated.remove(i);
                        if report_safe(&tolerated).context(
                            "empty report after removing one value, this shouldn't be pssible",
                        )? {
                            return Ok(1);
                        }
                    }
                    Ok(0)
                }
                true => Ok(1),
            }
        })
        .map(All)
        .sum();

    let safe_reports = safe_reports?;

    println!("total safe reports: {safe_reports}");
    Ok(())
}

type Report = Vec<u32>;

fn report_safe(report: &[u32]) -> Option<bool> {
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
