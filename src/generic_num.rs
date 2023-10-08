use num_traits::ops::wrapping::WrappingNeg;
use serde::{Deserialize, Serialize};
use std::fmt::{Binary, Display, LowerHex, Octal, UpperHex};

use crate::{
  conv_trait::{BinaryConvert, DecimalConvert, HexConvert, OctalConvert},
  Base, DataSize,
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum GenericNum {
  Byte(i8),
  Word(i16),
  DWord(i32),
  QWord(i64),
  OWord(i128),
}

impl Default for GenericNum {
  fn default() -> Self {
    Self::Byte(0)
  }
}

impl WrappingNeg for GenericNum {
  fn wrapping_neg(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::Byte(i.wrapping_neg()),
      GenericNum::Word(i) => GenericNum::Word(i.wrapping_neg()),
      GenericNum::DWord(i) => GenericNum::DWord(i.wrapping_neg()),
      GenericNum::QWord(i) => GenericNum::QWord(i.wrapping_neg()),
      GenericNum::OWord(i) => GenericNum::OWord(i.wrapping_neg()),
    }
  }
}

impl Binary for GenericNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GenericNum::Byte(i) => write!(f, "{:08b}", i),
      GenericNum::Word(i) => write!(f, "{:016b}", i),
      GenericNum::DWord(i) => write!(f, "{:032b}", i),
      GenericNum::QWord(i) => write!(f, "{:064b}", i),
      GenericNum::OWord(i) => write!(f, "{:0128b}", i),
    }
  }
}

impl Octal for GenericNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GenericNum::Byte(i) => write!(f, "{:03o}", i),
      GenericNum::Word(i) => write!(f, "{:06o}", i),
      GenericNum::DWord(i) => write!(f, "{:012o}", i),
      GenericNum::QWord(i) => write!(f, "{:024o}", i),
      GenericNum::OWord(i) => write!(f, "{:048o}", i),
    }
  }
}

impl Display for GenericNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GenericNum::Byte(i) => write!(f, "{}", i),
      GenericNum::Word(i) => write!(f, "{}", i),
      GenericNum::DWord(i) => write!(f, "{}", i),
      GenericNum::QWord(i) => write!(f, "{}", i),
      GenericNum::OWord(i) => write!(f, "{}", i),
    }
  }
}

impl LowerHex for GenericNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GenericNum::Byte(i) => write!(f, "{:02x}", i),
      GenericNum::Word(i) => write!(f, "{:04x}", i),
      GenericNum::DWord(i) => write!(f, "{:08x}", i),
      GenericNum::QWord(i) => write!(f, "{:016x}", i),
      GenericNum::OWord(i) => write!(f, "{:032x}", i),
    }
  }
}

impl UpperHex for GenericNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GenericNum::Byte(i) => write!(f, "{:02X}", i),
      GenericNum::Word(i) => write!(f, "{:04X}", i),
      GenericNum::DWord(i) => write!(f, "{:08X}", i),
      GenericNum::QWord(i) => write!(f, "{:016X}", i),
      GenericNum::OWord(i) => write!(f, "{:032X}", i),
    }
  }
}

impl GenericNum {
  fn use_least_size(num: i128) -> GenericNum {
    if (num as i8 as i128) == num {
      return GenericNum::Byte(num as i8);
    }
    if (num as i16 as i128) == num {
      return GenericNum::Word(num as i16);
    }
    if (num as i32 as i128) == num {
      return GenericNum::DWord(num as i32);
    }
    if (num as i64 as i128) == num {
      return GenericNum::QWord(num as i64);
    }
    return GenericNum::OWord(num as i128);
  }

  pub fn sign_bit(&self) -> bool {
    match self {
      GenericNum::Byte(i) => (*i as i8) < 0,
      GenericNum::Word(i) => (*i as i16) < 0,
      GenericNum::DWord(i) => (*i as i32) < 0,
      GenericNum::QWord(i) => (*i as i64) < 0,
      GenericNum::OWord(i) => (*i as i64) < 0,
    }
  }

  pub fn is_zero(&self) -> bool {
    if let Self::QWord(i) = self.to_qword() {
      i == 0
    } else {
      unreachable!()
    }
  }

  pub fn shl(&mut self) {
    match self {
      GenericNum::Byte(i) => *i <<= 1,
      GenericNum::Word(i) => *i <<= 1,
      GenericNum::DWord(i) => *i <<= 1,
      GenericNum::QWord(i) => *i <<= 1,
      GenericNum::OWord(i) => *i <<= 1,
    };
  }

  pub fn shr(&mut self) {
    match self {
      GenericNum::Byte(i) => *i >>= 1,
      GenericNum::Word(i) => *i >>= 1,
      GenericNum::DWord(i) => *i >>= 1,
      GenericNum::QWord(i) => *i >>= 1,
      GenericNum::OWord(i) => *i >>= 1,
    };
  }

  pub fn get_bit(&self, idx: usize) -> bool {
    if idx >= (self.data_size() as usize * 8) {
      return false;
    }
    match self {
      GenericNum::Byte(i) => ((i >> idx) & 1) == 1,
      GenericNum::Word(i) => ((i >> idx) & 1) == 1,
      GenericNum::DWord(i) => ((i >> idx) & 1) == 1,
      GenericNum::QWord(i) => ((i >> idx) & 1) == 1,
      GenericNum::OWord(i) => ((i >> idx) & 1) == 1,
    }
  }

  pub fn flip_bit(&mut self, idx: usize) {
    if idx >= (self.data_size() as usize * 8) {
      return;
    }
    match self {
      GenericNum::Byte(i) => *i ^= 1 << idx,
      GenericNum::Word(i) => *i ^= 1 << idx,
      GenericNum::DWord(i) => *i ^= 1 << idx,
      GenericNum::QWord(i) => *i ^= 1 << idx,
      GenericNum::OWord(i) => *i ^= 1 << idx,
    }
  }

  pub fn from_base(input: &str, base: Base) -> Option<Self> {
    match base {
      Base::Binary => Self::from_binary(input),
      Base::Octal => Self::from_octal(input),
      Base::Decimal => Self::from_decimal(input),
      Base::Hex => Self::from_hex(input),
    }
  }

  pub fn to_base(&self, base: Base) -> String {
    match base {
      Base::Binary => self.to_binary(),
      Base::Octal => self.to_octal(),
      Base::Decimal => self.to_decimal(),
      Base::Hex => self.to_hex_lower(),
    }
  }

  pub fn data_size(&self) -> DataSize {
    match self {
      GenericNum::Byte(_) => DataSize::Byte,
      GenericNum::Word(_) => DataSize::Word,
      GenericNum::DWord(_) => DataSize::DWord,
      GenericNum::QWord(_) => DataSize::QWord,
      GenericNum::OWord(_) => DataSize::OWord,
    }
  }

  pub fn to_size(&self, size: DataSize) -> Self {
    match size {
      DataSize::Byte => self.to_byte(),
      DataSize::Word => self.to_word(),
      DataSize::DWord => self.to_dword(),
      DataSize::QWord => self.to_qword(),
      DataSize::OWord => self.to_oword(),
    }
  }

  pub fn to_byte(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::Byte(*i as u8 as i8),
      GenericNum::Word(i) => GenericNum::Byte(*i as u8 as i8),
      GenericNum::DWord(i) => GenericNum::Byte(*i as u8 as i8),
      GenericNum::QWord(i) => GenericNum::Byte(*i as u8 as i8),
      GenericNum::OWord(i) => GenericNum::Byte(*i as u8 as i8),
    }
  }

  pub fn to_word(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::Word(*i as u16 as i16),
      GenericNum::Word(i) => GenericNum::Word(*i as u16 as i16),
      GenericNum::DWord(i) => GenericNum::Word(*i as u16 as i16),
      GenericNum::QWord(i) => GenericNum::Word(*i as u16 as i16),
      GenericNum::OWord(i) => GenericNum::Word(*i as u16 as i16),
    }
  }

  pub fn to_dword(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::DWord(*i as u32 as i32),
      GenericNum::Word(i) => GenericNum::DWord(*i as u32 as i32),
      GenericNum::DWord(i) => GenericNum::DWord(*i as u32 as i32),
      GenericNum::QWord(i) => GenericNum::DWord(*i as u32 as i32),
      GenericNum::OWord(i) => GenericNum::DWord(*i as u32 as i32),
    }
  }

  pub fn to_qword(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::QWord(*i as u64 as i64),
      GenericNum::Word(i) => GenericNum::QWord(*i as u64 as i64),
      GenericNum::DWord(i) => GenericNum::QWord(*i as u64 as i64),
      GenericNum::QWord(i) => GenericNum::QWord(*i as u64 as i64),
      GenericNum::OWord(i) => GenericNum::QWord(*i as u64 as i64),
    }
  }

  pub fn to_oword(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::OWord(*i as u128 as i128),
      GenericNum::Word(i) => GenericNum::OWord(*i as u128 as i128),
      GenericNum::DWord(i) => GenericNum::OWord(*i as u128 as i128),
      GenericNum::QWord(i) => GenericNum::OWord(*i as u128 as i128),
      GenericNum::OWord(i) => GenericNum::OWord(*i as u128 as i128),
    }
  }
}

impl BinaryConvert for GenericNum {
  fn from_binary(input: &str) -> Option<Self> {
    match i128::from_binary(input) {
      Some(s) => Some(Self::use_least_size(s)),
      None => None,
    }
  }
}

impl OctalConvert for GenericNum {
  fn from_octal(input: &str) -> Option<Self> {
    match i128::from_octal(input) {
      Some(s) => Some(Self::use_least_size(s)),
      None => None,
    }
  }
}

impl DecimalConvert for GenericNum {
  fn from_decimal(input: &str) -> Option<Self> {
    match i128::from_decimal(input) {
      Some(s) => Some(Self::use_least_size(s)),
      None => None,
    }
  }
}

impl HexConvert for GenericNum {
  fn from_hex(input: &str) -> Option<Self> {
    match i128::from_hex(input) {
      Some(s) => Some(Self::use_least_size(s)),
      None => None,
    }
  }
}
