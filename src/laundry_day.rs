//Problem        : Laundry Day
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT

use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;
use std::ascii::AsciiExt;

fn main() {
  let stdin = io::stdin();
  let mut clothes = Vec::<String>::new();
  while let Some(Ok(x)) = stdin.lock().lines().next() {
    clothes.push(x);
  }
  let sorted_clothes = laundry_sort(clothes);
  for x in sorted_clothes {
    println!("{}", x);
  }
}

fn laundry_sort(clothes: Vec<String>) -> Vec<String> {
  let mut grouped_clothes = BTreeMap::new();
  for clothe in clothes.into_iter() {
    let counter = grouped_clothes.entry(clothe.to_ascii_lowercase())
      .or_insert(0);
    *counter += 1;
  }

  let mut sorted_clothes = Vec::new();
  for clothe in grouped_clothes {
    let (name, count) = clothe;
    if name.contains("sock") {
      if count > 1 {
        sorted_clothes.push(format!("{}|{}", count / 2, name));
      }
      if count % 2 == 1 {
        sorted_clothes.push(format!("{}|{}", 0, name));
      }
    } else {
      sorted_clothes.push(format!("{}|{}", count, name));
    }
  }

  sorted_clothes
}

#[test]
fn test_sample() {
  let clothes = vec![
  "white shirt".to_string(),
  "polka dot sock".to_string(),
  "red sock".to_string(),
  "superhero shirt".to_string(),
  "torn jeans".to_string(),
  "polka dot sock".to_string(),
  "white shirt".to_string(),
  "polka dot sock".to_string(),
  ];
  let sorted_clothes_result = vec![
  "1|polka dot sock".to_string(),
  "0|polka dot sock".to_string(),
  "0|red sock".to_string(),
  "1|superhero shirt".to_string(),
  "1|torn jeans".to_string(),
  "2|white shirt".to_string(),
  ];
  let sorted_clothes = laundry_sort(clothes);
  assert_eq!(sorted_clothes_result, sorted_clothes);
}

#[test]
fn test_lexical_order() {
  let clothes = vec![
  "polka dot shirt".to_string(),
  "polka cot shirt".to_string(),
  ];
  let sorted_clothes_result = vec![
  "1|polka cot shirt".to_string(),
  "1|polka dot shirt".to_string(),
  ];
  let sorted_clothes = laundry_sort(clothes);
  assert_eq!(sorted_clothes_result, sorted_clothes);
}

#[test]
fn test_case() {
  let clothes = vec![
  "White shirt".to_string(),
  "polkA dot sock".to_string(),
  "red sOck".to_string(),
  "superhero shirt".to_string(),
  "torn jeans".to_string(),
  "polka dot sock".to_string(),
  "white shirt".to_string(),
  "polka dot sock".to_string(),
  ];
  let sorted_clothes_result = vec![
  "1|polka dot sock".to_string(),
  "0|polka dot sock".to_string(),
  "0|red sock".to_string(),
  "1|superhero shirt".to_string(),
  "1|torn jeans".to_string(),
  "2|white shirt".to_string(),
  ];
  let sorted_clothes = laundry_sort(clothes);
  assert_eq!(sorted_clothes_result, sorted_clothes);
}
