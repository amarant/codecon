//Problem        : Expecto Palindronum
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let word = stdin.lock().lines().next().unwrap().unwrap();
  let l = word.len();
  for index in (1..l).rev() {
    let take = index/2 + 1;
    let forward = word.chars().take(take);
    let reverse = word.chars().rev().skip(l-index-1).take(take);
    let mut both = forward.zip(reverse);
    if both.all(|(f, r)| {f == r}) {
      println!("{}", 2*word.len() - index -1);
      return;
    }
  }
  
  println!("{}", word.len()*2-1);
}
