use std::num;
use std::fmt;
use std::io::File;
use std::cmp;

struct ForAgainstSpread {
  team: String,
  goals_for: int,
  goals_against: int,
  spread: int
}

impl ForAgainstSpread {
  fn new(team: String, g_for: int, g_against: int) -> ForAgainstSpread {
    ForAgainstSpread {
      team: team,
      goals_for: g_for,
      goals_against: g_against,
      spread: num::abs(g_for - g_against) }
  }
}

impl Eq for ForAgainstSpread {}

impl PartialEq for ForAgainstSpread {
  fn eq(&self, other: &ForAgainstSpread) -> bool {
    self.spread == other.spread
  }
}

impl fmt::Show for ForAgainstSpread {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {}, {})",
    self.team, self.goals_for, self.goals_against, self.spread)
  }
}

impl Ord for ForAgainstSpread {
  fn cmp(&self, other: &ForAgainstSpread) -> Ordering {
    self.spread.cmp(&other.spread)
  }
}

impl cmp::PartialOrd for ForAgainstSpread {
  fn partial_cmp(&self, other: &ForAgainstSpread) -> Option<Ordering> {
    self.spread.partial_cmp(&other.spread)
  }
}

fn parse_line<'a>(line: &'a str) -> Option<ForAgainstSpread> {
  if !line.char_at(4).is_digit() {
    None
  } else {
    let l: Vec<&str> = line.words().collect();
    let team: String = String::from_str(l[1]);
    let g_for: int = num::from_str_radix(l[6], 10).unwrap();
    let g_against: int = num::from_str_radix(l[8], 10).unwrap();
    Some(ForAgainstSpread::new(team, g_for, g_against))
  }
}

fn find_lowest_spread(mut days_list: Vec<ForAgainstSpread>) -> ForAgainstSpread {
  days_list.sort();
  days_list.remove(0).unwrap()
}

fn main() {
  let path = Path::new("assets/football.dat");
  let mut file = File::open(&path);
  let data = file.read_to_end().unwrap();
  let data_string = String::from_utf8(data);
  let mut games: Vec<ForAgainstSpread> = Vec::new();

  for line in data_string.unwrap().as_slice().lines() {
    match parse_line(line) {
      Some(x) => games.push(x),
      None => ()
    }
  }
  let lowest: ForAgainstSpread = find_lowest_spread(games); 
  println!("{}: point-spread: {}", lowest.team, lowest.spread);
}

mod tests {
  #[test]
  fn test_parse_line() {
    let line = "    1. Arsenal         38    26   9   3    79  -  36    87";
    let game_spread = super::ForAgainstSpread::new(String::from_str("Arsenal"), 79, 36);

    assert_eq!(Some(game_spread), super::parse_line(line));
  }
}
