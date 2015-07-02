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
  let pal_number = palindrom_anagram_number(word);
  println!("{}", pal_number);
}

fn palindrom_anagram_number(word: String) -> u64 {
  let mut char_set = HashMap::new();
  for character in word.chars() {
    let counter = char_set.entry(character).or_insert(0i8);
    *counter += 1i8;
  }
  let mut has_odd = false;
  let mut is_anagram = true;
  let mut duplicates = 1u64;
  for &count in char_set.values() {
    if count % 2 == 1 {
      if has_odd {
        is_anagram = false;
        break;
      } else {
        has_odd = true;
      }
    } else {
      duplicates *= factorial((count as u64) / 2u64);
    }
  }

  if is_anagram {
    factorial((word.len() / 2) as u64) / duplicates
  } else {
    0
  }
}

fn factorial(number: u64) -> u64 {
  if number <= 1 {
    number
  } else {
    number * factorial(number-1)
  }
}

#[test]
fn test_samples() {
  assert_eq!(2, palindrom_anagram_number("bbaa".to_string()));
  assert_eq!(0, palindrom_anagram_number("abcdef".to_string()));
  assert_eq!(6, palindrom_anagram_number("bbaacc".to_string()));
}
