use anyhow::anyhow;
use std::{fs::File, io::Read, str::FromStr};

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

    let updates: Vec<Update> = parts
        .next()
        .expect("rules and updates should be sparated by an empty line")
        .lines()
        .map(|update| {
            update
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<u8>, _>>()
        })
        .collect::<Result<_, _>>()?;

    let fixed_updates = updates.into_iter().flat_map(|mut update| {
        if check_update(&update, &rules).is_none() {
            return None;
        }
        while let Some((i, j)) = check_update(&update, &rules) {
            update.swap(i, j);
        }
        Some(update)
    });

    let total_sum: u32 = fixed_updates
        .map(|update| update[update.len() / 2] as u32)
        .sum();
    println!("sum for fixed updates: {total_sum}");

    Ok(())
}

fn check_update(update: &Update, rules: &[Rule]) -> Option<(usize, usize)> {
    rules
        .iter()
        .flat_map(|rule| {
            let (before_postition, _) =
                update.iter().enumerate().find(|(_, &x)| x == rule.before)?;
            let (after_position, _) = update.iter().enumerate().find(|(_, &x)| x == rule.after)?;
            // find rule violation here
            (before_postition > after_position).then_some((before_postition, after_position))
        })
        .next()
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
