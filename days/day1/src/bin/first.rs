fn main() -> anyhow::Result<()> {
    let (mut list1, mut list2) = day1::read_numbers("input.txt")?;

    list1.sort();
    list2.sort();

    let total_distance: u32 = list1
        .into_iter()
        .zip(list2)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("total_distance: {total_distance}");
    Ok(())
}
