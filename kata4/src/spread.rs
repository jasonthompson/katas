// Part Three: DRY Fusion
//
// Take the two programs written previously and factor
// out as much common code as possible, leaving you with two smaller programs
// and some kind of shared functionality.
//
use std::fmt;
use std::num;
use std::cmp;

pub trait Spread<T> {
  fn spread(&self) -> int {}
}

enum SpreadType {
  LOWEST,
  HIGHEST
}

impl Eq for Spread {}

impl PartialEq for Box<Spread> {
  fn eq<'a>(&self, other: &'a Spread) -> bool {
    self.spread == other.spread
  }
}

impl Ord for Box<Spread> {
  fn cmp(&self, other: Box<Spread>) -> Ordering {
    self.spread.cmp(&other.spread)
  }
}

impl cmp::PartialOrd for Box<Spread> {
  fn partial_cmp(&self, other: Box<Spread>) -> Option<Ordering> {
    self.spread.partial_cmp(&other.spread)
  }
}

fn find_spread(mut list: Box<Spread>, spread_type: SpreadType) -> Box<Spread> {
  list.sort();
  match spread_type {
    LOWEST => list.pop().unwrap(),
    HIGHEST => list.remove(0).unwrap()
  }
}

mod tests {
  use std::num;

  struct TestSpread {
    low: int,
    high: int
  }


  impl super::Spread for TestSpread {
    fn spread(&self) -> int {
      num::abs(self.high - self.low)
    }
  }

  #[test]
  fn test_find_spread() {
    let s = TestSpread { high: 10, low: 4 };
    assert_eq!(6, s.spread());
  }
}
