// http://codekata.com/kata/kata04-data-munging/
//
// Kata04: Data Munging
//
// Martin Fowler gave me a hard time for Kata02, complaining that it was yet
// another single-function, academic exercise. Which, or course, it was. So this
// week let’s mix things up a bit.
//
// Here’s an exercise in three parts to do with real world data. Try hard not to
// read ahead—do each part in turn.
//
// Part One: Weather Data
//
// In weather.dat you’ll find daily weather data for Morristown, NJ for June 2002.
// Download this text file, then write a program to output the day number (column one)
// with the smallest temperature spread (the maximum temperature is the second column,
// the minimum the third column).
//
#![feature(globs)]

use std::num;
use std::io::File;
use std::fmt;

pub struct DailyTempSpread {
  date: int,
  max: int,
  min: int,
  temp_spread: int
}

impl DailyTempSpread {
  fn new(day: int, max: int, min: int) -> DailyTempSpread {
    DailyTempSpread { date: day,
    max: max,
    min: min,
    temp_spread: max - min
    }
  }
}

impl std::cmp::PartialOrd for DailyTempSpread {
  fn partial_cmp(&self, other: &DailyTempSpread) -> Option<Ordering> {
    self.temp_spread.partial_cmp(&other.temp_spread)
  }
}

impl fmt::Show for DailyTempSpread {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {}, {})",
    self.date, self.max, self.min, self.temp_spread)
  }
}

impl Ord for DailyTempSpread {
  fn cmp(&self, other: &DailyTempSpread) -> Ordering {
    self.temp_spread.cmp(&other.temp_spread)
  }
}

impl Eq for DailyTempSpread {}

impl PartialEq for DailyTempSpread {
  fn eq(&self, other: &DailyTempSpread) -> bool {
    self.temp_spread == other.temp_spread
  }
}

fn sanitize<'a>(unsanitized: &'a str) -> &'a str {
  if unsanitized.ends_with("*") {
    return unsanitized.trim_right_chars('*')
  } else {
    return unsanitized
  }
}

pub fn parse_line<'a>(line: &'a str) -> Option<DailyTempSpread> {
  if line.is_empty() || !line.char_at(3).is_digit() {
    None
  } else {
    let l: Vec<&str> = line.words().collect();
    let date: int = num::from_str_radix(sanitize(l[0]), 10).unwrap();
    let max: int = num::from_str_radix(sanitize(l[1]), 10).unwrap();
    let min: int = num::from_str_radix(sanitize(l[2]), 10).unwrap();

    Some(DailyTempSpread::new( date, max, min ))
  }
}

fn find_highest_spread(mut days_list: Vec<DailyTempSpread>) -> DailyTempSpread {
  days_list.sort();
  days_list.pop().unwrap()
}

fn main() {
  let path = Path::new("assets/weather.dat");
  let mut file = File::open(&path);
  let data = file.read_to_end().unwrap();
  let data_string = String::from_utf8(data);
  let mut days: Vec<DailyTempSpread> = Vec::new();

  for line in data_string.unwrap().as_slice().lines() {
    match parse_line(line) {
      Some(x) => days.push(x),
      None => ()
    }
  }

  let highest: DailyTempSpread = find_highest_spread(days);
  println!("June {}: {} degrees", highest.date, highest.temp_spread);
}

mod tests {
  #[test]
  fn test_sanitize() {
    assert_eq!("32".as_slice(), super::sanitize("32*".as_slice()));
    assert_eq!("32".as_slice(), super::sanitize("32".as_slice()));
  }

  #[test]
  fn test_parse_line() {
    let line = "   4  77    59    68          51.1       0.00         110  9.1 130  12  8.6  62 40 1021.1";
    let day = super::DailyTempSpread::new( 4, 77, 59);

    assert_eq!(Some(day), super::parse_line(line));
    assert_eq!(None, super::parse_line("".as_slice()));

    let tricky_line = "  26  97*   64";
    let day2 = super::DailyTempSpread::new(26, 97, 64);
    assert_eq!(Some(day2), super::parse_line(tricky_line));

    let starts_with_word = "  mo  82.9  60.5  71.7    16  58.8       0.00              6.9          5.3";
    assert_eq!(None, super::parse_line(starts_with_word));
  }
}
