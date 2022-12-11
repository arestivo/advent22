use day11::Monkey;
use num::ToPrimitive;

fn main() {
  let lines = global::read_strings();
  let mut monkeys = day11::lines_to_monkeys(&lines);

  for _ in 0..20 {
    for m in 0..monkeys.len() {
      let monkey = monkeys[m].clone();
  
      for item in monkeys[m].items.clone() {
        let new_value = (Monkey::execute_operation(monkey.operation.clone(), item.clone()).to_f64().unwrap() / 3.0) as i64;

        if new_value % monkey.divisible == 0 { monkeys[monkey.iftrue].items.push(new_value) } 
        else { monkeys[monkey.iffalse].items.push(new_value) }

        monkeys[m].inspections += 1;
      }
  
      monkeys[m].items = vec![];
    }
  }

  monkeys.sort_by(|m1,m2| m1.inspections.cmp(&m2.inspections));
  monkeys.reverse();

  let i1 = monkeys[0].inspections.clone();
  let i2 = monkeys[1].inspections.clone();

  println!("{:?}", i1 * i2);
}