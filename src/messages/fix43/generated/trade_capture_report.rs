//! `TradeCaptureReport` (app message, MsgType `35=AE`): typed facade over the raw
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

/// Typed facade over a `TradeCaptureReport` [`Message`] (35=AE). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct TradeCaptureReport {
    msg: Message,
}

impl Default for TradeCaptureReport {
    fn default() -> Self {
        TradeCaptureReport::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `TradeCaptureReport`, checking the msg type.
impl TryFrom<Message> for TradeCaptureReport {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a TradeCaptureReport" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<TradeCaptureReport> for Message {
    fn from(typed_msg: TradeCaptureReport) -> Self {
        typed_msg.msg
    }
}

impl TradeCaptureReport {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "AE";

    /// An empty `TradeCaptureReport` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- TradeReportID (tag 571, STRING, required) ---

    /// Set `TradeReportID` (tag 571, required).
    pub fn set_trade_report_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRADE_REPORT_ID, v.to_string());
    }

    /// `TradeReportID` (tag 571, required).
    pub fn trade_report_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRADE_REPORT_ID)
    }

    /// Whether `TradeReportID` (tag 571) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_report_id(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_REPORT_ID)
    }

    // --- TradeReportTransType (tag 487, CHAR, optional) ---

    /// Set `TradeReportTransType` (tag 487, optional).
    pub fn set_trade_report_trans_type(&mut self, v: fields::TradeReportTransType) {
        self.msg.body_mut().set_field(tags::TRADE_REPORT_TRANS_TYPE, v.to_fix().to_string());
    }

    /// `TradeReportTransType` (tag 487, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TradeReportTransType`]'s.
    pub fn trade_report_trans_type(&self) -> Result<fields::TradeReportTransType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::TRADE_REPORT_TRANS_TYPE)?;
        fields::TradeReportTransType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TRADE_REPORT_TRANS_TYPE,
        })
    }

    /// Whether `TradeReportTransType` (tag 487) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_report_trans_type(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_REPORT_TRANS_TYPE)
    }

    // --- TradeRequestID (tag 568, STRING, optional) ---

    /// Set `TradeRequestID` (tag 568, optional).
    pub fn set_trade_request_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRADE_REQUEST_ID, v.to_string());
    }

    /// `TradeRequestID` (tag 568, optional).
    pub fn trade_request_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRADE_REQUEST_ID)
    }

    /// Whether `TradeRequestID` (tag 568) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_request_id(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_REQUEST_ID)
    }

    // --- ExecType (tag 150, CHAR, required) ---

    /// Set `ExecType` (tag 150, required).
    pub fn set_exec_type(&mut self, v: fields::ExecType) {
        self.msg.body_mut().set_field(tags::EXEC_TYPE, v.to_fix().to_string());
    }

    /// `ExecType` (tag 150, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ExecType`]'s.
    pub fn exec_type(&self) -> Result<fields::ExecType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::EXEC_TYPE)?;
        fields::ExecType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::EXEC_TYPE,
        })
    }

    /// Whether `ExecType` (tag 150) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exec_type(&self) -> bool {
        self.msg.body().has_field(tags::EXEC_TYPE)
    }

    // --- TradeReportRefID (tag 572, STRING, optional) ---

    /// Set `TradeReportRefID` (tag 572, optional).
    pub fn set_trade_report_ref_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRADE_REPORT_REF_ID, v.to_string());
    }

    /// `TradeReportRefID` (tag 572, optional).
    pub fn trade_report_ref_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRADE_REPORT_REF_ID)
    }

    /// Whether `TradeReportRefID` (tag 572) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_report_ref_id(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_REPORT_REF_ID)
    }

    // --- ExecID (tag 17, STRING, optional) ---

    /// Set `ExecID` (tag 17, optional).
    pub fn set_exec_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::EXEC_ID, v.to_string());
    }

    /// `ExecID` (tag 17, optional).
    pub fn exec_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::EXEC_ID)
    }

    /// Whether `ExecID` (tag 17) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exec_id(&self) -> bool {
        self.msg.body().has_field(tags::EXEC_ID)
    }

    // --- SecondaryExecID (tag 527, STRING, optional) ---

    /// Set `SecondaryExecID` (tag 527, optional).
    pub fn set_secondary_exec_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECONDARY_EXEC_ID, v.to_string());
    }

    /// `SecondaryExecID` (tag 527, optional).
    pub fn secondary_exec_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECONDARY_EXEC_ID)
    }

    /// Whether `SecondaryExecID` (tag 527) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_exec_id(&self) -> bool {
        self.msg.body().has_field(tags::SECONDARY_EXEC_ID)
    }

    // --- ExecRestatementReason (tag 378, INT, optional) ---

    /// Set `ExecRestatementReason` (tag 378, optional).
    pub fn set_exec_restatement_reason(&mut self, v: fields::ExecRestatementReason) {
        self.msg.body_mut().set_field(tags::EXEC_RESTATEMENT_REASON, v.to_fix().to_string());
    }

    /// `ExecRestatementReason` (tag 378, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ExecRestatementReason`]'s.
    pub fn exec_restatement_reason(&self) -> Result<fields::ExecRestatementReason, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::EXEC_RESTATEMENT_REASON)?;
        fields::ExecRestatementReason::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::EXEC_RESTATEMENT_REASON,
        })
    }

    /// Whether `ExecRestatementReason` (tag 378) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exec_restatement_reason(&self) -> bool {
        self.msg.body().has_field(tags::EXEC_RESTATEMENT_REASON)
    }

    // --- PreviouslyReported (tag 570, BOOLEAN, required) ---

    /// Set `PreviouslyReported` (tag 570, required); written in FIX `BOOLEAN` format.
    pub fn set_previously_reported(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::PREVIOUSLY_REPORTED, convert::format_fix_bool(v));
    }

    /// `PreviouslyReported` (tag 570, required). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn previously_reported(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::PREVIOUSLY_REPORTED)?;
        convert::parse_fix_bool(&raw, tags::PREVIOUSLY_REPORTED)
    }

    /// Whether `PreviouslyReported` (tag 570) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_previously_reported(&self) -> bool {
        self.msg.body().has_field(tags::PREVIOUSLY_REPORTED)
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

    // --- OrderQty (tag 38, QTY, optional) ---

    /// Set `OrderQty` (tag 38, optional); written in FIX `QTY` format.
    pub fn set_order_qty(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::ORDER_QTY, convert::format_fix_decimal(v));
    }

    /// `OrderQty` (tag 38, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn order_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::ORDER_QTY)?;
        convert::parse_fix_decimal(&raw, tags::ORDER_QTY)
    }

    /// Whether `OrderQty` (tag 38) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_qty(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_QTY)
    }

    // --- CashOrderQty (tag 152, QTY, optional) ---

    /// Set `CashOrderQty` (tag 152, optional); written in FIX `QTY` format.
    pub fn set_cash_order_qty(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::CASH_ORDER_QTY, convert::format_fix_decimal(v));
    }

    /// `CashOrderQty` (tag 152, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn cash_order_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::CASH_ORDER_QTY)?;
        convert::parse_fix_decimal(&raw, tags::CASH_ORDER_QTY)
    }

    /// Whether `CashOrderQty` (tag 152) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_order_qty(&self) -> bool {
        self.msg.body().has_field(tags::CASH_ORDER_QTY)
    }

    // --- OrderPercent (tag 516, PERCENTAGE, optional) ---

    /// Set `OrderPercent` (tag 516, optional).
    pub fn set_order_percent(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::ORDER_PERCENT, v.to_string());
    }

    /// `OrderPercent` (tag 516, optional).
    pub fn order_percent(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::ORDER_PERCENT)
    }

    /// Whether `OrderPercent` (tag 516) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_percent(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_PERCENT)
    }

    // --- RoundingDirection (tag 468, CHAR, optional) ---

    /// Set `RoundingDirection` (tag 468, optional).
    pub fn set_rounding_direction(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::ROUNDING_DIRECTION, v.to_string());
    }

    /// `RoundingDirection` (tag 468, optional).
    pub fn rounding_direction(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::ROUNDING_DIRECTION)
    }

    /// Whether `RoundingDirection` (tag 468) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_rounding_direction(&self) -> bool {
        self.msg.body().has_field(tags::ROUNDING_DIRECTION)
    }

    // --- RoundingModulus (tag 469, FLOAT, optional) ---

    /// Set `RoundingModulus` (tag 469, optional).
    pub fn set_rounding_modulus(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::ROUNDING_MODULUS, v.to_string());
    }

    /// `RoundingModulus` (tag 469, optional).
    pub fn rounding_modulus(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::ROUNDING_MODULUS)
    }

    /// Whether `RoundingModulus` (tag 469) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_rounding_modulus(&self) -> bool {
        self.msg.body().has_field(tags::ROUNDING_MODULUS)
    }

    // --- LastQty (tag 32, QTY, required) ---

    /// Set `LastQty` (tag 32, required); written in FIX `QTY` format.
    pub fn set_last_qty(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::LAST_QTY, convert::format_fix_decimal(v));
    }

    /// `LastQty` (tag 32, required). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn last_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::LAST_QTY)?;
        convert::parse_fix_decimal(&raw, tags::LAST_QTY)
    }

    /// Whether `LastQty` (tag 32) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_qty(&self) -> bool {
        self.msg.body().has_field(tags::LAST_QTY)
    }

    // --- LastPx (tag 31, PRICE, required) ---

    /// Set `LastPx` (tag 31, required); written in FIX `PRICE` format.
    pub fn set_last_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::LAST_PX, convert::format_fix_decimal(v));
    }

    /// `LastPx` (tag 31, required). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn last_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::LAST_PX)?;
        convert::parse_fix_decimal(&raw, tags::LAST_PX)
    }

    /// Whether `LastPx` (tag 31) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_px(&self) -> bool {
        self.msg.body().has_field(tags::LAST_PX)
    }

    // --- LastSpotRate (tag 194, PRICE, optional) ---

    /// Set `LastSpotRate` (tag 194, optional); written in FIX `PRICE` format.
    pub fn set_last_spot_rate(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::LAST_SPOT_RATE, convert::format_fix_decimal(v));
    }

    /// `LastSpotRate` (tag 194, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn last_spot_rate(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::LAST_SPOT_RATE)?;
        convert::parse_fix_decimal(&raw, tags::LAST_SPOT_RATE)
    }

    /// Whether `LastSpotRate` (tag 194) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_spot_rate(&self) -> bool {
        self.msg.body().has_field(tags::LAST_SPOT_RATE)
    }

    // --- LastForwardPoints (tag 195, PRICEOFFSET, optional) ---

    /// Set `LastForwardPoints` (tag 195, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_last_forward_points(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::LAST_FORWARD_POINTS, convert::format_fix_decimal(v));
    }

    /// `LastForwardPoints` (tag 195, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn last_forward_points(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::LAST_FORWARD_POINTS)?;
        convert::parse_fix_decimal(&raw, tags::LAST_FORWARD_POINTS)
    }

    /// Whether `LastForwardPoints` (tag 195) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_forward_points(&self) -> bool {
        self.msg.body().has_field(tags::LAST_FORWARD_POINTS)
    }

    // --- LastMkt (tag 30, EXCHANGE, optional) ---

    /// Set `LastMkt` (tag 30, optional).
    pub fn set_last_mkt(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LAST_MKT, v.to_string());
    }

    /// `LastMkt` (tag 30, optional).
    pub fn last_mkt(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LAST_MKT)
    }

    /// Whether `LastMkt` (tag 30) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_mkt(&self) -> bool {
        self.msg.body().has_field(tags::LAST_MKT)
    }

    // --- TradeDate (tag 75, LOCALMKTDATE, required) ---

    /// Set `TradeDate` (tag 75, required); written in FIX `LOCALMKTDATE` format.
    pub fn set_trade_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::TRADE_DATE, convert::format_fix_date(v));
    }

    /// `TradeDate` (tag 75, required). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn trade_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRADE_DATE)?;
        convert::parse_fix_date(&raw, tags::TRADE_DATE)
    }

    /// Whether `TradeDate` (tag 75) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_date(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_DATE)
    }

    // --- TransactTime (tag 60, UTCTIMESTAMP, required) ---

    /// Set `TransactTime` (tag 60, required); written in FIX `UTCTIMESTAMP` format.
    pub fn set_transact_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::TRANSACT_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TransactTime` (tag 60, required). `Err(InvalidFormat)` unless a valid FIX
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

    // --- MatchStatus (tag 573, CHAR, optional) ---

    /// Set `MatchStatus` (tag 573, optional).
    pub fn set_match_status(&mut self, v: fields::MatchStatus) {
        self.msg.body_mut().set_field(tags::MATCH_STATUS, v.to_fix().to_string());
    }

    /// `MatchStatus` (tag 573, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MatchStatus`]'s.
    pub fn match_status(&self) -> Result<fields::MatchStatus, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::MATCH_STATUS)?;
        fields::MatchStatus::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MATCH_STATUS,
        })
    }

    /// Whether `MatchStatus` (tag 573) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_match_status(&self) -> bool {
        self.msg.body().has_field(tags::MATCH_STATUS)
    }

    // --- MatchType (tag 574, STRING, optional) ---

    /// Set `MatchType` (tag 574, optional).
    pub fn set_match_type(&mut self, v: fields::MatchType) {
        self.msg.body_mut().set_field(tags::MATCH_TYPE, v.to_fix().to_string());
    }

    /// `MatchType` (tag 574, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MatchType`]'s.
    pub fn match_type(&self) -> Result<fields::MatchType, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MATCH_TYPE)?;
        fields::MatchType::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::MATCH_TYPE,
        })
    }

    /// Whether `MatchType` (tag 574) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_match_type(&self) -> bool {
        self.msg.body().has_field(tags::MATCH_TYPE)
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

    // --- NoSides group (count tag NO_SIDES) ---

    /// The `NoSides` group (count tag `NO_SIDES`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_sides(&self) -> Option<NoSides<'_>> {
        self.msg.body().get_group(tags::NO_SIDES).map(|group| NoSides { group })
    }

    /// The `NoSides` group (count tag `NO_SIDES`, required) for editing, or `None` if
    /// absent.
    pub fn no_sides_mut(&mut self) -> Option<NoSidesMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_SIDES).map(|group| NoSidesMut { group })
    }

    /// Create the `NoSides` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_sides_entry`] on the same group.
    pub fn set_no_sides(&mut self, count: u32) -> NoSidesMut<'_> {
        let group = self.msg.body_mut().set_group(tags::NO_SIDES, count, NO_SIDES_FIELD_ORDER);
        NoSidesMut { group }
    }

    /// Append one `NoSides` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_sides`] on the same group.
    pub fn add_no_sides_entry(&mut self, fill: impl FnOnce(&mut NoSidesEntryMut<'_>)) {
        let inner = self.msg.body_mut().add_group_instance(tags::NO_SIDES, NO_SIDES_FIELD_ORDER);
        fill(&mut NoSidesEntryMut { inner });
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

/// Wire order of a `NoSides` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_SIDES_FIELD_ORDER: &[u32] = &[
    tags::SIDE,
    tags::ORDER_ID,
    tags::SECONDARY_ORDER_ID,
    tags::CL_ORD_ID,
    tags::NO_PARTY_IDS,
    tags::ACCOUNT,
    tags::ACCOUNT_TYPE,
    tags::PROCESS_CODE,
    tags::ODD_LOT,
    tags::NO_CLEARING_INSTRUCTIONS,
    tags::CLEARING_FEE_INDICATOR,
    tags::TRADE_INPUT_SOURCE,
    tags::TRADE_INPUT_DEVICE,
    tags::CURRENCY,
    tags::COMPLIANCE_ID,
    tags::SOLICITED_FLAG,
    tags::ORDER_CAPACITY,
    tags::ORDER_RESTRICTIONS,
    tags::CUST_ORDER_CAPACITY,
    tags::TRANS_BKD_TIME,
    tags::TRADING_SESSION_ID,
    tags::TRADING_SESSION_SUB_ID,
    tags::COMMISSION,
    tags::COMM_TYPE,
    tags::COMM_CURRENCY,
    tags::FUND_RENEW_WAIV,
    tags::GROSS_TRADE_AMT,
    tags::NUM_DAYS_INTEREST,
    tags::EX_DATE,
    tags::ACCRUED_INTEREST_RATE,
    tags::ACCRUED_INTEREST_AMT,
    tags::CONCESSION,
    tags::TOTAL_TAKEDOWN,
    tags::NET_MONEY,
    tags::SETTL_CURR_AMT,
    tags::SETTL_CURRENCY,
    tags::SETTL_CURR_FX_RATE,
    tags::SETTL_CURR_FX_RATE_CALC,
    tags::POSITION_EFFECT,
    tags::TEXT,
    tags::ENCODED_TEXT_LEN,
    tags::ENCODED_TEXT,
    tags::MULTI_LEG_REPORTING_TYPE,
    tags::NO_CONT_AMTS,
    tags::NO_MISC_FEES,
];

/// One `NoSides` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoSidesEntry<'a> {
    inner: &'a FieldMap,
}

impl NoSidesEntry<'_> {
    /// `Side` (tag 54, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Side`]'s.
    pub fn side(&self) -> Result<fields::Side, FieldError> {
        let n = self.inner.get_field::<char>(tags::SIDE)?;
        fields::Side::from_fix(n).ok_or(FieldError::InvalidFormat { tag: tags::SIDE })
    }

    /// Whether `Side` (tag 54) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side(&self) -> bool {
        self.inner.has_field(tags::SIDE)
    }

    /// `OrderID` (tag 37, required).
    pub fn order_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ORDER_ID)
    }

    /// Whether `OrderID` (tag 37) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_id(&self) -> bool {
        self.inner.has_field(tags::ORDER_ID)
    }

    /// `SecondaryOrderID` (tag 198, optional).
    pub fn secondary_order_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECONDARY_ORDER_ID)
    }

    /// Whether `SecondaryOrderID` (tag 198) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_order_id(&self) -> bool {
        self.inner.has_field(tags::SECONDARY_ORDER_ID)
    }

    /// `ClOrdID` (tag 11, optional).
    pub fn cl_ord_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CL_ORD_ID)
    }

    /// Whether `ClOrdID` (tag 11) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cl_ord_id(&self) -> bool {
        self.inner.has_field(tags::CL_ORD_ID)
    }

    /// `Account` (tag 1, optional).
    pub fn account(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ACCOUNT)
    }

    /// Whether `Account` (tag 1) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_account(&self) -> bool {
        self.inner.has_field(tags::ACCOUNT)
    }

    /// `AccountType` (tag 581, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::AccountType`]'s.
    pub fn account_type(&self) -> Result<fields::AccountType, FieldError> {
        let n = self.inner.get_field::<i32>(tags::ACCOUNT_TYPE)?;
        fields::AccountType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ACCOUNT_TYPE,
        })
    }

    /// Whether `AccountType` (tag 581) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_account_type(&self) -> bool {
        self.inner.has_field(tags::ACCOUNT_TYPE)
    }

    /// `ProcessCode` (tag 81, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ProcessCode`]'s.
    pub fn process_code(&self) -> Result<fields::ProcessCode, FieldError> {
        let n = self.inner.get_field::<char>(tags::PROCESS_CODE)?;
        fields::ProcessCode::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PROCESS_CODE,
        })
    }

    /// Whether `ProcessCode` (tag 81) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_process_code(&self) -> bool {
        self.inner.has_field(tags::PROCESS_CODE)
    }

    /// `OddLot` (tag 575, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn odd_lot(&self) -> Result<bool, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ODD_LOT)?;
        convert::parse_fix_bool(&raw, tags::ODD_LOT)
    }

    /// Whether `OddLot` (tag 575) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_odd_lot(&self) -> bool {
        self.inner.has_field(tags::ODD_LOT)
    }

    /// `ClearingFeeIndicator` (tag 635, optional).
    pub fn clearing_fee_indicator(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CLEARING_FEE_INDICATOR)
    }

    /// Whether `ClearingFeeIndicator` (tag 635) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_clearing_fee_indicator(&self) -> bool {
        self.inner.has_field(tags::CLEARING_FEE_INDICATOR)
    }

    /// `TradeInputSource` (tag 578, optional).
    pub fn trade_input_source(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TRADE_INPUT_SOURCE)
    }

    /// Whether `TradeInputSource` (tag 578) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_input_source(&self) -> bool {
        self.inner.has_field(tags::TRADE_INPUT_SOURCE)
    }

    /// `TradeInputDevice` (tag 579, optional).
    pub fn trade_input_device(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TRADE_INPUT_DEVICE)
    }

    /// Whether `TradeInputDevice` (tag 579) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_input_device(&self) -> bool {
        self.inner.has_field(tags::TRADE_INPUT_DEVICE)
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

    /// `ComplianceID` (tag 376, optional).
    pub fn compliance_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::COMPLIANCE_ID)
    }

    /// Whether `ComplianceID` (tag 376) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_compliance_id(&self) -> bool {
        self.inner.has_field(tags::COMPLIANCE_ID)
    }

    /// `SolicitedFlag` (tag 377, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn solicited_flag(&self) -> Result<bool, FieldError> {
        let raw = self.inner.get_field::<String>(tags::SOLICITED_FLAG)?;
        convert::parse_fix_bool(&raw, tags::SOLICITED_FLAG)
    }

    /// Whether `SolicitedFlag` (tag 377) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_solicited_flag(&self) -> bool {
        self.inner.has_field(tags::SOLICITED_FLAG)
    }

    /// `OrderCapacity` (tag 528, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrderCapacity`]'s.
    pub fn order_capacity(&self) -> Result<fields::OrderCapacity, FieldError> {
        let n = self.inner.get_field::<char>(tags::ORDER_CAPACITY)?;
        fields::OrderCapacity::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORDER_CAPACITY,
        })
    }

    /// Whether `OrderCapacity` (tag 528) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_capacity(&self) -> bool {
        self.inner.has_field(tags::ORDER_CAPACITY)
    }

    /// `OrderRestrictions` (tag 529, optional).
    pub fn order_restrictions(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ORDER_RESTRICTIONS)
    }

    /// Whether `OrderRestrictions` (tag 529) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_restrictions(&self) -> bool {
        self.inner.has_field(tags::ORDER_RESTRICTIONS)
    }

    /// `CustOrderCapacity` (tag 582, optional).
    pub fn cust_order_capacity(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::CUST_ORDER_CAPACITY)
    }

    /// Whether `CustOrderCapacity` (tag 582) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cust_order_capacity(&self) -> bool {
        self.inner.has_field(tags::CUST_ORDER_CAPACITY)
    }

    /// `TransBkdTime` (tag 483, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn trans_bkd_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.inner.get_field::<String>(tags::TRANS_BKD_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRANS_BKD_TIME)
    }

    /// Whether `TransBkdTime` (tag 483) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trans_bkd_time(&self) -> bool {
        self.inner.has_field(tags::TRANS_BKD_TIME)
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

    /// `Commission` (tag 12, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn commission(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::COMMISSION)?;
        convert::parse_fix_decimal(&raw, tags::COMMISSION)
    }

    /// Whether `Commission` (tag 12) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_commission(&self) -> bool {
        self.inner.has_field(tags::COMMISSION)
    }

    /// `CommType` (tag 13, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::CommType`]'s.
    pub fn comm_type(&self) -> Result<fields::CommType, FieldError> {
        let n = self.inner.get_field::<char>(tags::COMM_TYPE)?;
        fields::CommType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::COMM_TYPE,
        })
    }

    /// Whether `CommType` (tag 13) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_comm_type(&self) -> bool {
        self.inner.has_field(tags::COMM_TYPE)
    }

    /// `CommCurrency` (tag 479, optional).
    pub fn comm_currency(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::COMM_CURRENCY)
    }

    /// Whether `CommCurrency` (tag 479) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_comm_currency(&self) -> bool {
        self.inner.has_field(tags::COMM_CURRENCY)
    }

    /// `FundRenewWaiv` (tag 497, optional).
    pub fn fund_renew_waiv(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::FUND_RENEW_WAIV)
    }

    /// Whether `FundRenewWaiv` (tag 497) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fund_renew_waiv(&self) -> bool {
        self.inner.has_field(tags::FUND_RENEW_WAIV)
    }

    /// `GrossTradeAmt` (tag 381, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn gross_trade_amt(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::GROSS_TRADE_AMT)?;
        convert::parse_fix_decimal(&raw, tags::GROSS_TRADE_AMT)
    }

    /// Whether `GrossTradeAmt` (tag 381) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_gross_trade_amt(&self) -> bool {
        self.inner.has_field(tags::GROSS_TRADE_AMT)
    }

    /// `NumDaysInterest` (tag 157, optional).
    pub fn num_days_interest(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::NUM_DAYS_INTEREST)
    }

    /// Whether `NumDaysInterest` (tag 157) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_num_days_interest(&self) -> bool {
        self.inner.has_field(tags::NUM_DAYS_INTEREST)
    }

    /// `ExDate` (tag 230, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn ex_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::EX_DATE)?;
        convert::parse_fix_date(&raw, tags::EX_DATE)
    }

    /// Whether `ExDate` (tag 230) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ex_date(&self) -> bool {
        self.inner.has_field(tags::EX_DATE)
    }

    /// `AccruedInterestRate` (tag 158, optional).
    pub fn accrued_interest_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::ACCRUED_INTEREST_RATE)
    }

    /// Whether `AccruedInterestRate` (tag 158) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_accrued_interest_rate(&self) -> bool {
        self.inner.has_field(tags::ACCRUED_INTEREST_RATE)
    }

    /// `AccruedInterestAmt` (tag 159, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn accrued_interest_amt(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ACCRUED_INTEREST_AMT)?;
        convert::parse_fix_decimal(&raw, tags::ACCRUED_INTEREST_AMT)
    }

    /// Whether `AccruedInterestAmt` (tag 159) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_accrued_interest_amt(&self) -> bool {
        self.inner.has_field(tags::ACCRUED_INTEREST_AMT)
    }

    /// `Concession` (tag 238, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn concession(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::CONCESSION)?;
        convert::parse_fix_decimal(&raw, tags::CONCESSION)
    }

    /// Whether `Concession` (tag 238) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_concession(&self) -> bool {
        self.inner.has_field(tags::CONCESSION)
    }

    /// `TotalTakedown` (tag 237, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn total_takedown(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::TOTAL_TAKEDOWN)?;
        convert::parse_fix_decimal(&raw, tags::TOTAL_TAKEDOWN)
    }

    /// Whether `TotalTakedown` (tag 237) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_takedown(&self) -> bool {
        self.inner.has_field(tags::TOTAL_TAKEDOWN)
    }

    /// `NetMoney` (tag 118, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn net_money(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::NET_MONEY)?;
        convert::parse_fix_decimal(&raw, tags::NET_MONEY)
    }

    /// Whether `NetMoney` (tag 118) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_net_money(&self) -> bool {
        self.inner.has_field(tags::NET_MONEY)
    }

    /// `SettlCurrAmt` (tag 119, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn settl_curr_amt(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::SETTL_CURR_AMT)?;
        convert::parse_fix_decimal(&raw, tags::SETTL_CURR_AMT)
    }

    /// Whether `SettlCurrAmt` (tag 119) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_curr_amt(&self) -> bool {
        self.inner.has_field(tags::SETTL_CURR_AMT)
    }

    /// `SettlCurrency` (tag 120, optional).
    pub fn settl_currency(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SETTL_CURRENCY)
    }

    /// Whether `SettlCurrency` (tag 120) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_currency(&self) -> bool {
        self.inner.has_field(tags::SETTL_CURRENCY)
    }

    /// `SettlCurrFxRate` (tag 155, optional).
    pub fn settl_curr_fx_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::SETTL_CURR_FX_RATE)
    }

    /// Whether `SettlCurrFxRate` (tag 155) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_curr_fx_rate(&self) -> bool {
        self.inner.has_field(tags::SETTL_CURR_FX_RATE)
    }

    /// `SettlCurrFxRateCalc` (tag 156, optional).
    pub fn settl_curr_fx_rate_calc(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::SETTL_CURR_FX_RATE_CALC)
    }

    /// Whether `SettlCurrFxRateCalc` (tag 156) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_curr_fx_rate_calc(&self) -> bool {
        self.inner.has_field(tags::SETTL_CURR_FX_RATE_CALC)
    }

    /// `PositionEffect` (tag 77, optional).
    pub fn position_effect(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::POSITION_EFFECT)
    }

    /// Whether `PositionEffect` (tag 77) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_position_effect(&self) -> bool {
        self.inner.has_field(tags::POSITION_EFFECT)
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

    /// `MultiLegReportingType` (tag 442, optional).
    pub fn multi_leg_reporting_type(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::MULTI_LEG_REPORTING_TYPE)
    }

    /// Whether `MultiLegReportingType` (tag 442) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_multi_leg_reporting_type(&self) -> bool {
        self.inner.has_field(tags::MULTI_LEG_REPORTING_TYPE)
    }

    /// The `NoPartyIds` group (count tag `NO_PARTY_IDS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_party_ids(&self) -> Option<NoPartyIds<'_>> {
        self.inner.get_group(tags::NO_PARTY_IDS).map(|group| NoPartyIds { group })
    }

    /// The `NoClearingInstructions` group (count tag `NO_CLEARING_INSTRUCTIONS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_clearing_instructions(&self) -> Option<NoClearingInstructions<'_>> {
        self.inner
            .get_group(tags::NO_CLEARING_INSTRUCTIONS)
            .map(|group| NoClearingInstructions { group })
    }

    /// The `NoContAmts` group (count tag `NO_CONT_AMTS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_cont_amts(&self) -> Option<NoContAmts<'_>> {
        self.inner.get_group(tags::NO_CONT_AMTS).map(|group| NoContAmts { group })
    }

    /// The `NoMiscFees` group (count tag `NO_MISC_FEES`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_misc_fees(&self) -> Option<NoMiscFees<'_>> {
        self.inner.get_group(tags::NO_MISC_FEES).map(|group| NoMiscFees { group })
    }
}

/// One `NoSides` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoSidesEntryMut::as_ref`].
pub struct NoSidesEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoSidesEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoSidesEntry<'_> {
        NoSidesEntry { inner: self.inner }
    }

    /// Set `Side` (tag 54, required).
    pub fn set_side(&mut self, v: fields::Side) {
        self.inner.set_field(tags::SIDE, v.to_fix().to_string());
    }

    /// Set `OrderID` (tag 37, required).
    pub fn set_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::ORDER_ID, v.to_string());
    }

    /// Set `SecondaryOrderID` (tag 198, optional).
    pub fn set_secondary_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECONDARY_ORDER_ID, v.to_string());
    }

    /// Set `ClOrdID` (tag 11, optional).
    pub fn set_cl_ord_id(&mut self, v: &str) {
        self.inner.set_field(tags::CL_ORD_ID, v.to_string());
    }

    /// Set `Account` (tag 1, optional).
    pub fn set_account(&mut self, v: &str) {
        self.inner.set_field(tags::ACCOUNT, v.to_string());
    }

    /// Set `AccountType` (tag 581, optional).
    pub fn set_account_type(&mut self, v: fields::AccountType) {
        self.inner.set_field(tags::ACCOUNT_TYPE, v.to_fix().to_string());
    }

    /// Set `ProcessCode` (tag 81, optional).
    pub fn set_process_code(&mut self, v: fields::ProcessCode) {
        self.inner.set_field(tags::PROCESS_CODE, v.to_fix().to_string());
    }

    /// Set `OddLot` (tag 575, optional); written in FIX `BOOLEAN` format.
    pub fn set_odd_lot(&mut self, v: bool) {
        self.inner.set_field(tags::ODD_LOT, convert::format_fix_bool(v));
    }

    /// Set `ClearingFeeIndicator` (tag 635, optional).
    pub fn set_clearing_fee_indicator(&mut self, v: &str) {
        self.inner.set_field(tags::CLEARING_FEE_INDICATOR, v.to_string());
    }

    /// Set `TradeInputSource` (tag 578, optional).
    pub fn set_trade_input_source(&mut self, v: &str) {
        self.inner.set_field(tags::TRADE_INPUT_SOURCE, v.to_string());
    }

    /// Set `TradeInputDevice` (tag 579, optional).
    pub fn set_trade_input_device(&mut self, v: &str) {
        self.inner.set_field(tags::TRADE_INPUT_DEVICE, v.to_string());
    }

    /// Set `Currency` (tag 15, optional).
    pub fn set_currency(&mut self, v: &str) {
        self.inner.set_field(tags::CURRENCY, v.to_string());
    }

    /// Set `ComplianceID` (tag 376, optional).
    pub fn set_compliance_id(&mut self, v: &str) {
        self.inner.set_field(tags::COMPLIANCE_ID, v.to_string());
    }

    /// Set `SolicitedFlag` (tag 377, optional); written in FIX `BOOLEAN` format.
    pub fn set_solicited_flag(&mut self, v: bool) {
        self.inner.set_field(tags::SOLICITED_FLAG, convert::format_fix_bool(v));
    }

    /// Set `OrderCapacity` (tag 528, optional).
    pub fn set_order_capacity(&mut self, v: fields::OrderCapacity) {
        self.inner.set_field(tags::ORDER_CAPACITY, v.to_fix().to_string());
    }

    /// Set `OrderRestrictions` (tag 529, optional).
    pub fn set_order_restrictions(&mut self, v: &str) {
        self.inner.set_field(tags::ORDER_RESTRICTIONS, v.to_string());
    }

    /// Set `CustOrderCapacity` (tag 582, optional).
    pub fn set_cust_order_capacity(&mut self, v: i32) {
        self.inner.set_field(tags::CUST_ORDER_CAPACITY, v.to_string());
    }

    /// Set `TransBkdTime` (tag 483, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_trans_bkd_time(&mut self, v: DateTime<Utc>) {
        self.inner.set_field(tags::TRANS_BKD_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
    }

    /// Set `Commission` (tag 12, optional); written in FIX `AMT` format.
    pub fn set_commission(&mut self, v: Decimal) {
        self.inner.set_field(tags::COMMISSION, convert::format_fix_decimal(v));
    }

    /// Set `CommType` (tag 13, optional).
    pub fn set_comm_type(&mut self, v: fields::CommType) {
        self.inner.set_field(tags::COMM_TYPE, v.to_fix().to_string());
    }

    /// Set `CommCurrency` (tag 479, optional).
    pub fn set_comm_currency(&mut self, v: &str) {
        self.inner.set_field(tags::COMM_CURRENCY, v.to_string());
    }

    /// Set `FundRenewWaiv` (tag 497, optional).
    pub fn set_fund_renew_waiv(&mut self, v: char) {
        self.inner.set_field(tags::FUND_RENEW_WAIV, v.to_string());
    }

    /// Set `GrossTradeAmt` (tag 381, optional); written in FIX `AMT` format.
    pub fn set_gross_trade_amt(&mut self, v: Decimal) {
        self.inner.set_field(tags::GROSS_TRADE_AMT, convert::format_fix_decimal(v));
    }

    /// Set `NumDaysInterest` (tag 157, optional).
    pub fn set_num_days_interest(&mut self, v: i32) {
        self.inner.set_field(tags::NUM_DAYS_INTEREST, v.to_string());
    }

    /// Set `ExDate` (tag 230, optional); written in FIX `UTCDATE` format.
    pub fn set_ex_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::EX_DATE, convert::format_fix_date(v));
    }

    /// Set `AccruedInterestRate` (tag 158, optional).
    pub fn set_accrued_interest_rate(&mut self, v: f64) {
        self.inner.set_field(tags::ACCRUED_INTEREST_RATE, v.to_string());
    }

    /// Set `AccruedInterestAmt` (tag 159, optional); written in FIX `AMT` format.
    pub fn set_accrued_interest_amt(&mut self, v: Decimal) {
        self.inner.set_field(tags::ACCRUED_INTEREST_AMT, convert::format_fix_decimal(v));
    }

    /// Set `Concession` (tag 238, optional); written in FIX `AMT` format.
    pub fn set_concession(&mut self, v: Decimal) {
        self.inner.set_field(tags::CONCESSION, convert::format_fix_decimal(v));
    }

    /// Set `TotalTakedown` (tag 237, optional); written in FIX `AMT` format.
    pub fn set_total_takedown(&mut self, v: Decimal) {
        self.inner.set_field(tags::TOTAL_TAKEDOWN, convert::format_fix_decimal(v));
    }

    /// Set `NetMoney` (tag 118, optional); written in FIX `AMT` format.
    pub fn set_net_money(&mut self, v: Decimal) {
        self.inner.set_field(tags::NET_MONEY, convert::format_fix_decimal(v));
    }

    /// Set `SettlCurrAmt` (tag 119, optional); written in FIX `AMT` format.
    pub fn set_settl_curr_amt(&mut self, v: Decimal) {
        self.inner.set_field(tags::SETTL_CURR_AMT, convert::format_fix_decimal(v));
    }

    /// Set `SettlCurrency` (tag 120, optional).
    pub fn set_settl_currency(&mut self, v: &str) {
        self.inner.set_field(tags::SETTL_CURRENCY, v.to_string());
    }

    /// Set `SettlCurrFxRate` (tag 155, optional).
    pub fn set_settl_curr_fx_rate(&mut self, v: f64) {
        self.inner.set_field(tags::SETTL_CURR_FX_RATE, v.to_string());
    }

    /// Set `SettlCurrFxRateCalc` (tag 156, optional).
    pub fn set_settl_curr_fx_rate_calc(&mut self, v: char) {
        self.inner.set_field(tags::SETTL_CURR_FX_RATE_CALC, v.to_string());
    }

    /// Set `PositionEffect` (tag 77, optional).
    pub fn set_position_effect(&mut self, v: char) {
        self.inner.set_field(tags::POSITION_EFFECT, v.to_string());
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

    /// Set `MultiLegReportingType` (tag 442, optional).
    pub fn set_multi_leg_reporting_type(&mut self, v: char) {
        self.inner.set_field(tags::MULTI_LEG_REPORTING_TYPE, v.to_string());
    }

    /// The `NoPartyIds` group (count tag `NO_PARTY_IDS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_party_ids_mut(&mut self) -> Option<NoPartyIdsMut<'_>> {
        self.inner.get_group_mut(tags::NO_PARTY_IDS).map(|group| NoPartyIdsMut { group })
    }

    /// Create the `NoPartyIds` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_party_ids_entry`] on the same group.
    pub fn set_no_party_ids(&mut self, count: u32) -> NoPartyIdsMut<'_> {
        let group = self.inner.set_group(tags::NO_PARTY_IDS, count, NO_PARTY_IDS_FIELD_ORDER);
        NoPartyIdsMut { group }
    }

    /// Append one `NoPartyIds` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_party_ids`] on the same group.
    pub fn add_no_party_ids_entry(&mut self, fill: impl FnOnce(&mut NoPartyIdsEntryMut<'_>)) {
        let inner = self.inner.add_group_instance(tags::NO_PARTY_IDS, NO_PARTY_IDS_FIELD_ORDER);
        fill(&mut NoPartyIdsEntryMut { inner });
    }

    /// The `NoClearingInstructions` group (count tag `NO_CLEARING_INSTRUCTIONS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_clearing_instructions_mut(&mut self) -> Option<NoClearingInstructionsMut<'_>> {
        self.inner
            .get_group_mut(tags::NO_CLEARING_INSTRUCTIONS)
            .map(|group| NoClearingInstructionsMut { group })
    }

    /// Create the `NoClearingInstructions` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_clearing_instructions_entry`] on the same group.
    pub fn set_no_clearing_instructions(&mut self, count: u32) -> NoClearingInstructionsMut<'_> {
        let group = self.inner.set_group(
            tags::NO_CLEARING_INSTRUCTIONS,
            count,
            NO_CLEARING_INSTRUCTIONS_FIELD_ORDER,
        );
        NoClearingInstructionsMut { group }
    }

    /// Append one `NoClearingInstructions` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_clearing_instructions`] on the same group.
    pub fn add_no_clearing_instructions_entry(
        &mut self,
        fill: impl FnOnce(&mut NoClearingInstructionsEntryMut<'_>),
    ) {
        let inner = self.inner.add_group_instance(
            tags::NO_CLEARING_INSTRUCTIONS,
            NO_CLEARING_INSTRUCTIONS_FIELD_ORDER,
        );
        fill(&mut NoClearingInstructionsEntryMut { inner });
    }

    /// The `NoContAmts` group (count tag `NO_CONT_AMTS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_cont_amts_mut(&mut self) -> Option<NoContAmtsMut<'_>> {
        self.inner.get_group_mut(tags::NO_CONT_AMTS).map(|group| NoContAmtsMut { group })
    }

    /// Create the `NoContAmts` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_cont_amts_entry`] on the same group.
    pub fn set_no_cont_amts(&mut self, count: u32) -> NoContAmtsMut<'_> {
        let group = self.inner.set_group(tags::NO_CONT_AMTS, count, NO_CONT_AMTS_FIELD_ORDER);
        NoContAmtsMut { group }
    }

    /// Append one `NoContAmts` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_cont_amts`] on the same group.
    pub fn add_no_cont_amts_entry(&mut self, fill: impl FnOnce(&mut NoContAmtsEntryMut<'_>)) {
        let inner = self.inner.add_group_instance(tags::NO_CONT_AMTS, NO_CONT_AMTS_FIELD_ORDER);
        fill(&mut NoContAmtsEntryMut { inner });
    }

    /// The `NoMiscFees` group (count tag `NO_MISC_FEES`, optional) for editing, or `None` if
    /// absent.
    pub fn no_misc_fees_mut(&mut self) -> Option<NoMiscFeesMut<'_>> {
        self.inner.get_group_mut(tags::NO_MISC_FEES).map(|group| NoMiscFeesMut { group })
    }

    /// Create the `NoMiscFees` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_misc_fees_entry`] on the same group.
    pub fn set_no_misc_fees(&mut self, count: u32) -> NoMiscFeesMut<'_> {
        let group = self.inner.set_group(tags::NO_MISC_FEES, count, NO_MISC_FEES_FIELD_ORDER);
        NoMiscFeesMut { group }
    }

    /// Append one `NoMiscFees` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_misc_fees`] on the same group.
    pub fn add_no_misc_fees_entry(&mut self, fill: impl FnOnce(&mut NoMiscFeesEntryMut<'_>)) {
        let inner = self.inner.add_group_instance(tags::NO_MISC_FEES, NO_MISC_FEES_FIELD_ORDER);
        fill(&mut NoMiscFeesEntryMut { inner });
    }
}

/// The `NoSides` repeating group (count tag `NO_SIDES`), read-only: its
/// instances as [`NoSidesEntry`] views.
pub struct NoSides<'a> {
    group: &'a Group,
}

impl<'a> NoSides<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoSidesEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoSidesEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoSidesEntry<'a> {
        self.get(i).expect("NoSides index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoSidesEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoSidesEntry { inner })
    }
}

/// The `NoSides` repeating group (count tag `NO_SIDES`), mutable: its
/// instances as [`NoSidesEntryMut`] views.
pub struct NoSidesMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoSidesMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoSidesEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoSidesEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoSidesEntryMut<'_> {
        self.get_mut(i).expect("NoSides index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoSidesEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoSidesEntryMut { inner })
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

/// Wire order of a `NoClearingInstructions` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_CLEARING_INSTRUCTIONS_FIELD_ORDER: &[u32] = &[tags::CLEARING_INSTRUCTION];

/// One `NoClearingInstructions` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoClearingInstructionsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoClearingInstructionsEntry<'_> {
    /// `ClearingInstruction` (tag 577, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ClearingInstruction`]'s.
    pub fn clearing_instruction(&self) -> Result<fields::ClearingInstruction, FieldError> {
        let n = self.inner.get_field::<i32>(tags::CLEARING_INSTRUCTION)?;
        fields::ClearingInstruction::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::CLEARING_INSTRUCTION,
        })
    }

    /// Whether `ClearingInstruction` (tag 577) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_clearing_instruction(&self) -> bool {
        self.inner.has_field(tags::CLEARING_INSTRUCTION)
    }
}

/// One `NoClearingInstructions` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoClearingInstructionsEntryMut::as_ref`].
pub struct NoClearingInstructionsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoClearingInstructionsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoClearingInstructionsEntry<'_> {
        NoClearingInstructionsEntry { inner: self.inner }
    }

    /// Set `ClearingInstruction` (tag 577, optional).
    pub fn set_clearing_instruction(&mut self, v: fields::ClearingInstruction) {
        self.inner.set_field(tags::CLEARING_INSTRUCTION, v.to_fix().to_string());
    }
}

/// The `NoClearingInstructions` repeating group (count tag `NO_CLEARING_INSTRUCTIONS`), read-only: its
/// instances as [`NoClearingInstructionsEntry`] views.
pub struct NoClearingInstructions<'a> {
    group: &'a Group,
}

impl<'a> NoClearingInstructions<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoClearingInstructionsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoClearingInstructionsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoClearingInstructionsEntry<'a> {
        self.get(i).expect("NoClearingInstructions index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoClearingInstructionsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoClearingInstructionsEntry { inner })
    }
}

/// The `NoClearingInstructions` repeating group (count tag `NO_CLEARING_INSTRUCTIONS`), mutable: its
/// instances as [`NoClearingInstructionsEntryMut`] views.
pub struct NoClearingInstructionsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoClearingInstructionsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoClearingInstructionsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoClearingInstructionsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoClearingInstructionsEntryMut<'_> {
        self.get_mut(i).expect("NoClearingInstructions index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoClearingInstructionsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoClearingInstructionsEntryMut { inner })
    }
}

/// Wire order of a `NoContAmts` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_CONT_AMTS_FIELD_ORDER: &[u32] = &[
    tags::CONT_AMT_TYPE,
    tags::CONT_AMT_VALUE,
    tags::CONT_AMT_CURR,
];

/// One `NoContAmts` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoContAmtsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoContAmtsEntry<'_> {
    /// `ContAmtType` (tag 519, optional).
    pub fn cont_amt_type(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::CONT_AMT_TYPE)
    }

    /// Whether `ContAmtType` (tag 519) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cont_amt_type(&self) -> bool {
        self.inner.has_field(tags::CONT_AMT_TYPE)
    }

    /// `ContAmtValue` (tag 520, optional).
    pub fn cont_amt_value(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::CONT_AMT_VALUE)
    }

    /// Whether `ContAmtValue` (tag 520) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cont_amt_value(&self) -> bool {
        self.inner.has_field(tags::CONT_AMT_VALUE)
    }

    /// `ContAmtCurr` (tag 521, optional).
    pub fn cont_amt_curr(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CONT_AMT_CURR)
    }

    /// Whether `ContAmtCurr` (tag 521) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cont_amt_curr(&self) -> bool {
        self.inner.has_field(tags::CONT_AMT_CURR)
    }
}

/// One `NoContAmts` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoContAmtsEntryMut::as_ref`].
pub struct NoContAmtsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoContAmtsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoContAmtsEntry<'_> {
        NoContAmtsEntry { inner: self.inner }
    }

    /// Set `ContAmtType` (tag 519, optional).
    pub fn set_cont_amt_type(&mut self, v: i32) {
        self.inner.set_field(tags::CONT_AMT_TYPE, v.to_string());
    }

    /// Set `ContAmtValue` (tag 520, optional).
    pub fn set_cont_amt_value(&mut self, v: f64) {
        self.inner.set_field(tags::CONT_AMT_VALUE, v.to_string());
    }

    /// Set `ContAmtCurr` (tag 521, optional).
    pub fn set_cont_amt_curr(&mut self, v: &str) {
        self.inner.set_field(tags::CONT_AMT_CURR, v.to_string());
    }
}

/// The `NoContAmts` repeating group (count tag `NO_CONT_AMTS`), read-only: its
/// instances as [`NoContAmtsEntry`] views.
pub struct NoContAmts<'a> {
    group: &'a Group,
}

impl<'a> NoContAmts<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoContAmtsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoContAmtsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoContAmtsEntry<'a> {
        self.get(i).expect("NoContAmts index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoContAmtsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoContAmtsEntry { inner })
    }
}

/// The `NoContAmts` repeating group (count tag `NO_CONT_AMTS`), mutable: its
/// instances as [`NoContAmtsEntryMut`] views.
pub struct NoContAmtsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoContAmtsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoContAmtsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoContAmtsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoContAmtsEntryMut<'_> {
        self.get_mut(i).expect("NoContAmts index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoContAmtsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoContAmtsEntryMut { inner })
    }
}

/// Wire order of a `NoMiscFees` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_MISC_FEES_FIELD_ORDER: &[u32] =
    &[tags::MISC_FEE_AMT, tags::MISC_FEE_CURR, tags::MISC_FEE_TYPE];

/// One `NoMiscFees` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoMiscFeesEntry<'a> {
    inner: &'a FieldMap,
}

impl NoMiscFeesEntry<'_> {
    /// `MiscFeeAmt` (tag 137, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn misc_fee_amt(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MISC_FEE_AMT)?;
        convert::parse_fix_decimal(&raw, tags::MISC_FEE_AMT)
    }

    /// Whether `MiscFeeAmt` (tag 137) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_misc_fee_amt(&self) -> bool {
        self.inner.has_field(tags::MISC_FEE_AMT)
    }

    /// `MiscFeeCurr` (tag 138, optional).
    pub fn misc_fee_curr(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MISC_FEE_CURR)
    }

    /// Whether `MiscFeeCurr` (tag 138) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_misc_fee_curr(&self) -> bool {
        self.inner.has_field(tags::MISC_FEE_CURR)
    }

    /// `MiscFeeType` (tag 139, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MiscFeeType`]'s.
    pub fn misc_fee_type(&self) -> Result<fields::MiscFeeType, FieldError> {
        let n = self.inner.get_field::<char>(tags::MISC_FEE_TYPE)?;
        fields::MiscFeeType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MISC_FEE_TYPE,
        })
    }

    /// Whether `MiscFeeType` (tag 139) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_misc_fee_type(&self) -> bool {
        self.inner.has_field(tags::MISC_FEE_TYPE)
    }
}

/// One `NoMiscFees` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoMiscFeesEntryMut::as_ref`].
pub struct NoMiscFeesEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoMiscFeesEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoMiscFeesEntry<'_> {
        NoMiscFeesEntry { inner: self.inner }
    }

    /// Set `MiscFeeAmt` (tag 137, optional); written in FIX `AMT` format.
    pub fn set_misc_fee_amt(&mut self, v: Decimal) {
        self.inner.set_field(tags::MISC_FEE_AMT, convert::format_fix_decimal(v));
    }

    /// Set `MiscFeeCurr` (tag 138, optional).
    pub fn set_misc_fee_curr(&mut self, v: &str) {
        self.inner.set_field(tags::MISC_FEE_CURR, v.to_string());
    }

    /// Set `MiscFeeType` (tag 139, optional).
    pub fn set_misc_fee_type(&mut self, v: fields::MiscFeeType) {
        self.inner.set_field(tags::MISC_FEE_TYPE, v.to_fix().to_string());
    }
}

/// The `NoMiscFees` repeating group (count tag `NO_MISC_FEES`), read-only: its
/// instances as [`NoMiscFeesEntry`] views.
pub struct NoMiscFees<'a> {
    group: &'a Group,
}

impl<'a> NoMiscFees<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoMiscFeesEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoMiscFeesEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoMiscFeesEntry<'a> {
        self.get(i).expect("NoMiscFees index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoMiscFeesEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoMiscFeesEntry { inner })
    }
}

/// The `NoMiscFees` repeating group (count tag `NO_MISC_FEES`), mutable: its
/// instances as [`NoMiscFeesEntryMut`] views.
pub struct NoMiscFeesMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoMiscFeesMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoMiscFeesEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoMiscFeesEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoMiscFeesEntryMut<'_> {
        self.get_mut(i).expect("NoMiscFees index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoMiscFeesEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoMiscFeesEntryMut { inner })
    }
}
