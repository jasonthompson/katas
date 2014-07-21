#![crate_name = "weather"]

#![feature(globs)]
use std::num;

fn sanitize<'a>(unsanitized: &'a str) -> &'a str {
  if unsanitized.ends_with("*") {
    return unsanitized.trim_right_chars('*')
  } else {
    return unsanitized
  }
}

fn to_num(str_num: &str) -> f32 {
  let f_num: f32 = num::from_str_radix(str_num, 10).unwrap();
  f_num
}

fn parse_line(line: Vec<&str>) {
  let v: Vec<&str> = line.words().collect();

  if v.is_empty() || !v[0].char_at(0).is_digit() {
    return
  };

  // let day = Day::new(v);
  v 
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
    let line = "4  77    59    68          51.1       0.00         110  9.1 130  12  8.6  62 40 1021.1".as_slice();
    assert_eq!(, super::parse_line(line));
  }
}
