use num_traits::ops::wrapping::WrappingNeg;
use std::fmt::{Binary, Display, LowerHex, Octal, UpperHex};

pub trait BinaryConvert: Binary + WrappingNeg + Sized {
  fn to_binary(&self) -> String {
    let size = std::mem::size_of::<Self>() * 8;
    format!("{:0width$b}", self, width = size)
  }

  fn from_binary(input: &str) -> Option<Self>;
}

pub trait OctalConvert: Octal + WrappingNeg + Sized {
  fn to_octal(&self) -> String {
    let size = std::mem::size_of::<Self>() * 3;
    format!("{:0width$o}", self, width = size)
  }

  fn from_octal(input: &str) -> Option<Self>;
}

pub trait DecimalConvert: Display + WrappingNeg + Sized {
  fn to_decimal(&self) -> String {
    format!("{}", self)
  }

  fn from_decimal(input: &str) -> Option<Self>;
}

pub trait HexConvert: LowerHex + UpperHex + WrappingNeg + Sized {
  fn to_hex_lower(&self) -> String {
    let size = std::mem::size_of::<Self>() * 2;
    format!("{:0width$x}", self, width = size)
  }

  fn to_hex_upper(&self) -> String {
    let size = std::mem::size_of::<Self>() * 2;
    format!("{:0width$X}", self, width = size)
  }

  fn from_hex(input: &str) -> Option<Self>;
}

macro_rules! _impl_convert {
  ($type:ty) => {
    impl BinaryConvert for $type {
      fn from_binary(input: &str) -> Option<Self> {
        match Self::from_str_radix(input, 2) {
          Ok(s) => Some(s),
          Err(_) => None,
        }
      }
    }
    impl OctalConvert for $type {
      fn from_octal(input: &str) -> Option<Self> {
        match Self::from_str_radix(input, 8) {
          Ok(s) => Some(s),
          Err(_) => None,
        }
      }
    }
    impl DecimalConvert for $type {
      fn from_decimal(input: &str) -> Option<Self> {
        match Self::from_str_radix(input, 10) {
          Ok(s) => Some(s),
          Err(_) => None,
        }
      }
    }
    impl HexConvert for $type {
      fn from_hex(input: &str) -> Option<Self> {
        match Self::from_str_radix(input, 16) {
          Ok(s) => Some(s),
          Err(_) => None,
        }
      }
    }
  };

  ($type1:ty, $($type2:ty),+) => {
    _impl_convert!($type1);
    _impl_convert!($($type2),+);
  };
}

_impl_convert!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

pub trait SeperateString: ToString {
  fn seperate(&self, chunk_size: usize, seperator: &str) -> String {
    if chunk_size == 0 {
      return self.to_string();
    }
    self
      .to_string()
      .as_bytes()
      .rchunks(chunk_size)
      .rev()
      .map(std::str::from_utf8)
      .collect::<Result<Vec<&str>, _>>()
      .unwrap()
      .join(seperator)
  }
}

impl SeperateString for String {
}
