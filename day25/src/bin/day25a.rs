use priority_queue::PriorityQueue;

fn main() {
  let lines = global::read_strings();

  let mut sum = 0;

  for line in lines {
    sum += from_snafu(line);
  }

  println!("{}", to_snafu(sum));
}

fn from_snafu(snafu: String) -> i64 {
  let b5 = snafu.chars().map(|c| match c {'-' => -1, '=' => -2, c => char::to_digit(c, 10).unwrap() as i64});
  let mut place = 1;
  let mut res: i64 = 0;

  for d in b5.rev() {
    res += d * place;
    place *= 5;
  }

  res
}

fn largest_snafu(partial_snafu: &str) -> i64 {
  let snafu = partial_snafu.replace('?', "2");
  from_snafu(snafu)
}

fn lowest_snafu(partial_snafu: &str) -> i64 {
  let snafu = partial_snafu.replace('?', "=");
  from_snafu(snafu)
}

fn average_snafu(partial_snafu: &str) -> i64 {
  let snafu = partial_snafu.replace('?', "0");
  from_snafu(snafu)
}

fn snafu_neighbours(partial_snafu: &str) -> Vec<String> {
  ['0', '1', '2', '-', '='].map(|d| partial_snafu.replacen('?', &d.to_string(), 1)).to_vec()
}

fn to_snafu(num: i64) -> String {
  let mut largest = "".to_string();
  while largest_snafu(&largest) < num { largest.push('?') }
  
  let mut pq = PriorityQueue::new();
  pq.push(largest.clone(), (average_snafu(&largest) - num).abs());

  while !pq.is_empty() {
    let (current, _) = pq.pop().unwrap();
    if !current.contains('?') { return current; }
    let next = snafu_neighbours(&current);

    for n in next {
      if largest_snafu(&n) >= num && lowest_snafu(&n) <= num {
        pq.push(n.clone(), (average_snafu(&n) - num).abs());
      }
    }
  }

  unreachable!()
}