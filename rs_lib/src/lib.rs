use crc32c::crc32c_append;
use crc32fast::Hasher;
use wasm_bindgen::prelude::*;

// https://github.com/napi-rs/node-rs/blob/main/packages/crc32/src/lib.rs
#[wasm_bindgen]
pub fn crc32(input: &[u8], initial_state: Option<u32>) -> u32 {
  let mut hasher = Hasher::new_with_initial(initial_state.unwrap_or(0));
  hasher.update(input.as_ref());
  hasher.finalize()
}

#[wasm_bindgen]
pub fn crc32c(input: &[u8], initial_state: Option<u32>) -> u32 {
  crc32c_append(initial_state.unwrap_or(0), input.as_ref())
}
