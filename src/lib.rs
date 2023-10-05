#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Base {
  Binary = 2,
  Octal = 8,
  Decimal = 10,
  Hex = 16,
}

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum ReprSize {
  Byte = u8::MAX as u64,
  Word = u16::MAX as u64,
  DWord = u32::MAX as u64,
  QWord = u64::MAX as u64,
}

pub fn parse_num(input: &str, base: Base) -> Option<u128> {
  let prefix = match base {
    Base::Binary => "0b",
    Base::Octal => "0o",
    Base::Decimal => "0",
    Base::Hex => "0x",
  };
  let input = input.trim().trim_start_matches(prefix);
  match u128::from_str_radix(input, base as u32) {
    Ok(n) => Some(n),
    Err(_) => None,
  }
}

fn truncate_num(input: u128, size: ReprSize) -> u128 {
  let mask = size as u128;
  input & mask
}

pub fn format_num(input: u128, size: ReprSize, base: Base) -> String {
  let input = truncate_num(input, size);
  match base {
    Base::Binary => format!("{:0b}", input),
    Base::Octal => format!("{:0o}", input),
    Base::Decimal => format!("{}", input),
    Base::Hex => format!("{:0x}", input),
  }
}

fn main() {
  let input = "0x12";
  let n = parse_num(input, Base::Hex).unwrap();
  let s = format_num(n, ReprSize::DWord, Base::Octal);
  println!("{}", s);
}
