mod conv_trait;
mod generic_num;

pub use conv_trait::*;
pub use generic_num::*;
use serde::{Deserialize, Serialize};

#[repr(usize)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base {
  Binary = 2,
  Octal = 8,
  Decimal = 10,
  Hex = 16,
}

#[repr(usize)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataSize {
  Byte = 1,
  Word = 2,
  DWord = 4,
  QWord = 8,
  OWord = 16,
}

fn main() {
  let mut i = GenericNum::Byte(0x80u8 as i8);
  println!("{}", i.to_base(Base::Binary));
  i.shl();
  println!("{}", i.to_base(Base::Binary));
}
