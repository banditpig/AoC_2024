use std::collections::{ HashMap};
use sortedlist_rs::SortedList;
use crate::utils::load_input;
fn count_occurrences(numbers: Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts
}
// Advent of Code 2020 - Day 1
pub fn part2() {
    let data = load_input("../data/day1.txt");
    let (left, right) = make_lists(&data);
    let occurrences = count_occurrences(right.to_vec());

    let total = left.to_vec().iter().fold(0, |mut acc, x| {
        if let Some(c) = occurrences.get(x) {
            acc += x * *c as i32;
        }
        acc
    });
    println!("{}", total);

}
pub fn part1() {
    let data = load_input("../data/day1.txt");
    let (left, right) = make_lists(&data);

    let mut total = 0;

    (0..data.len()).for_each(|ix| {
        total += left[ix].abs_diff(right[ix]);
    });
    println!("{}", total);


}

fn make_lists(data: &Vec<&str>) -> (SortedList<i32>, SortedList<i32>) {
    let (left, right): (Vec<i32>, Vec<i32>) = data
        .into_iter()
        .map(|x| {
            let mut nums = x.splitn(2, "   ").map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    (SortedList::from(left), SortedList::from(right))
}