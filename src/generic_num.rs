use num_traits::ops::wrapping::WrappingNeg;
use std::fmt::{Binary, Display, LowerHex, Octal, UpperHex};

use crate::{
  conv_trait::{BinaryConvert, DecimalConvert, HexConvert, OctalConvert},
  Base, DataSize,
};

#[derive(Clone, Copy, Debug)]
pub enum GenericNum {
  Byte(u8),
  Word(u16),
  DWord(u32),
  QWord(u64),
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
    }
  }
}

impl GenericNum {
  fn use_least_size(num: u64) -> GenericNum {
    if (num & 0xFF) == num {
      return GenericNum::Byte(num as u8);
    }
    if (num & 0xFFFF) == num {
      return GenericNum::Word(num as u16);
    }
    if (num & 0xFFFFFFFF) == num {
      return GenericNum::DWord(num as u32);
    }
    return GenericNum::QWord(num as u64);
  }

  pub fn sign_bit(&self) -> bool {
    match self {
      GenericNum::Byte(i) => (*i as i8) < 0,
      GenericNum::Word(i) => (*i as i16) < 0,
      GenericNum::DWord(i) => (*i as i32) < 0,
      GenericNum::QWord(i) => (*i as i64) < 0,
    }
  }

  pub fn is_zero(&self) -> bool {
    if let Self::QWord(i) = self.to_qword() {
      i == 0
    } else {
      unreachable!()
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
    }
  }

  pub fn to_size(&self, size: DataSize) -> Self {
    match size {
      DataSize::Byte => self.to_byte(),
      DataSize::Word => self.to_word(),
      DataSize::DWord => self.to_dword(),
      DataSize::QWord => self.to_qword(),
    }
  }

  pub fn to_byte(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::Byte(*i as u8),
      GenericNum::Word(i) => GenericNum::Byte(*i as u8),
      GenericNum::DWord(i) => GenericNum::Byte(*i as u8),
      GenericNum::QWord(i) => GenericNum::Byte(*i as u8),
    }
  }

  pub fn to_word(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::Word(*i as u16),
      GenericNum::Word(i) => GenericNum::Word(*i as u16),
      GenericNum::DWord(i) => GenericNum::Word(*i as u16),
      GenericNum::QWord(i) => GenericNum::Word(*i as u16),
    }
  }

  pub fn to_dword(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::DWord(*i as u32),
      GenericNum::Word(i) => GenericNum::DWord(*i as u32),
      GenericNum::DWord(i) => GenericNum::DWord(*i as u32),
      GenericNum::QWord(i) => GenericNum::DWord(*i as u32),
    }
  }

  pub fn to_qword(&self) -> Self {
    match self {
      GenericNum::Byte(i) => GenericNum::QWord(*i as u64),
      GenericNum::Word(i) => GenericNum::QWord(*i as u64),
      GenericNum::DWord(i) => GenericNum::QWord(*i as u64),
      GenericNum::QWord(i) => GenericNum::QWord(*i as u64),
    }
  }
}

impl BinaryConvert for GenericNum {
  fn from_binary(input: &str) -> Option<Self> {
    match u64::from_str_radix(input, 2) {
      Ok(s) => Some(Self::use_least_size(s)),
      Err(_) => None,
    }
  }
}

impl OctalConvert for GenericNum {
  fn from_octal(input: &str) -> Option<Self> {
    match u64::from_str_radix(input, 8) {
      Ok(s) => Some(Self::use_least_size(s)),
      Err(_) => None,
    }
  }
}

impl DecimalConvert for GenericNum {
  fn from_decimal(input: &str) -> Option<Self> {
    match u64::from_str_radix(input, 10) {
      Ok(s) => Some(Self::use_least_size(s)),
      Err(_) => None,
    }
  }
}

impl HexConvert for GenericNum {
  fn from_hex(input: &str) -> Option<Self> {
    match u64::from_str_radix(input, 16) {
      Ok(s) => Some(Self::use_least_size(s)),
      Err(_) => None,
    }
  }
}
