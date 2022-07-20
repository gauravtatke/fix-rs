#![allow(unused_imports)]
#![allow(dead_code)]
use std::convert::TryFrom;
use std::fmt::{self, Formatter};
use std::str::FromStr;

use crate::quickfix_errors::*;

#[derive(Debug, Clone, Copy)]
pub enum FixType {
    INT,
    FLOAT,
    STRING,
    BOOL,
    CHAR,
}

// seems unnecessaery
// #[derive(Debug)]
// pub struct FixTypeField {
//     pub field_type: FixType,
//     pub data: String
// }

// impl fmt::Display for FixTypeField {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.data)
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Int(i64);

impl Int {
    pub fn new<T: Into<i64>>(value: T) -> Int {
        Int(value.into())
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl From<Int> for FixTypeField {
//     fn from(value: Int) -> FixTypeField {
//         FixTypeField {
//             field_type: FixType::INT,
//             data: value.to_string()
//         }
//     }
// }

impl<T: Into<i64>> From<T> for Int {
    fn from(value: T) -> Int {
        Int::new(value)
    }
}

// impl TryFrom<FixTypeField> for Int {
//     type Error = FixTypeFieldParseError;

//     fn try_from(value: FixTypeField) -> Result<Self, Self::Error> {
//         match value.field_type {
//             FixType::INT => {
//                 match value.data.parse::<i64>() {
//                     Ok(i) => Ok(Int::new(i)),
//                     Err(_) => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotInt
//                     })
//                 }
//             },
//             _ => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotInt
//                     })
//         }
//     }
// }

impl FromStr for Int {
    type Err = SessionRejectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(i) => Ok(Int::new(i)),
            Err(e) => Err(SessionRejectError::parse_err(Some(Box::new(e)))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Float(f64);

impl Float {
    pub fn new<T: Into<f64>>(value: T) -> Float {
        Float(value.into())
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl From<Float> for FixTypeField {
//     fn from(value: Float) -> FixTypeField {
//         FixTypeField {
//             field_type: FixType::FLOAT,
//             data: value.to_string()
//         }
//     }
// }

impl<T: Into<f64>> From<T> for Float {
    fn from(value: T) -> Float {
        Float::new(value)
    }
}

impl FromStr for Float {
    type Err = SessionRejectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(f) => Ok(Float::new(f)),
            Err(e) => Err(SessionRejectError::parse_err(Some(Box::new(e)))),
        }
    }
}

// impl TryFrom<FixTypeField> for Float {
//     type Error = FixTypeFieldParseError;

//     fn try_from(value: FixTypeField) -> Result<Self, Self::Error> {
//         match value.field_type {
//             FixType::FLOAT => {
//                 match value.data.parse::<f64>() {
//                     Ok(i) => Ok(Float::new(i)),
//                     Err(_) => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotFloat
//                     })
//                 }
//             },
//             _ => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotFloat
//                     })
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Str (String);

// impl Str {
//     pub fn new<T: Into<String>>(value: T) -> Str {
//         Str (value.into())
//     }
// }

// impl fmt::Display for Str {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// impl From<Str> for FixTypeField {
//     fn from(value: Str) -> FixTypeField {
//         FixTypeField {
//             field_type: FixType::STRING,
//             data: value.to_string()
//         }
//     }
// }

// impl From<String> for FixTypeField {
//     fn from(value: String) -> FixTypeField {
//         FixTypeField {
//             field_type: FixType::STRING,
//             data: value
//         }
//     }
// }

// impl From<FixTypeField> for Str {
//     fn from(value: FixTypeField) -> Str {
//         Str::new(value.data)
//     }
// }

// impl From<FixTypeField> for String {
//     fn from(value: FixTypeField) -> String {
//         value.data.to_string()
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Char(char);

impl Char {
    pub fn new<T: Into<char>>(value: T) -> Char {
        Char(value.into())
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl From<Char> for FixTypeField {
//     fn from(value: Char) -> FixTypeField {
//         FixTypeField {
//             field_type: FixType::CHAR,
//             data: value.to_string()
//         }
//     }
// }

impl<T: Into<char>> From<T> for Char {
    fn from(value: T) -> Char {
        Char::new(value)
    }
}

impl FromStr for Char {
    type Err = SessionRejectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<char>() {
            Ok(c) if c.is_ascii() => Ok(Char::new(c)),
            Ok(_) => Err(SessionRejectError::parse_err(None)),
            Err(e) => Err(SessionRejectError::parse_err(Some(Box::new(e)))),
        }
    }
}

// impl TryFrom<FixTypeField> for Char {
//     type Error = FixTypeFieldParseError;

//     fn try_from(value: FixTypeField) -> Result<Self, Self::Error> {
//         match value.field_type {
//             FixType::CHAR => {
//                 match value.data.parse::<char>() {
//                     Ok(i) => Ok(Char::new(i)),
//                     Err(_) => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotChar
//                     })
//                 }
//             },
//             _ => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotChar
//                     })
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Bool(char);

impl Bool {
    pub fn new<T: Into<bool>>(value: T) -> Bool {
        if value.into() {
            Bool('Y')
        } else {
            Bool('N')
        }
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl From<Bool> for FixTypeField {
//     fn from(value: Bool) -> FixTypeField {
//         FixTypeField {
//             field_type: FixType::BOOL,
//             data: value.to_string()
//         }
//     }
// }

impl<T: Into<bool>> From<T> for Bool {
    fn from(value: T) -> Bool {
        Bool::new(value)
    }
}

impl FromStr for Bool {
    type Err = SessionRejectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<char>() {
            Ok(ch) => {
                if ch.eq_ignore_ascii_case(&'y') {
                    Ok(Bool::new(true))
                } else if ch.eq_ignore_ascii_case(&'n') {
                    Ok(Bool::new(false))
                } else {
                    Err(SessionRejectError::parse_err(None))
                }
            }
            Err(e) => Err(SessionRejectError::parse_err(Some(Box::new(e)))),
        }
    }
}

// impl TryFrom<FixTypeField> for Bool {
//     type Error = FixTypeFieldParseError;

//     fn try_from(value: FixTypeField) -> Result<Self, Self::Error> {
//         match value.field_type {
//             FixType::BOOL => {
//                 match value.data.parse::<bool>() {
//                     Ok(i) => Ok(Bool::new(i)),
//                     Err(_) => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotBool
//                     })
//                 }
//             },
//             _ => Err(FixTypeFieldParseError {
//                         kind: FixTypeFieldParseErrorKind::NotBool
//                     })
//         }
//     }
// }

#[cfg(test)]
mod types_tests {}
