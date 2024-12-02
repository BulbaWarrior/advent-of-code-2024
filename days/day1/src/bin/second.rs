use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let (list1, list2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut values = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            let left = values.next().unwrap();
            let right = values.next().unwrap();
            (left, right)
        })
        .unzip();

    let mut counts = HashMap::new();
    for num in list2 {
        counts.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    let similarity: u32 = list1
        .into_iter()
        .map(|x| x * counts.get(&x).unwrap_or(&0))
        .sum();
    println!("total_similarity: {similarity}");
}
