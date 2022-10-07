extern crate url;

use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use std::str::Utf8Error;

fn main() -> Result<(), Utf8Error> {
  let input = "test-sample-encoding";

  let iter = utf8_percent_encode(input, NON_ALPHANUMERIC);
  let encoded: String = iter.collect();

  let iter = percent_decode(encoded.as_bytes());
  let decoded = iter.decode_utf8()?;
  print!("{:?}", decoded);

  Ok(())
}
