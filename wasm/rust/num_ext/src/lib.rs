extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

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
pub fn nth_fibo(i: usize) -> u64 {
  let mut prev: u64 = 0;
  let mut curr: u64 = 1;
  for _ in 1..i {
      let next = prev + curr;
      prev = curr;
      curr = next;
  }
  curr
}
