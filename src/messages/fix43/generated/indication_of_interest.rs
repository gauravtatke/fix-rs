//! `IndicationOfInterest` (app message, MsgType `35=6`): typed facade over the raw
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

/// Typed facade over a `IndicationOfInterest` [`Message`] (35=6). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct IndicationOfInterest {
    msg: Message,
}

impl Default for IndicationOfInterest {
    fn default() -> Self {
        IndicationOfInterest::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `IndicationOfInterest`, checking the msg type.
impl TryFrom<Message> for IndicationOfInterest {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a IndicationOfInterest" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<IndicationOfInterest> for Message {
    fn from(typed_msg: IndicationOfInterest) -> Self {
        typed_msg.msg
    }
}

impl IndicationOfInterest {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "6";

    /// An empty `IndicationOfInterest` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- IOIID (tag 23, STRING, required) ---

    /// Set `IOIID` (tag 23, required).
    pub fn set_ioi_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::IOI_ID, v.to_string());
    }

    /// `IOIID` (tag 23, required).
    pub fn ioi_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::IOI_ID)
    }

    /// Whether `IOIID` (tag 23) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_id(&self) -> bool {
        self.msg.body().has_field(tags::IOI_ID)
    }

    // --- IOITransType (tag 28, CHAR, required) ---

    /// Set `IOITransType` (tag 28, required).
    pub fn set_ioi_trans_type(&mut self, v: fields::IoiTransType) {
        self.msg.body_mut().set_field(tags::IOI_TRANS_TYPE, v.to_fix().to_string());
    }

    /// `IOITransType` (tag 28, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::IoiTransType`]'s.
    pub fn ioi_trans_type(&self) -> Result<fields::IoiTransType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::IOI_TRANS_TYPE)?;
        fields::IoiTransType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::IOI_TRANS_TYPE,
        })
    }

    /// Whether `IOITransType` (tag 28) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_trans_type(&self) -> bool {
        self.msg.body().has_field(tags::IOI_TRANS_TYPE)
    }

    // --- IOIRefID (tag 26, STRING, optional) ---

    /// Set `IOIRefID` (tag 26, optional).
    pub fn set_ioi_ref_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::IOI_REF_ID, v.to_string());
    }

    /// `IOIRefID` (tag 26, optional).
    pub fn ioi_ref_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::IOI_REF_ID)
    }

    /// Whether `IOIRefID` (tag 26) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_ref_id(&self) -> bool {
        self.msg.body().has_field(tags::IOI_REF_ID)
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

    // --- Side (tag 54, CHAR, required) ---

    /// Set `Side` (tag 54, required).
    pub fn set_side(&mut self, v: fields::Side) {
        self.msg.body_mut().set_field(tags::SIDE, v.to_fix().to_string());
    }

    /// `Side` (tag 54, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Side`]'s.
    pub fn side(&self) -> Result<fields::Side, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SIDE)?;
        fields::Side::from_fix(n).ok_or(FieldError::InvalidFormat { tag: tags::SIDE })
    }

    /// Whether `Side` (tag 54) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side(&self) -> bool {
        self.msg.body().has_field(tags::SIDE)
    }

    // --- QuantityType (tag 465, INT, optional) ---

    /// Set `QuantityType` (tag 465, optional).
    pub fn set_quantity_type(&mut self, v: fields::QuantityType) {
        self.msg.body_mut().set_field(tags::QUANTITY_TYPE, v.to_fix().to_string());
    }

    /// `QuantityType` (tag 465, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::QuantityType`]'s.
    pub fn quantity_type(&self) -> Result<fields::QuantityType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::QUANTITY_TYPE)?;
        fields::QuantityType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::QUANTITY_TYPE,
        })
    }

    /// Whether `QuantityType` (tag 465) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quantity_type(&self) -> bool {
        self.msg.body().has_field(tags::QUANTITY_TYPE)
    }

    // --- IOIQty (tag 27, STRING, required) ---

    /// Set `IOIQty` (tag 27, required).
    pub fn set_ioi_qty(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::IOI_QTY, v.to_string());
    }

    /// `IOIQty` (tag 27, required).
    pub fn ioi_qty(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::IOI_QTY)
    }

    /// Whether `IOIQty` (tag 27) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_qty(&self) -> bool {
        self.msg.body().has_field(tags::IOI_QTY)
    }

    // --- PriceType (tag 423, INT, optional) ---

    /// Set `PriceType` (tag 423, optional).
    pub fn set_price_type(&mut self, v: fields::PriceType) {
        self.msg.body_mut().set_field(tags::PRICE_TYPE, v.to_fix().to_string());
    }

    /// `PriceType` (tag 423, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::PriceType`]'s.
    pub fn price_type(&self) -> Result<fields::PriceType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::PRICE_TYPE)?;
        fields::PriceType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PRICE_TYPE,
        })
    }

    /// Whether `PriceType` (tag 423) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_price_type(&self) -> bool {
        self.msg.body().has_field(tags::PRICE_TYPE)
    }

    // --- Price (tag 44, PRICE, optional) ---

    /// Set `Price` (tag 44, optional); written in FIX `PRICE` format.
    pub fn set_price(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::PRICE, convert::format_fix_decimal(v));
    }

    /// `Price` (tag 44, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn price(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::PRICE)?;
        convert::parse_fix_decimal(&raw, tags::PRICE)
    }

    /// Whether `Price` (tag 44) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_price(&self) -> bool {
        self.msg.body().has_field(tags::PRICE)
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

    // --- IOIQltyInd (tag 25, CHAR, optional) ---

    /// Set `IOIQltyInd` (tag 25, optional).
    pub fn set_ioi_qlty_ind(&mut self, v: fields::IoiQltyInd) {
        self.msg.body_mut().set_field(tags::IOI_QLTY_IND, v.to_fix().to_string());
    }

    /// `IOIQltyInd` (tag 25, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::IoiQltyInd`]'s.
    pub fn ioi_qlty_ind(&self) -> Result<fields::IoiQltyInd, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::IOI_QLTY_IND)?;
        fields::IoiQltyInd::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::IOI_QLTY_IND,
        })
    }

    /// Whether `IOIQltyInd` (tag 25) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_qlty_ind(&self) -> bool {
        self.msg.body().has_field(tags::IOI_QLTY_IND)
    }

    // --- IOINaturalFlag (tag 130, BOOLEAN, optional) ---

    /// Set `IOINaturalFlag` (tag 130, optional); written in FIX `BOOLEAN` format.
    pub fn set_ioi_natural_flag(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::IOI_NATURAL_FLAG, convert::format_fix_bool(v));
    }

    /// `IOINaturalFlag` (tag 130, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn ioi_natural_flag(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::IOI_NATURAL_FLAG)?;
        convert::parse_fix_bool(&raw, tags::IOI_NATURAL_FLAG)
    }

    /// Whether `IOINaturalFlag` (tag 130) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_natural_flag(&self) -> bool {
        self.msg.body().has_field(tags::IOI_NATURAL_FLAG)
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

    // --- URLLink (tag 149, STRING, optional) ---

    /// Set `URLLink` (tag 149, optional).
    pub fn set_url_link(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::URL_LINK, v.to_string());
    }

    /// `URLLink` (tag 149, optional).
    pub fn url_link(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::URL_LINK)
    }

    /// Whether `URLLink` (tag 149) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_url_link(&self) -> bool {
        self.msg.body().has_field(tags::URL_LINK)
    }

    // --- Spread (tag 218, PRICEOFFSET, optional) ---

    /// Set `Spread` (tag 218, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_spread(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::SPREAD, convert::format_fix_decimal(v));
    }

    /// `Spread` (tag 218, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn spread(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SPREAD)?;
        convert::parse_fix_decimal(&raw, tags::SPREAD)
    }

    /// Whether `Spread` (tag 218) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_spread(&self) -> bool {
        self.msg.body().has_field(tags::SPREAD)
    }

    // --- BenchmarkCurveCurrency (tag 220, CURRENCY, optional) ---

    /// Set `BenchmarkCurveCurrency` (tag 220, optional).
    pub fn set_benchmark_curve_currency(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BENCHMARK_CURVE_CURRENCY, v.to_string());
    }

    /// `BenchmarkCurveCurrency` (tag 220, optional).
    pub fn benchmark_curve_currency(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BENCHMARK_CURVE_CURRENCY)
    }

    /// Whether `BenchmarkCurveCurrency` (tag 220) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_benchmark_curve_currency(&self) -> bool {
        self.msg.body().has_field(tags::BENCHMARK_CURVE_CURRENCY)
    }

    // --- BenchmarkCurveName (tag 221, STRING, optional) ---

    /// Set `BenchmarkCurveName` (tag 221, optional).
    pub fn set_benchmark_curve_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BENCHMARK_CURVE_NAME, v.to_string());
    }

    /// `BenchmarkCurveName` (tag 221, optional).
    pub fn benchmark_curve_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BENCHMARK_CURVE_NAME)
    }

    /// Whether `BenchmarkCurveName` (tag 221) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_benchmark_curve_name(&self) -> bool {
        self.msg.body().has_field(tags::BENCHMARK_CURVE_NAME)
    }

    // --- BenchmarkCurvePoint (tag 222, STRING, optional) ---

    /// Set `BenchmarkCurvePoint` (tag 222, optional).
    pub fn set_benchmark_curve_point(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BENCHMARK_CURVE_POINT, v.to_string());
    }

    /// `BenchmarkCurvePoint` (tag 222, optional).
    pub fn benchmark_curve_point(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BENCHMARK_CURVE_POINT)
    }

    /// Whether `BenchmarkCurvePoint` (tag 222) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_benchmark_curve_point(&self) -> bool {
        self.msg.body().has_field(tags::BENCHMARK_CURVE_POINT)
    }

    // --- Benchmark (tag 219, CHAR, optional) ---

    /// Set `Benchmark` (tag 219, optional).
    pub fn set_benchmark(&mut self, v: fields::Benchmark) {
        self.msg.body_mut().set_field(tags::BENCHMARK, v.to_fix().to_string());
    }

    /// `Benchmark` (tag 219, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Benchmark`]'s.
    pub fn benchmark(&self) -> Result<fields::Benchmark, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::BENCHMARK)?;
        fields::Benchmark::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::BENCHMARK,
        })
    }

    /// Whether `Benchmark` (tag 219) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_benchmark(&self) -> bool {
        self.msg.body().has_field(tags::BENCHMARK)
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

    // --- NoIoiQualifiers group (count tag NO_IOI_QUALIFIERS) ---

    /// The `NoIoiQualifiers` group (count tag `NO_IOI_QUALIFIERS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_ioi_qualifiers(&self) -> Option<NoIoiQualifiers<'_>> {
        self.msg.body().get_group(tags::NO_IOI_QUALIFIERS).map(|group| NoIoiQualifiers { group })
    }

    /// The `NoIoiQualifiers` group (count tag `NO_IOI_QUALIFIERS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_ioi_qualifiers_mut(&mut self) -> Option<NoIoiQualifiersMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_IOI_QUALIFIERS)
            .map(|group| NoIoiQualifiersMut { group })
    }

    /// Create the `NoIoiQualifiers` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_ioi_qualifiers_entry`] on the same group.
    pub fn set_no_ioi_qualifiers(&mut self, count: u32) -> NoIoiQualifiersMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_IOI_QUALIFIERS,
            count,
            NO_IOI_QUALIFIERS_FIELD_ORDER,
        );
        NoIoiQualifiersMut { group }
    }

    /// Append one `NoIoiQualifiers` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_ioi_qualifiers`] on the same group.
    pub fn add_no_ioi_qualifiers_entry(
        &mut self,
        fill: impl FnOnce(&mut NoIoiQualifiersEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_IOI_QUALIFIERS, NO_IOI_QUALIFIERS_FIELD_ORDER);
        fill(&mut NoIoiQualifiersEntryMut { inner });
    }

    // --- NoRoutingIds group (count tag NO_ROUTING_IDS) ---

    /// The `NoRoutingIds` group (count tag `NO_ROUTING_IDS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_routing_ids(&self) -> Option<NoRoutingIds<'_>> {
        self.msg.body().get_group(tags::NO_ROUTING_IDS).map(|group| NoRoutingIds { group })
    }

    /// The `NoRoutingIds` group (count tag `NO_ROUTING_IDS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_routing_ids_mut(&mut self) -> Option<NoRoutingIdsMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_ROUTING_IDS)
            .map(|group| NoRoutingIdsMut { group })
    }

    /// Create the `NoRoutingIds` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_routing_ids_entry`] on the same group.
    pub fn set_no_routing_ids(&mut self, count: u32) -> NoRoutingIdsMut<'_> {
        let group =
            self.msg.body_mut().set_group(tags::NO_ROUTING_IDS, count, NO_ROUTING_IDS_FIELD_ORDER);
        NoRoutingIdsMut { group }
    }

    /// Append one `NoRoutingIds` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_routing_ids`] on the same group.
    pub fn add_no_routing_ids_entry(&mut self, fill: impl FnOnce(&mut NoRoutingIdsEntryMut<'_>)) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_ROUTING_IDS, NO_ROUTING_IDS_FIELD_ORDER);
        fill(&mut NoRoutingIdsEntryMut { inner });
    }

    /// The underlying raw message (e.g. for fields this facade has no accessor for).
    pub fn message(&self) -> &Message {
        &self.msg
    }

    /// Mutable access to the underlying raw message (e.g. to set a custom field, D1a).
    pub fn message_mut(&mut self) -> &mut Message {
        &mut self.msg
    }

    /// Adopt `msg` without checking its MsgType: for the version's message enum, whose
    /// `From<Message>` has already matched 35 before wrapping. `pub(super)` keeps it inside
    /// `generated/`; everything else goes through the checked `TryFrom<Message>`.
    pub(super) fn wrap(msg: Message) -> Self {
        Self { msg }
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

/// Wire order of a `NoIoiQualifiers` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_IOI_QUALIFIERS_FIELD_ORDER: &[u32] = &[tags::IOI_QUALIFIER];

/// One `NoIoiQualifiers` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoIoiQualifiersEntry<'a> {
    inner: &'a FieldMap,
}

impl NoIoiQualifiersEntry<'_> {
    /// `IOIQualifier` (tag 104, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::IoiQualifier`]'s.
    pub fn ioi_qualifier(&self) -> Result<fields::IoiQualifier, FieldError> {
        let n = self.inner.get_field::<char>(tags::IOI_QUALIFIER)?;
        fields::IoiQualifier::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::IOI_QUALIFIER,
        })
    }

    /// Whether `IOIQualifier` (tag 104) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_qualifier(&self) -> bool {
        self.inner.has_field(tags::IOI_QUALIFIER)
    }
}

/// One `NoIoiQualifiers` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoIoiQualifiersEntryMut::as_ref`].
pub struct NoIoiQualifiersEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoIoiQualifiersEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoIoiQualifiersEntry<'_> {
        NoIoiQualifiersEntry { inner: self.inner }
    }

    /// Set `IOIQualifier` (tag 104, optional).
    pub fn set_ioi_qualifier(&mut self, v: fields::IoiQualifier) {
        self.inner.set_field(tags::IOI_QUALIFIER, v.to_fix().to_string());
    }
}

/// The `NoIoiQualifiers` repeating group (count tag `NO_IOI_QUALIFIERS`), read-only: its
/// instances as [`NoIoiQualifiersEntry`] views.
pub struct NoIoiQualifiers<'a> {
    group: &'a Group,
}

impl<'a> NoIoiQualifiers<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoIoiQualifiersEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoIoiQualifiersEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoIoiQualifiersEntry<'a> {
        self.get(i).expect("NoIoiQualifiers index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoIoiQualifiersEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoIoiQualifiersEntry { inner })
    }
}

/// The `NoIoiQualifiers` repeating group (count tag `NO_IOI_QUALIFIERS`), mutable: its
/// instances as [`NoIoiQualifiersEntryMut`] views.
pub struct NoIoiQualifiersMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoIoiQualifiersMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoIoiQualifiersEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoIoiQualifiersEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoIoiQualifiersEntryMut<'_> {
        self.get_mut(i).expect("NoIoiQualifiers index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoIoiQualifiersEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoIoiQualifiersEntryMut { inner })
    }
}

/// Wire order of a `NoRoutingIds` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_ROUTING_IDS_FIELD_ORDER: &[u32] = &[tags::ROUTING_TYPE, tags::ROUTING_ID];

/// One `NoRoutingIds` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoRoutingIdsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoRoutingIdsEntry<'_> {
    /// `RoutingType` (tag 216, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::RoutingType`]'s.
    pub fn routing_type(&self) -> Result<fields::RoutingType, FieldError> {
        let n = self.inner.get_field::<i32>(tags::ROUTING_TYPE)?;
        fields::RoutingType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ROUTING_TYPE,
        })
    }

    /// Whether `RoutingType` (tag 216) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_routing_type(&self) -> bool {
        self.inner.has_field(tags::ROUTING_TYPE)
    }

    /// `RoutingID` (tag 217, optional).
    pub fn routing_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ROUTING_ID)
    }

    /// Whether `RoutingID` (tag 217) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_routing_id(&self) -> bool {
        self.inner.has_field(tags::ROUTING_ID)
    }
}

/// One `NoRoutingIds` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoRoutingIdsEntryMut::as_ref`].
pub struct NoRoutingIdsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoRoutingIdsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoRoutingIdsEntry<'_> {
        NoRoutingIdsEntry { inner: self.inner }
    }

    /// Set `RoutingType` (tag 216, optional).
    pub fn set_routing_type(&mut self, v: fields::RoutingType) {
        self.inner.set_field(tags::ROUTING_TYPE, v.to_fix().to_string());
    }

    /// Set `RoutingID` (tag 217, optional).
    pub fn set_routing_id(&mut self, v: &str) {
        self.inner.set_field(tags::ROUTING_ID, v.to_string());
    }
}

/// The `NoRoutingIds` repeating group (count tag `NO_ROUTING_IDS`), read-only: its
/// instances as [`NoRoutingIdsEntry`] views.
pub struct NoRoutingIds<'a> {
    group: &'a Group,
}

impl<'a> NoRoutingIds<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoRoutingIdsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoRoutingIdsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoRoutingIdsEntry<'a> {
        self.get(i).expect("NoRoutingIds index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoRoutingIdsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoRoutingIdsEntry { inner })
    }
}

/// The `NoRoutingIds` repeating group (count tag `NO_ROUTING_IDS`), mutable: its
/// instances as [`NoRoutingIdsEntryMut`] views.
pub struct NoRoutingIdsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoRoutingIdsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoRoutingIdsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoRoutingIdsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoRoutingIdsEntryMut<'_> {
        self.get_mut(i).expect("NoRoutingIds index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoRoutingIdsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoRoutingIdsEntryMut { inner })
    }
}
