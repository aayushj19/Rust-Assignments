/*
  Problem 9: Borrow and Count

  Write a function that takes an immutable reference to a Vec<i32> and returns the count of elements greater than a given threshold.
  This exercises borrowing without taking ownership.

  Run the tests for this problem with:
    cargo test --test borrow_count_test
*/

fn check(values: &Vec<i32>, threshold: i32) -> usize {
  let mut count = 0;
  for v in values {
    if *v > threshold {
      count +=1;
    }
  }
  return count;
}

pub fn count_above(values: &Vec<i32>, threshold: i32) -> usize {
    return check(values, threshold);
    todo!()
}
