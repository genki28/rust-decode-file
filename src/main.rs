extern crate url;

use argh::FromArgs;
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use std::str::Utf8Error;

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
  /// whether or not to jump
  #[argh(switch, short = 'j')]
  jump: bool,

  /// how high to go
  #[argh(option)]
  height: usize,

  /// an optional nickname for the pilot
  #[argh(option)]
  pilot_nickname: Option<String>,

  /// an optional direction which is "up" by default
  #[argh(option, default = "String::from(\"only up\")")]
  direction: String,
}

fn main() -> Result<(), Utf8Error> {
  let args: GoUp = argh::from_env();
  print!("jump to {:?}", args.jump);
  print!("height to {:?}", args.height);
  print!("pilot_nickname to {:?}", args.pilot_nickname);
  print!("direction to {:?}", args.direction);
  let input = "test-sample-encoding";

  let iter = utf8_percent_encode(input, NON_ALPHANUMERIC);
  let encoded: String = iter.collect();

  let iter = percent_decode(encoded.as_bytes());
  let decoded = iter.decode_utf8()?;
  print!("{:?}", decoded);

  Ok(())
}
