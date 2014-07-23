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

struct Day {
  date: int,
  max: f32,
  min: f32,
  temp_spread: f32
  }

impl Day {
  fn new(day: int, max: f32, min: f32) -> Day {
    Day { date: day,
          max: max,
          min: min,
          temp_spread: max - min
    }
  }
}

impl fmt::Show for Day {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {:.2f}, {:.2f}, {:.2f})",
           self.date, self.max, self.min, self.temp_spread)
  }
}

// Only implemented in order to run tests. Is this the best way to do this?
impl std::cmp::PartialEq for Day {
  fn eq(&self, other: &Day) -> bool {
    self.date == other.date &&
    self.min == other.min &&
    self.max == other.max
  }
}

fn sanitize<'a>(unsanitized: &'a str) -> &'a str {
  if unsanitized.ends_with("*") {
    return unsanitized.trim_right_chars('*')
  } else {
    return unsanitized
  }
}

fn parse_line<'a>(line: &'a str) -> Option<Day> {
  if line.is_empty() || !line.char_at(3).is_digit() {
    None
  } else {
    let l: Vec<&str> = line.words().collect();
    let date: int = num::from_str_radix(sanitize(l[0]), 10).unwrap();
    let max: f32 = num::from_str_radix(sanitize(l[1]), 10).unwrap();
    let min: f32 = num::from_str_radix(sanitize(l[2]), 10).unwrap();

    Some(Day::new( date, max, min ))
  }
}

fn main() {
  let path = Path::new("assets/weather.dat");
  let mut file = File::open(&path);
  let data = file.read_to_end().unwrap();
  let data_string = String::from_utf8(data);

  for line in data_string.unwrap().as_slice().lines() {
    match parse_line(line) {
      Some(x) => println!("{}", x),
      None => ()
    }
  }
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
    let day = super::Day::new( 4, 77.0, 59.0);

    assert_eq!(Some(day), super::parse_line(line));
    assert_eq!(None, super::parse_line("".as_slice()));

    let tricky_line = "  26  97*   64";
    let day2 = super::Day::new(26, 97.0, 64.0);
    assert_eq!(Some(day2), super::parse_line(tricky_line));

    let starts_with_word = "  mo  82.9  60.5  71.7    16  58.8       0.00              6.9          5.3";
    assert_eq!(None, super::parse_line(starts_with_word));
  }
}
