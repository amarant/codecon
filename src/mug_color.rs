//Problem        : Mug Color
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let length_string = stdin.lock().lines().next().unwrap().unwrap();
  let length = length_string.parse::<i32>().unwrap();
  let mut is_colors = [true, true, true, true, true];
  for _ in 0..length {
    match stdin.lock().lines().next().unwrap().unwrap().as_ref() {
      "White" => is_colors[0] = false,
      "Yellow" => is_colors[1] = false,
      "Blue" => is_colors[2] = false,
      "Black" => is_colors[3] = false,
      "Red" => is_colors[4] = false,
      _ => (),
    }
  }

  let index = is_colors.iter()
    .position(|value| *value).unwrap();
  match index {
    0 => println!("White"),
    1 => println!("Yellow"),
    2 => println!("Blue"),
    3 => println!("Black"),
    4 => println!("Red"),
    _ => ()
  }
}
