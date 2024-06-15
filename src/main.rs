use std::{collections::HashSet, hash::Hash};

// fn unique(a: Vec<i32>) -> Vec<i32> {
//     let set: HashSet<i32> = a.into_iter().collect();
//     let uvec: Vec<i32> = set.into_iter().collect();
//     uvec
// }

// advanced 1: use generic types
fn unique<T>(a: Vec<T>) -> Vec<T>
    where T: Hash + Eq
{
    let set: HashSet<T> = a.into_iter().collect();
    let uvec: Vec<T> = set.into_iter().collect();
    uvec
}

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

// TODO: Implement unique<T> for PartialOrd
// #[test]
// fn unsorted_list_with_duplicates_f64() {
//     let input = vec![1.0, 5.0, 2.0, 2.0, 1.0];
//     let mut expected_output = vec![1.0, 2.0, 5.0];
//     let mut actual_output = unique(input);
//     expected_output.sort_by(|a,b| a.partial_cmp(b).unwrap());
//     actual_output.sort_by(|a,b| a.partial_cmp(b).unwrap());
//     assert_eq!(actual_output, expected_output);
// }

#[test]
fn unsorted_list_with_duplicates_str() {
    let input = vec!["1", "5", "2", "2", "1"];
    let mut expected_output = vec!["1", "2", "5"];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
