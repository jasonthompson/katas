use std::io::File;
//use std::char;

struct Day {
  date: int,
  max_temp: f32,
  min_temp: f32,
  temp_spread: f32 }

impl Day {
  fn new(data: (int, f32, f32)) -> Day {
      let (day, max, min) = data;

    Day { date: day,
          max_temp: max,
          min_temp: min,
          temp_spread: 0.0 }
  }
}

fn sanitize(s: &str) -> &str {
  if s.ends_with("*") {
    s.trim_right_chars('*')
  }
}


fn parse_line(line: Vec<&str>) {
  let v: Vec<&str> = line.words().collect();

  if v.is_empty() || !v[0].char_at(0).is_digit() {
    return
  };

  // let day = Day::new(v);
  println!("{}", v);
}

// fn main() {
//   let path = Path::new("assets/weather.dat");
//   let mut file = File::open(&path);
//   let data = file.read_to_end().unwrap();
//   let data_string = String::from_utf8(data);

//   for line in data_string.unwrap().as_slice().lines() {
//     parse_line(line);
//   }
// }

#[config(test)]
mod tests {
  use super::sanitize;
  #[test]
  fn test_sanitize() {
    assert_eq!("32".as_slice(), sanitize("32*".as_slice()));
  }
}
