//! `MarketDataSnapshotFullRefresh` (app message, MsgType `35=W`): typed facade over the raw
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

/// Typed facade over a `MarketDataSnapshotFullRefresh` [`Message`] (35=W). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct MarketDataSnapshotFullRefresh {
    msg: Message,
}

impl Default for MarketDataSnapshotFullRefresh {
    fn default() -> Self {
        MarketDataSnapshotFullRefresh::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `MarketDataSnapshotFullRefresh`, checking the msg type.
impl TryFrom<Message> for MarketDataSnapshotFullRefresh {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a MarketDataSnapshotFullRefresh" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<MarketDataSnapshotFullRefresh> for Message {
    fn from(typed_msg: MarketDataSnapshotFullRefresh) -> Self {
        typed_msg.msg
    }
}

impl MarketDataSnapshotFullRefresh {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "W";

    /// An empty `MarketDataSnapshotFullRefresh` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- MDReqID (tag 262, STRING, optional) ---

    /// Set `MDReqID` (tag 262, optional).
    pub fn set_md_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::MD_REQ_ID, v.to_string());
    }

    /// `MDReqID` (tag 262, optional).
    pub fn md_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::MD_REQ_ID)
    }

    /// Whether `MDReqID` (tag 262) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_req_id(&self) -> bool {
        self.msg.body().has_field(tags::MD_REQ_ID)
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

    // --- FinancialStatus (tag 291, MULTIPLEVALUESTRING, optional) ---

    /// Set `FinancialStatus` (tag 291, optional).
    pub fn set_financial_status(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::FINANCIAL_STATUS, v.to_string());
    }

    /// `FinancialStatus` (tag 291, optional).
    pub fn financial_status(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::FINANCIAL_STATUS)
    }

    /// Whether `FinancialStatus` (tag 291) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_financial_status(&self) -> bool {
        self.msg.body().has_field(tags::FINANCIAL_STATUS)
    }

    // --- CorporateAction (tag 292, MULTIPLEVALUESTRING, optional) ---

    /// Set `CorporateAction` (tag 292, optional).
    pub fn set_corporate_action(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CORPORATE_ACTION, v.to_string());
    }

    /// `CorporateAction` (tag 292, optional).
    pub fn corporate_action(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CORPORATE_ACTION)
    }

    /// Whether `CorporateAction` (tag 292) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_corporate_action(&self) -> bool {
        self.msg.body().has_field(tags::CORPORATE_ACTION)
    }

    // --- TotalVolumeTraded (tag 387, QTY, optional) ---

    /// Set `TotalVolumeTraded` (tag 387, optional); written in FIX `QTY` format.
    pub fn set_total_volume_traded(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::TOTAL_VOLUME_TRADED, convert::format_fix_decimal(v));
    }

    /// `TotalVolumeTraded` (tag 387, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn total_volume_traded(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TOTAL_VOLUME_TRADED)?;
        convert::parse_fix_decimal(&raw, tags::TOTAL_VOLUME_TRADED)
    }

    /// Whether `TotalVolumeTraded` (tag 387) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_volume_traded(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_VOLUME_TRADED)
    }

    // --- TotalVolumeTradedDate (tag 449, UTCDATE, optional) ---

    /// Set `TotalVolumeTradedDate` (tag 449, optional); written in FIX `UTCDATE` format.
    pub fn set_total_volume_traded_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::TOTAL_VOLUME_TRADED_DATE, convert::format_fix_date(v));
    }

    /// `TotalVolumeTradedDate` (tag 449, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn total_volume_traded_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TOTAL_VOLUME_TRADED_DATE)?;
        convert::parse_fix_date(&raw, tags::TOTAL_VOLUME_TRADED_DATE)
    }

    /// Whether `TotalVolumeTradedDate` (tag 449) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_volume_traded_date(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_VOLUME_TRADED_DATE)
    }

    // --- TotalVolumeTradedTime (tag 450, UTCTIMEONLY, optional) ---

    /// Set `TotalVolumeTradedTime` (tag 450, optional); written in FIX `UTCTIMEONLY` format.
    pub fn set_total_volume_traded_time(&mut self, v: NaiveTime) {
        self.msg
            .body_mut()
            .set_field(tags::TOTAL_VOLUME_TRADED_TIME, convert::format_fix_time_only(v));
    }

    /// `TotalVolumeTradedTime` (tag 450, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMEONLY`.
    pub fn total_volume_traded_time(&self) -> Result<NaiveTime, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TOTAL_VOLUME_TRADED_TIME)?;
        convert::parse_fix_time_only(&raw, tags::TOTAL_VOLUME_TRADED_TIME)
    }

    /// Whether `TotalVolumeTradedTime` (tag 450) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_volume_traded_time(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_VOLUME_TRADED_TIME)
    }

    // --- NetChgPrevDay (tag 451, PRICEOFFSET, optional) ---

    /// Set `NetChgPrevDay` (tag 451, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_net_chg_prev_day(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::NET_CHG_PREV_DAY, convert::format_fix_decimal(v));
    }

    /// `NetChgPrevDay` (tag 451, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn net_chg_prev_day(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::NET_CHG_PREV_DAY)?;
        convert::parse_fix_decimal(&raw, tags::NET_CHG_PREV_DAY)
    }

    /// Whether `NetChgPrevDay` (tag 451) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_net_chg_prev_day(&self) -> bool {
        self.msg.body().has_field(tags::NET_CHG_PREV_DAY)
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

    // --- NoMdEntries group (count tag NO_MD_ENTRIES) ---

    /// The `NoMdEntries` group (count tag `NO_MD_ENTRIES`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_md_entries(&self) -> Option<NoMdEntries<'_>> {
        self.msg.body().get_group(tags::NO_MD_ENTRIES).map(|group| NoMdEntries { group })
    }

    /// The `NoMdEntries` group (count tag `NO_MD_ENTRIES`, required) for editing, or `None` if
    /// absent.
    pub fn no_md_entries_mut(&mut self) -> Option<NoMdEntriesMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_MD_ENTRIES).map(|group| NoMdEntriesMut { group })
    }

    /// Create the `NoMdEntries` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_md_entries_entry`] on the same group.
    pub fn set_no_md_entries(&mut self, count: u32) -> NoMdEntriesMut<'_> {
        let group =
            self.msg.body_mut().set_group(tags::NO_MD_ENTRIES, count, NO_MD_ENTRIES_FIELD_ORDER);
        NoMdEntriesMut { group }
    }

    /// Append one `NoMdEntries` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_md_entries`] on the same group.
    pub fn add_no_md_entries_entry(&mut self, fill: impl FnOnce(&mut NoMdEntriesEntryMut<'_>)) {
        let inner =
            self.msg.body_mut().add_group_instance(tags::NO_MD_ENTRIES, NO_MD_ENTRIES_FIELD_ORDER);
        fill(&mut NoMdEntriesEntryMut { inner });
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

/// Wire order of a `NoMdEntries` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_MD_ENTRIES_FIELD_ORDER: &[u32] = &[
    tags::MD_ENTRY_TYPE,
    tags::MD_ENTRY_PX,
    tags::CURRENCY,
    tags::MD_ENTRY_SIZE,
    tags::MD_ENTRY_DATE,
    tags::MD_ENTRY_TIME,
    tags::TICK_DIRECTION,
    tags::MD_MKT,
    tags::TRADING_SESSION_ID,
    tags::TRADING_SESSION_SUB_ID,
    tags::QUOTE_CONDITION,
    tags::TRADE_CONDITION,
    tags::MD_ENTRY_ORIGINATOR,
    tags::LOCATION_ID,
    tags::DESK_ID,
    tags::OPEN_CLOSE_SETTLE_FLAG,
    tags::TIME_IN_FORCE,
    tags::EXPIRE_DATE,
    tags::EXPIRE_TIME,
    tags::MIN_QTY,
    tags::EXEC_INST,
    tags::SELLER_DAYS,
    tags::ORDER_ID,
    tags::QUOTE_ENTRY_ID,
    tags::MD_ENTRY_BUYER,
    tags::MD_ENTRY_SELLER,
    tags::NUMBER_OF_ORDERS,
    tags::MD_ENTRY_POSITION_NO,
    tags::SCOPE,
    tags::TEXT,
    tags::ENCODED_TEXT_LEN,
    tags::ENCODED_TEXT,
];

/// One `NoMdEntries` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoMdEntriesEntry<'a> {
    inner: &'a FieldMap,
}

impl NoMdEntriesEntry<'_> {
    /// `MDEntryType` (tag 269, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MdEntryType`]'s.
    pub fn md_entry_type(&self) -> Result<fields::MdEntryType, FieldError> {
        let n = self.inner.get_field::<char>(tags::MD_ENTRY_TYPE)?;
        fields::MdEntryType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MD_ENTRY_TYPE,
        })
    }

    /// Whether `MDEntryType` (tag 269) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_type(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_TYPE)
    }

    /// `MDEntryPx` (tag 270, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn md_entry_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MD_ENTRY_PX)?;
        convert::parse_fix_decimal(&raw, tags::MD_ENTRY_PX)
    }

    /// Whether `MDEntryPx` (tag 270) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_px(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_PX)
    }

    /// `Currency` (tag 15, optional).
    pub fn currency(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CURRENCY)
    }

    /// Whether `Currency` (tag 15) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_currency(&self) -> bool {
        self.inner.has_field(tags::CURRENCY)
    }

    /// `MDEntrySize` (tag 271, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn md_entry_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MD_ENTRY_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::MD_ENTRY_SIZE)
    }

    /// Whether `MDEntrySize` (tag 271) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_size(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_SIZE)
    }

    /// `MDEntryDate` (tag 272, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn md_entry_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MD_ENTRY_DATE)?;
        convert::parse_fix_date(&raw, tags::MD_ENTRY_DATE)
    }

    /// Whether `MDEntryDate` (tag 272) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_date(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_DATE)
    }

    /// `MDEntryTime` (tag 273, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMEONLY`.
    pub fn md_entry_time(&self) -> Result<NaiveTime, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MD_ENTRY_TIME)?;
        convert::parse_fix_time_only(&raw, tags::MD_ENTRY_TIME)
    }

    /// Whether `MDEntryTime` (tag 273) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_time(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_TIME)
    }

    /// `TickDirection` (tag 274, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TickDirection`]'s.
    pub fn tick_direction(&self) -> Result<fields::TickDirection, FieldError> {
        let n = self.inner.get_field::<char>(tags::TICK_DIRECTION)?;
        fields::TickDirection::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TICK_DIRECTION,
        })
    }

    /// Whether `TickDirection` (tag 274) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_tick_direction(&self) -> bool {
        self.inner.has_field(tags::TICK_DIRECTION)
    }

    /// `MDMkt` (tag 275, optional).
    pub fn md_mkt(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MD_MKT)
    }

    /// Whether `MDMkt` (tag 275) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_mkt(&self) -> bool {
        self.inner.has_field(tags::MD_MKT)
    }

    /// `TradingSessionID` (tag 336, optional).
    pub fn trading_session_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TRADING_SESSION_ID)
    }

    /// Whether `TradingSessionID` (tag 336) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trading_session_id(&self) -> bool {
        self.inner.has_field(tags::TRADING_SESSION_ID)
    }

    /// `TradingSessionSubID` (tag 625, optional).
    pub fn trading_session_sub_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TRADING_SESSION_SUB_ID)
    }

    /// Whether `TradingSessionSubID` (tag 625) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trading_session_sub_id(&self) -> bool {
        self.inner.has_field(tags::TRADING_SESSION_SUB_ID)
    }

    /// `QuoteCondition` (tag 276, optional).
    pub fn quote_condition(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::QUOTE_CONDITION)
    }

    /// Whether `QuoteCondition` (tag 276) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_condition(&self) -> bool {
        self.inner.has_field(tags::QUOTE_CONDITION)
    }

    /// `TradeCondition` (tag 277, optional).
    pub fn trade_condition(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TRADE_CONDITION)
    }

    /// Whether `TradeCondition` (tag 277) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_condition(&self) -> bool {
        self.inner.has_field(tags::TRADE_CONDITION)
    }

    /// `MDEntryOriginator` (tag 282, optional).
    pub fn md_entry_originator(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MD_ENTRY_ORIGINATOR)
    }

    /// Whether `MDEntryOriginator` (tag 282) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_originator(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_ORIGINATOR)
    }

    /// `LocationID` (tag 283, optional).
    pub fn location_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LOCATION_ID)
    }

    /// Whether `LocationID` (tag 283) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_location_id(&self) -> bool {
        self.inner.has_field(tags::LOCATION_ID)
    }

    /// `DeskID` (tag 284, optional).
    pub fn desk_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::DESK_ID)
    }

    /// Whether `DeskID` (tag 284) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_desk_id(&self) -> bool {
        self.inner.has_field(tags::DESK_ID)
    }

    /// `OpenCloseSettleFlag` (tag 286, optional).
    pub fn open_close_settle_flag(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::OPEN_CLOSE_SETTLE_FLAG)
    }

    /// Whether `OpenCloseSettleFlag` (tag 286) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_open_close_settle_flag(&self) -> bool {
        self.inner.has_field(tags::OPEN_CLOSE_SETTLE_FLAG)
    }

    /// `TimeInForce` (tag 59, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TimeInForce`]'s.
    pub fn time_in_force(&self) -> Result<fields::TimeInForce, FieldError> {
        let n = self.inner.get_field::<char>(tags::TIME_IN_FORCE)?;
        fields::TimeInForce::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TIME_IN_FORCE,
        })
    }

    /// Whether `TimeInForce` (tag 59) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_time_in_force(&self) -> bool {
        self.inner.has_field(tags::TIME_IN_FORCE)
    }

    /// `ExpireDate` (tag 432, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn expire_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::EXPIRE_DATE)?;
        convert::parse_fix_date(&raw, tags::EXPIRE_DATE)
    }

    /// Whether `ExpireDate` (tag 432) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_expire_date(&self) -> bool {
        self.inner.has_field(tags::EXPIRE_DATE)
    }

    /// `ExpireTime` (tag 126, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn expire_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.inner.get_field::<String>(tags::EXPIRE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::EXPIRE_TIME)
    }

    /// Whether `ExpireTime` (tag 126) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_expire_time(&self) -> bool {
        self.inner.has_field(tags::EXPIRE_TIME)
    }

    /// `MinQty` (tag 110, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn min_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MIN_QTY)?;
        convert::parse_fix_decimal(&raw, tags::MIN_QTY)
    }

    /// Whether `MinQty` (tag 110) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_min_qty(&self) -> bool {
        self.inner.has_field(tags::MIN_QTY)
    }

    /// `ExecInst` (tag 18, optional).
    pub fn exec_inst(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::EXEC_INST)
    }

    /// Whether `ExecInst` (tag 18) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exec_inst(&self) -> bool {
        self.inner.has_field(tags::EXEC_INST)
    }

    /// `SellerDays` (tag 287, optional).
    pub fn seller_days(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::SELLER_DAYS)
    }

    /// Whether `SellerDays` (tag 287) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_seller_days(&self) -> bool {
        self.inner.has_field(tags::SELLER_DAYS)
    }

    /// `OrderID` (tag 37, optional).
    pub fn order_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ORDER_ID)
    }

    /// Whether `OrderID` (tag 37) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_id(&self) -> bool {
        self.inner.has_field(tags::ORDER_ID)
    }

    /// `QuoteEntryID` (tag 299, optional).
    pub fn quote_entry_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::QUOTE_ENTRY_ID)
    }

    /// Whether `QuoteEntryID` (tag 299) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_entry_id(&self) -> bool {
        self.inner.has_field(tags::QUOTE_ENTRY_ID)
    }

    /// `MDEntryBuyer` (tag 288, optional).
    pub fn md_entry_buyer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MD_ENTRY_BUYER)
    }

    /// Whether `MDEntryBuyer` (tag 288) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_buyer(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_BUYER)
    }

    /// `MDEntrySeller` (tag 289, optional).
    pub fn md_entry_seller(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MD_ENTRY_SELLER)
    }

    /// Whether `MDEntrySeller` (tag 289) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_seller(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_SELLER)
    }

    /// `NumberOfOrders` (tag 346, optional).
    pub fn number_of_orders(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::NUMBER_OF_ORDERS)
    }

    /// Whether `NumberOfOrders` (tag 346) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_number_of_orders(&self) -> bool {
        self.inner.has_field(tags::NUMBER_OF_ORDERS)
    }

    /// `MDEntryPositionNo` (tag 290, optional).
    pub fn md_entry_position_no(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::MD_ENTRY_POSITION_NO)
    }

    /// Whether `MDEntryPositionNo` (tag 290) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_entry_position_no(&self) -> bool {
        self.inner.has_field(tags::MD_ENTRY_POSITION_NO)
    }

    /// `Scope` (tag 546, optional).
    pub fn scope(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SCOPE)
    }

    /// Whether `Scope` (tag 546) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_scope(&self) -> bool {
        self.inner.has_field(tags::SCOPE)
    }

    /// `Text` (tag 58, optional).
    pub fn text(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TEXT)
    }

    /// Whether `Text` (tag 58) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_text(&self) -> bool {
        self.inner.has_field(tags::TEXT)
    }

    /// `EncodedTextLen` (tag 354, optional).
    pub fn encoded_text_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_TEXT_LEN)
    }

    /// Whether `EncodedTextLen` (tag 354) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_TEXT_LEN)
    }

    /// `EncodedText` (tag 355, optional).
    pub fn encoded_text(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_TEXT)
    }

    /// Whether `EncodedText` (tag 355) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text(&self) -> bool {
        self.inner.has_field(tags::ENCODED_TEXT)
    }
}

/// One `NoMdEntries` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoMdEntriesEntryMut::as_ref`].
pub struct NoMdEntriesEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoMdEntriesEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoMdEntriesEntry<'_> {
        NoMdEntriesEntry { inner: self.inner }
    }

    /// Set `MDEntryType` (tag 269, required).
    pub fn set_md_entry_type(&mut self, v: fields::MdEntryType) {
        self.inner.set_field(tags::MD_ENTRY_TYPE, v.to_fix().to_string());
    }

    /// Set `MDEntryPx` (tag 270, optional); written in FIX `PRICE` format.
    pub fn set_md_entry_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::MD_ENTRY_PX, convert::format_fix_decimal(v));
    }

    /// Set `Currency` (tag 15, optional).
    pub fn set_currency(&mut self, v: &str) {
        self.inner.set_field(tags::CURRENCY, v.to_string());
    }

    /// Set `MDEntrySize` (tag 271, optional); written in FIX `QTY` format.
    pub fn set_md_entry_size(&mut self, v: Decimal) {
        self.inner.set_field(tags::MD_ENTRY_SIZE, convert::format_fix_decimal(v));
    }

    /// Set `MDEntryDate` (tag 272, optional); written in FIX `UTCDATE` format.
    pub fn set_md_entry_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::MD_ENTRY_DATE, convert::format_fix_date(v));
    }

    /// Set `MDEntryTime` (tag 273, optional); written in FIX `UTCTIMEONLY` format.
    pub fn set_md_entry_time(&mut self, v: NaiveTime) {
        self.inner.set_field(tags::MD_ENTRY_TIME, convert::format_fix_time_only(v));
    }

    /// Set `TickDirection` (tag 274, optional).
    pub fn set_tick_direction(&mut self, v: fields::TickDirection) {
        self.inner.set_field(tags::TICK_DIRECTION, v.to_fix().to_string());
    }

    /// Set `MDMkt` (tag 275, optional).
    pub fn set_md_mkt(&mut self, v: &str) {
        self.inner.set_field(tags::MD_MKT, v.to_string());
    }

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
    }

    /// Set `QuoteCondition` (tag 276, optional).
    pub fn set_quote_condition(&mut self, v: &str) {
        self.inner.set_field(tags::QUOTE_CONDITION, v.to_string());
    }

    /// Set `TradeCondition` (tag 277, optional).
    pub fn set_trade_condition(&mut self, v: &str) {
        self.inner.set_field(tags::TRADE_CONDITION, v.to_string());
    }

    /// Set `MDEntryOriginator` (tag 282, optional).
    pub fn set_md_entry_originator(&mut self, v: &str) {
        self.inner.set_field(tags::MD_ENTRY_ORIGINATOR, v.to_string());
    }

    /// Set `LocationID` (tag 283, optional).
    pub fn set_location_id(&mut self, v: &str) {
        self.inner.set_field(tags::LOCATION_ID, v.to_string());
    }

    /// Set `DeskID` (tag 284, optional).
    pub fn set_desk_id(&mut self, v: &str) {
        self.inner.set_field(tags::DESK_ID, v.to_string());
    }

    /// Set `OpenCloseSettleFlag` (tag 286, optional).
    pub fn set_open_close_settle_flag(&mut self, v: &str) {
        self.inner.set_field(tags::OPEN_CLOSE_SETTLE_FLAG, v.to_string());
    }

    /// Set `TimeInForce` (tag 59, optional).
    pub fn set_time_in_force(&mut self, v: fields::TimeInForce) {
        self.inner.set_field(tags::TIME_IN_FORCE, v.to_fix().to_string());
    }

    /// Set `ExpireDate` (tag 432, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_expire_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::EXPIRE_DATE, convert::format_fix_date(v));
    }

    /// Set `ExpireTime` (tag 126, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_expire_time(&mut self, v: DateTime<Utc>) {
        self.inner.set_field(tags::EXPIRE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// Set `MinQty` (tag 110, optional); written in FIX `QTY` format.
    pub fn set_min_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::MIN_QTY, convert::format_fix_decimal(v));
    }

    /// Set `ExecInst` (tag 18, optional).
    pub fn set_exec_inst(&mut self, v: &str) {
        self.inner.set_field(tags::EXEC_INST, v.to_string());
    }

    /// Set `SellerDays` (tag 287, optional).
    pub fn set_seller_days(&mut self, v: i32) {
        self.inner.set_field(tags::SELLER_DAYS, v.to_string());
    }

    /// Set `OrderID` (tag 37, optional).
    pub fn set_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::ORDER_ID, v.to_string());
    }

    /// Set `QuoteEntryID` (tag 299, optional).
    pub fn set_quote_entry_id(&mut self, v: &str) {
        self.inner.set_field(tags::QUOTE_ENTRY_ID, v.to_string());
    }

    /// Set `MDEntryBuyer` (tag 288, optional).
    pub fn set_md_entry_buyer(&mut self, v: &str) {
        self.inner.set_field(tags::MD_ENTRY_BUYER, v.to_string());
    }

    /// Set `MDEntrySeller` (tag 289, optional).
    pub fn set_md_entry_seller(&mut self, v: &str) {
        self.inner.set_field(tags::MD_ENTRY_SELLER, v.to_string());
    }

    /// Set `NumberOfOrders` (tag 346, optional).
    pub fn set_number_of_orders(&mut self, v: i32) {
        self.inner.set_field(tags::NUMBER_OF_ORDERS, v.to_string());
    }

    /// Set `MDEntryPositionNo` (tag 290, optional).
    pub fn set_md_entry_position_no(&mut self, v: i32) {
        self.inner.set_field(tags::MD_ENTRY_POSITION_NO, v.to_string());
    }

    /// Set `Scope` (tag 546, optional).
    pub fn set_scope(&mut self, v: &str) {
        self.inner.set_field(tags::SCOPE, v.to_string());
    }

    /// Set `Text` (tag 58, optional).
    pub fn set_text(&mut self, v: &str) {
        self.inner.set_field(tags::TEXT, v.to_string());
    }

    /// Set `EncodedTextLen` (tag 354, optional).
    pub fn set_encoded_text_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_TEXT_LEN, v.to_string());
    }

    /// Set `EncodedText` (tag 355, optional).
    pub fn set_encoded_text(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_TEXT, v.to_string());
    }
}

/// The `NoMdEntries` repeating group (count tag `NO_MD_ENTRIES`), read-only: its
/// instances as [`NoMdEntriesEntry`] views.
pub struct NoMdEntries<'a> {
    group: &'a Group,
}

impl<'a> NoMdEntries<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoMdEntriesEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoMdEntriesEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoMdEntriesEntry<'a> {
        self.get(i).expect("NoMdEntries index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoMdEntriesEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoMdEntriesEntry { inner })
    }
}

/// The `NoMdEntries` repeating group (count tag `NO_MD_ENTRIES`), mutable: its
/// instances as [`NoMdEntriesEntryMut`] views.
pub struct NoMdEntriesMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoMdEntriesMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoMdEntriesEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoMdEntriesEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoMdEntriesEntryMut<'_> {
        self.get_mut(i).expect("NoMdEntries index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoMdEntriesEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoMdEntriesEntryMut { inner })
    }
}
