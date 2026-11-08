//! `Quote` (app message, MsgType `35=S`): typed facade over the raw
//! [`Message`].
//!
//! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen` from the FIX.4.3
//! data dictionary; `cargo test -p codegen` fails if this file drifts from the generator's output.

// Every generated message imports the same set; not every message uses all of it.
#![allow(unused_imports)]

use super::fields;
use crate::convert;
use crate::core::message::{FieldMap, Group, Message};
use crate::errors::{FieldError, TypedError};
// Tags come from the shared, version-neutral registry (`crate::tags`): a tag number means the
// same field in every FIX version. MsgType (35) lives in the header.
use crate::tags;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;

/// Typed facade over a `Quote` [`Message`] (35=S). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct Quote {
    msg: Message,
}

impl Default for Quote {
    fn default() -> Self {
        Quote::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `Quote`, checking the msg type.
impl TryFrom<Message> for Quote {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a Quote" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<Quote> for Message {
    fn from(typed_msg: Quote) -> Self {
        typed_msg.msg
    }
}

impl Quote {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "S";

    /// An empty `Quote` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- QuoteReqID (tag 131, STRING, optional) ---

    /// Set `QuoteReqID` (tag 131, optional).
    pub fn set_quote_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::QUOTE_REQ_ID, v.to_string());
    }

    /// `QuoteReqID` (tag 131, optional).
    pub fn quote_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::QUOTE_REQ_ID)
    }

    /// Whether `QuoteReqID` (tag 131) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_req_id(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_REQ_ID)
    }

    // --- QuoteID (tag 117, STRING, required) ---

    /// Set `QuoteID` (tag 117, required).
    pub fn set_quote_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::QUOTE_ID, v.to_string());
    }

    /// `QuoteID` (tag 117, required).
    pub fn quote_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::QUOTE_ID)
    }

    /// Whether `QuoteID` (tag 117) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_id(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_ID)
    }

    // --- QuoteType (tag 537, INT, optional) ---

    /// Set `QuoteType` (tag 537, optional).
    pub fn set_quote_type(&mut self, v: fields::QuoteType) {
        self.msg.body_mut().set_field(tags::QUOTE_TYPE, v.to_fix().to_string());
    }

    /// `QuoteType` (tag 537, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::QuoteType`]'s.
    pub fn quote_type(&self) -> Result<fields::QuoteType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::QUOTE_TYPE)?;
        fields::QuoteType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::QUOTE_TYPE,
        })
    }

    /// Whether `QuoteType` (tag 537) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_type(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_TYPE)
    }

    // --- QuoteResponseLevel (tag 301, INT, optional) ---

    /// Set `QuoteResponseLevel` (tag 301, optional).
    pub fn set_quote_response_level(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::QUOTE_RESPONSE_LEVEL, v.to_string());
    }

    /// `QuoteResponseLevel` (tag 301, optional).
    pub fn quote_response_level(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::QUOTE_RESPONSE_LEVEL)
    }

    /// Whether `QuoteResponseLevel` (tag 301) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_response_level(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_RESPONSE_LEVEL)
    }

    // --- Account (tag 1, STRING, optional) ---

    /// Set `Account` (tag 1, optional).
    pub fn set_account(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ACCOUNT, v.to_string());
    }

    /// `Account` (tag 1, optional).
    pub fn account(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ACCOUNT)
    }

    /// Whether `Account` (tag 1) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_account(&self) -> bool {
        self.msg.body().has_field(tags::ACCOUNT)
    }

    // --- AccountType (tag 581, INT, optional) ---

    /// Set `AccountType` (tag 581, optional).
    pub fn set_account_type(&mut self, v: fields::AccountType) {
        self.msg.body_mut().set_field(tags::ACCOUNT_TYPE, v.to_fix().to_string());
    }

    /// `AccountType` (tag 581, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::AccountType`]'s.
    pub fn account_type(&self) -> Result<fields::AccountType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::ACCOUNT_TYPE)?;
        fields::AccountType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ACCOUNT_TYPE,
        })
    }

    /// Whether `AccountType` (tag 581) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_account_type(&self) -> bool {
        self.msg.body().has_field(tags::ACCOUNT_TYPE)
    }

    // --- TradingSessionID (tag 336, STRING, optional) ---

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// `TradingSessionID` (tag 336, optional).
    pub fn trading_session_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRADING_SESSION_ID)
    }

    /// Whether `TradingSessionID` (tag 336) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trading_session_id(&self) -> bool {
        self.msg.body().has_field(tags::TRADING_SESSION_ID)
    }

    // --- TradingSessionSubID (tag 625, STRING, optional) ---

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
    }

    /// `TradingSessionSubID` (tag 625, optional).
    pub fn trading_session_sub_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRADING_SESSION_SUB_ID)
    }

    /// Whether `TradingSessionSubID` (tag 625) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trading_session_sub_id(&self) -> bool {
        self.msg.body().has_field(tags::TRADING_SESSION_SUB_ID)
    }

    // --- Symbol (tag 55, STRING, required) ---

    /// Set `Symbol` (tag 55, required).
    pub fn set_symbol(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SYMBOL, v.to_string());
    }

    /// `Symbol` (tag 55, required).
    pub fn symbol(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SYMBOL)
    }

    /// Whether `Symbol` (tag 55) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_symbol(&self) -> bool {
        self.msg.body().has_field(tags::SYMBOL)
    }

    // --- SymbolSfx (tag 65, STRING, optional) ---

    /// Set `SymbolSfx` (tag 65, optional).
    pub fn set_symbol_sfx(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SYMBOL_SFX, v.to_string());
    }

    /// `SymbolSfx` (tag 65, optional).
    pub fn symbol_sfx(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SYMBOL_SFX)
    }

    /// Whether `SymbolSfx` (tag 65) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_symbol_sfx(&self) -> bool {
        self.msg.body().has_field(tags::SYMBOL_SFX)
    }

    // --- SecurityID (tag 48, STRING, optional) ---

    /// Set `SecurityID` (tag 48, optional).
    pub fn set_security_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_ID, v.to_string());
    }

    /// `SecurityID` (tag 48, optional).
    pub fn security_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_ID)
    }

    /// Whether `SecurityID` (tag 48) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_id(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_ID)
    }

    // --- SecurityIDSource (tag 22, STRING, optional) ---

    /// Set `SecurityIDSource` (tag 22, optional).
    pub fn set_security_id_source(&mut self, v: fields::SecurityIdSource) {
        self.msg.body_mut().set_field(tags::SECURITY_ID_SOURCE, v.to_fix().to_string());
    }

    /// `SecurityIDSource` (tag 22, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityIdSource`]'s.
    pub fn security_id_source(&self) -> Result<fields::SecurityIdSource, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SECURITY_ID_SOURCE)?;
        fields::SecurityIdSource::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_ID_SOURCE,
        })
    }

    /// Whether `SecurityIDSource` (tag 22) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_id_source(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_ID_SOURCE)
    }

    // --- Product (tag 460, INT, optional) ---

    /// Set `Product` (tag 460, optional).
    pub fn set_product(&mut self, v: fields::Product) {
        self.msg.body_mut().set_field(tags::PRODUCT, v.to_fix().to_string());
    }

    /// `Product` (tag 460, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Product`]'s.
    pub fn product(&self) -> Result<fields::Product, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::PRODUCT)?;
        fields::Product::from_fix(n).ok_or(FieldError::InvalidFormat { tag: tags::PRODUCT })
    }

    /// Whether `Product` (tag 460) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_product(&self) -> bool {
        self.msg.body().has_field(tags::PRODUCT)
    }

    // --- CFICode (tag 461, STRING, optional) ---

    /// Set `CFICode` (tag 461, optional).
    pub fn set_cfi_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CFI_CODE, v.to_string());
    }

    /// `CFICode` (tag 461, optional).
    pub fn cfi_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CFI_CODE)
    }

    /// Whether `CFICode` (tag 461) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cfi_code(&self) -> bool {
        self.msg.body().has_field(tags::CFI_CODE)
    }

    // --- SecurityType (tag 167, STRING, optional) ---

    /// Set `SecurityType` (tag 167, optional).
    pub fn set_security_type(&mut self, v: fields::SecurityType) {
        self.msg.body_mut().set_field(tags::SECURITY_TYPE, v.to_fix().to_string());
    }

    /// `SecurityType` (tag 167, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityType`]'s.
    pub fn security_type(&self) -> Result<fields::SecurityType, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SECURITY_TYPE)?;
        fields::SecurityType::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_TYPE,
        })
    }

    /// Whether `SecurityType` (tag 167) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_type(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_TYPE)
    }

    // --- MaturityMonthYear (tag 200, MONTHYEAR, optional) ---

    /// Set `MaturityMonthYear` (tag 200, optional).
    pub fn set_maturity_month_year(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::MATURITY_MONTH_YEAR, v.to_string());
    }

    /// `MaturityMonthYear` (tag 200, optional).
    pub fn maturity_month_year(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::MATURITY_MONTH_YEAR)
    }

    /// Whether `MaturityMonthYear` (tag 200) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_maturity_month_year(&self) -> bool {
        self.msg.body().has_field(tags::MATURITY_MONTH_YEAR)
    }

    // --- MaturityDate (tag 541, LOCALMKTDATE, optional) ---

    /// Set `MaturityDate` (tag 541, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_maturity_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::MATURITY_DATE, convert::format_fix_date(v));
    }

    /// `MaturityDate` (tag 541, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn maturity_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MATURITY_DATE)?;
        convert::parse_fix_date(&raw, tags::MATURITY_DATE)
    }

    /// Whether `MaturityDate` (tag 541) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_maturity_date(&self) -> bool {
        self.msg.body().has_field(tags::MATURITY_DATE)
    }

    // --- CouponPaymentDate (tag 224, UTCDATE, optional) ---

    /// Set `CouponPaymentDate` (tag 224, optional); written in FIX `UTCDATE` format.
    pub fn set_coupon_payment_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::COUPON_PAYMENT_DATE, convert::format_fix_date(v));
    }

    /// `CouponPaymentDate` (tag 224, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn coupon_payment_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::COUPON_PAYMENT_DATE)?;
        convert::parse_fix_date(&raw, tags::COUPON_PAYMENT_DATE)
    }

    /// Whether `CouponPaymentDate` (tag 224) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_coupon_payment_date(&self) -> bool {
        self.msg.body().has_field(tags::COUPON_PAYMENT_DATE)
    }

    // --- IssueDate (tag 225, UTCDATE, optional) ---

    /// Set `IssueDate` (tag 225, optional); written in FIX `UTCDATE` format.
    pub fn set_issue_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::ISSUE_DATE, convert::format_fix_date(v));
    }

    /// `IssueDate` (tag 225, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn issue_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::ISSUE_DATE)?;
        convert::parse_fix_date(&raw, tags::ISSUE_DATE)
    }

    /// Whether `IssueDate` (tag 225) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_issue_date(&self) -> bool {
        self.msg.body().has_field(tags::ISSUE_DATE)
    }

    // --- RepoCollateralSecurityType (tag 239, STRING, optional) ---

    /// Set `RepoCollateralSecurityType` (tag 239, optional).
    pub fn set_repo_collateral_security_type(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REPO_COLLATERAL_SECURITY_TYPE, v.to_string());
    }

    /// `RepoCollateralSecurityType` (tag 239, optional).
    pub fn repo_collateral_security_type(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// Whether `RepoCollateralSecurityType` (tag 239) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_repo_collateral_security_type(&self) -> bool {
        self.msg.body().has_field(tags::REPO_COLLATERAL_SECURITY_TYPE)
    }

    // --- RepurchaseTerm (tag 226, INT, optional) ---

    /// Set `RepurchaseTerm` (tag 226, optional).
    pub fn set_repurchase_term(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::REPURCHASE_TERM, v.to_string());
    }

    /// `RepurchaseTerm` (tag 226, optional).
    pub fn repurchase_term(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::REPURCHASE_TERM)
    }

    /// Whether `RepurchaseTerm` (tag 226) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_repurchase_term(&self) -> bool {
        self.msg.body().has_field(tags::REPURCHASE_TERM)
    }

    // --- RepurchaseRate (tag 227, PERCENTAGE, optional) ---

    /// Set `RepurchaseRate` (tag 227, optional).
    pub fn set_repurchase_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::REPURCHASE_RATE, v.to_string());
    }

    /// `RepurchaseRate` (tag 227, optional).
    pub fn repurchase_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::REPURCHASE_RATE)
    }

    /// Whether `RepurchaseRate` (tag 227) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_repurchase_rate(&self) -> bool {
        self.msg.body().has_field(tags::REPURCHASE_RATE)
    }

    // --- Factor (tag 228, FLOAT, optional) ---

    /// Set `Factor` (tag 228, optional).
    pub fn set_factor(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::FACTOR, v.to_string());
    }

    /// `Factor` (tag 228, optional).
    pub fn factor(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::FACTOR)
    }

    /// Whether `Factor` (tag 228) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_factor(&self) -> bool {
        self.msg.body().has_field(tags::FACTOR)
    }

    // --- CreditRating (tag 255, STRING, optional) ---

    /// Set `CreditRating` (tag 255, optional).
    pub fn set_credit_rating(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CREDIT_RATING, v.to_string());
    }

    /// `CreditRating` (tag 255, optional).
    pub fn credit_rating(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CREDIT_RATING)
    }

    /// Whether `CreditRating` (tag 255) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_credit_rating(&self) -> bool {
        self.msg.body().has_field(tags::CREDIT_RATING)
    }

    // --- InstrRegistry (tag 543, STRING, optional) ---

    /// Set `InstrRegistry` (tag 543, optional).
    pub fn set_instr_registry(&mut self, v: fields::InstrRegistry) {
        self.msg.body_mut().set_field(tags::INSTR_REGISTRY, v.to_fix().to_string());
    }

    /// `InstrRegistry` (tag 543, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::InstrRegistry`]'s.
    pub fn instr_registry(&self) -> Result<fields::InstrRegistry, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::INSTR_REGISTRY)?;
        fields::InstrRegistry::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::INSTR_REGISTRY,
        })
    }

    /// Whether `InstrRegistry` (tag 543) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_instr_registry(&self) -> bool {
        self.msg.body().has_field(tags::INSTR_REGISTRY)
    }

    // --- CountryOfIssue (tag 470, COUNTRY, optional) ---

    /// Set `CountryOfIssue` (tag 470, optional).
    pub fn set_country_of_issue(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::COUNTRY_OF_ISSUE, v.to_string());
    }

    /// `CountryOfIssue` (tag 470, optional).
    pub fn country_of_issue(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::COUNTRY_OF_ISSUE)
    }

    /// Whether `CountryOfIssue` (tag 470) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_country_of_issue(&self) -> bool {
        self.msg.body().has_field(tags::COUNTRY_OF_ISSUE)
    }

    // --- StateOrProvinceOfIssue (tag 471, STRING, optional) ---

    /// Set `StateOrProvinceOfIssue` (tag 471, optional).
    pub fn set_state_or_province_of_issue(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::STATE_OR_PROVINCE_OF_ISSUE, v.to_string());
    }

    /// `StateOrProvinceOfIssue` (tag 471, optional).
    pub fn state_or_province_of_issue(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// Whether `StateOrProvinceOfIssue` (tag 471) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_state_or_province_of_issue(&self) -> bool {
        self.msg.body().has_field(tags::STATE_OR_PROVINCE_OF_ISSUE)
    }

    // --- LocaleOfIssue (tag 472, STRING, optional) ---

    /// Set `LocaleOfIssue` (tag 472, optional).
    pub fn set_locale_of_issue(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LOCALE_OF_ISSUE, v.to_string());
    }

    /// `LocaleOfIssue` (tag 472, optional).
    pub fn locale_of_issue(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LOCALE_OF_ISSUE)
    }

    /// Whether `LocaleOfIssue` (tag 472) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_locale_of_issue(&self) -> bool {
        self.msg.body().has_field(tags::LOCALE_OF_ISSUE)
    }

    // --- RedemptionDate (tag 240, UTCDATE, optional) ---

    /// Set `RedemptionDate` (tag 240, optional); written in FIX `UTCDATE` format.
    pub fn set_redemption_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::REDEMPTION_DATE, convert::format_fix_date(v));
    }

    /// `RedemptionDate` (tag 240, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn redemption_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::REDEMPTION_DATE)?;
        convert::parse_fix_date(&raw, tags::REDEMPTION_DATE)
    }

    /// Whether `RedemptionDate` (tag 240) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_redemption_date(&self) -> bool {
        self.msg.body().has_field(tags::REDEMPTION_DATE)
    }

    // --- StrikePrice (tag 202, PRICE, optional) ---

    /// Set `StrikePrice` (tag 202, optional); written in FIX `PRICE` format.
    pub fn set_strike_price(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::STRIKE_PRICE, convert::format_fix_decimal(v));
    }

    /// `StrikePrice` (tag 202, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn strike_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::STRIKE_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::STRIKE_PRICE)
    }

    /// Whether `StrikePrice` (tag 202) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_strike_price(&self) -> bool {
        self.msg.body().has_field(tags::STRIKE_PRICE)
    }

    // --- OptAttribute (tag 206, CHAR, optional) ---

    /// Set `OptAttribute` (tag 206, optional).
    pub fn set_opt_attribute(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::OPT_ATTRIBUTE, v.to_string());
    }

    /// `OptAttribute` (tag 206, optional).
    pub fn opt_attribute(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::OPT_ATTRIBUTE)
    }

    /// Whether `OptAttribute` (tag 206) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_opt_attribute(&self) -> bool {
        self.msg.body().has_field(tags::OPT_ATTRIBUTE)
    }

    // --- ContractMultiplier (tag 231, FLOAT, optional) ---

    /// Set `ContractMultiplier` (tag 231, optional).
    pub fn set_contract_multiplier(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::CONTRACT_MULTIPLIER, v.to_string());
    }

    /// `ContractMultiplier` (tag 231, optional).
    pub fn contract_multiplier(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::CONTRACT_MULTIPLIER)
    }

    /// Whether `ContractMultiplier` (tag 231) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_contract_multiplier(&self) -> bool {
        self.msg.body().has_field(tags::CONTRACT_MULTIPLIER)
    }

    // --- CouponRate (tag 223, PERCENTAGE, optional) ---

    /// Set `CouponRate` (tag 223, optional).
    pub fn set_coupon_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::COUPON_RATE, v.to_string());
    }

    /// `CouponRate` (tag 223, optional).
    pub fn coupon_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::COUPON_RATE)
    }

    /// Whether `CouponRate` (tag 223) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_coupon_rate(&self) -> bool {
        self.msg.body().has_field(tags::COUPON_RATE)
    }

    // --- SecurityExchange (tag 207, EXCHANGE, optional) ---

    /// Set `SecurityExchange` (tag 207, optional).
    pub fn set_security_exchange(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_EXCHANGE, v.to_string());
    }

    /// `SecurityExchange` (tag 207, optional).
    pub fn security_exchange(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_EXCHANGE)
    }

    /// Whether `SecurityExchange` (tag 207) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_exchange(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_EXCHANGE)
    }

    // --- Issuer (tag 106, STRING, optional) ---

    /// Set `Issuer` (tag 106, optional).
    pub fn set_issuer(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ISSUER, v.to_string());
    }

    /// `Issuer` (tag 106, optional).
    pub fn issuer(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ISSUER)
    }

    /// Whether `Issuer` (tag 106) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_issuer(&self) -> bool {
        self.msg.body().has_field(tags::ISSUER)
    }

    // --- EncodedIssuerLen (tag 348, LENGTH, optional) ---

    /// Set `EncodedIssuerLen` (tag 348, optional).
    pub fn set_encoded_issuer_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_ISSUER_LEN, v.to_string());
    }

    /// `EncodedIssuerLen` (tag 348, optional).
    pub fn encoded_issuer_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_ISSUER_LEN)
    }

    /// Whether `EncodedIssuerLen` (tag 348) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_issuer_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_ISSUER_LEN)
    }

    // --- EncodedIssuer (tag 349, DATA, optional) ---

    /// Set `EncodedIssuer` (tag 349, optional).
    pub fn set_encoded_issuer(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_ISSUER, v.to_string());
    }

    /// `EncodedIssuer` (tag 349, optional).
    pub fn encoded_issuer(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_ISSUER)
    }

    /// Whether `EncodedIssuer` (tag 349) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_issuer(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_ISSUER)
    }

    // --- SecurityDesc (tag 107, STRING, optional) ---

    /// Set `SecurityDesc` (tag 107, optional).
    pub fn set_security_desc(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_DESC, v.to_string());
    }

    /// `SecurityDesc` (tag 107, optional).
    pub fn security_desc(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_DESC)
    }

    /// Whether `SecurityDesc` (tag 107) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_desc(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_DESC)
    }

    // --- EncodedSecurityDescLen (tag 350, LENGTH, optional) ---

    /// Set `EncodedSecurityDescLen` (tag 350, optional).
    pub fn set_encoded_security_desc_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_SECURITY_DESC_LEN, v.to_string());
    }

    /// `EncodedSecurityDescLen` (tag 350, optional).
    pub fn encoded_security_desc_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_SECURITY_DESC_LEN)
    }

    /// Whether `EncodedSecurityDescLen` (tag 350) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_security_desc_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_SECURITY_DESC_LEN)
    }

    // --- EncodedSecurityDesc (tag 351, DATA, optional) ---

    /// Set `EncodedSecurityDesc` (tag 351, optional).
    pub fn set_encoded_security_desc(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_SECURITY_DESC, v.to_string());
    }

    /// `EncodedSecurityDesc` (tag 351, optional).
    pub fn encoded_security_desc(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_SECURITY_DESC)
    }

    /// Whether `EncodedSecurityDesc` (tag 351) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_security_desc(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_SECURITY_DESC)
    }

    // --- BidPx (tag 132, PRICE, optional) ---

    /// Set `BidPx` (tag 132, optional); written in FIX `PRICE` format.
    pub fn set_bid_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::BID_PX, convert::format_fix_decimal(v));
    }

    /// `BidPx` (tag 132, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn bid_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::BID_PX)?;
        convert::parse_fix_decimal(&raw, tags::BID_PX)
    }

    /// Whether `BidPx` (tag 132) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_px(&self) -> bool {
        self.msg.body().has_field(tags::BID_PX)
    }

    // --- OfferPx (tag 133, PRICE, optional) ---

    /// Set `OfferPx` (tag 133, optional); written in FIX `PRICE` format.
    pub fn set_offer_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::OFFER_PX, convert::format_fix_decimal(v));
    }

    /// `OfferPx` (tag 133, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn offer_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::OFFER_PX)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_PX)
    }

    /// Whether `OfferPx` (tag 133) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_px(&self) -> bool {
        self.msg.body().has_field(tags::OFFER_PX)
    }

    // --- MktBidPx (tag 645, PRICE, optional) ---

    /// Set `MktBidPx` (tag 645, optional); written in FIX `PRICE` format.
    pub fn set_mkt_bid_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MKT_BID_PX, convert::format_fix_decimal(v));
    }

    /// `MktBidPx` (tag 645, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn mkt_bid_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MKT_BID_PX)?;
        convert::parse_fix_decimal(&raw, tags::MKT_BID_PX)
    }

    /// Whether `MktBidPx` (tag 645) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mkt_bid_px(&self) -> bool {
        self.msg.body().has_field(tags::MKT_BID_PX)
    }

    // --- MktOfferPx (tag 646, PRICE, optional) ---

    /// Set `MktOfferPx` (tag 646, optional); written in FIX `PRICE` format.
    pub fn set_mkt_offer_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MKT_OFFER_PX, convert::format_fix_decimal(v));
    }

    /// `MktOfferPx` (tag 646, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn mkt_offer_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MKT_OFFER_PX)?;
        convert::parse_fix_decimal(&raw, tags::MKT_OFFER_PX)
    }

    /// Whether `MktOfferPx` (tag 646) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mkt_offer_px(&self) -> bool {
        self.msg.body().has_field(tags::MKT_OFFER_PX)
    }

    // --- MinBidSize (tag 647, QTY, optional) ---

    /// Set `MinBidSize` (tag 647, optional); written in FIX `QTY` format.
    pub fn set_min_bid_size(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MIN_BID_SIZE, convert::format_fix_decimal(v));
    }

    /// `MinBidSize` (tag 647, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn min_bid_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MIN_BID_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::MIN_BID_SIZE)
    }

    /// Whether `MinBidSize` (tag 647) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_min_bid_size(&self) -> bool {
        self.msg.body().has_field(tags::MIN_BID_SIZE)
    }

    // --- BidSize (tag 134, QTY, optional) ---

    /// Set `BidSize` (tag 134, optional); written in FIX `QTY` format.
    pub fn set_bid_size(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::BID_SIZE, convert::format_fix_decimal(v));
    }

    /// `BidSize` (tag 134, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn bid_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::BID_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::BID_SIZE)
    }

    /// Whether `BidSize` (tag 134) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_size(&self) -> bool {
        self.msg.body().has_field(tags::BID_SIZE)
    }

    // --- MinOfferSize (tag 648, QTY, optional) ---

    /// Set `MinOfferSize` (tag 648, optional); written in FIX `QTY` format.
    pub fn set_min_offer_size(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MIN_OFFER_SIZE, convert::format_fix_decimal(v));
    }

    /// `MinOfferSize` (tag 648, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn min_offer_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MIN_OFFER_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::MIN_OFFER_SIZE)
    }

    /// Whether `MinOfferSize` (tag 648) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_min_offer_size(&self) -> bool {
        self.msg.body().has_field(tags::MIN_OFFER_SIZE)
    }

    // --- OfferSize (tag 135, QTY, optional) ---

    /// Set `OfferSize` (tag 135, optional); written in FIX `QTY` format.
    pub fn set_offer_size(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::OFFER_SIZE, convert::format_fix_decimal(v));
    }

    /// `OfferSize` (tag 135, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn offer_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::OFFER_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_SIZE)
    }

    /// Whether `OfferSize` (tag 135) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_size(&self) -> bool {
        self.msg.body().has_field(tags::OFFER_SIZE)
    }

    // --- ValidUntilTime (tag 62, UTCTIMESTAMP, optional) ---

    /// Set `ValidUntilTime` (tag 62, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_valid_until_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::VALID_UNTIL_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `ValidUntilTime` (tag 62, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn valid_until_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::VALID_UNTIL_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::VALID_UNTIL_TIME)
    }

    /// Whether `ValidUntilTime` (tag 62) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_valid_until_time(&self) -> bool {
        self.msg.body().has_field(tags::VALID_UNTIL_TIME)
    }

    // --- BidSpotRate (tag 188, PRICE, optional) ---

    /// Set `BidSpotRate` (tag 188, optional); written in FIX `PRICE` format.
    pub fn set_bid_spot_rate(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::BID_SPOT_RATE, convert::format_fix_decimal(v));
    }

    /// `BidSpotRate` (tag 188, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn bid_spot_rate(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::BID_SPOT_RATE)?;
        convert::parse_fix_decimal(&raw, tags::BID_SPOT_RATE)
    }

    /// Whether `BidSpotRate` (tag 188) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_spot_rate(&self) -> bool {
        self.msg.body().has_field(tags::BID_SPOT_RATE)
    }

    // --- OfferSpotRate (tag 190, PRICE, optional) ---

    /// Set `OfferSpotRate` (tag 190, optional); written in FIX `PRICE` format.
    pub fn set_offer_spot_rate(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::OFFER_SPOT_RATE, convert::format_fix_decimal(v));
    }

    /// `OfferSpotRate` (tag 190, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn offer_spot_rate(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::OFFER_SPOT_RATE)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_SPOT_RATE)
    }

    /// Whether `OfferSpotRate` (tag 190) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_spot_rate(&self) -> bool {
        self.msg.body().has_field(tags::OFFER_SPOT_RATE)
    }

    // --- BidForwardPoints (tag 189, PRICEOFFSET, optional) ---

    /// Set `BidForwardPoints` (tag 189, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_bid_forward_points(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::BID_FORWARD_POINTS, convert::format_fix_decimal(v));
    }

    /// `BidForwardPoints` (tag 189, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn bid_forward_points(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::BID_FORWARD_POINTS)?;
        convert::parse_fix_decimal(&raw, tags::BID_FORWARD_POINTS)
    }

    /// Whether `BidForwardPoints` (tag 189) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_forward_points(&self) -> bool {
        self.msg.body().has_field(tags::BID_FORWARD_POINTS)
    }

    // --- OfferForwardPoints (tag 191, PRICEOFFSET, optional) ---

    /// Set `OfferForwardPoints` (tag 191, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_offer_forward_points(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::OFFER_FORWARD_POINTS, convert::format_fix_decimal(v));
    }

    /// `OfferForwardPoints` (tag 191, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn offer_forward_points(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::OFFER_FORWARD_POINTS)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_FORWARD_POINTS)
    }

    /// Whether `OfferForwardPoints` (tag 191) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_forward_points(&self) -> bool {
        self.msg.body().has_field(tags::OFFER_FORWARD_POINTS)
    }

    // --- MidPx (tag 631, PRICE, optional) ---

    /// Set `MidPx` (tag 631, optional); written in FIX `PRICE` format.
    pub fn set_mid_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MID_PX, convert::format_fix_decimal(v));
    }

    /// `MidPx` (tag 631, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn mid_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MID_PX)?;
        convert::parse_fix_decimal(&raw, tags::MID_PX)
    }

    /// Whether `MidPx` (tag 631) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mid_px(&self) -> bool {
        self.msg.body().has_field(tags::MID_PX)
    }

    // --- BidYield (tag 632, PERCENTAGE, optional) ---

    /// Set `BidYield` (tag 632, optional).
    pub fn set_bid_yield(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::BID_YIELD, v.to_string());
    }

    /// `BidYield` (tag 632, optional).
    pub fn bid_yield(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::BID_YIELD)
    }

    /// Whether `BidYield` (tag 632) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_yield(&self) -> bool {
        self.msg.body().has_field(tags::BID_YIELD)
    }

    // --- MidYield (tag 633, PERCENTAGE, optional) ---

    /// Set `MidYield` (tag 633, optional).
    pub fn set_mid_yield(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::MID_YIELD, v.to_string());
    }

    /// `MidYield` (tag 633, optional).
    pub fn mid_yield(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::MID_YIELD)
    }

    /// Whether `MidYield` (tag 633) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mid_yield(&self) -> bool {
        self.msg.body().has_field(tags::MID_YIELD)
    }

    // --- OfferYield (tag 634, PERCENTAGE, optional) ---

    /// Set `OfferYield` (tag 634, optional).
    pub fn set_offer_yield(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::OFFER_YIELD, v.to_string());
    }

    /// `OfferYield` (tag 634, optional).
    pub fn offer_yield(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::OFFER_YIELD)
    }

    /// Whether `OfferYield` (tag 634) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_yield(&self) -> bool {
        self.msg.body().has_field(tags::OFFER_YIELD)
    }

    // --- TransactTime (tag 60, UTCTIMESTAMP, optional) ---

    /// Set `TransactTime` (tag 60, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_transact_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::TRANSACT_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TransactTime` (tag 60, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn transact_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRANSACT_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRANSACT_TIME)
    }

    /// Whether `TransactTime` (tag 60) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_transact_time(&self) -> bool {
        self.msg.body().has_field(tags::TRANSACT_TIME)
    }

    // --- SettlmntTyp (tag 63, CHAR, optional) ---

    /// Set `SettlmntTyp` (tag 63, optional).
    pub fn set_settlmnt_typ(&mut self, v: fields::SettlmntTyp) {
        self.msg.body_mut().set_field(tags::SETTLMNT_TYP, v.to_fix().to_string());
    }

    /// `SettlmntTyp` (tag 63, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SettlmntTyp`]'s.
    pub fn settlmnt_typ(&self) -> Result<fields::SettlmntTyp, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SETTLMNT_TYP)?;
        fields::SettlmntTyp::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SETTLMNT_TYP,
        })
    }

    /// Whether `SettlmntTyp` (tag 63) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settlmnt_typ(&self) -> bool {
        self.msg.body().has_field(tags::SETTLMNT_TYP)
    }

    // --- FutSettDate (tag 64, LOCALMKTDATE, optional) ---

    /// Set `FutSettDate` (tag 64, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_fut_sett_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::FUT_SETT_DATE, convert::format_fix_date(v));
    }

    /// `FutSettDate` (tag 64, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn fut_sett_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::FUT_SETT_DATE)?;
        convert::parse_fix_date(&raw, tags::FUT_SETT_DATE)
    }

    /// Whether `FutSettDate` (tag 64) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fut_sett_date(&self) -> bool {
        self.msg.body().has_field(tags::FUT_SETT_DATE)
    }

    // --- OrdType (tag 40, CHAR, optional) ---

    /// Set `OrdType` (tag 40, optional).
    pub fn set_ord_type(&mut self, v: fields::OrdType) {
        self.msg.body_mut().set_field(tags::ORD_TYPE, v.to_fix().to_string());
    }

    /// `OrdType` (tag 40, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrdType`]'s.
    pub fn ord_type(&self) -> Result<fields::OrdType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::ORD_TYPE)?;
        fields::OrdType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORD_TYPE,
        })
    }

    /// Whether `OrdType` (tag 40) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ord_type(&self) -> bool {
        self.msg.body().has_field(tags::ORD_TYPE)
    }

    // --- FutSettDate2 (tag 193, LOCALMKTDATE, optional) ---

    /// Set `FutSettDate2` (tag 193, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_fut_sett_date2(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::FUT_SETT_DATE2, convert::format_fix_date(v));
    }

    /// `FutSettDate2` (tag 193, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn fut_sett_date2(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::FUT_SETT_DATE2)?;
        convert::parse_fix_date(&raw, tags::FUT_SETT_DATE2)
    }

    /// Whether `FutSettDate2` (tag 193) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fut_sett_date2(&self) -> bool {
        self.msg.body().has_field(tags::FUT_SETT_DATE2)
    }

    // --- OrderQty2 (tag 192, QTY, optional) ---

    /// Set `OrderQty2` (tag 192, optional); written in FIX `QTY` format.
    pub fn set_order_qty2(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::ORDER_QTY2, convert::format_fix_decimal(v));
    }

    /// `OrderQty2` (tag 192, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn order_qty2(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::ORDER_QTY2)?;
        convert::parse_fix_decimal(&raw, tags::ORDER_QTY2)
    }

    /// Whether `OrderQty2` (tag 192) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_qty2(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_QTY2)
    }

    // --- BidForwardPoints2 (tag 642, PRICEOFFSET, optional) ---

    /// Set `BidForwardPoints2` (tag 642, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_bid_forward_points2(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::BID_FORWARD_POINTS2, convert::format_fix_decimal(v));
    }

    /// `BidForwardPoints2` (tag 642, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn bid_forward_points2(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::BID_FORWARD_POINTS2)?;
        convert::parse_fix_decimal(&raw, tags::BID_FORWARD_POINTS2)
    }

    /// Whether `BidForwardPoints2` (tag 642) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_forward_points2(&self) -> bool {
        self.msg.body().has_field(tags::BID_FORWARD_POINTS2)
    }

    // --- OfferForwardPoints2 (tag 643, PRICEOFFSET, optional) ---

    /// Set `OfferForwardPoints2` (tag 643, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_offer_forward_points2(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::OFFER_FORWARD_POINTS2, convert::format_fix_decimal(v));
    }

    /// `OfferForwardPoints2` (tag 643, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn offer_forward_points2(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::OFFER_FORWARD_POINTS2)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_FORWARD_POINTS2)
    }

    /// Whether `OfferForwardPoints2` (tag 643) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_forward_points2(&self) -> bool {
        self.msg.body().has_field(tags::OFFER_FORWARD_POINTS2)
    }

    // --- Currency (tag 15, CURRENCY, optional) ---

    /// Set `Currency` (tag 15, optional).
    pub fn set_currency(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CURRENCY, v.to_string());
    }

    /// `Currency` (tag 15, optional).
    pub fn currency(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CURRENCY)
    }

    /// Whether `Currency` (tag 15) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_currency(&self) -> bool {
        self.msg.body().has_field(tags::CURRENCY)
    }

    // --- SettlCurrBidFxRate (tag 656, FLOAT, optional) ---

    /// Set `SettlCurrBidFxRate` (tag 656, optional).
    pub fn set_settl_curr_bid_fx_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::SETTL_CURR_BID_FX_RATE, v.to_string());
    }

    /// `SettlCurrBidFxRate` (tag 656, optional).
    pub fn settl_curr_bid_fx_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::SETTL_CURR_BID_FX_RATE)
    }

    /// Whether `SettlCurrBidFxRate` (tag 656) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_curr_bid_fx_rate(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_CURR_BID_FX_RATE)
    }

    // --- SettlCurrOfferFxRate (tag 657, FLOAT, optional) ---

    /// Set `SettlCurrOfferFxRate` (tag 657, optional).
    pub fn set_settl_curr_offer_fx_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::SETTL_CURR_OFFER_FX_RATE, v.to_string());
    }

    /// `SettlCurrOfferFxRate` (tag 657, optional).
    pub fn settl_curr_offer_fx_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::SETTL_CURR_OFFER_FX_RATE)
    }

    /// Whether `SettlCurrOfferFxRate` (tag 657) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_curr_offer_fx_rate(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_CURR_OFFER_FX_RATE)
    }

    // --- SettlCurrFxRateCalc (tag 156, CHAR, optional) ---

    /// Set `SettlCurrFxRateCalc` (tag 156, optional).
    pub fn set_settl_curr_fx_rate_calc(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::SETTL_CURR_FX_RATE_CALC, v.to_string());
    }

    /// `SettlCurrFxRateCalc` (tag 156, optional).
    pub fn settl_curr_fx_rate_calc(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::SETTL_CURR_FX_RATE_CALC)
    }

    /// Whether `SettlCurrFxRateCalc` (tag 156) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_curr_fx_rate_calc(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_CURR_FX_RATE_CALC)
    }

    // --- CommType (tag 13, CHAR, optional) ---

    /// Set `CommType` (tag 13, optional).
    pub fn set_comm_type(&mut self, v: fields::CommType) {
        self.msg.body_mut().set_field(tags::COMM_TYPE, v.to_fix().to_string());
    }

    /// `CommType` (tag 13, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::CommType`]'s.
    pub fn comm_type(&self) -> Result<fields::CommType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::COMM_TYPE)?;
        fields::CommType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::COMM_TYPE,
        })
    }

    /// Whether `CommType` (tag 13) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_comm_type(&self) -> bool {
        self.msg.body().has_field(tags::COMM_TYPE)
    }

    // --- Commission (tag 12, AMT, optional) ---

    /// Set `Commission` (tag 12, optional); written in FIX `AMT` format.
    pub fn set_commission(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::COMMISSION, convert::format_fix_decimal(v));
    }

    /// `Commission` (tag 12, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn commission(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::COMMISSION)?;
        convert::parse_fix_decimal(&raw, tags::COMMISSION)
    }

    /// Whether `Commission` (tag 12) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_commission(&self) -> bool {
        self.msg.body().has_field(tags::COMMISSION)
    }

    // --- CustOrderCapacity (tag 582, INT, optional) ---

    /// Set `CustOrderCapacity` (tag 582, optional).
    pub fn set_cust_order_capacity(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::CUST_ORDER_CAPACITY, v.to_string());
    }

    /// `CustOrderCapacity` (tag 582, optional).
    pub fn cust_order_capacity(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::CUST_ORDER_CAPACITY)
    }

    /// Whether `CustOrderCapacity` (tag 582) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cust_order_capacity(&self) -> bool {
        self.msg.body().has_field(tags::CUST_ORDER_CAPACITY)
    }

    // --- ExDestination (tag 100, EXCHANGE, optional) ---

    /// Set `ExDestination` (tag 100, optional).
    pub fn set_ex_destination(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::EX_DESTINATION, v.to_string());
    }

    /// `ExDestination` (tag 100, optional).
    pub fn ex_destination(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::EX_DESTINATION)
    }

    /// Whether `ExDestination` (tag 100) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ex_destination(&self) -> bool {
        self.msg.body().has_field(tags::EX_DESTINATION)
    }

    // --- Text (tag 58, STRING, optional) ---

    /// Set `Text` (tag 58, optional).
    pub fn set_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TEXT, v.to_string());
    }

    /// `Text` (tag 58, optional).
    pub fn text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TEXT)
    }

    /// Whether `Text` (tag 58) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_text(&self) -> bool {
        self.msg.body().has_field(tags::TEXT)
    }

    // --- EncodedTextLen (tag 354, LENGTH, optional) ---

    /// Set `EncodedTextLen` (tag 354, optional).
    pub fn set_encoded_text_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_TEXT_LEN, v.to_string());
    }

    /// `EncodedTextLen` (tag 354, optional).
    pub fn encoded_text_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_TEXT_LEN)
    }

    /// Whether `EncodedTextLen` (tag 354) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_TEXT_LEN)
    }

    // --- EncodedText (tag 355, DATA, optional) ---

    /// Set `EncodedText` (tag 355, optional).
    pub fn set_encoded_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_TEXT, v.to_string());
    }

    /// `EncodedText` (tag 355, optional).
    pub fn encoded_text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_TEXT)
    }

    /// Whether `EncodedText` (tag 355) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_TEXT)
    }

    // --- NoPartyIds group (count tag NO_PARTY_IDS) ---

    /// The `NoPartyIds` group (count tag `NO_PARTY_IDS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_party_ids(&self) -> Option<NoPartyIds<'_>> {
        self.msg.body().get_group(tags::NO_PARTY_IDS).map(|group| NoPartyIds { group })
    }

    /// The `NoPartyIds` group (count tag `NO_PARTY_IDS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_party_ids_mut(&mut self) -> Option<NoPartyIdsMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_PARTY_IDS).map(|group| NoPartyIdsMut { group })
    }

    /// Create the `NoPartyIds` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_party_ids_entry`] on the same group.
    pub fn set_no_party_ids(&mut self, count: u32) -> NoPartyIdsMut<'_> {
        let group =
            self.msg.body_mut().set_group(tags::NO_PARTY_IDS, count, NO_PARTY_IDS_FIELD_ORDER);
        NoPartyIdsMut { group }
    }

    /// Append one `NoPartyIds` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_party_ids`] on the same group.
    pub fn add_no_party_ids_entry(&mut self, fill: impl FnOnce(&mut NoPartyIdsEntryMut<'_>)) {
        let inner =
            self.msg.body_mut().add_group_instance(tags::NO_PARTY_IDS, NO_PARTY_IDS_FIELD_ORDER);
        fill(&mut NoPartyIdsEntryMut { inner });
    }

    // --- NoSecurityAltId group (count tag NO_SECURITY_ALT_ID) ---

    /// The `NoSecurityAltId` group (count tag `NO_SECURITY_ALT_ID`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_security_alt_id(&self) -> Option<NoSecurityAltId<'_>> {
        self.msg.body().get_group(tags::NO_SECURITY_ALT_ID).map(|group| NoSecurityAltId { group })
    }

    /// The `NoSecurityAltId` group (count tag `NO_SECURITY_ALT_ID`, optional) for editing, or `None` if
    /// absent.
    pub fn no_security_alt_id_mut(&mut self) -> Option<NoSecurityAltIdMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_SECURITY_ALT_ID)
            .map(|group| NoSecurityAltIdMut { group })
    }

    /// Create the `NoSecurityAltId` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_security_alt_id_entry`] on the same group.
    pub fn set_no_security_alt_id(&mut self, count: u32) -> NoSecurityAltIdMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_SECURITY_ALT_ID,
            count,
            NO_SECURITY_ALT_ID_FIELD_ORDER,
        );
        NoSecurityAltIdMut { group }
    }

    /// Append one `NoSecurityAltId` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_security_alt_id`] on the same group.
    pub fn add_no_security_alt_id_entry(
        &mut self,
        fill: impl FnOnce(&mut NoSecurityAltIdEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_SECURITY_ALT_ID, NO_SECURITY_ALT_ID_FIELD_ORDER);
        fill(&mut NoSecurityAltIdEntryMut { inner });
    }

    /// The underlying raw message (e.g. for fields this facade has no accessor for).
    pub fn message(&self) -> &Message {
        &self.msg
    }

    /// Mutable access to the underlying raw message (e.g. to set a custom field, D1a).
    pub fn message_mut(&mut self) -> &mut Message {
        &mut self.msg
    }
}

/// Wire order of a `NoPartyIds` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_PARTY_IDS_FIELD_ORDER: &[u32] = &[
    tags::PARTY_ID,
    tags::PARTY_ID_SOURCE,
    tags::PARTY_ROLE,
    tags::PARTY_SUB_ID,
];

/// One `NoPartyIds` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoPartyIdsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoPartyIdsEntry<'_> {
    /// `PartyID` (tag 448, optional).
    pub fn party_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::PARTY_ID)
    }

    /// Whether `PartyID` (tag 448) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_party_id(&self) -> bool {
        self.inner.has_field(tags::PARTY_ID)
    }

    /// `PartyIDSource` (tag 447, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::PartyIdSource`]'s.
    pub fn party_id_source(&self) -> Result<fields::PartyIdSource, FieldError> {
        let n = self.inner.get_field::<char>(tags::PARTY_ID_SOURCE)?;
        fields::PartyIdSource::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PARTY_ID_SOURCE,
        })
    }

    /// Whether `PartyIDSource` (tag 447) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_party_id_source(&self) -> bool {
        self.inner.has_field(tags::PARTY_ID_SOURCE)
    }

    /// `PartyRole` (tag 452, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::PartyRole`]'s.
    pub fn party_role(&self) -> Result<fields::PartyRole, FieldError> {
        let n = self.inner.get_field::<i32>(tags::PARTY_ROLE)?;
        fields::PartyRole::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PARTY_ROLE,
        })
    }

    /// Whether `PartyRole` (tag 452) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_party_role(&self) -> bool {
        self.inner.has_field(tags::PARTY_ROLE)
    }

    /// `PartySubID` (tag 523, optional).
    pub fn party_sub_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::PARTY_SUB_ID)
    }

    /// Whether `PartySubID` (tag 523) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_party_sub_id(&self) -> bool {
        self.inner.has_field(tags::PARTY_SUB_ID)
    }
}

/// One `NoPartyIds` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoPartyIdsEntryMut::as_ref`].
pub struct NoPartyIdsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoPartyIdsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoPartyIdsEntry<'_> {
        NoPartyIdsEntry { inner: self.inner }
    }

    /// Set `PartyID` (tag 448, optional).
    pub fn set_party_id(&mut self, v: &str) {
        self.inner.set_field(tags::PARTY_ID, v.to_string());
    }

    /// Set `PartyIDSource` (tag 447, optional).
    pub fn set_party_id_source(&mut self, v: fields::PartyIdSource) {
        self.inner.set_field(tags::PARTY_ID_SOURCE, v.to_fix().to_string());
    }

    /// Set `PartyRole` (tag 452, optional).
    pub fn set_party_role(&mut self, v: fields::PartyRole) {
        self.inner.set_field(tags::PARTY_ROLE, v.to_fix().to_string());
    }

    /// Set `PartySubID` (tag 523, optional).
    pub fn set_party_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::PARTY_SUB_ID, v.to_string());
    }
}

/// The `NoPartyIds` repeating group (count tag `NO_PARTY_IDS`), read-only: its
/// instances as [`NoPartyIdsEntry`] views.
pub struct NoPartyIds<'a> {
    group: &'a Group,
}

impl<'a> NoPartyIds<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoPartyIdsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoPartyIdsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoPartyIdsEntry<'a> {
        self.get(i).expect("NoPartyIds index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoPartyIdsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoPartyIdsEntry { inner })
    }
}

/// The `NoPartyIds` repeating group (count tag `NO_PARTY_IDS`), mutable: its
/// instances as [`NoPartyIdsEntryMut`] views.
pub struct NoPartyIdsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoPartyIdsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoPartyIdsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoPartyIdsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoPartyIdsEntryMut<'_> {
        self.get_mut(i).expect("NoPartyIds index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoPartyIdsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoPartyIdsEntryMut { inner })
    }
}

/// Wire order of a `NoSecurityAltId` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_SECURITY_ALT_ID_FIELD_ORDER: &[u32] =
    &[tags::SECURITY_ALT_ID, tags::SECURITY_ALT_ID_SOURCE];

/// One `NoSecurityAltId` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoSecurityAltIdEntry<'a> {
    inner: &'a FieldMap,
}

impl NoSecurityAltIdEntry<'_> {
    /// `SecurityAltID` (tag 455, optional).
    pub fn security_alt_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECURITY_ALT_ID)
    }

    /// Whether `SecurityAltID` (tag 455) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_alt_id(&self) -> bool {
        self.inner.has_field(tags::SECURITY_ALT_ID)
    }

    /// `SecurityAltIDSource` (tag 456, optional).
    pub fn security_alt_id_source(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECURITY_ALT_ID_SOURCE)
    }

    /// Whether `SecurityAltIDSource` (tag 456) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_alt_id_source(&self) -> bool {
        self.inner.has_field(tags::SECURITY_ALT_ID_SOURCE)
    }
}

/// One `NoSecurityAltId` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoSecurityAltIdEntryMut::as_ref`].
pub struct NoSecurityAltIdEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoSecurityAltIdEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoSecurityAltIdEntry<'_> {
        NoSecurityAltIdEntry { inner: self.inner }
    }

    /// Set `SecurityAltID` (tag 455, optional).
    pub fn set_security_alt_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECURITY_ALT_ID, v.to_string());
    }

    /// Set `SecurityAltIDSource` (tag 456, optional).
    pub fn set_security_alt_id_source(&mut self, v: &str) {
        self.inner.set_field(tags::SECURITY_ALT_ID_SOURCE, v.to_string());
    }
}

/// The `NoSecurityAltId` repeating group (count tag `NO_SECURITY_ALT_ID`), read-only: its
/// instances as [`NoSecurityAltIdEntry`] views.
pub struct NoSecurityAltId<'a> {
    group: &'a Group,
}

impl<'a> NoSecurityAltId<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoSecurityAltIdEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoSecurityAltIdEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoSecurityAltIdEntry<'a> {
        self.get(i).expect("NoSecurityAltId index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoSecurityAltIdEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoSecurityAltIdEntry { inner })
    }
}

/// The `NoSecurityAltId` repeating group (count tag `NO_SECURITY_ALT_ID`), mutable: its
/// instances as [`NoSecurityAltIdEntryMut`] views.
pub struct NoSecurityAltIdMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoSecurityAltIdMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoSecurityAltIdEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoSecurityAltIdEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoSecurityAltIdEntryMut<'_> {
        self.get_mut(i).expect("NoSecurityAltId index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoSecurityAltIdEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoSecurityAltIdEntryMut { inner })
    }
}
