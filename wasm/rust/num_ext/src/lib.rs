extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

#[wasm_bindgen]
extern {
  pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn nth_prime(i: usize) -> usize {
  primal::StreamingSieve::nth_prime(i)
}

#[wasm_bindgen]
pub fn nth_fibo(i: usize) -> String {
  let mut prev: BigUint = Zero::zero();
  let mut curr: BigUint = One::one();
  for _ in 1..i {
      let next = prev + &curr;
      prev = replace(&mut curr, next);
  }
  curr.to_string()
}
