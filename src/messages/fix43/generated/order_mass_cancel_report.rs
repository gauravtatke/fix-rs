//! `OrderMassCancelReport` (app message, MsgType `35=r`): typed facade over the raw
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

/// Typed facade over a `OrderMassCancelReport` [`Message`] (35=r). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct OrderMassCancelReport {
    msg: Message,
}

impl Default for OrderMassCancelReport {
    fn default() -> Self {
        OrderMassCancelReport::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `OrderMassCancelReport`, checking the msg type.
impl TryFrom<Message> for OrderMassCancelReport {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a OrderMassCancelReport" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<OrderMassCancelReport> for Message {
    fn from(typed_msg: OrderMassCancelReport) -> Self {
        typed_msg.msg
    }
}

impl OrderMassCancelReport {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "r";

    /// An empty `OrderMassCancelReport` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- ClOrdID (tag 11, STRING, optional) ---

    /// Set `ClOrdID` (tag 11, optional).
    pub fn set_cl_ord_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CL_ORD_ID, v.to_string());
    }

    /// `ClOrdID` (tag 11, optional).
    pub fn cl_ord_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CL_ORD_ID)
    }

    /// Whether `ClOrdID` (tag 11) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cl_ord_id(&self) -> bool {
        self.msg.body().has_field(tags::CL_ORD_ID)
    }

    // --- SecondaryClOrdID (tag 526, STRING, optional) ---

    /// Set `SecondaryClOrdID` (tag 526, optional).
    pub fn set_secondary_cl_ord_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECONDARY_CL_ORD_ID, v.to_string());
    }

    /// `SecondaryClOrdID` (tag 526, optional).
    pub fn secondary_cl_ord_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECONDARY_CL_ORD_ID)
    }

    /// Whether `SecondaryClOrdID` (tag 526) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_cl_ord_id(&self) -> bool {
        self.msg.body().has_field(tags::SECONDARY_CL_ORD_ID)
    }

    // --- OrderID (tag 37, STRING, required) ---

    /// Set `OrderID` (tag 37, required).
    pub fn set_order_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ORDER_ID, v.to_string());
    }

    /// `OrderID` (tag 37, required).
    pub fn order_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ORDER_ID)
    }

    /// Whether `OrderID` (tag 37) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_id(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_ID)
    }

    // --- SecondaryOrderID (tag 198, STRING, optional) ---

    /// Set `SecondaryOrderID` (tag 198, optional).
    pub fn set_secondary_order_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECONDARY_ORDER_ID, v.to_string());
    }

    /// `SecondaryOrderID` (tag 198, optional).
    pub fn secondary_order_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECONDARY_ORDER_ID)
    }

    /// Whether `SecondaryOrderID` (tag 198) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_order_id(&self) -> bool {
        self.msg.body().has_field(tags::SECONDARY_ORDER_ID)
    }

    // --- MassCancelRequestType (tag 530, CHAR, required) ---

    /// Set `MassCancelRequestType` (tag 530, required).
    pub fn set_mass_cancel_request_type(&mut self, v: fields::MassCancelRequestType) {
        self.msg.body_mut().set_field(tags::MASS_CANCEL_REQUEST_TYPE, v.to_fix().to_string());
    }

    /// `MassCancelRequestType` (tag 530, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MassCancelRequestType`]'s.
    pub fn mass_cancel_request_type(&self) -> Result<fields::MassCancelRequestType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::MASS_CANCEL_REQUEST_TYPE)?;
        fields::MassCancelRequestType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MASS_CANCEL_REQUEST_TYPE,
        })
    }

    /// Whether `MassCancelRequestType` (tag 530) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mass_cancel_request_type(&self) -> bool {
        self.msg.body().has_field(tags::MASS_CANCEL_REQUEST_TYPE)
    }

    // --- MassCancelResponse (tag 531, CHAR, required) ---

    /// Set `MassCancelResponse` (tag 531, required).
    pub fn set_mass_cancel_response(&mut self, v: fields::MassCancelResponse) {
        self.msg.body_mut().set_field(tags::MASS_CANCEL_RESPONSE, v.to_fix().to_string());
    }

    /// `MassCancelResponse` (tag 531, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MassCancelResponse`]'s.
    pub fn mass_cancel_response(&self) -> Result<fields::MassCancelResponse, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::MASS_CANCEL_RESPONSE)?;
        fields::MassCancelResponse::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MASS_CANCEL_RESPONSE,
        })
    }

    /// Whether `MassCancelResponse` (tag 531) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mass_cancel_response(&self) -> bool {
        self.msg.body().has_field(tags::MASS_CANCEL_RESPONSE)
    }

    // --- MassCancelRejectReason (tag 532, CHAR, optional) ---

    /// Set `MassCancelRejectReason` (tag 532, optional).
    pub fn set_mass_cancel_reject_reason(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::MASS_CANCEL_REJECT_REASON, v.to_string());
    }

    /// `MassCancelRejectReason` (tag 532, optional).
    pub fn mass_cancel_reject_reason(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::MASS_CANCEL_REJECT_REASON)
    }

    /// Whether `MassCancelRejectReason` (tag 532) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mass_cancel_reject_reason(&self) -> bool {
        self.msg.body().has_field(tags::MASS_CANCEL_REJECT_REASON)
    }

    // --- TotalAffectedOrders (tag 533, INT, optional) ---

    /// Set `TotalAffectedOrders` (tag 533, optional).
    pub fn set_total_affected_orders(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::TOTAL_AFFECTED_ORDERS, v.to_string());
    }

    /// `TotalAffectedOrders` (tag 533, optional).
    pub fn total_affected_orders(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::TOTAL_AFFECTED_ORDERS)
    }

    /// Whether `TotalAffectedOrders` (tag 533) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_affected_orders(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_AFFECTED_ORDERS)
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

    // --- Side (tag 54, CHAR, optional) ---

    /// Set `Side` (tag 54, optional).
    pub fn set_side(&mut self, v: fields::Side) {
        self.msg.body_mut().set_field(tags::SIDE, v.to_fix().to_string());
    }

    /// `Side` (tag 54, optional). `Err(InvalidFormat)` if the value isn't one
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

    // --- NoAffectedOrders group (count tag NO_AFFECTED_ORDERS) ---

    /// The `NoAffectedOrders` group (count tag `NO_AFFECTED_ORDERS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_affected_orders(&self) -> Option<NoAffectedOrders<'_>> {
        self.msg.body().get_group(tags::NO_AFFECTED_ORDERS).map(|group| NoAffectedOrders { group })
    }

    /// The `NoAffectedOrders` group (count tag `NO_AFFECTED_ORDERS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_affected_orders_mut(&mut self) -> Option<NoAffectedOrdersMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_AFFECTED_ORDERS)
            .map(|group| NoAffectedOrdersMut { group })
    }

    /// Create the `NoAffectedOrders` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_affected_orders_entry`] on the same group.
    pub fn set_no_affected_orders(&mut self, count: u32) -> NoAffectedOrdersMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_AFFECTED_ORDERS,
            count,
            NO_AFFECTED_ORDERS_FIELD_ORDER,
        );
        NoAffectedOrdersMut { group }
    }

    /// Append one `NoAffectedOrders` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_affected_orders`] on the same group.
    pub fn add_no_affected_orders_entry(
        &mut self,
        fill: impl FnOnce(&mut NoAffectedOrdersEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_AFFECTED_ORDERS, NO_AFFECTED_ORDERS_FIELD_ORDER);
        fill(&mut NoAffectedOrdersEntryMut { inner });
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

    /// Adopt `msg` without checking its MsgType: for the version's message enum, whose
    /// `From<Message>` has already matched 35 before wrapping. `pub(super)` keeps it inside
    /// `generated/`; everything else goes through the checked `TryFrom<Message>`.
    pub(super) fn wrap(msg: Message) -> Self {
        Self { msg }
    }
}

/// Wire order of a `NoAffectedOrders` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_AFFECTED_ORDERS_FIELD_ORDER: &[u32] = &[
    tags::ORIG_CL_ORD_ID,
    tags::AFFECTED_ORDER_ID,
    tags::AFFECTED_SECONDARY_ORDER_ID,
];

/// One `NoAffectedOrders` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoAffectedOrdersEntry<'a> {
    inner: &'a FieldMap,
}

impl NoAffectedOrdersEntry<'_> {
    /// `OrigClOrdID` (tag 41, optional).
    pub fn orig_cl_ord_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ORIG_CL_ORD_ID)
    }

    /// Whether `OrigClOrdID` (tag 41) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_orig_cl_ord_id(&self) -> bool {
        self.inner.has_field(tags::ORIG_CL_ORD_ID)
    }

    /// `AffectedOrderID` (tag 535, optional).
    pub fn affected_order_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::AFFECTED_ORDER_ID)
    }

    /// Whether `AffectedOrderID` (tag 535) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_affected_order_id(&self) -> bool {
        self.inner.has_field(tags::AFFECTED_ORDER_ID)
    }

    /// `AffectedSecondaryOrderID` (tag 536, optional).
    pub fn affected_secondary_order_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::AFFECTED_SECONDARY_ORDER_ID)
    }

    /// Whether `AffectedSecondaryOrderID` (tag 536) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_affected_secondary_order_id(&self) -> bool {
        self.inner.has_field(tags::AFFECTED_SECONDARY_ORDER_ID)
    }
}

/// One `NoAffectedOrders` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoAffectedOrdersEntryMut::as_ref`].
pub struct NoAffectedOrdersEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoAffectedOrdersEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoAffectedOrdersEntry<'_> {
        NoAffectedOrdersEntry { inner: self.inner }
    }

    /// Set `OrigClOrdID` (tag 41, optional).
    pub fn set_orig_cl_ord_id(&mut self, v: &str) {
        self.inner.set_field(tags::ORIG_CL_ORD_ID, v.to_string());
    }

    /// Set `AffectedOrderID` (tag 535, optional).
    pub fn set_affected_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::AFFECTED_ORDER_ID, v.to_string());
    }

    /// Set `AffectedSecondaryOrderID` (tag 536, optional).
    pub fn set_affected_secondary_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::AFFECTED_SECONDARY_ORDER_ID, v.to_string());
    }
}

/// The `NoAffectedOrders` repeating group (count tag `NO_AFFECTED_ORDERS`), read-only: its
/// instances as [`NoAffectedOrdersEntry`] views.
pub struct NoAffectedOrders<'a> {
    group: &'a Group,
}

impl<'a> NoAffectedOrders<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoAffectedOrdersEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoAffectedOrdersEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoAffectedOrdersEntry<'a> {
        self.get(i).expect("NoAffectedOrders index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoAffectedOrdersEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoAffectedOrdersEntry { inner })
    }
}

/// The `NoAffectedOrders` repeating group (count tag `NO_AFFECTED_ORDERS`), mutable: its
/// instances as [`NoAffectedOrdersEntryMut`] views.
pub struct NoAffectedOrdersMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoAffectedOrdersMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoAffectedOrdersEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoAffectedOrdersEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoAffectedOrdersEntryMut<'_> {
        self.get_mut(i).expect("NoAffectedOrders index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoAffectedOrdersEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoAffectedOrdersEntryMut { inner })
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
