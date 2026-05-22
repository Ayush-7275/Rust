/*
  Problem 1: Sum of Two Numbers

  Write a function that takes two i32 integers and returns their sum.

  Run the tests for this problem with:
    cargo test --test sum_test
*/
fn sum(a : i32 , b:i32)->i32{
    let sum = a+b;
    return sum;
}


fn main(){
  print!("The sum of the number is -> {}",sum(2948,012973))
}