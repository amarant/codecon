//Problem        : A Compliance Problem
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
  let stdin = io::stdin();
  let word = stdin.lock().lines().next().unwrap().unwrap();
  if is_palindrom_anagram(word) {
    println!("yes");
  } else {
    println!("no");
  }
}

fn is_palindrom_anagram(word: String) -> bool {
  let mut char_set = HashMap::new();
  for character in word.chars() {
    let counter = char_set.entry(character).or_insert(0i8);
    *counter += 1i8;
  }
  let odds_count = char_set.values()
    .filter(|&count| count % 2 == 1)
    .count();
  odds_count <= 1
}

#[test]
fn test_samples() {
  assert_eq!(true, is_palindrom_anagram("aa".to_string()));
  assert_eq!(true, is_palindrom_anagram("abba".to_string()));
  assert_eq!(true, is_palindrom_anagram("nnmm".to_string()));
  assert_eq!(false, is_palindrom_anagram("trail".to_string()));
}
