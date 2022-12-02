use global;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 { panic!("Missing Part!") }

  let problem = args[1].as_str();

  match problem {
    "a" => parta(),
    "b" => partb(),
    _ => println!("Unknown Part!")
  }
}

fn parta() {
  fn value(game: &str) -> Result<i32, &str> {
    match game {
        "A X" => Ok(3 + 1),
        "A Y" => Ok(6 + 2),
        "A Z" => Ok(0 + 3),
        "B X" => Ok(0 + 1),
        "B Y" => Ok(3 + 2),
        "B Z" => Ok(6 + 3),
        "C X" => Ok(6 + 1),
        "C Y" => Ok(0 + 2),
        "C Z" => Ok(3 + 3),
        &_ => Err("Invalid Game")
    }
  }

  let total: i32 = global::read_strings().iter().map(|g| value(g).unwrap()).sum();

  println!("{total:?}")
}

fn partb() {
    fn value(game: &str) -> Result<i32, &str> {
        match game {
            "A X" => Ok(0 + 3),
            "A Y" => Ok(3 + 1),
            "A Z" => Ok(6 + 2),
            "B X" => Ok(0 + 1),
            "B Y" => Ok(3 + 2),
            "B Z" => Ok(6 + 3),
            "C X" => Ok(0 + 2),
            "C Y" => Ok(3 + 3),
            "C Z" => Ok(6 + 1),
            &_ => Err("Invalid Game")
        }
      }
    
      let total: i32 = global::read_strings().iter().map(|g| value(g).unwrap()).sum();
    
      println!("{total:?}")
}