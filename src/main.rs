/// Return the median of a sequence of `f32`'s.
fn median(a: &mut [f32]) -> Option<f32> {
    match a.len() {
        0 => None, // empty vector
        n => {
            a.sort_by(|a, b| a.partial_cmp(b).unwrap()); // panic on NaN
            let mid = n / 2; // find the middle index
            match (n % 2) == 0 {
                true => {
                    // even number of elements
                    let ave = (a[mid - 1] + a[mid]) / 2.0;
                    Some(ave)
                }
                false => {
                    // odd number of elements
                    Some(a[mid])
                }
            }
        }
    }
}

fn main() {
    let answer = median(&mut [1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let mut input = vec![];
    let expected_output = None;
    let actual_output = median(&mut input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    // Experiment with defining a slice for this test:
    let input = &mut [1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let mut input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(&mut input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let mut input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(&mut input);
    assert_eq!(actual_output, expected_output);
}
