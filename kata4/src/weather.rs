#![feature(globs)]

use std::num;
use std::io::File;
use std::fmt;

struct Day {
  date: int,
  max: f32,
  min: f32,
  //temp_spread: f32 
  }

impl Day {
  fn new(day: int, min: f32, max: f32) -> Day {
    Day { date: day,
          max: max,
          min: min,
          //temp_spread: 0.0 
    }
  }
}

impl fmt::Show for Day {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.date, self.max, self.min)
  }
}

fn sanitize<'a>(unsanitized: &'a str) -> &'a str {
  if unsanitized.ends_with("*") {
    return unsanitized.trim_right_chars('*')
  } else {
    return unsanitized
  }
}

// fn to_num(str_num: &str) -> f32 {
//   let f_num: f32 = num::from_str_radix(str_num, 10).unwrap();
//   f_num
// }

fn parse_line<'a>(line: &'a str) -> Option<Day> {
  if line.is_empty() || !line.char_at(3).is_digit() {
    None
  } else {
    let l: Vec<&str> = line.words().collect();
    let date: int = num::from_str_radix(sanitize(l[0]), 10).unwrap();
    let min: f32 = num::from_str_radix(sanitize(l[1]), 10).unwrap();
    let max: f32 = num::from_str_radix(sanitize(l[2]), 10).unwrap();

    Some(Day::new( date, max, min ))
      //map(|x| to_num(sanitize(x))).collect();
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
    //println!("{}", line);
  }
}

mod tests {
  #[test]
  fn test_sanitize() {
    assert_eq!("32".as_slice(), super::sanitize("32*".as_slice()));
    assert_eq!("32".as_slice(), super::sanitize("32".as_slice()));
  }

  #[test]
  fn test_to_num() {
    assert_eq!(super::to_num("10.2".as_slice()), 10.2f32);
  }

  #[test]
  fn test_parse_line() {
    let line = "4  77    59    68          51.1       0.00         110  9.1 130  12  8.6  62 40 1021.1";
    let day = Day::new( 4, 77.0, 59.0 );

    assert_eq!(day, super::parse_line(line).unwrap());
    assert_eq!(None, super::parse_line("".as_slice()));

    let tricky_line = "26  97*   64";
    assert_eq!(vec!(26.0f32, 97.0, 64.0), super::parse_line(tricky_line).unwrap());
  }
}
