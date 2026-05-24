/*
  Problem 4: Array Sum

  Write a function that takes a reference to an array of exactly 5 i32 values and returns their sum.

  Run the tests for this problem with:
    cargo test --test array_sum_test
*/

pub fn array_sum(arr: &[i32; 5]) -> i32 {
    let arr_iter = arr.iter();
    let sum = arr_iter.sum();
    sum
}

fn main(){
  let nums = [1,2,3,4,5];
  print!("{}",array_sum(&nums)); 
}