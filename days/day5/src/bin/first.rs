use std::{fs::File, io::Read, str::FromStr};

use advent_of_code_2024::All;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut parts = input.split("\n\n");
    let rules = parts
        .next()
        .expect("rules and updates should be sparated by an empty line");
    let rules: Vec<Rule> = rules
        .lines()
        .map(|rule| rule.parse())
        .collect::<Result<_, _>>()?;

    let All(result): All<u32, anyhow::Error> = parts
        .next()
        .expect("rules and updates should be sparated by an empty line")
        .lines()
        .map(|update| {
            let update = update
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<_, _>>()?;
            let update_valid = check_update(&update, &rules);
            let value = if update_valid {
                update[update.len() / 2] as u32
            } else {
                0
            };
            Ok(value)
        })
        .map(All)
        .sum();

    let total_sum = result?;
    println!("sum of valid updates: {total_sum}");

    Ok(())
}

fn check_update(update: &Update, rules: &[Rule]) -> bool {
    rules
        .iter()
        .flat_map(|rule| {
            let (before_postition, _) =
                update.iter().enumerate().find(|(_, &x)| x == rule.before)?;
            let (after_position, _) = update.iter().enumerate().find(|(_, &x)| x == rule.after)?;
            Some(before_postition < after_position)
        })
        .all(|x| x)
}
type Update = Vec<u8>;

struct Rule {
    before: u8,
    after: u8,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [before, after] = s.split("|").collect::<Vec<_>>()[..] else {
            return Err(anyhow!(
                "rules should contain two numbers separated by a pipe"
            ));
        };

        Ok(Self {
            before: before.parse()?,
            after: after.parse()?,
        })
    }
}
