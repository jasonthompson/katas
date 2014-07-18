use std::io::File;
use std::string;
use std::char;

fn parse_line(line: &str) {
  let v: Vec<&str> = line.words().collect();
  println!("{}", v);
}

fn main() {
  let path = Path::new("assets/weather.dat");
  let mut file = File::open(&path);
  let data = file.read_to_end().unwrap();
  let data_string = String::from_utf8(data);

  for line in data_string.unwrap().as_slice().lines() {
    parse_line(line);
  }
}
