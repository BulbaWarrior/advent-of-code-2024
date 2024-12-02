use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut values = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            let left = values.next().unwrap();
            let right = values.next().unwrap();
            (left, right)
        })
        .unzip();

    list1.sort();
    list2.sort();

    let total_distance: u32 = list1
        .into_iter()
        .zip(list2)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("total_distance: {total_distance}");
}
