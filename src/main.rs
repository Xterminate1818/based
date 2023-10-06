mod conv_trait;
mod generic_num;

pub use conv_trait::*;
pub use generic_num::*;

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum Base {
  Binary = 2,
  Octal = 8,
  Decimal = 10,
  Hex = 16,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum DataSize {
  Byte = 1,
  Word = 2,
  DWord = 4,
  QWord = 8,
}

fn main() {
  let i = GenericNum::DWord(13).to_binary();
  println!("{}", i)
}
