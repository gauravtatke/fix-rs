//! `SecurityDefinition` (app message, MsgType `35=d`): typed facade over the raw
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

/// Typed facade over a `SecurityDefinition` [`Message`] (35=d). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct SecurityDefinition {
    msg: Message,
}

impl Default for SecurityDefinition {
    fn default() -> Self {
        SecurityDefinition::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `SecurityDefinition`, checking the msg type.
impl TryFrom<Message> for SecurityDefinition {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a SecurityDefinition" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<SecurityDefinition> for Message {
    fn from(typed_msg: SecurityDefinition) -> Self {
        typed_msg.msg
    }
}

impl SecurityDefinition {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "d";

    /// An empty `SecurityDefinition` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- SecurityReqID (tag 320, STRING, required) ---

    /// Set `SecurityReqID` (tag 320, required).
    pub fn set_security_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_REQ_ID, v.to_string());
    }

    /// `SecurityReqID` (tag 320, required).
    pub fn security_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_REQ_ID)
    }

    /// Whether `SecurityReqID` (tag 320) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_req_id(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_REQ_ID)
    }

    // --- SecurityResponseID (tag 322, STRING, required) ---

    /// Set `SecurityResponseID` (tag 322, required).
    pub fn set_security_response_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_RESPONSE_ID, v.to_string());
    }

    /// `SecurityResponseID` (tag 322, required).
    pub fn security_response_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_RESPONSE_ID)
    }

    /// Whether `SecurityResponseID` (tag 322) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_response_id(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_RESPONSE_ID)
    }

    // --- SecurityResponseType (tag 323, INT, required) ---

    /// Set `SecurityResponseType` (tag 323, required).
    pub fn set_security_response_type(&mut self, v: fields::SecurityResponseType) {
        self.msg.body_mut().set_field(tags::SECURITY_RESPONSE_TYPE, v.to_fix().to_string());
    }

    /// `SecurityResponseType` (tag 323, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityResponseType`]'s.
    pub fn security_response_type(&self) -> Result<fields::SecurityResponseType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::SECURITY_RESPONSE_TYPE)?;
        fields::SecurityResponseType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_RESPONSE_TYPE,
        })
    }

    /// Whether `SecurityResponseType` (tag 323) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_response_type(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_RESPONSE_TYPE)
    }

    // --- Symbol (tag 55, STRING, optional) ---

    /// Set `Symbol` (tag 55, optional).
    pub fn set_symbol(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SYMBOL, v.to_string());
    }

    /// `Symbol` (tag 55, optional).
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

    // --- RoundLot (tag 561, QTY, optional) ---

    /// Set `RoundLot` (tag 561, optional); written in FIX `QTY` format.
    pub fn set_round_lot(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::ROUND_LOT, convert::format_fix_decimal(v));
    }

    /// `RoundLot` (tag 561, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn round_lot(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::ROUND_LOT)?;
        convert::parse_fix_decimal(&raw, tags::ROUND_LOT)
    }

    /// Whether `RoundLot` (tag 561) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_round_lot(&self) -> bool {
        self.msg.body().has_field(tags::ROUND_LOT)
    }

    // --- MinTradeVol (tag 562, QTY, optional) ---

    /// Set `MinTradeVol` (tag 562, optional); written in FIX `QTY` format.
    pub fn set_min_trade_vol(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MIN_TRADE_VOL, convert::format_fix_decimal(v));
    }

    /// `MinTradeVol` (tag 562, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn min_trade_vol(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MIN_TRADE_VOL)?;
        convert::parse_fix_decimal(&raw, tags::MIN_TRADE_VOL)
    }

    /// Whether `MinTradeVol` (tag 562) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_min_trade_vol(&self) -> bool {
        self.msg.body().has_field(tags::MIN_TRADE_VOL)
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

    // --- NoLegs group (count tag NO_LEGS) ---

    /// The `NoLegs` group (count tag `NO_LEGS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_legs(&self) -> Option<NoLegs<'_>> {
        self.msg.body().get_group(tags::NO_LEGS).map(|group| NoLegs { group })
    }

    /// The `NoLegs` group (count tag `NO_LEGS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_legs_mut(&mut self) -> Option<NoLegsMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_LEGS).map(|group| NoLegsMut { group })
    }

    /// Create the `NoLegs` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_legs_entry`] on the same group.
    pub fn set_no_legs(&mut self, count: u32) -> NoLegsMut<'_> {
        let group = self.msg.body_mut().set_group(tags::NO_LEGS, count, NO_LEGS_FIELD_ORDER);
        NoLegsMut { group }
    }

    /// Append one `NoLegs` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_legs`] on the same group.
    pub fn add_no_legs_entry(&mut self, fill: impl FnOnce(&mut NoLegsEntryMut<'_>)) {
        let inner = self.msg.body_mut().add_group_instance(tags::NO_LEGS, NO_LEGS_FIELD_ORDER);
        fill(&mut NoLegsEntryMut { inner });
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

/// Wire order of a `NoLegs` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_LEGS_FIELD_ORDER: &[u32] = &[
    tags::LEG_SYMBOL,
    tags::LEG_SYMBOL_SFX,
    tags::LEG_SECURITY_ID,
    tags::LEG_SECURITY_ID_SOURCE,
    tags::NO_LEG_SECURITY_ALT_ID,
    tags::LEG_PRODUCT,
    tags::LEG_CFI_CODE,
    tags::LEG_SECURITY_TYPE,
    tags::LEG_MATURITY_MONTH_YEAR,
    tags::LEG_MATURITY_DATE,
    tags::LEG_COUPON_PAYMENT_DATE,
    tags::LEG_ISSUE_DATE,
    tags::LEG_REPO_COLLATERAL_SECURITY_TYPE,
    tags::LEG_REPURCHASE_TERM,
    tags::LEG_REPURCHASE_RATE,
    tags::LEG_FACTOR,
    tags::LEG_CREDIT_RATING,
    tags::LEG_INSTR_REGISTRY,
    tags::LEG_COUNTRY_OF_ISSUE,
    tags::LEG_STATE_OR_PROVINCE_OF_ISSUE,
    tags::LEG_LOCALE_OF_ISSUE,
    tags::LEG_REDEMPTION_DATE,
    tags::LEG_STRIKE_PRICE,
    tags::LEG_OPT_ATTRIBUTE,
    tags::LEG_CONTRACT_MULTIPLIER,
    tags::LEG_COUPON_RATE,
    tags::LEG_SECURITY_EXCHANGE,
    tags::LEG_ISSUER,
    tags::ENCODED_LEG_ISSUER_LEN,
    tags::ENCODED_LEG_ISSUER,
    tags::LEG_SECURITY_DESC,
    tags::ENCODED_LEG_SECURITY_DESC_LEN,
    tags::ENCODED_LEG_SECURITY_DESC,
    tags::LEG_RATIO_QTY,
    tags::LEG_SIDE,
    tags::LEG_CURRENCY,
];

/// One `NoLegs` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoLegsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoLegsEntry<'_> {
    /// `LegSymbol` (tag 600, optional).
    pub fn leg_symbol(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SYMBOL)
    }

    /// Whether `LegSymbol` (tag 600) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_symbol(&self) -> bool {
        self.inner.has_field(tags::LEG_SYMBOL)
    }

    /// `LegSymbolSfx` (tag 601, optional).
    pub fn leg_symbol_sfx(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SYMBOL_SFX)
    }

    /// Whether `LegSymbolSfx` (tag 601) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_symbol_sfx(&self) -> bool {
        self.inner.has_field(tags::LEG_SYMBOL_SFX)
    }

    /// `LegSecurityID` (tag 602, optional).
    pub fn leg_security_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_ID)
    }

    /// Whether `LegSecurityID` (tag 602) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_id(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_ID)
    }

    /// `LegSecurityIDSource` (tag 603, optional).
    pub fn leg_security_id_source(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_ID_SOURCE)
    }

    /// Whether `LegSecurityIDSource` (tag 603) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_id_source(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_ID_SOURCE)
    }

    /// `LegProduct` (tag 607, optional).
    pub fn leg_product(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::LEG_PRODUCT)
    }

    /// Whether `LegProduct` (tag 607) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_product(&self) -> bool {
        self.inner.has_field(tags::LEG_PRODUCT)
    }

    /// `LegCFICode` (tag 608, optional).
    pub fn leg_cfi_code(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_CFI_CODE)
    }

    /// Whether `LegCFICode` (tag 608) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_cfi_code(&self) -> bool {
        self.inner.has_field(tags::LEG_CFI_CODE)
    }

    /// `LegSecurityType` (tag 609, optional).
    pub fn leg_security_type(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_TYPE)
    }

    /// Whether `LegSecurityType` (tag 609) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_type(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_TYPE)
    }

    /// `LegMaturityMonthYear` (tag 610, optional).
    pub fn leg_maturity_month_year(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_MATURITY_MONTH_YEAR)
    }

    /// Whether `LegMaturityMonthYear` (tag 610) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_maturity_month_year(&self) -> bool {
        self.inner.has_field(tags::LEG_MATURITY_MONTH_YEAR)
    }

    /// `LegMaturityDate` (tag 611, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn leg_maturity_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_MATURITY_DATE)?;
        convert::parse_fix_date(&raw, tags::LEG_MATURITY_DATE)
    }

    /// Whether `LegMaturityDate` (tag 611) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_maturity_date(&self) -> bool {
        self.inner.has_field(tags::LEG_MATURITY_DATE)
    }

    /// `LegCouponPaymentDate` (tag 248, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn leg_coupon_payment_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_COUPON_PAYMENT_DATE)?;
        convert::parse_fix_date(&raw, tags::LEG_COUPON_PAYMENT_DATE)
    }

    /// Whether `LegCouponPaymentDate` (tag 248) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_coupon_payment_date(&self) -> bool {
        self.inner.has_field(tags::LEG_COUPON_PAYMENT_DATE)
    }

    /// `LegIssueDate` (tag 249, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn leg_issue_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_ISSUE_DATE)?;
        convert::parse_fix_date(&raw, tags::LEG_ISSUE_DATE)
    }

    /// Whether `LegIssueDate` (tag 249) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_issue_date(&self) -> bool {
        self.inner.has_field(tags::LEG_ISSUE_DATE)
    }

    /// `LegRepoCollateralSecurityType` (tag 250, optional).
    pub fn leg_repo_collateral_security_type(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// Whether `LegRepoCollateralSecurityType` (tag 250) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_repo_collateral_security_type(&self) -> bool {
        self.inner.has_field(tags::LEG_REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// `LegRepurchaseTerm` (tag 251, optional).
    pub fn leg_repurchase_term(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::LEG_REPURCHASE_TERM)
    }

    /// Whether `LegRepurchaseTerm` (tag 251) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_repurchase_term(&self) -> bool {
        self.inner.has_field(tags::LEG_REPURCHASE_TERM)
    }

    /// `LegRepurchaseRate` (tag 252, optional).
    pub fn leg_repurchase_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LEG_REPURCHASE_RATE)
    }

    /// Whether `LegRepurchaseRate` (tag 252) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_repurchase_rate(&self) -> bool {
        self.inner.has_field(tags::LEG_REPURCHASE_RATE)
    }

    /// `LegFactor` (tag 253, optional).
    pub fn leg_factor(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LEG_FACTOR)
    }

    /// Whether `LegFactor` (tag 253) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_factor(&self) -> bool {
        self.inner.has_field(tags::LEG_FACTOR)
    }

    /// `LegCreditRating` (tag 257, optional).
    pub fn leg_credit_rating(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_CREDIT_RATING)
    }

    /// Whether `LegCreditRating` (tag 257) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_credit_rating(&self) -> bool {
        self.inner.has_field(tags::LEG_CREDIT_RATING)
    }

    /// `LegInstrRegistry` (tag 599, optional).
    pub fn leg_instr_registry(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_INSTR_REGISTRY)
    }

    /// Whether `LegInstrRegistry` (tag 599) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_instr_registry(&self) -> bool {
        self.inner.has_field(tags::LEG_INSTR_REGISTRY)
    }

    /// `LegCountryOfIssue` (tag 596, optional).
    pub fn leg_country_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_COUNTRY_OF_ISSUE)
    }

    /// Whether `LegCountryOfIssue` (tag 596) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_country_of_issue(&self) -> bool {
        self.inner.has_field(tags::LEG_COUNTRY_OF_ISSUE)
    }

    /// `LegStateOrProvinceOfIssue` (tag 597, optional).
    pub fn leg_state_or_province_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// Whether `LegStateOrProvinceOfIssue` (tag 597) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_state_or_province_of_issue(&self) -> bool {
        self.inner.has_field(tags::LEG_STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// `LegLocaleOfIssue` (tag 598, optional).
    pub fn leg_locale_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_LOCALE_OF_ISSUE)
    }

    /// Whether `LegLocaleOfIssue` (tag 598) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_locale_of_issue(&self) -> bool {
        self.inner.has_field(tags::LEG_LOCALE_OF_ISSUE)
    }

    /// `LegRedemptionDate` (tag 254, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn leg_redemption_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_REDEMPTION_DATE)?;
        convert::parse_fix_date(&raw, tags::LEG_REDEMPTION_DATE)
    }

    /// Whether `LegRedemptionDate` (tag 254) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_redemption_date(&self) -> bool {
        self.inner.has_field(tags::LEG_REDEMPTION_DATE)
    }

    /// `LegStrikePrice` (tag 612, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn leg_strike_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_STRIKE_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::LEG_STRIKE_PRICE)
    }

    /// Whether `LegStrikePrice` (tag 612) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_strike_price(&self) -> bool {
        self.inner.has_field(tags::LEG_STRIKE_PRICE)
    }

    /// `LegOptAttribute` (tag 613, optional).
    pub fn leg_opt_attribute(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::LEG_OPT_ATTRIBUTE)
    }

    /// Whether `LegOptAttribute` (tag 613) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_opt_attribute(&self) -> bool {
        self.inner.has_field(tags::LEG_OPT_ATTRIBUTE)
    }

    /// `LegContractMultiplier` (tag 614, optional).
    pub fn leg_contract_multiplier(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LEG_CONTRACT_MULTIPLIER)
    }

    /// Whether `LegContractMultiplier` (tag 614) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_contract_multiplier(&self) -> bool {
        self.inner.has_field(tags::LEG_CONTRACT_MULTIPLIER)
    }

    /// `LegCouponRate` (tag 615, optional).
    pub fn leg_coupon_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LEG_COUPON_RATE)
    }

    /// Whether `LegCouponRate` (tag 615) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_coupon_rate(&self) -> bool {
        self.inner.has_field(tags::LEG_COUPON_RATE)
    }

    /// `LegSecurityExchange` (tag 616, optional).
    pub fn leg_security_exchange(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_EXCHANGE)
    }

    /// Whether `LegSecurityExchange` (tag 616) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_exchange(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_EXCHANGE)
    }

    /// `LegIssuer` (tag 617, optional).
    pub fn leg_issuer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_ISSUER)
    }

    /// Whether `LegIssuer` (tag 617) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_issuer(&self) -> bool {
        self.inner.has_field(tags::LEG_ISSUER)
    }

    /// `EncodedLegIssuerLen` (tag 618, optional).
    pub fn encoded_leg_issuer_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_LEG_ISSUER_LEN)
    }

    /// Whether `EncodedLegIssuerLen` (tag 618) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_leg_issuer_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_LEG_ISSUER_LEN)
    }

    /// `EncodedLegIssuer` (tag 619, optional).
    pub fn encoded_leg_issuer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_LEG_ISSUER)
    }

    /// Whether `EncodedLegIssuer` (tag 619) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_leg_issuer(&self) -> bool {
        self.inner.has_field(tags::ENCODED_LEG_ISSUER)
    }

    /// `LegSecurityDesc` (tag 620, optional).
    pub fn leg_security_desc(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_DESC)
    }

    /// Whether `LegSecurityDesc` (tag 620) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_desc(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_DESC)
    }

    /// `EncodedLegSecurityDescLen` (tag 621, optional).
    pub fn encoded_leg_security_desc_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_LEG_SECURITY_DESC_LEN)
    }

    /// Whether `EncodedLegSecurityDescLen` (tag 621) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_leg_security_desc_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_LEG_SECURITY_DESC_LEN)
    }

    /// `EncodedLegSecurityDesc` (tag 622, optional).
    pub fn encoded_leg_security_desc(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_LEG_SECURITY_DESC)
    }

    /// Whether `EncodedLegSecurityDesc` (tag 622) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_leg_security_desc(&self) -> bool {
        self.inner.has_field(tags::ENCODED_LEG_SECURITY_DESC)
    }

    /// `LegRatioQty` (tag 623, optional).
    pub fn leg_ratio_qty(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LEG_RATIO_QTY)
    }

    /// Whether `LegRatioQty` (tag 623) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_ratio_qty(&self) -> bool {
        self.inner.has_field(tags::LEG_RATIO_QTY)
    }

    /// `LegSide` (tag 624, optional).
    pub fn leg_side(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::LEG_SIDE)
    }

    /// Whether `LegSide` (tag 624) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_side(&self) -> bool {
        self.inner.has_field(tags::LEG_SIDE)
    }

    /// `LegCurrency` (tag 556, optional).
    pub fn leg_currency(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_CURRENCY)
    }

    /// Whether `LegCurrency` (tag 556) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_currency(&self) -> bool {
        self.inner.has_field(tags::LEG_CURRENCY)
    }

    /// The `NoLegSecurityAltId` group (count tag `NO_LEG_SECURITY_ALT_ID`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_leg_security_alt_id(&self) -> Option<NoLegSecurityAltId<'_>> {
        self.inner.get_group(tags::NO_LEG_SECURITY_ALT_ID).map(|group| NoLegSecurityAltId { group })
    }
}

/// One `NoLegs` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoLegsEntryMut::as_ref`].
pub struct NoLegsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoLegsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoLegsEntry<'_> {
        NoLegsEntry { inner: self.inner }
    }

    /// Set `LegSymbol` (tag 600, optional).
    pub fn set_leg_symbol(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SYMBOL, v.to_string());
    }

    /// Set `LegSymbolSfx` (tag 601, optional).
    pub fn set_leg_symbol_sfx(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SYMBOL_SFX, v.to_string());
    }

    /// Set `LegSecurityID` (tag 602, optional).
    pub fn set_leg_security_id(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_ID, v.to_string());
    }

    /// Set `LegSecurityIDSource` (tag 603, optional).
    pub fn set_leg_security_id_source(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_ID_SOURCE, v.to_string());
    }

    /// Set `LegProduct` (tag 607, optional).
    pub fn set_leg_product(&mut self, v: i32) {
        self.inner.set_field(tags::LEG_PRODUCT, v.to_string());
    }

    /// Set `LegCFICode` (tag 608, optional).
    pub fn set_leg_cfi_code(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_CFI_CODE, v.to_string());
    }

    /// Set `LegSecurityType` (tag 609, optional).
    pub fn set_leg_security_type(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_TYPE, v.to_string());
    }

    /// Set `LegMaturityMonthYear` (tag 610, optional).
    pub fn set_leg_maturity_month_year(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_MATURITY_MONTH_YEAR, v.to_string());
    }

    /// Set `LegMaturityDate` (tag 611, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_leg_maturity_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::LEG_MATURITY_DATE, convert::format_fix_date(v));
    }

    /// Set `LegCouponPaymentDate` (tag 248, optional); written in FIX `UTCDATE` format.
    pub fn set_leg_coupon_payment_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::LEG_COUPON_PAYMENT_DATE, convert::format_fix_date(v));
    }

    /// Set `LegIssueDate` (tag 249, optional); written in FIX `UTCDATE` format.
    pub fn set_leg_issue_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::LEG_ISSUE_DATE, convert::format_fix_date(v));
    }

    /// Set `LegRepoCollateralSecurityType` (tag 250, optional).
    pub fn set_leg_repo_collateral_security_type(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_REPO_COLLATERAL_SECURITY_TYPE, v.to_string());
    }

    /// Set `LegRepurchaseTerm` (tag 251, optional).
    pub fn set_leg_repurchase_term(&mut self, v: i32) {
        self.inner.set_field(tags::LEG_REPURCHASE_TERM, v.to_string());
    }

    /// Set `LegRepurchaseRate` (tag 252, optional).
    pub fn set_leg_repurchase_rate(&mut self, v: f64) {
        self.inner.set_field(tags::LEG_REPURCHASE_RATE, v.to_string());
    }

    /// Set `LegFactor` (tag 253, optional).
    pub fn set_leg_factor(&mut self, v: f64) {
        self.inner.set_field(tags::LEG_FACTOR, v.to_string());
    }

    /// Set `LegCreditRating` (tag 257, optional).
    pub fn set_leg_credit_rating(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_CREDIT_RATING, v.to_string());
    }

    /// Set `LegInstrRegistry` (tag 599, optional).
    pub fn set_leg_instr_registry(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_INSTR_REGISTRY, v.to_string());
    }

    /// Set `LegCountryOfIssue` (tag 596, optional).
    pub fn set_leg_country_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_COUNTRY_OF_ISSUE, v.to_string());
    }

    /// Set `LegStateOrProvinceOfIssue` (tag 597, optional).
    pub fn set_leg_state_or_province_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_STATE_OR_PROVINCE_OF_ISSUE, v.to_string());
    }

    /// Set `LegLocaleOfIssue` (tag 598, optional).
    pub fn set_leg_locale_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_LOCALE_OF_ISSUE, v.to_string());
    }

    /// Set `LegRedemptionDate` (tag 254, optional); written in FIX `UTCDATE` format.
    pub fn set_leg_redemption_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::LEG_REDEMPTION_DATE, convert::format_fix_date(v));
    }

    /// Set `LegStrikePrice` (tag 612, optional); written in FIX `PRICE` format.
    pub fn set_leg_strike_price(&mut self, v: Decimal) {
        self.inner.set_field(tags::LEG_STRIKE_PRICE, convert::format_fix_decimal(v));
    }

    /// Set `LegOptAttribute` (tag 613, optional).
    pub fn set_leg_opt_attribute(&mut self, v: char) {
        self.inner.set_field(tags::LEG_OPT_ATTRIBUTE, v.to_string());
    }

    /// Set `LegContractMultiplier` (tag 614, optional).
    pub fn set_leg_contract_multiplier(&mut self, v: f64) {
        self.inner.set_field(tags::LEG_CONTRACT_MULTIPLIER, v.to_string());
    }

    /// Set `LegCouponRate` (tag 615, optional).
    pub fn set_leg_coupon_rate(&mut self, v: f64) {
        self.inner.set_field(tags::LEG_COUPON_RATE, v.to_string());
    }

    /// Set `LegSecurityExchange` (tag 616, optional).
    pub fn set_leg_security_exchange(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_EXCHANGE, v.to_string());
    }

    /// Set `LegIssuer` (tag 617, optional).
    pub fn set_leg_issuer(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_ISSUER, v.to_string());
    }

    /// Set `EncodedLegIssuerLen` (tag 618, optional).
    pub fn set_encoded_leg_issuer_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_LEG_ISSUER_LEN, v.to_string());
    }

    /// Set `EncodedLegIssuer` (tag 619, optional).
    pub fn set_encoded_leg_issuer(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_LEG_ISSUER, v.to_string());
    }

    /// Set `LegSecurityDesc` (tag 620, optional).
    pub fn set_leg_security_desc(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_DESC, v.to_string());
    }

    /// Set `EncodedLegSecurityDescLen` (tag 621, optional).
    pub fn set_encoded_leg_security_desc_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_LEG_SECURITY_DESC_LEN, v.to_string());
    }

    /// Set `EncodedLegSecurityDesc` (tag 622, optional).
    pub fn set_encoded_leg_security_desc(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_LEG_SECURITY_DESC, v.to_string());
    }

    /// Set `LegRatioQty` (tag 623, optional).
    pub fn set_leg_ratio_qty(&mut self, v: f64) {
        self.inner.set_field(tags::LEG_RATIO_QTY, v.to_string());
    }

    /// Set `LegSide` (tag 624, optional).
    pub fn set_leg_side(&mut self, v: char) {
        self.inner.set_field(tags::LEG_SIDE, v.to_string());
    }

    /// Set `LegCurrency` (tag 556, optional).
    pub fn set_leg_currency(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_CURRENCY, v.to_string());
    }

    /// The `NoLegSecurityAltId` group (count tag `NO_LEG_SECURITY_ALT_ID`, optional) for editing, or `None` if
    /// absent.
    pub fn no_leg_security_alt_id_mut(&mut self) -> Option<NoLegSecurityAltIdMut<'_>> {
        self.inner
            .get_group_mut(tags::NO_LEG_SECURITY_ALT_ID)
            .map(|group| NoLegSecurityAltIdMut { group })
    }

    /// Create the `NoLegSecurityAltId` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_leg_security_alt_id_entry`] on the same group.
    pub fn set_no_leg_security_alt_id(&mut self, count: u32) -> NoLegSecurityAltIdMut<'_> {
        let group = self.inner.set_group(
            tags::NO_LEG_SECURITY_ALT_ID,
            count,
            NO_LEG_SECURITY_ALT_ID_FIELD_ORDER,
        );
        NoLegSecurityAltIdMut { group }
    }

    /// Append one `NoLegSecurityAltId` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_leg_security_alt_id`] on the same group.
    pub fn add_no_leg_security_alt_id_entry(
        &mut self,
        fill: impl FnOnce(&mut NoLegSecurityAltIdEntryMut<'_>),
    ) {
        let inner = self
            .inner
            .add_group_instance(tags::NO_LEG_SECURITY_ALT_ID, NO_LEG_SECURITY_ALT_ID_FIELD_ORDER);
        fill(&mut NoLegSecurityAltIdEntryMut { inner });
    }
}

/// The `NoLegs` repeating group (count tag `NO_LEGS`), read-only: its
/// instances as [`NoLegsEntry`] views.
pub struct NoLegs<'a> {
    group: &'a Group,
}

impl<'a> NoLegs<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoLegsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoLegsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoLegsEntry<'a> {
        self.get(i).expect("NoLegs index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoLegsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoLegsEntry { inner })
    }
}

/// The `NoLegs` repeating group (count tag `NO_LEGS`), mutable: its
/// instances as [`NoLegsEntryMut`] views.
pub struct NoLegsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoLegsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoLegsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoLegsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoLegsEntryMut<'_> {
        self.get_mut(i).expect("NoLegs index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoLegsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoLegsEntryMut { inner })
    }
}

/// Wire order of a `NoLegSecurityAltId` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_LEG_SECURITY_ALT_ID_FIELD_ORDER: &[u32] =
    &[tags::LEG_SECURITY_ALT_ID, tags::LEG_SECURITY_ALT_ID_SOURCE];

/// One `NoLegSecurityAltId` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoLegSecurityAltIdEntry<'a> {
    inner: &'a FieldMap,
}

impl NoLegSecurityAltIdEntry<'_> {
    /// `LegSecurityAltID` (tag 605, optional).
    pub fn leg_security_alt_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_ALT_ID)
    }

    /// Whether `LegSecurityAltID` (tag 605) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_alt_id(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_ALT_ID)
    }

    /// `LegSecurityAltIDSource` (tag 606, optional).
    pub fn leg_security_alt_id_source(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_SECURITY_ALT_ID_SOURCE)
    }

    /// Whether `LegSecurityAltIDSource` (tag 606) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_security_alt_id_source(&self) -> bool {
        self.inner.has_field(tags::LEG_SECURITY_ALT_ID_SOURCE)
    }
}

/// One `NoLegSecurityAltId` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoLegSecurityAltIdEntryMut::as_ref`].
pub struct NoLegSecurityAltIdEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoLegSecurityAltIdEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoLegSecurityAltIdEntry<'_> {
        NoLegSecurityAltIdEntry { inner: self.inner }
    }

    /// Set `LegSecurityAltID` (tag 605, optional).
    pub fn set_leg_security_alt_id(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_ALT_ID, v.to_string());
    }

    /// Set `LegSecurityAltIDSource` (tag 606, optional).
    pub fn set_leg_security_alt_id_source(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_SECURITY_ALT_ID_SOURCE, v.to_string());
    }
}

/// The `NoLegSecurityAltId` repeating group (count tag `NO_LEG_SECURITY_ALT_ID`), read-only: its
/// instances as [`NoLegSecurityAltIdEntry`] views.
pub struct NoLegSecurityAltId<'a> {
    group: &'a Group,
}

impl<'a> NoLegSecurityAltId<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoLegSecurityAltIdEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoLegSecurityAltIdEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoLegSecurityAltIdEntry<'a> {
        self.get(i).expect("NoLegSecurityAltId index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoLegSecurityAltIdEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoLegSecurityAltIdEntry { inner })
    }
}

/// The `NoLegSecurityAltId` repeating group (count tag `NO_LEG_SECURITY_ALT_ID`), mutable: its
/// instances as [`NoLegSecurityAltIdEntryMut`] views.
pub struct NoLegSecurityAltIdMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoLegSecurityAltIdMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoLegSecurityAltIdEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoLegSecurityAltIdEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoLegSecurityAltIdEntryMut<'_> {
        self.get_mut(i).expect("NoLegSecurityAltId index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoLegSecurityAltIdEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoLegSecurityAltIdEntryMut { inner })
    }
}
