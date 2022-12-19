use day17::{Chamber, Rock};

fn main() {
  let line = global::read_single_line();
  let shapes = day17::create_rock_shapes();

  let mut chamber = Chamber::default();
  let mut rock = Rock::first(&chamber, &shapes);

  let mut gush = 0;

  for _ in 0..2022{
    loop {
      let c = line.chars().nth(gush).unwrap();
      gush = (gush + 1) % line.len();

      match c {
        '<' => { rock = rock.left(&chamber) }
        '>' => { rock = rock.right(&chamber) }
        _ => unreachable!()
      }

      rock = rock.down(&chamber);
      if rock.landed { break; }
    }

    chamber.add(rock.clone());
    rock = rock.next(&chamber, &shapes).clone();    
  }

  chamber.draw();

  println!("{}", chamber.top);
}