//! Generated-field types (hand-written reference).
//!
//! Value-constrained FIX fields become real Rust **enums** (D3) — better than QFJ's
//! loose `static final` char/int constants, because an invalid value is
//! unrepresentable. Each enum maps 1:1 to its FIX enum values and owns its on-wire
//! conversion. (The generator will emit one of these per value-constrained field.)

/// `EncryptMethod` (tag 98, FIX type INT). FIX 4.3 values 0–6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EncryptMethod {
    None = 0,
    Pkcs = 1,
    Des = 2,
    PkcsDes = 3,
    PgpDes = 4,
    PgpDesMd5 = 5,
    PemDesMd5 = 6,
}

impl EncryptMethod {
    /// On-wire integer for this value (tag 98 is FIX type INT).
    pub fn to_fix(self) -> i32 {
        self as i32
    }

    /// Parse the on-wire integer; `None` for a value outside the FIX enum.
    pub fn from_fix(n: i32) -> Option<Self> {
        Some(match n {
            0 => Self::None,
            1 => Self::Pkcs,
            2 => Self::Des,
            3 => Self::PkcsDes,
            4 => Self::PgpDes,
            5 => Self::PgpDesMd5,
            6 => Self::PemDesMd5,
            _ => return None,
        })
    }
}
