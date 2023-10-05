#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum BaseRepr {
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

#[derive(Clone, Copy, Debug)]
pub struct GenericNumber(u64);

impl GenericNumber {
  pub fn from_str(input: &str, base: BaseRepr) -> Option<Self> {
    let prefix = match base {
      BaseRepr::Binary => "0b",
      BaseRepr::Octal => "0o",
      BaseRepr::Decimal => "0",
      BaseRepr::Hex => "0x",
    };
    let input = input.trim().trim_start_matches(prefix);
    match u64::from_str_radix(input, base as u32) {
      Ok(n) => Some(Self(n)),
      Err(_) => None,
    }
  }

  pub fn from_decimal(input: u64) -> Self {
    Self(input)
  }

  fn truncate(&self, bytes: ReprSize) -> u64 {
    self.0 & (bytes as u64)
  }

  pub fn sign_bit(&self, bytes: ReprSize) -> bool {
    let sign_bit = match bytes {
      ReprSize::Byte => 0x80,
      ReprSize::Word => 0x8000,
      ReprSize::DWord => 0x80000000,
      ReprSize::QWord => 0x8000000000000000,
    };
    (self.0 & sign_bit) > 0
  }

  pub fn to_str(&self, base: BaseRepr, bytes: ReprSize) -> String {
    let truncated = self.truncate(bytes);
    match base {
      BaseRepr::Binary => format!("{:0b}", truncated),
      BaseRepr::Octal => format!("{:0o}", truncated),
      BaseRepr::Decimal => format!("{}", truncated),
      BaseRepr::Hex => format!("{:0x}", truncated),
    }
  }
}

impl Default for GenericNumber {
  fn default() -> Self {
    Self::from_decimal(0)
  }
}
