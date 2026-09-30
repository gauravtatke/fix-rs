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

/// `Side` (tag 54, FIX type CHAR). FIX 4.3's 12 allowed values. Char values (not sequential),
/// so both directions are explicit matches (no `#[repr]`/`as` trick like the INT `EncryptMethod`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
    BuyMinus,
    SellPlus,
    SellShort,
    SellShortExempt,
    Undisclosed,
    Cross,
    CrossShort,
    CrossShortExempt,
    AsDefined,
    Opposite,
}

impl Side {
    /// On-wire char (tag 54 is FIX type CHAR).
    pub fn to_fix(self) -> char {
        match self {
            Self::Buy => '1',
            Self::Sell => '2',
            Self::BuyMinus => '3',
            Self::SellPlus => '4',
            Self::SellShort => '5',
            Self::SellShortExempt => '6',
            Self::Undisclosed => '7',
            Self::Cross => '8',
            Self::CrossShort => '9',
            Self::CrossShortExempt => 'A',
            Self::AsDefined => 'B',
            Self::Opposite => 'C',
        }
    }

    /// Parse the on-wire char; `None` for a value outside the FIX enum.
    pub fn from_fix(c: char) -> Option<Self> {
        Some(match c {
            '1' => Self::Buy,
            '2' => Self::Sell,
            '3' => Self::BuyMinus,
            '4' => Self::SellPlus,
            '5' => Self::SellShort,
            '6' => Self::SellShortExempt,
            '7' => Self::Undisclosed,
            '8' => Self::Cross,
            '9' => Self::CrossShort,
            'A' => Self::CrossShortExempt,
            'B' => Self::AsDefined,
            'C' => Self::Opposite,
            _ => return None,
        })
    }
}

/// `OrdType` (tag 40, FIX type CHAR). FIX 4.3's 23 allowed values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdType {
    Market,
    Limit,
    Stop,
    StopLimit,
    MarketOnClose,
    WithOrWithout,
    LimitOrBetter,
    LimitWithOrWithout,
    OnBasis,
    OnClose,
    LimitOnClose,
    ForexC,
    PreviouslyQuoted,
    PreviouslyIndicated,
    ForexF,
    ForexG,
    ForexH,
    Funari,
    MarketIfTouched,
    MarketWithLeftoverAsLimit,
    PreviousFundValuationPoint,
    NextFundValuationPoint,
    Pegged,
}

impl OrdType {
    /// On-wire char (tag 40 is FIX type CHAR).
    pub fn to_fix(self) -> char {
        match self {
            Self::Market => '1',
            Self::Limit => '2',
            Self::Stop => '3',
            Self::StopLimit => '4',
            Self::MarketOnClose => '5',
            Self::WithOrWithout => '6',
            Self::LimitOrBetter => '7',
            Self::LimitWithOrWithout => '8',
            Self::OnBasis => '9',
            Self::OnClose => 'A',
            Self::LimitOnClose => 'B',
            Self::ForexC => 'C',
            Self::PreviouslyQuoted => 'D',
            Self::PreviouslyIndicated => 'E',
            Self::ForexF => 'F',
            Self::ForexG => 'G',
            Self::ForexH => 'H',
            Self::Funari => 'I',
            Self::MarketIfTouched => 'J',
            Self::MarketWithLeftoverAsLimit => 'K',
            Self::PreviousFundValuationPoint => 'L',
            Self::NextFundValuationPoint => 'M',
            Self::Pegged => 'P',
        }
    }

    /// Parse the on-wire char; `None` for a value outside the FIX enum.
    pub fn from_fix(c: char) -> Option<Self> {
        Some(match c {
            '1' => Self::Market,
            '2' => Self::Limit,
            '3' => Self::Stop,
            '4' => Self::StopLimit,
            '5' => Self::MarketOnClose,
            '6' => Self::WithOrWithout,
            '7' => Self::LimitOrBetter,
            '8' => Self::LimitWithOrWithout,
            '9' => Self::OnBasis,
            'A' => Self::OnClose,
            'B' => Self::LimitOnClose,
            'C' => Self::ForexC,
            'D' => Self::PreviouslyQuoted,
            'E' => Self::PreviouslyIndicated,
            'F' => Self::ForexF,
            'G' => Self::ForexG,
            'H' => Self::ForexH,
            'I' => Self::Funari,
            'J' => Self::MarketIfTouched,
            'K' => Self::MarketWithLeftoverAsLimit,
            'L' => Self::PreviousFundValuationPoint,
            'M' => Self::NextFundValuationPoint,
            'P' => Self::Pegged,
            _ => return None,
        })
    }
}

// Tests live in the hand-owned `super::super::tests::fields` module — this file is generated.
