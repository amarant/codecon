//Problem        : Messed up Rugby
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT

use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Error};
use std::collections::BTreeSet;

fn main() {
  let stdin = io::stdin();
  let mut x = stdin.lock().lines();
  let val = x.next().unwrap().unwrap();
  if let Ok(target_score) = val.parse::<u16>() {
    let target_scores = find_score(target_score);
    for score in target_scores {
      println!("{:?} {:?} {:?}", score.conversions, score.tries, score.kicks);
    }
  }
}

#[derive(Hash, Eq, Clone)]
struct HandEggScore {
  conversions: u16,
  tries: u16,
  kicks: u16,
  //scoreType: ScoreType,
}

impl HandEggScore {
  fn new(conversions: u16, tries: u16, kicks: u16,) -> HandEggScore {
    HandEggScore { conversions:conversions, tries:tries, kicks:kicks }
  }
  fn points(&self) -> u16 {
    //println!("{:?}, {:?}, {:?}", self.conversions, self.tries, self.kicks);
    self.conversions * 7 + self.tries * 3 + self.kicks * 2
  }
}

impl PartialEq for HandEggScore {
  fn eq(&self, other: &HandEggScore) -> bool {
    self.conversions == other.conversions
    && self.tries == other.tries
    && self.kicks == other.kicks
  }
}

impl Ord for HandEggScore {
    fn cmp(&self, other: &HandEggScore) -> Ordering {
      match self.conversions.cmp(&other.conversions) {
        Ordering::Equal => match self.tries.cmp(&other.tries) {
          Ordering::Equal => self.tries.cmp(&other.tries),
          a => return a,
        },
        a => return a,
      }
    }
}

impl PartialOrd for HandEggScore {
  fn partial_cmp(&self, other: &HandEggScore) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Debug for HandEggScore {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{} {} {}",
      self.conversions, self.tries, self.kicks)
  }
}

fn find_score(target_score: u16) -> Vec<HandEggScore> {
  let mut target_scores : BTreeSet<HandEggScore> = BTreeSet::new();
  for conversion in 0..(target_score/7 + 1) {
    for trie in 0..((target_score - conversion*7)/3 + 1) {
      for kick in 0..((target_score - conversion*7 - trie*3)/2 + 1) {
        let score = HandEggScore::new(conversion,
          trie, kick);
        match score.points().cmp(&target_score) {
          Ordering::Greater => {break;},
          Ordering::Equal => {target_scores.insert(score);},
          _ => (),
        }
      }
    }
  }
  if target_scores.is_empty() {
    vec!(HandEggScore::new(0, 0, 0))
  } else {
    target_scores.into_iter().collect()
  }
}

#[test]
fn test_sample1() {
  let test_result = vec![
    HandEggScore::new(0, 0, 5),
    HandEggScore::new(0, 2, 2),
    HandEggScore::new(1, 1, 0),
  ];
  let score = find_score(10);
  assert_eq!(test_result, score);
}

#[test]
fn test_sample2() {
  let test_result = vec![
    HandEggScore::new(0, 1, 7),
    HandEggScore::new(0, 3, 4),
    HandEggScore::new(0, 5, 1),
    HandEggScore::new(1, 0, 5),
    HandEggScore::new(1, 2, 2),
    HandEggScore::new(2, 1, 0),
  ];
  let score = find_score(17);
  assert_eq!(test_result, score);
}

#[bench]
fn bench_for_0_222(b: &mut Bencher) {
  b.iter(|| {
    for i in 0..223 {
      find_score(i);
    }
  });
}

