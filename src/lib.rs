pub mod conv_trait;
pub mod generic_num;

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

impl Base {
  pub fn digits_per_byte(&self) -> usize {
    match self {
      Base::Binary => 4,
      Base::Octal => 3,
      Base::Decimal => 0,
      Base::Hex => 2,
    }
  }
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
