//! `Allocation` (app message, MsgType `35=J`): typed facade over the raw
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

/// Typed facade over a `Allocation` [`Message`] (35=J). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct Allocation {
    msg: Message,
}

impl Default for Allocation {
    fn default() -> Self {
        Allocation::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `Allocation`, checking the msg type.
impl TryFrom<Message> for Allocation {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a Allocation" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<Allocation> for Message {
    fn from(typed_msg: Allocation) -> Self {
        typed_msg.msg
    }
}

impl Allocation {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "J";

    /// An empty `Allocation` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- AllocID (tag 70, STRING, required) ---

    /// Set `AllocID` (tag 70, required).
    pub fn set_alloc_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ALLOC_ID, v.to_string());
    }

    /// `AllocID` (tag 70, required).
    pub fn alloc_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ALLOC_ID)
    }

    /// Whether `AllocID` (tag 70) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_id(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_ID)
    }

    // --- AllocTransType (tag 71, CHAR, required) ---

    /// Set `AllocTransType` (tag 71, required).
    pub fn set_alloc_trans_type(&mut self, v: fields::AllocTransType) {
        self.msg.body_mut().set_field(tags::ALLOC_TRANS_TYPE, v.to_fix().to_string());
    }

    /// `AllocTransType` (tag 71, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::AllocTransType`]'s.
    pub fn alloc_trans_type(&self) -> Result<fields::AllocTransType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::ALLOC_TRANS_TYPE)?;
        fields::AllocTransType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ALLOC_TRANS_TYPE,
        })
    }

    /// Whether `AllocTransType` (tag 71) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_trans_type(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_TRANS_TYPE)
    }

    // --- AllocType (tag 626, INT, required) ---

    /// Set `AllocType` (tag 626, required).
    pub fn set_alloc_type(&mut self, v: fields::AllocType) {
        self.msg.body_mut().set_field(tags::ALLOC_TYPE, v.to_fix().to_string());
    }

    /// `AllocType` (tag 626, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::AllocType`]'s.
    pub fn alloc_type(&self) -> Result<fields::AllocType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::ALLOC_TYPE)?;
        fields::AllocType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ALLOC_TYPE,
        })
    }

    /// Whether `AllocType` (tag 626) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_type(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_TYPE)
    }

    // --- RefAllocID (tag 72, STRING, optional) ---

    /// Set `RefAllocID` (tag 72, optional).
    pub fn set_ref_alloc_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REF_ALLOC_ID, v.to_string());
    }

    /// `RefAllocID` (tag 72, optional).
    pub fn ref_alloc_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REF_ALLOC_ID)
    }

    /// Whether `RefAllocID` (tag 72) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ref_alloc_id(&self) -> bool {
        self.msg.body().has_field(tags::REF_ALLOC_ID)
    }

    // --- AllocLinkID (tag 196, STRING, optional) ---

    /// Set `AllocLinkID` (tag 196, optional).
    pub fn set_alloc_link_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ALLOC_LINK_ID, v.to_string());
    }

    /// `AllocLinkID` (tag 196, optional).
    pub fn alloc_link_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ALLOC_LINK_ID)
    }

    /// Whether `AllocLinkID` (tag 196) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_link_id(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_LINK_ID)
    }

    // --- AllocLinkType (tag 197, INT, optional) ---

    /// Set `AllocLinkType` (tag 197, optional).
    pub fn set_alloc_link_type(&mut self, v: fields::AllocLinkType) {
        self.msg.body_mut().set_field(tags::ALLOC_LINK_TYPE, v.to_fix().to_string());
    }

    /// `AllocLinkType` (tag 197, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::AllocLinkType`]'s.
    pub fn alloc_link_type(&self) -> Result<fields::AllocLinkType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::ALLOC_LINK_TYPE)?;
        fields::AllocLinkType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ALLOC_LINK_TYPE,
        })
    }

    /// Whether `AllocLinkType` (tag 197) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_link_type(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_LINK_TYPE)
    }

    // --- BookingRefID (tag 466, STRING, optional) ---

    /// Set `BookingRefID` (tag 466, optional).
    pub fn set_booking_ref_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BOOKING_REF_ID, v.to_string());
    }

    /// `BookingRefID` (tag 466, optional).
    pub fn booking_ref_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BOOKING_REF_ID)
    }

    /// Whether `BookingRefID` (tag 466) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_booking_ref_id(&self) -> bool {
        self.msg.body().has_field(tags::BOOKING_REF_ID)
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

    // --- Quantity (tag 53, QTY, required) ---

    /// Set `Quantity` (tag 53, required); written in FIX `QTY` format.
    pub fn set_quantity(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::QUANTITY, convert::format_fix_decimal(v));
    }

    /// `Quantity` (tag 53, required). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn quantity(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::QUANTITY)?;
        convert::parse_fix_decimal(&raw, tags::QUANTITY)
    }

    /// Whether `Quantity` (tag 53) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quantity(&self) -> bool {
        self.msg.body().has_field(tags::QUANTITY)
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

    // --- TradeOriginationDate (tag 229, UTCDATE, optional) ---

    /// Set `TradeOriginationDate` (tag 229, optional); written in FIX `UTCDATE` format.
    pub fn set_trade_origination_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::TRADE_ORIGINATION_DATE, convert::format_fix_date(v));
    }

    /// `TradeOriginationDate` (tag 229, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn trade_origination_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRADE_ORIGINATION_DATE)?;
        convert::parse_fix_date(&raw, tags::TRADE_ORIGINATION_DATE)
    }

    /// Whether `TradeOriginationDate` (tag 229) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_origination_date(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_ORIGINATION_DATE)
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

    // --- AvgPx (tag 6, PRICE, required) ---

    /// Set `AvgPx` (tag 6, required); written in FIX `PRICE` format.
    pub fn set_avg_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::AVG_PX, convert::format_fix_decimal(v));
    }

    /// `AvgPx` (tag 6, required). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn avg_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::AVG_PX)?;
        convert::parse_fix_decimal(&raw, tags::AVG_PX)
    }

    /// Whether `AvgPx` (tag 6) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_avg_px(&self) -> bool {
        self.msg.body().has_field(tags::AVG_PX)
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

    // --- AvgPrxPrecision (tag 74, INT, optional) ---

    /// Set `AvgPrxPrecision` (tag 74, optional).
    pub fn set_avg_prx_precision(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::AVG_PRX_PRECISION, v.to_string());
    }

    /// `AvgPrxPrecision` (tag 74, optional).
    pub fn avg_prx_precision(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::AVG_PRX_PRECISION)
    }

    /// Whether `AvgPrxPrecision` (tag 74) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_avg_prx_precision(&self) -> bool {
        self.msg.body().has_field(tags::AVG_PRX_PRECISION)
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

    // --- GrossTradeAmt (tag 381, AMT, optional) ---

    /// Set `GrossTradeAmt` (tag 381, optional); written in FIX `AMT` format.
    pub fn set_gross_trade_amt(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::GROSS_TRADE_AMT, convert::format_fix_decimal(v));
    }

    /// `GrossTradeAmt` (tag 381, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn gross_trade_amt(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::GROSS_TRADE_AMT)?;
        convert::parse_fix_decimal(&raw, tags::GROSS_TRADE_AMT)
    }

    /// Whether `GrossTradeAmt` (tag 381) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_gross_trade_amt(&self) -> bool {
        self.msg.body().has_field(tags::GROSS_TRADE_AMT)
    }

    // --- Concession (tag 238, AMT, optional) ---

    /// Set `Concession` (tag 238, optional); written in FIX `AMT` format.
    pub fn set_concession(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::CONCESSION, convert::format_fix_decimal(v));
    }

    /// `Concession` (tag 238, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn concession(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::CONCESSION)?;
        convert::parse_fix_decimal(&raw, tags::CONCESSION)
    }

    /// Whether `Concession` (tag 238) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_concession(&self) -> bool {
        self.msg.body().has_field(tags::CONCESSION)
    }

    // --- TotalTakedown (tag 237, AMT, optional) ---

    /// Set `TotalTakedown` (tag 237, optional); written in FIX `AMT` format.
    pub fn set_total_takedown(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::TOTAL_TAKEDOWN, convert::format_fix_decimal(v));
    }

    /// `TotalTakedown` (tag 237, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn total_takedown(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TOTAL_TAKEDOWN)?;
        convert::parse_fix_decimal(&raw, tags::TOTAL_TAKEDOWN)
    }

    /// Whether `TotalTakedown` (tag 237) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_takedown(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_TAKEDOWN)
    }

    // --- NetMoney (tag 118, AMT, optional) ---

    /// Set `NetMoney` (tag 118, optional); written in FIX `AMT` format.
    pub fn set_net_money(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::NET_MONEY, convert::format_fix_decimal(v));
    }

    /// `NetMoney` (tag 118, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn net_money(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::NET_MONEY)?;
        convert::parse_fix_decimal(&raw, tags::NET_MONEY)
    }

    /// Whether `NetMoney` (tag 118) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_net_money(&self) -> bool {
        self.msg.body().has_field(tags::NET_MONEY)
    }

    // --- PositionEffect (tag 77, CHAR, optional) ---

    /// Set `PositionEffect` (tag 77, optional).
    pub fn set_position_effect(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::POSITION_EFFECT, v.to_string());
    }

    /// `PositionEffect` (tag 77, optional).
    pub fn position_effect(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::POSITION_EFFECT)
    }

    /// Whether `PositionEffect` (tag 77) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_position_effect(&self) -> bool {
        self.msg.body().has_field(tags::POSITION_EFFECT)
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

    // --- NumDaysInterest (tag 157, INT, optional) ---

    /// Set `NumDaysInterest` (tag 157, optional).
    pub fn set_num_days_interest(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::NUM_DAYS_INTEREST, v.to_string());
    }

    /// `NumDaysInterest` (tag 157, optional).
    pub fn num_days_interest(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::NUM_DAYS_INTEREST)
    }

    /// Whether `NumDaysInterest` (tag 157) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_num_days_interest(&self) -> bool {
        self.msg.body().has_field(tags::NUM_DAYS_INTEREST)
    }

    // --- AccruedInterestRate (tag 158, PERCENTAGE, optional) ---

    /// Set `AccruedInterestRate` (tag 158, optional).
    pub fn set_accrued_interest_rate(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::ACCRUED_INTEREST_RATE, v.to_string());
    }

    /// `AccruedInterestRate` (tag 158, optional).
    pub fn accrued_interest_rate(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::ACCRUED_INTEREST_RATE)
    }

    /// Whether `AccruedInterestRate` (tag 158) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_accrued_interest_rate(&self) -> bool {
        self.msg.body().has_field(tags::ACCRUED_INTEREST_RATE)
    }

    // --- TotalAccruedInterestAmt (tag 540, AMT, optional) ---

    /// Set `TotalAccruedInterestAmt` (tag 540, optional); written in FIX `AMT` format.
    pub fn set_total_accrued_interest_amt(&mut self, v: Decimal) {
        self.msg
            .body_mut()
            .set_field(tags::TOTAL_ACCRUED_INTEREST_AMT, convert::format_fix_decimal(v));
    }

    /// `TotalAccruedInterestAmt` (tag 540, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn total_accrued_interest_amt(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TOTAL_ACCRUED_INTEREST_AMT)?;
        convert::parse_fix_decimal(&raw, tags::TOTAL_ACCRUED_INTEREST_AMT)
    }

    /// Whether `TotalAccruedInterestAmt` (tag 540) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_accrued_interest_amt(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_ACCRUED_INTEREST_AMT)
    }

    // --- LegalConfirm (tag 650, BOOLEAN, optional) ---

    /// Set `LegalConfirm` (tag 650, optional); written in FIX `BOOLEAN` format.
    pub fn set_legal_confirm(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::LEGAL_CONFIRM, convert::format_fix_bool(v));
    }

    /// `LegalConfirm` (tag 650, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn legal_confirm(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::LEGAL_CONFIRM)?;
        convert::parse_fix_bool(&raw, tags::LEGAL_CONFIRM)
    }

    /// Whether `LegalConfirm` (tag 650) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_legal_confirm(&self) -> bool {
        self.msg.body().has_field(tags::LEGAL_CONFIRM)
    }

    // --- NoOrders group (count tag NO_ORDERS) ---

    /// The `NoOrders` group (count tag `NO_ORDERS`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_orders(&self) -> Option<NoOrders<'_>> {
        self.msg.body().get_group(tags::NO_ORDERS).map(|group| NoOrders { group })
    }

    /// The `NoOrders` group (count tag `NO_ORDERS`, required) for editing, or `None` if
    /// absent.
    pub fn no_orders_mut(&mut self) -> Option<NoOrdersMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_ORDERS).map(|group| NoOrdersMut { group })
    }

    /// Create the `NoOrders` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_orders_entry`] on the same group.
    pub fn set_no_orders(&mut self, count: u32) -> NoOrdersMut<'_> {
        let group = self.msg.body_mut().set_group(tags::NO_ORDERS, count, NO_ORDERS_FIELD_ORDER);
        NoOrdersMut { group }
    }

    /// Append one `NoOrders` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_orders`] on the same group.
    pub fn add_no_orders_entry(&mut self, fill: impl FnOnce(&mut NoOrdersEntryMut<'_>)) {
        let inner = self.msg.body_mut().add_group_instance(tags::NO_ORDERS, NO_ORDERS_FIELD_ORDER);
        fill(&mut NoOrdersEntryMut { inner });
    }

    // --- NoExecs group (count tag NO_EXECS) ---

    /// The `NoExecs` group (count tag `NO_EXECS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_execs(&self) -> Option<NoExecs<'_>> {
        self.msg.body().get_group(tags::NO_EXECS).map(|group| NoExecs { group })
    }

    /// The `NoExecs` group (count tag `NO_EXECS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_execs_mut(&mut self) -> Option<NoExecsMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_EXECS).map(|group| NoExecsMut { group })
    }

    /// Create the `NoExecs` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_execs_entry`] on the same group.
    pub fn set_no_execs(&mut self, count: u32) -> NoExecsMut<'_> {
        let group = self.msg.body_mut().set_group(tags::NO_EXECS, count, NO_EXECS_FIELD_ORDER);
        NoExecsMut { group }
    }

    /// Append one `NoExecs` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_execs`] on the same group.
    pub fn add_no_execs_entry(&mut self, fill: impl FnOnce(&mut NoExecsEntryMut<'_>)) {
        let inner = self.msg.body_mut().add_group_instance(tags::NO_EXECS, NO_EXECS_FIELD_ORDER);
        fill(&mut NoExecsEntryMut { inner });
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

    // --- NoAllocs group (count tag NO_ALLOCS) ---

    /// The `NoAllocs` group (count tag `NO_ALLOCS`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_allocs(&self) -> Option<NoAllocs<'_>> {
        self.msg.body().get_group(tags::NO_ALLOCS).map(|group| NoAllocs { group })
    }

    /// The `NoAllocs` group (count tag `NO_ALLOCS`, required) for editing, or `None` if
    /// absent.
    pub fn no_allocs_mut(&mut self) -> Option<NoAllocsMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_ALLOCS).map(|group| NoAllocsMut { group })
    }

    /// Create the `NoAllocs` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_allocs_entry`] on the same group.
    pub fn set_no_allocs(&mut self, count: u32) -> NoAllocsMut<'_> {
        let group = self.msg.body_mut().set_group(tags::NO_ALLOCS, count, NO_ALLOCS_FIELD_ORDER);
        NoAllocsMut { group }
    }

    /// Append one `NoAllocs` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_allocs`] on the same group.
    pub fn add_no_allocs_entry(&mut self, fill: impl FnOnce(&mut NoAllocsEntryMut<'_>)) {
        let inner = self.msg.body_mut().add_group_instance(tags::NO_ALLOCS, NO_ALLOCS_FIELD_ORDER);
        fill(&mut NoAllocsEntryMut { inner });
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

/// Wire order of a `NoOrders` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_ORDERS_FIELD_ORDER: &[u32] = &[
    tags::CL_ORD_ID,
    tags::ORDER_ID,
    tags::SECONDARY_ORDER_ID,
    tags::SECONDARY_CL_ORD_ID,
    tags::LIST_ID,
];

/// One `NoOrders` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoOrdersEntry<'a> {
    inner: &'a FieldMap,
}

impl NoOrdersEntry<'_> {
    /// `ClOrdID` (tag 11, required).
    pub fn cl_ord_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CL_ORD_ID)
    }

    /// Whether `ClOrdID` (tag 11) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cl_ord_id(&self) -> bool {
        self.inner.has_field(tags::CL_ORD_ID)
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

    /// `SecondaryOrderID` (tag 198, optional).
    pub fn secondary_order_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECONDARY_ORDER_ID)
    }

    /// Whether `SecondaryOrderID` (tag 198) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_order_id(&self) -> bool {
        self.inner.has_field(tags::SECONDARY_ORDER_ID)
    }

    /// `SecondaryClOrdID` (tag 526, optional).
    pub fn secondary_cl_ord_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECONDARY_CL_ORD_ID)
    }

    /// Whether `SecondaryClOrdID` (tag 526) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_cl_ord_id(&self) -> bool {
        self.inner.has_field(tags::SECONDARY_CL_ORD_ID)
    }

    /// `ListID` (tag 66, optional).
    pub fn list_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LIST_ID)
    }

    /// Whether `ListID` (tag 66) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_id(&self) -> bool {
        self.inner.has_field(tags::LIST_ID)
    }
}

/// One `NoOrders` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoOrdersEntryMut::as_ref`].
pub struct NoOrdersEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoOrdersEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoOrdersEntry<'_> {
        NoOrdersEntry { inner: self.inner }
    }

    /// Set `ClOrdID` (tag 11, required).
    pub fn set_cl_ord_id(&mut self, v: &str) {
        self.inner.set_field(tags::CL_ORD_ID, v.to_string());
    }

    /// Set `OrderID` (tag 37, optional).
    pub fn set_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::ORDER_ID, v.to_string());
    }

    /// Set `SecondaryOrderID` (tag 198, optional).
    pub fn set_secondary_order_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECONDARY_ORDER_ID, v.to_string());
    }

    /// Set `SecondaryClOrdID` (tag 526, optional).
    pub fn set_secondary_cl_ord_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECONDARY_CL_ORD_ID, v.to_string());
    }

    /// Set `ListID` (tag 66, optional).
    pub fn set_list_id(&mut self, v: &str) {
        self.inner.set_field(tags::LIST_ID, v.to_string());
    }
}

/// The `NoOrders` repeating group (count tag `NO_ORDERS`), read-only: its
/// instances as [`NoOrdersEntry`] views.
pub struct NoOrders<'a> {
    group: &'a Group,
}

impl<'a> NoOrders<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoOrdersEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoOrdersEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoOrdersEntry<'a> {
        self.get(i).expect("NoOrders index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoOrdersEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoOrdersEntry { inner })
    }
}

/// The `NoOrders` repeating group (count tag `NO_ORDERS`), mutable: its
/// instances as [`NoOrdersEntryMut`] views.
pub struct NoOrdersMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoOrdersMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoOrdersEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoOrdersEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoOrdersEntryMut<'_> {
        self.get_mut(i).expect("NoOrders index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoOrdersEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoOrdersEntryMut { inner })
    }
}

/// Wire order of a `NoExecs` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_EXECS_FIELD_ORDER: &[u32] = &[
    tags::LAST_QTY,
    tags::EXEC_ID,
    tags::SECONDARY_EXEC_ID,
    tags::LAST_PX,
    tags::LAST_CAPACITY,
];

/// One `NoExecs` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoExecsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoExecsEntry<'_> {
    /// `LastQty` (tag 32, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn last_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LAST_QTY)?;
        convert::parse_fix_decimal(&raw, tags::LAST_QTY)
    }

    /// Whether `LastQty` (tag 32) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_qty(&self) -> bool {
        self.inner.has_field(tags::LAST_QTY)
    }

    /// `ExecID` (tag 17, optional).
    pub fn exec_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::EXEC_ID)
    }

    /// Whether `ExecID` (tag 17) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exec_id(&self) -> bool {
        self.inner.has_field(tags::EXEC_ID)
    }

    /// `SecondaryExecID` (tag 527, optional).
    pub fn secondary_exec_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECONDARY_EXEC_ID)
    }

    /// Whether `SecondaryExecID` (tag 527) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_exec_id(&self) -> bool {
        self.inner.has_field(tags::SECONDARY_EXEC_ID)
    }

    /// `LastPx` (tag 31, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn last_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LAST_PX)?;
        convert::parse_fix_decimal(&raw, tags::LAST_PX)
    }

    /// Whether `LastPx` (tag 31) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_px(&self) -> bool {
        self.inner.has_field(tags::LAST_PX)
    }

    /// `LastCapacity` (tag 29, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::LastCapacity`]'s.
    pub fn last_capacity(&self) -> Result<fields::LastCapacity, FieldError> {
        let n = self.inner.get_field::<char>(tags::LAST_CAPACITY)?;
        fields::LastCapacity::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::LAST_CAPACITY,
        })
    }

    /// Whether `LastCapacity` (tag 29) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_capacity(&self) -> bool {
        self.inner.has_field(tags::LAST_CAPACITY)
    }
}

/// One `NoExecs` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoExecsEntryMut::as_ref`].
pub struct NoExecsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoExecsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoExecsEntry<'_> {
        NoExecsEntry { inner: self.inner }
    }

    /// Set `LastQty` (tag 32, optional); written in FIX `QTY` format.
    pub fn set_last_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::LAST_QTY, convert::format_fix_decimal(v));
    }

    /// Set `ExecID` (tag 17, optional).
    pub fn set_exec_id(&mut self, v: &str) {
        self.inner.set_field(tags::EXEC_ID, v.to_string());
    }

    /// Set `SecondaryExecID` (tag 527, optional).
    pub fn set_secondary_exec_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECONDARY_EXEC_ID, v.to_string());
    }

    /// Set `LastPx` (tag 31, optional); written in FIX `PRICE` format.
    pub fn set_last_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::LAST_PX, convert::format_fix_decimal(v));
    }

    /// Set `LastCapacity` (tag 29, optional).
    pub fn set_last_capacity(&mut self, v: fields::LastCapacity) {
        self.inner.set_field(tags::LAST_CAPACITY, v.to_fix().to_string());
    }
}

/// The `NoExecs` repeating group (count tag `NO_EXECS`), read-only: its
/// instances as [`NoExecsEntry`] views.
pub struct NoExecs<'a> {
    group: &'a Group,
}

impl<'a> NoExecs<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoExecsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoExecsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoExecsEntry<'a> {
        self.get(i).expect("NoExecs index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoExecsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoExecsEntry { inner })
    }
}

/// The `NoExecs` repeating group (count tag `NO_EXECS`), mutable: its
/// instances as [`NoExecsEntryMut`] views.
pub struct NoExecsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoExecsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoExecsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoExecsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoExecsEntryMut<'_> {
        self.get_mut(i).expect("NoExecs index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoExecsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoExecsEntryMut { inner })
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

/// Wire order of a `NoAllocs` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_ALLOCS_FIELD_ORDER: &[u32] = &[
    tags::ALLOC_ACCOUNT,
    tags::ALLOC_PRICE,
    tags::ALLOC_QTY,
    tags::INDIVIDUAL_ALLOC_ID,
    tags::PROCESS_CODE,
    tags::NO_NESTED_PARTY_IDS,
    tags::NOTIFY_BROKER_OF_CREDIT,
    tags::ALLOC_HANDL_INST,
    tags::ALLOC_TEXT,
    tags::ENCODED_ALLOC_TEXT_LEN,
    tags::ENCODED_ALLOC_TEXT,
    tags::COMMISSION,
    tags::COMM_TYPE,
    tags::COMM_CURRENCY,
    tags::FUND_RENEW_WAIV,
    tags::ALLOC_AVG_PX,
    tags::ALLOC_NET_MONEY,
    tags::SETTL_CURR_AMT,
    tags::SETTL_CURRENCY,
    tags::SETTL_CURR_FX_RATE,
    tags::SETTL_CURR_FX_RATE_CALC,
    tags::ACCRUED_INTEREST_AMT,
    tags::SETTL_INST_MODE,
    tags::NO_MISC_FEES,
];

/// One `NoAllocs` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoAllocsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoAllocsEntry<'_> {
    /// `AllocAccount` (tag 79, required).
    pub fn alloc_account(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ALLOC_ACCOUNT)
    }

    /// Whether `AllocAccount` (tag 79) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_account(&self) -> bool {
        self.inner.has_field(tags::ALLOC_ACCOUNT)
    }

    /// `AllocPrice` (tag 366, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn alloc_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ALLOC_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::ALLOC_PRICE)
    }

    /// Whether `AllocPrice` (tag 366) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_price(&self) -> bool {
        self.inner.has_field(tags::ALLOC_PRICE)
    }

    /// `AllocQty` (tag 80, required). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn alloc_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ALLOC_QTY)?;
        convert::parse_fix_decimal(&raw, tags::ALLOC_QTY)
    }

    /// Whether `AllocQty` (tag 80) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_qty(&self) -> bool {
        self.inner.has_field(tags::ALLOC_QTY)
    }

    /// `IndividualAllocID` (tag 467, optional).
    pub fn individual_alloc_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::INDIVIDUAL_ALLOC_ID)
    }

    /// Whether `IndividualAllocID` (tag 467) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_individual_alloc_id(&self) -> bool {
        self.inner.has_field(tags::INDIVIDUAL_ALLOC_ID)
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

    /// `NotifyBrokerOfCredit` (tag 208, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn notify_broker_of_credit(&self) -> Result<bool, FieldError> {
        let raw = self.inner.get_field::<String>(tags::NOTIFY_BROKER_OF_CREDIT)?;
        convert::parse_fix_bool(&raw, tags::NOTIFY_BROKER_OF_CREDIT)
    }

    /// Whether `NotifyBrokerOfCredit` (tag 208) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_notify_broker_of_credit(&self) -> bool {
        self.inner.has_field(tags::NOTIFY_BROKER_OF_CREDIT)
    }

    /// `AllocHandlInst` (tag 209, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::AllocHandlInst`]'s.
    pub fn alloc_handl_inst(&self) -> Result<fields::AllocHandlInst, FieldError> {
        let n = self.inner.get_field::<i32>(tags::ALLOC_HANDL_INST)?;
        fields::AllocHandlInst::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ALLOC_HANDL_INST,
        })
    }

    /// Whether `AllocHandlInst` (tag 209) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_handl_inst(&self) -> bool {
        self.inner.has_field(tags::ALLOC_HANDL_INST)
    }

    /// `AllocText` (tag 161, optional).
    pub fn alloc_text(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ALLOC_TEXT)
    }

    /// Whether `AllocText` (tag 161) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_text(&self) -> bool {
        self.inner.has_field(tags::ALLOC_TEXT)
    }

    /// `EncodedAllocTextLen` (tag 360, optional).
    pub fn encoded_alloc_text_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_ALLOC_TEXT_LEN)
    }

    /// Whether `EncodedAllocTextLen` (tag 360) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_alloc_text_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_ALLOC_TEXT_LEN)
    }

    /// `EncodedAllocText` (tag 361, optional).
    pub fn encoded_alloc_text(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_ALLOC_TEXT)
    }

    /// Whether `EncodedAllocText` (tag 361) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_alloc_text(&self) -> bool {
        self.inner.has_field(tags::ENCODED_ALLOC_TEXT)
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

    /// `AllocAvgPx` (tag 153, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn alloc_avg_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ALLOC_AVG_PX)?;
        convert::parse_fix_decimal(&raw, tags::ALLOC_AVG_PX)
    }

    /// Whether `AllocAvgPx` (tag 153) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_avg_px(&self) -> bool {
        self.inner.has_field(tags::ALLOC_AVG_PX)
    }

    /// `AllocNetMoney` (tag 154, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn alloc_net_money(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ALLOC_NET_MONEY)?;
        convert::parse_fix_decimal(&raw, tags::ALLOC_NET_MONEY)
    }

    /// Whether `AllocNetMoney` (tag 154) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_net_money(&self) -> bool {
        self.inner.has_field(tags::ALLOC_NET_MONEY)
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

    /// `SettlInstMode` (tag 160, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SettlInstMode`]'s.
    pub fn settl_inst_mode(&self) -> Result<fields::SettlInstMode, FieldError> {
        let n = self.inner.get_field::<char>(tags::SETTL_INST_MODE)?;
        fields::SettlInstMode::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SETTL_INST_MODE,
        })
    }

    /// Whether `SettlInstMode` (tag 160) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_mode(&self) -> bool {
        self.inner.has_field(tags::SETTL_INST_MODE)
    }

    /// The `NoNestedPartyIds` group (count tag `NO_NESTED_PARTY_IDS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_nested_party_ids(&self) -> Option<NoNestedPartyIds<'_>> {
        self.inner.get_group(tags::NO_NESTED_PARTY_IDS).map(|group| NoNestedPartyIds { group })
    }

    /// The `NoMiscFees` group (count tag `NO_MISC_FEES`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_misc_fees(&self) -> Option<NoMiscFees<'_>> {
        self.inner.get_group(tags::NO_MISC_FEES).map(|group| NoMiscFees { group })
    }
}

/// One `NoAllocs` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoAllocsEntryMut::as_ref`].
pub struct NoAllocsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoAllocsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoAllocsEntry<'_> {
        NoAllocsEntry { inner: self.inner }
    }

    /// Set `AllocAccount` (tag 79, required).
    pub fn set_alloc_account(&mut self, v: &str) {
        self.inner.set_field(tags::ALLOC_ACCOUNT, v.to_string());
    }

    /// Set `AllocPrice` (tag 366, optional); written in FIX `PRICE` format.
    pub fn set_alloc_price(&mut self, v: Decimal) {
        self.inner.set_field(tags::ALLOC_PRICE, convert::format_fix_decimal(v));
    }

    /// Set `AllocQty` (tag 80, required); written in FIX `QTY` format.
    pub fn set_alloc_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::ALLOC_QTY, convert::format_fix_decimal(v));
    }

    /// Set `IndividualAllocID` (tag 467, optional).
    pub fn set_individual_alloc_id(&mut self, v: &str) {
        self.inner.set_field(tags::INDIVIDUAL_ALLOC_ID, v.to_string());
    }

    /// Set `ProcessCode` (tag 81, optional).
    pub fn set_process_code(&mut self, v: fields::ProcessCode) {
        self.inner.set_field(tags::PROCESS_CODE, v.to_fix().to_string());
    }

    /// Set `NotifyBrokerOfCredit` (tag 208, optional); written in FIX `BOOLEAN` format.
    pub fn set_notify_broker_of_credit(&mut self, v: bool) {
        self.inner.set_field(tags::NOTIFY_BROKER_OF_CREDIT, convert::format_fix_bool(v));
    }

    /// Set `AllocHandlInst` (tag 209, optional).
    pub fn set_alloc_handl_inst(&mut self, v: fields::AllocHandlInst) {
        self.inner.set_field(tags::ALLOC_HANDL_INST, v.to_fix().to_string());
    }

    /// Set `AllocText` (tag 161, optional).
    pub fn set_alloc_text(&mut self, v: &str) {
        self.inner.set_field(tags::ALLOC_TEXT, v.to_string());
    }

    /// Set `EncodedAllocTextLen` (tag 360, optional).
    pub fn set_encoded_alloc_text_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_ALLOC_TEXT_LEN, v.to_string());
    }

    /// Set `EncodedAllocText` (tag 361, optional).
    pub fn set_encoded_alloc_text(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_ALLOC_TEXT, v.to_string());
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

    /// Set `AllocAvgPx` (tag 153, optional); written in FIX `PRICE` format.
    pub fn set_alloc_avg_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::ALLOC_AVG_PX, convert::format_fix_decimal(v));
    }

    /// Set `AllocNetMoney` (tag 154, optional); written in FIX `AMT` format.
    pub fn set_alloc_net_money(&mut self, v: Decimal) {
        self.inner.set_field(tags::ALLOC_NET_MONEY, convert::format_fix_decimal(v));
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

    /// Set `AccruedInterestAmt` (tag 159, optional); written in FIX `AMT` format.
    pub fn set_accrued_interest_amt(&mut self, v: Decimal) {
        self.inner.set_field(tags::ACCRUED_INTEREST_AMT, convert::format_fix_decimal(v));
    }

    /// Set `SettlInstMode` (tag 160, optional).
    pub fn set_settl_inst_mode(&mut self, v: fields::SettlInstMode) {
        self.inner.set_field(tags::SETTL_INST_MODE, v.to_fix().to_string());
    }

    /// The `NoNestedPartyIds` group (count tag `NO_NESTED_PARTY_IDS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_nested_party_ids_mut(&mut self) -> Option<NoNestedPartyIdsMut<'_>> {
        self.inner
            .get_group_mut(tags::NO_NESTED_PARTY_IDS)
            .map(|group| NoNestedPartyIdsMut { group })
    }

    /// Create the `NoNestedPartyIds` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_nested_party_ids_entry`] on the same group.
    pub fn set_no_nested_party_ids(&mut self, count: u32) -> NoNestedPartyIdsMut<'_> {
        let group =
            self.inner.set_group(tags::NO_NESTED_PARTY_IDS, count, NO_NESTED_PARTY_IDS_FIELD_ORDER);
        NoNestedPartyIdsMut { group }
    }

    /// Append one `NoNestedPartyIds` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_nested_party_ids`] on the same group.
    pub fn add_no_nested_party_ids_entry(
        &mut self,
        fill: impl FnOnce(&mut NoNestedPartyIdsEntryMut<'_>),
    ) {
        let inner = self
            .inner
            .add_group_instance(tags::NO_NESTED_PARTY_IDS, NO_NESTED_PARTY_IDS_FIELD_ORDER);
        fill(&mut NoNestedPartyIdsEntryMut { inner });
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

/// The `NoAllocs` repeating group (count tag `NO_ALLOCS`), read-only: its
/// instances as [`NoAllocsEntry`] views.
pub struct NoAllocs<'a> {
    group: &'a Group,
}

impl<'a> NoAllocs<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoAllocsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoAllocsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoAllocsEntry<'a> {
        self.get(i).expect("NoAllocs index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoAllocsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoAllocsEntry { inner })
    }
}

/// The `NoAllocs` repeating group (count tag `NO_ALLOCS`), mutable: its
/// instances as [`NoAllocsEntryMut`] views.
pub struct NoAllocsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoAllocsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoAllocsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoAllocsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoAllocsEntryMut<'_> {
        self.get_mut(i).expect("NoAllocs index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoAllocsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoAllocsEntryMut { inner })
    }
}

/// Wire order of a `NoNestedPartyIds` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_NESTED_PARTY_IDS_FIELD_ORDER: &[u32] = &[
    tags::NESTED_PARTY_ID,
    tags::NESTED_PARTY_ID_SOURCE,
    tags::NESTED_PARTY_ROLE,
    tags::NESTED_PARTY_SUB_ID,
];

/// One `NoNestedPartyIds` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoNestedPartyIdsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoNestedPartyIdsEntry<'_> {
    /// `NestedPartyID` (tag 524, optional).
    pub fn nested_party_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::NESTED_PARTY_ID)
    }

    /// Whether `NestedPartyID` (tag 524) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_nested_party_id(&self) -> bool {
        self.inner.has_field(tags::NESTED_PARTY_ID)
    }

    /// `NestedPartyIDSource` (tag 525, optional).
    pub fn nested_party_id_source(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::NESTED_PARTY_ID_SOURCE)
    }

    /// Whether `NestedPartyIDSource` (tag 525) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_nested_party_id_source(&self) -> bool {
        self.inner.has_field(tags::NESTED_PARTY_ID_SOURCE)
    }

    /// `NestedPartyRole` (tag 538, optional).
    pub fn nested_party_role(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::NESTED_PARTY_ROLE)
    }

    /// Whether `NestedPartyRole` (tag 538) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_nested_party_role(&self) -> bool {
        self.inner.has_field(tags::NESTED_PARTY_ROLE)
    }

    /// `NestedPartySubID` (tag 545, optional).
    pub fn nested_party_sub_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::NESTED_PARTY_SUB_ID)
    }

    /// Whether `NestedPartySubID` (tag 545) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_nested_party_sub_id(&self) -> bool {
        self.inner.has_field(tags::NESTED_PARTY_SUB_ID)
    }
}

/// One `NoNestedPartyIds` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoNestedPartyIdsEntryMut::as_ref`].
pub struct NoNestedPartyIdsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoNestedPartyIdsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoNestedPartyIdsEntry<'_> {
        NoNestedPartyIdsEntry { inner: self.inner }
    }

    /// Set `NestedPartyID` (tag 524, optional).
    pub fn set_nested_party_id(&mut self, v: &str) {
        self.inner.set_field(tags::NESTED_PARTY_ID, v.to_string());
    }

    /// Set `NestedPartyIDSource` (tag 525, optional).
    pub fn set_nested_party_id_source(&mut self, v: char) {
        self.inner.set_field(tags::NESTED_PARTY_ID_SOURCE, v.to_string());
    }

    /// Set `NestedPartyRole` (tag 538, optional).
    pub fn set_nested_party_role(&mut self, v: i32) {
        self.inner.set_field(tags::NESTED_PARTY_ROLE, v.to_string());
    }

    /// Set `NestedPartySubID` (tag 545, optional).
    pub fn set_nested_party_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::NESTED_PARTY_SUB_ID, v.to_string());
    }
}

/// The `NoNestedPartyIds` repeating group (count tag `NO_NESTED_PARTY_IDS`), read-only: its
/// instances as [`NoNestedPartyIdsEntry`] views.
pub struct NoNestedPartyIds<'a> {
    group: &'a Group,
}

impl<'a> NoNestedPartyIds<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoNestedPartyIdsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoNestedPartyIdsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoNestedPartyIdsEntry<'a> {
        self.get(i).expect("NoNestedPartyIds index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoNestedPartyIdsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoNestedPartyIdsEntry { inner })
    }
}

/// The `NoNestedPartyIds` repeating group (count tag `NO_NESTED_PARTY_IDS`), mutable: its
/// instances as [`NoNestedPartyIdsEntryMut`] views.
pub struct NoNestedPartyIdsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoNestedPartyIdsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoNestedPartyIdsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoNestedPartyIdsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoNestedPartyIdsEntryMut<'_> {
        self.get_mut(i).expect("NoNestedPartyIds index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoNestedPartyIdsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoNestedPartyIdsEntryMut { inner })
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
