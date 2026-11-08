//! `DerivativeSecurityListRequest` (app message, MsgType `35=z`): typed facade over the raw
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

/// Typed facade over a `DerivativeSecurityListRequest` [`Message`] (35=z). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct DerivativeSecurityListRequest {
    msg: Message,
}

impl Default for DerivativeSecurityListRequest {
    fn default() -> Self {
        DerivativeSecurityListRequest::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `DerivativeSecurityListRequest`, checking the msg type.
impl TryFrom<Message> for DerivativeSecurityListRequest {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a DerivativeSecurityListRequest" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<DerivativeSecurityListRequest> for Message {
    fn from(typed_msg: DerivativeSecurityListRequest) -> Self {
        typed_msg.msg
    }
}

impl DerivativeSecurityListRequest {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "z";

    /// An empty `DerivativeSecurityListRequest` with only MsgType (35) set.
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

    // --- SecurityListRequestType (tag 559, INT, required) ---

    /// Set `SecurityListRequestType` (tag 559, required).
    pub fn set_security_list_request_type(&mut self, v: fields::SecurityListRequestType) {
        self.msg.body_mut().set_field(tags::SECURITY_LIST_REQUEST_TYPE, v.to_fix().to_string());
    }

    /// `SecurityListRequestType` (tag 559, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityListRequestType`]'s.
    pub fn security_list_request_type(
        &self,
    ) -> Result<fields::SecurityListRequestType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::SECURITY_LIST_REQUEST_TYPE)?;
        fields::SecurityListRequestType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_LIST_REQUEST_TYPE,
        })
    }

    /// Whether `SecurityListRequestType` (tag 559) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_list_request_type(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_LIST_REQUEST_TYPE)
    }

    // --- UnderlyingSymbol (tag 311, STRING, optional) ---

    /// Set `UnderlyingSymbol` (tag 311, optional).
    pub fn set_underlying_symbol(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SYMBOL, v.to_string());
    }

    /// `UnderlyingSymbol` (tag 311, optional).
    pub fn underlying_symbol(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SYMBOL)
    }

    /// Whether `UnderlyingSymbol` (tag 311) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_symbol(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SYMBOL)
    }

    // --- UnderlyingSymbolSfx (tag 312, STRING, optional) ---

    /// Set `UnderlyingSymbolSfx` (tag 312, optional).
    pub fn set_underlying_symbol_sfx(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SYMBOL_SFX, v.to_string());
    }

    /// `UnderlyingSymbolSfx` (tag 312, optional).
    pub fn underlying_symbol_sfx(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SYMBOL_SFX)
    }

    /// Whether `UnderlyingSymbolSfx` (tag 312) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_symbol_sfx(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SYMBOL_SFX)
    }

    // --- UnderlyingSecurityID (tag 309, STRING, optional) ---

    /// Set `UnderlyingSecurityID` (tag 309, optional).
    pub fn set_underlying_security_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SECURITY_ID, v.to_string());
    }

    /// `UnderlyingSecurityID` (tag 309, optional).
    pub fn underlying_security_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SECURITY_ID)
    }

    /// Whether `UnderlyingSecurityID` (tag 309) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_id(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SECURITY_ID)
    }

    // --- UnderlyingSecurityIDSource (tag 305, STRING, optional) ---

    /// Set `UnderlyingSecurityIDSource` (tag 305, optional).
    pub fn set_underlying_security_id_source(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SECURITY_ID_SOURCE, v.to_string());
    }

    /// `UnderlyingSecurityIDSource` (tag 305, optional).
    pub fn underlying_security_id_source(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SECURITY_ID_SOURCE)
    }

    /// Whether `UnderlyingSecurityIDSource` (tag 305) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_id_source(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SECURITY_ID_SOURCE)
    }

    // --- UnderlyingProduct (tag 462, INT, optional) ---

    /// Set `UnderlyingProduct` (tag 462, optional).
    pub fn set_underlying_product(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::UNDERLYING_PRODUCT, v.to_string());
    }

    /// `UnderlyingProduct` (tag 462, optional).
    pub fn underlying_product(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::UNDERLYING_PRODUCT)
    }

    /// Whether `UnderlyingProduct` (tag 462) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_product(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_PRODUCT)
    }

    // --- UnderlyingCFICode (tag 463, STRING, optional) ---

    /// Set `UnderlyingCFICode` (tag 463, optional).
    pub fn set_underlying_cfi_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_CFI_CODE, v.to_string());
    }

    /// `UnderlyingCFICode` (tag 463, optional).
    pub fn underlying_cfi_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_CFI_CODE)
    }

    /// Whether `UnderlyingCFICode` (tag 463) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_cfi_code(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_CFI_CODE)
    }

    // --- UnderlyingSecurityType (tag 310, STRING, optional) ---

    /// Set `UnderlyingSecurityType` (tag 310, optional).
    pub fn set_underlying_security_type(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SECURITY_TYPE, v.to_string());
    }

    /// `UnderlyingSecurityType` (tag 310, optional).
    pub fn underlying_security_type(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SECURITY_TYPE)
    }

    /// Whether `UnderlyingSecurityType` (tag 310) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_type(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SECURITY_TYPE)
    }

    // --- UnderlyingMaturityMonthYear (tag 313, MONTHYEAR, optional) ---

    /// Set `UnderlyingMaturityMonthYear` (tag 313, optional).
    pub fn set_underlying_maturity_month_year(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_MATURITY_MONTH_YEAR, v.to_string());
    }

    /// `UnderlyingMaturityMonthYear` (tag 313, optional).
    pub fn underlying_maturity_month_year(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_MATURITY_MONTH_YEAR)
    }

    /// Whether `UnderlyingMaturityMonthYear` (tag 313) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_maturity_month_year(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_MATURITY_MONTH_YEAR)
    }

    // --- UnderlyingMaturityDate (tag 542, LOCALMKTDATE, optional) ---

    /// Set `UnderlyingMaturityDate` (tag 542, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_underlying_maturity_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::UNDERLYING_MATURITY_DATE, convert::format_fix_date(v));
    }

    /// `UnderlyingMaturityDate` (tag 542, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn underlying_maturity_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::UNDERLYING_MATURITY_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_MATURITY_DATE)
    }

    /// Whether `UnderlyingMaturityDate` (tag 542) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_maturity_date(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_MATURITY_DATE)
    }

    // --- UnderlyingCouponPaymentDate (tag 241, UTCDATE, optional) ---

    /// Set `UnderlyingCouponPaymentDate` (tag 241, optional); written in FIX `UTCDATE` format.
    pub fn set_underlying_coupon_payment_date(&mut self, v: NaiveDate) {
        self.msg
            .body_mut()
            .set_field(tags::UNDERLYING_COUPON_PAYMENT_DATE, convert::format_fix_date(v));
    }

    /// `UnderlyingCouponPaymentDate` (tag 241, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn underlying_coupon_payment_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::UNDERLYING_COUPON_PAYMENT_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_COUPON_PAYMENT_DATE)
    }

    /// Whether `UnderlyingCouponPaymentDate` (tag 241) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_coupon_payment_date(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_COUPON_PAYMENT_DATE)
    }

    // --- UnderlyingIssueDate (tag 242, UTCDATE, optional) ---

    /// Set `UnderlyingIssueDate` (tag 242, optional); written in FIX `UTCDATE` format.
    pub fn set_underlying_issue_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::UNDERLYING_ISSUE_DATE, convert::format_fix_date(v));
    }

    /// `UnderlyingIssueDate` (tag 242, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn underlying_issue_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::UNDERLYING_ISSUE_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_ISSUE_DATE)
    }

    /// Whether `UnderlyingIssueDate` (tag 242) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_issue_date(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_ISSUE_DATE)
    }

    // --- UnderlyingRepoCollateralSecurityType (tag 243, STRING, optional) ---

    /// Set `UnderlyingRepoCollateralSecurityType` (tag 243, optional).
    pub fn set_underlying_repo_collateral_security_type(&mut self, v: &str) {
        self.msg
            .body_mut()
            .set_field(tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE, v.to_string());
    }

    /// `UnderlyingRepoCollateralSecurityType` (tag 243, optional).
    pub fn underlying_repo_collateral_security_type(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// Whether `UnderlyingRepoCollateralSecurityType` (tag 243) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_repo_collateral_security_type(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE)
    }

    // --- UnderlyingRepurchaseTerm (tag 244, INT, optional) ---

    /// Set `UnderlyingRepurchaseTerm` (tag 244, optional).
    pub fn set_underlying_repurchase_term(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::UNDERLYING_REPURCHASE_TERM, v.to_string());
    }

    /// `UnderlyingRepurchaseTerm` (tag 244, optional).
    pub fn underlying_repurchase_term(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::UNDERLYING_REPURCHASE_TERM)
    }

    /// Whether `UnderlyingRepurchaseTerm` (tag 244) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_repurchase_term(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_REPURCHASE_TERM)
    }

    // --- UnderlyingRepurchaseRate (tag 245, PERCENTAGE, optional) ---

    /// Set `UnderlyingRepurchaseRate` (tag 245, optional).
    pub fn set_underlying_repurchase_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::UNDERLYING_REPURCHASE_RATE, v.to_string());
    }

    /// `UnderlyingRepurchaseRate` (tag 245, optional).
    pub fn underlying_repurchase_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::UNDERLYING_REPURCHASE_RATE)
    }

    /// Whether `UnderlyingRepurchaseRate` (tag 245) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_repurchase_rate(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_REPURCHASE_RATE)
    }

    // --- UnderlyingFactor (tag 246, FLOAT, optional) ---

    /// Set `UnderlyingFactor` (tag 246, optional).
    pub fn set_underlying_factor(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::UNDERLYING_FACTOR, v.to_string());
    }

    /// `UnderlyingFactor` (tag 246, optional).
    pub fn underlying_factor(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::UNDERLYING_FACTOR)
    }

    /// Whether `UnderlyingFactor` (tag 246) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_factor(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_FACTOR)
    }

    // --- UnderlyingCreditRating (tag 256, STRING, optional) ---

    /// Set `UnderlyingCreditRating` (tag 256, optional).
    pub fn set_underlying_credit_rating(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_CREDIT_RATING, v.to_string());
    }

    /// `UnderlyingCreditRating` (tag 256, optional).
    pub fn underlying_credit_rating(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_CREDIT_RATING)
    }

    /// Whether `UnderlyingCreditRating` (tag 256) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_credit_rating(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_CREDIT_RATING)
    }

    // --- UnderlyingInstrRegistry (tag 595, STRING, optional) ---

    /// Set `UnderlyingInstrRegistry` (tag 595, optional).
    pub fn set_underlying_instr_registry(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_INSTR_REGISTRY, v.to_string());
    }

    /// `UnderlyingInstrRegistry` (tag 595, optional).
    pub fn underlying_instr_registry(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_INSTR_REGISTRY)
    }

    /// Whether `UnderlyingInstrRegistry` (tag 595) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_instr_registry(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_INSTR_REGISTRY)
    }

    // --- UnderlyingCountryOfIssue (tag 592, COUNTRY, optional) ---

    /// Set `UnderlyingCountryOfIssue` (tag 592, optional).
    pub fn set_underlying_country_of_issue(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_COUNTRY_OF_ISSUE, v.to_string());
    }

    /// `UnderlyingCountryOfIssue` (tag 592, optional).
    pub fn underlying_country_of_issue(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_COUNTRY_OF_ISSUE)
    }

    /// Whether `UnderlyingCountryOfIssue` (tag 592) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_country_of_issue(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_COUNTRY_OF_ISSUE)
    }

    // --- UnderlyingStateOrProvinceOfIssue (tag 593, STRING, optional) ---

    /// Set `UnderlyingStateOrProvinceOfIssue` (tag 593, optional).
    pub fn set_underlying_state_or_province_of_issue(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE, v.to_string());
    }

    /// `UnderlyingStateOrProvinceOfIssue` (tag 593, optional).
    pub fn underlying_state_or_province_of_issue(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// Whether `UnderlyingStateOrProvinceOfIssue` (tag 593) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_state_or_province_of_issue(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE)
    }

    // --- UnderlyingLocaleOfIssue (tag 594, STRING, optional) ---

    /// Set `UnderlyingLocaleOfIssue` (tag 594, optional).
    pub fn set_underlying_locale_of_issue(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_LOCALE_OF_ISSUE, v.to_string());
    }

    /// `UnderlyingLocaleOfIssue` (tag 594, optional).
    pub fn underlying_locale_of_issue(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_LOCALE_OF_ISSUE)
    }

    /// Whether `UnderlyingLocaleOfIssue` (tag 594) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_locale_of_issue(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_LOCALE_OF_ISSUE)
    }

    // --- UnderlyingRedemptionDate (tag 247, UTCDATE, optional) ---

    /// Set `UnderlyingRedemptionDate` (tag 247, optional); written in FIX `UTCDATE` format.
    pub fn set_underlying_redemption_date(&mut self, v: NaiveDate) {
        self.msg
            .body_mut()
            .set_field(tags::UNDERLYING_REDEMPTION_DATE, convert::format_fix_date(v));
    }

    /// `UnderlyingRedemptionDate` (tag 247, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn underlying_redemption_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::UNDERLYING_REDEMPTION_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_REDEMPTION_DATE)
    }

    /// Whether `UnderlyingRedemptionDate` (tag 247) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_redemption_date(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_REDEMPTION_DATE)
    }

    // --- UnderlyingPutOrCall (tag 315, INT, optional) ---

    /// Set `UnderlyingPutOrCall` (tag 315, optional).
    pub fn set_underlying_put_or_call(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::UNDERLYING_PUT_OR_CALL, v.to_string());
    }

    /// `UnderlyingPutOrCall` (tag 315, optional).
    pub fn underlying_put_or_call(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::UNDERLYING_PUT_OR_CALL)
    }

    /// Whether `UnderlyingPutOrCall` (tag 315) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_put_or_call(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_PUT_OR_CALL)
    }

    // --- UnderlyingStrikePrice (tag 316, PRICE, optional) ---

    /// Set `UnderlyingStrikePrice` (tag 316, optional); written in FIX `PRICE` format.
    pub fn set_underlying_strike_price(&mut self, v: Decimal) {
        self.msg
            .body_mut()
            .set_field(tags::UNDERLYING_STRIKE_PRICE, convert::format_fix_decimal(v));
    }

    /// `UnderlyingStrikePrice` (tag 316, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn underlying_strike_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::UNDERLYING_STRIKE_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::UNDERLYING_STRIKE_PRICE)
    }

    /// Whether `UnderlyingStrikePrice` (tag 316) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_strike_price(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_STRIKE_PRICE)
    }

    // --- UnderlyingOptAttribute (tag 317, CHAR, optional) ---

    /// Set `UnderlyingOptAttribute` (tag 317, optional).
    pub fn set_underlying_opt_attribute(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::UNDERLYING_OPT_ATTRIBUTE, v.to_string());
    }

    /// `UnderlyingOptAttribute` (tag 317, optional).
    pub fn underlying_opt_attribute(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::UNDERLYING_OPT_ATTRIBUTE)
    }

    /// Whether `UnderlyingOptAttribute` (tag 317) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_opt_attribute(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_OPT_ATTRIBUTE)
    }

    // --- UnderlyingContractMultiplier (tag 436, FLOAT, optional) ---

    /// Set `UnderlyingContractMultiplier` (tag 436, optional).
    pub fn set_underlying_contract_multiplier(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::UNDERLYING_CONTRACT_MULTIPLIER, v.to_string());
    }

    /// `UnderlyingContractMultiplier` (tag 436, optional).
    pub fn underlying_contract_multiplier(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::UNDERLYING_CONTRACT_MULTIPLIER)
    }

    /// Whether `UnderlyingContractMultiplier` (tag 436) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_contract_multiplier(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_CONTRACT_MULTIPLIER)
    }

    // --- UnderlyingCouponRate (tag 435, PERCENTAGE, optional) ---

    /// Set `UnderlyingCouponRate` (tag 435, optional).
    pub fn set_underlying_coupon_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::UNDERLYING_COUPON_RATE, v.to_string());
    }

    /// `UnderlyingCouponRate` (tag 435, optional).
    pub fn underlying_coupon_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::UNDERLYING_COUPON_RATE)
    }

    /// Whether `UnderlyingCouponRate` (tag 435) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_coupon_rate(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_COUPON_RATE)
    }

    // --- UnderlyingSecurityExchange (tag 308, EXCHANGE, optional) ---

    /// Set `UnderlyingSecurityExchange` (tag 308, optional).
    pub fn set_underlying_security_exchange(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SECURITY_EXCHANGE, v.to_string());
    }

    /// `UnderlyingSecurityExchange` (tag 308, optional).
    pub fn underlying_security_exchange(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SECURITY_EXCHANGE)
    }

    /// Whether `UnderlyingSecurityExchange` (tag 308) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_exchange(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SECURITY_EXCHANGE)
    }

    // --- UnderlyingIssuer (tag 306, STRING, optional) ---

    /// Set `UnderlyingIssuer` (tag 306, optional).
    pub fn set_underlying_issuer(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_ISSUER, v.to_string());
    }

    /// `UnderlyingIssuer` (tag 306, optional).
    pub fn underlying_issuer(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_ISSUER)
    }

    /// Whether `UnderlyingIssuer` (tag 306) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_issuer(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_ISSUER)
    }

    // --- EncodedUnderlyingIssuerLen (tag 362, LENGTH, optional) ---

    /// Set `EncodedUnderlyingIssuerLen` (tag 362, optional).
    pub fn set_encoded_underlying_issuer_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_UNDERLYING_ISSUER_LEN, v.to_string());
    }

    /// `EncodedUnderlyingIssuerLen` (tag 362, optional).
    pub fn encoded_underlying_issuer_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_UNDERLYING_ISSUER_LEN)
    }

    /// Whether `EncodedUnderlyingIssuerLen` (tag 362) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_issuer_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_UNDERLYING_ISSUER_LEN)
    }

    // --- EncodedUnderlyingIssuer (tag 363, DATA, optional) ---

    /// Set `EncodedUnderlyingIssuer` (tag 363, optional).
    pub fn set_encoded_underlying_issuer(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_UNDERLYING_ISSUER, v.to_string());
    }

    /// `EncodedUnderlyingIssuer` (tag 363, optional).
    pub fn encoded_underlying_issuer(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_UNDERLYING_ISSUER)
    }

    /// Whether `EncodedUnderlyingIssuer` (tag 363) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_issuer(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_UNDERLYING_ISSUER)
    }

    // --- UnderlyingSecurityDesc (tag 307, STRING, optional) ---

    /// Set `UnderlyingSecurityDesc` (tag 307, optional).
    pub fn set_underlying_security_desc(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::UNDERLYING_SECURITY_DESC, v.to_string());
    }

    /// `UnderlyingSecurityDesc` (tag 307, optional).
    pub fn underlying_security_desc(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::UNDERLYING_SECURITY_DESC)
    }

    /// Whether `UnderlyingSecurityDesc` (tag 307) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_desc(&self) -> bool {
        self.msg.body().has_field(tags::UNDERLYING_SECURITY_DESC)
    }

    // --- EncodedUnderlyingSecurityDescLen (tag 364, LENGTH, optional) ---

    /// Set `EncodedUnderlyingSecurityDescLen` (tag 364, optional).
    pub fn set_encoded_underlying_security_desc_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN, v.to_string());
    }

    /// `EncodedUnderlyingSecurityDescLen` (tag 364, optional).
    pub fn encoded_underlying_security_desc_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN)
    }

    /// Whether `EncodedUnderlyingSecurityDescLen` (tag 364) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_security_desc_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN)
    }

    // --- EncodedUnderlyingSecurityDesc (tag 365, DATA, optional) ---

    /// Set `EncodedUnderlyingSecurityDesc` (tag 365, optional).
    pub fn set_encoded_underlying_security_desc(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_UNDERLYING_SECURITY_DESC, v.to_string());
    }

    /// `EncodedUnderlyingSecurityDesc` (tag 365, optional).
    pub fn encoded_underlying_security_desc(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_UNDERLYING_SECURITY_DESC)
    }

    /// Whether `EncodedUnderlyingSecurityDesc` (tag 365) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_security_desc(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_UNDERLYING_SECURITY_DESC)
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

    // --- SubscriptionRequestType (tag 263, CHAR, optional) ---

    /// Set `SubscriptionRequestType` (tag 263, optional).
    pub fn set_subscription_request_type(&mut self, v: fields::SubscriptionRequestType) {
        self.msg.body_mut().set_field(tags::SUBSCRIPTION_REQUEST_TYPE, v.to_fix().to_string());
    }

    /// `SubscriptionRequestType` (tag 263, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SubscriptionRequestType`]'s.
    pub fn subscription_request_type(&self) -> Result<fields::SubscriptionRequestType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SUBSCRIPTION_REQUEST_TYPE)?;
        fields::SubscriptionRequestType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SUBSCRIPTION_REQUEST_TYPE,
        })
    }

    /// Whether `SubscriptionRequestType` (tag 263) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_subscription_request_type(&self) -> bool {
        self.msg.body().has_field(tags::SUBSCRIPTION_REQUEST_TYPE)
    }

    // --- NoUnderlyingSecurityAltId group (count tag NO_UNDERLYING_SECURITY_ALT_ID) ---

    /// The `NoUnderlyingSecurityAltId` group (count tag `NO_UNDERLYING_SECURITY_ALT_ID`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_underlying_security_alt_id(&self) -> Option<NoUnderlyingSecurityAltId<'_>> {
        self.msg
            .body()
            .get_group(tags::NO_UNDERLYING_SECURITY_ALT_ID)
            .map(|group| NoUnderlyingSecurityAltId { group })
    }

    /// The `NoUnderlyingSecurityAltId` group (count tag `NO_UNDERLYING_SECURITY_ALT_ID`, optional) for editing, or `None` if
    /// absent.
    pub fn no_underlying_security_alt_id_mut(
        &mut self,
    ) -> Option<NoUnderlyingSecurityAltIdMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_UNDERLYING_SECURITY_ALT_ID)
            .map(|group| NoUnderlyingSecurityAltIdMut { group })
    }

    /// Create the `NoUnderlyingSecurityAltId` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_underlying_security_alt_id_entry`] on the same group.
    pub fn set_no_underlying_security_alt_id(
        &mut self,
        count: u32,
    ) -> NoUnderlyingSecurityAltIdMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_UNDERLYING_SECURITY_ALT_ID,
            count,
            NO_UNDERLYING_SECURITY_ALT_ID_FIELD_ORDER,
        );
        NoUnderlyingSecurityAltIdMut { group }
    }

    /// Append one `NoUnderlyingSecurityAltId` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_underlying_security_alt_id`] on the same group.
    pub fn add_no_underlying_security_alt_id_entry(
        &mut self,
        fill: impl FnOnce(&mut NoUnderlyingSecurityAltIdEntryMut<'_>),
    ) {
        let inner = self.msg.body_mut().add_group_instance(
            tags::NO_UNDERLYING_SECURITY_ALT_ID,
            NO_UNDERLYING_SECURITY_ALT_ID_FIELD_ORDER,
        );
        fill(&mut NoUnderlyingSecurityAltIdEntryMut { inner });
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

/// Wire order of a `NoUnderlyingSecurityAltId` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_UNDERLYING_SECURITY_ALT_ID_FIELD_ORDER: &[u32] = &[
    tags::UNDERLYING_SECURITY_ALT_ID,
    tags::UNDERLYING_SECURITY_ALT_ID_SOURCE,
];

/// One `NoUnderlyingSecurityAltId` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoUnderlyingSecurityAltIdEntry<'a> {
    inner: &'a FieldMap,
}

impl NoUnderlyingSecurityAltIdEntry<'_> {
    /// `UnderlyingSecurityAltID` (tag 458, optional).
    pub fn underlying_security_alt_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_ALT_ID)
    }

    /// Whether `UnderlyingSecurityAltID` (tag 458) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_alt_id(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_ALT_ID)
    }

    /// `UnderlyingSecurityAltIDSource` (tag 459, optional).
    pub fn underlying_security_alt_id_source(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_ALT_ID_SOURCE)
    }

    /// Whether `UnderlyingSecurityAltIDSource` (tag 459) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_alt_id_source(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_ALT_ID_SOURCE)
    }
}

/// One `NoUnderlyingSecurityAltId` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoUnderlyingSecurityAltIdEntryMut::as_ref`].
pub struct NoUnderlyingSecurityAltIdEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoUnderlyingSecurityAltIdEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoUnderlyingSecurityAltIdEntry<'_> {
        NoUnderlyingSecurityAltIdEntry { inner: self.inner }
    }

    /// Set `UnderlyingSecurityAltID` (tag 458, optional).
    pub fn set_underlying_security_alt_id(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_ALT_ID, v.to_string());
    }

    /// Set `UnderlyingSecurityAltIDSource` (tag 459, optional).
    pub fn set_underlying_security_alt_id_source(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_ALT_ID_SOURCE, v.to_string());
    }
}

/// The `NoUnderlyingSecurityAltId` repeating group (count tag `NO_UNDERLYING_SECURITY_ALT_ID`), read-only: its
/// instances as [`NoUnderlyingSecurityAltIdEntry`] views.
pub struct NoUnderlyingSecurityAltId<'a> {
    group: &'a Group,
}

impl<'a> NoUnderlyingSecurityAltId<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoUnderlyingSecurityAltIdEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoUnderlyingSecurityAltIdEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoUnderlyingSecurityAltIdEntry<'a> {
        self.get(i).expect("NoUnderlyingSecurityAltId index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoUnderlyingSecurityAltIdEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoUnderlyingSecurityAltIdEntry { inner })
    }
}

/// The `NoUnderlyingSecurityAltId` repeating group (count tag `NO_UNDERLYING_SECURITY_ALT_ID`), mutable: its
/// instances as [`NoUnderlyingSecurityAltIdEntryMut`] views.
pub struct NoUnderlyingSecurityAltIdMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoUnderlyingSecurityAltIdMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoUnderlyingSecurityAltIdEntryMut<'_>> {
        self.group
            .as_mut_slice()
            .get_mut(i)
            .map(|inner| NoUnderlyingSecurityAltIdEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoUnderlyingSecurityAltIdEntryMut<'_> {
        self.get_mut(i).expect("NoUnderlyingSecurityAltId index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoUnderlyingSecurityAltIdEntryMut<'_>> {
        self.group
            .as_mut_slice()
            .iter_mut()
            .map(|inner| NoUnderlyingSecurityAltIdEntryMut { inner })
    }
}
