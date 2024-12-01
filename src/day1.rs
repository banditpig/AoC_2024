use std::collections::{BTreeSet, HashMap};
use std::path::absolute;
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
    let mut data = load_input("../data/day1.txt");
    let (left, right) = make_lists(&data);
    let mut occurences    = count_occurrences(right.to_vec());

    let total = left.to_vec().iter().fold(0, |mut acc, x| {
        if let Some(c) = occurences.get(x) {
            acc += x * *c as i32;
        }
        acc
    });
    println!("{}", total);

}
pub fn part1() {
    let mut data = load_input("../data/day1.txt");
    let (left, right) = make_lists(&data);

    let mut total = 0;


    // let total = (0..data.len()).fold(0, |mut acc, ix| {
    //     acc += left[ix].abs_diff(right[ix]);
    //     acc
    // });

    //
    // for ix in 0 ..data.len() {
    //     total += left[ix].abs_diff(right[ix]);
    // }

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
// fn make_lists(data: &mut Vec<&str>) -> (SortedList<i32>, SortedList<i32>) {
//     let mut left = SortedList::new();
//     let mut right = SortedList::new();
//     data.iter().for_each(|x| {
//         //split x into two numbers
//         let nums = x.split("   ").collect::<Vec<&str>>();
//         let num1 = nums[0].parse::<i32>().unwrap();
//         let num2 = nums[1].parse::<i32>().unwrap();
//         left.insert(num1);
//         right.insert(num2);
//     });
//     (left, right)
// }