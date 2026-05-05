/*
  Problem 2: Shadowing Transformer

  Write a function that takes a u32, doubles it, then converts the result to a String.
  You should shadow the variable at each step within the function body.

  Run the tests for this problem with:
    cargo test --test shadowing_test
*/

fn double_integer(x: u32) -> u32 {
  return x * 2;
}
pub fn shadow_transform(x: u32) -> String {
  let ans = double_integer(x);
  let y = ans.to_string();
  return y;
  todo!()
}
