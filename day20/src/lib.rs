pub fn mix(original: &[i64], times: u64) -> Vec<i64> {
  let original = original.to_owned();
  let mut result = original.clone();
  let mut indexes: Vec<usize> = (0..original.len()).collect();
 
  for _ in 0..times {
    (0..original.len()).for_each(|i| {
      let from = indexes.iter().position(|v| *v == i).unwrap();
      let to = (from as i64 + original[i]).rem_euclid((result.len() - 1) as i64) as usize;
  
      let v = result.remove(from);
      result.insert(to, v);
  
      let i = indexes.remove(from);
      indexes.insert(to, i);  
    });
  }

  result
}

pub fn sum_grooves(numbers: &[i64]) -> i64 {
  let zero = numbers.iter().position(|v| *v == 0).unwrap();
  numbers[(zero + 1000) % numbers.len()] + numbers[(zero + 2000) % numbers.len()] + numbers[(zero + 3000) % numbers.len()]
}