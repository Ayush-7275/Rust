use std::any::type_name_of_val;
/*
  Problem 2: Shadowing Transformer

  Write a function that takes a u32, doubles it, then converts the result to a String.
  You should shadow the variable at each step within the function body.

  Run the tests for this problem with:
    cargo test --test shadowing_test
*/
fn converter( num : u32)->String{
    let num = num * 2;
    let num = num.to_string();
    return num;
    
}

fn main(){
    let x = 23423;
    let returned_value = converter(x);
    print!("The number passed to fn is {} and the returned value is {} and it type {}",x,returned_value,type_name_of_val(&returned_value))
}