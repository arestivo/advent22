use std::{io, io::prelude::*};

pub fn read_strings() -> Vec<String> {
  io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}