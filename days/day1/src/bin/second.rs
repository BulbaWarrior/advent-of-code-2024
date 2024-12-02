use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let (list1, list2) = day1::read_numbers("input.txt")?;

    let mut counts = HashMap::new();
    for num in list2 {
        counts.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    let similarity: u32 = list1
        .into_iter()
        .map(|x| x * counts.get(&x).unwrap_or(&0))
        .sum();
    println!("total_similarity: {similarity}");

    Ok(())
}
