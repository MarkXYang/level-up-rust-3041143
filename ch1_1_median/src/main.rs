
fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }
    a.sort_by( |x: &f32, y: &f32| { x.partial_cmp(y).unwrap()});
    let n_elements: usize = a.len();
    let middle: usize = n_elements / 2;
    let a_is_even: bool = n_elements %2 == 0;

    let med = if a_is_even {
        (a[middle] + a[middle - 1]) / 2.0
    }
    else {
        a[middle]
    };
    Some(med)
}
fn main() {
    let answer: Option<f32> = median(vec![1.0, 2.0, 5.0]);

    println!("median([1, 2, 5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input: Vec<f32> = vec![];
    let expected_output: Option<f32> = None;
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input: Vec<f32> = vec![1.0, 4.0, 5.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}
#[test]
fn unsorted_list() {
    let input: Vec<f32> = vec![2.0, 1.0, 5.0, 4.0];
    let expected_output: Option<f32> = Some(3.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}
