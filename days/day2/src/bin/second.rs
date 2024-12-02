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
        line.unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Report>()
    });

    let safe_reports: u32 = reports
        .map(|report| match report_safe(&report).unwrap() {
            false => {
                for i in 0..report.len() {
                    let mut tolerated = report.clone();
                    tolerated.remove(i);
                    if report_safe(&tolerated).unwrap() {
                        return 1;
                    }
                }
                0
            }
            true => 1,
        })
        .sum();

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
