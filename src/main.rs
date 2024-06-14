fn median(a: Vec<f32>) -> Option<f32> {
    match a.len() {
        0 => None, // empty vector
        n => {
            let mut a = a.clone(); // create a mutable vector for sorting
            a.sort_by(|a, b| a.partial_cmp(b).unwrap());
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
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
