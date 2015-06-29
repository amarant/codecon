//Problem        : Base Arithmetic
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT
use std::io;
use std::io::prelude::*;
use std::i64;

fn main() {
  let stdin = io::stdin();
  let first = stdin.lock().lines().next().unwrap().unwrap();
  let second = stdin.lock().lines().next().unwrap().unwrap();

  let res = base_arithmetic(first, second);
  println!("{}", res);
}

fn base_arithmetic(first: String, second: String) -> String{
  let first_num = i64::from_str_radix(first.as_ref(), find_min_base(&first).unwrap()).unwrap();
  let second_num = i64::from_str_radix(second.as_ref(), find_min_base(&second).unwrap()).unwrap();
  (first_num + second_num).to_string()
}

fn find_min_base(num: &String) -> Option<u32>{
  match num.chars().max() {
    Some(base) => match base.to_digit(16) {
      Some(0) => Some(2),
      Some(digit) => Some(digit+1),
      _ => None
    },
    _ => None
  }
}

#[test]
fn find_min_base_test() {
  assert_eq!(find_min_base(&"01".to_string()).unwrap(), 2);
  assert_eq!(find_min_base(&"1A".to_string()).unwrap(), 11);
  assert_eq!(find_min_base(&"0".to_string()).unwrap(), 2);
}
