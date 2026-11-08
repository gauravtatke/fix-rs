//! `MultilegOrderCancelReplaceRequest` (app message, MsgType `35=AC`): typed facade over the raw
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

/// Typed facade over a `MultilegOrderCancelReplaceRequest` [`Message`] (35=AC). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct MultilegOrderCancelReplaceRequest {
    msg: Message,
}

impl Default for MultilegOrderCancelReplaceRequest {
    fn default() -> Self {
        MultilegOrderCancelReplaceRequest::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `MultilegOrderCancelReplaceRequest`, checking the msg type.
impl TryFrom<Message> for MultilegOrderCancelReplaceRequest {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a MultilegOrderCancelReplaceRequest" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<MultilegOrderCancelReplaceRequest> for Message {
    fn from(typed_msg: MultilegOrderCancelReplaceRequest) -> Self {
        typed_msg.msg
    }
}

impl MultilegOrderCancelReplaceRequest {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "AC";

    /// An empty `MultilegOrderCancelReplaceRequest` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- OrderID (tag 37, STRING, optional) ---

    /// Set `OrderID` (tag 37, optional).
    pub fn set_order_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ORDER_ID, v.to_string());
    }

    /// `OrderID` (tag 37, optional).
    pub fn order_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ORDER_ID)
    }

    /// Whether `OrderID` (tag 37) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_id(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_ID)
    }

    // --- OrigClOrdID (tag 41, STRING, required) ---

    /// Set `OrigClOrdID` (tag 41, required).
    pub fn set_orig_cl_ord_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ORIG_CL_ORD_ID, v.to_string());
    }

    /// `OrigClOrdID` (tag 41, required).
    pub fn orig_cl_ord_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ORIG_CL_ORD_ID)
    }

    /// Whether `OrigClOrdID` (tag 41) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_orig_cl_ord_id(&self) -> bool {
        self.msg.body().has_field(tags::ORIG_CL_ORD_ID)
    }

    // --- ClOrdID (tag 11, STRING, required) ---

    /// Set `ClOrdID` (tag 11, required).
    pub fn set_cl_ord_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CL_ORD_ID, v.to_string());
    }

    /// `ClOrdID` (tag 11, required).
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

    // --- ClOrdLinkID (tag 583, STRING, optional) ---

    /// Set `ClOrdLinkID` (tag 583, optional).
    pub fn set_cl_ord_link_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CL_ORD_LINK_ID, v.to_string());
    }

    /// `ClOrdLinkID` (tag 583, optional).
    pub fn cl_ord_link_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CL_ORD_LINK_ID)
    }

    /// Whether `ClOrdLinkID` (tag 583) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cl_ord_link_id(&self) -> bool {
        self.msg.body().has_field(tags::CL_ORD_LINK_ID)
    }

    // --- OrigOrdModTime (tag 586, UTCTIMESTAMP, optional) ---

    /// Set `OrigOrdModTime` (tag 586, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_orig_ord_mod_time(&mut self, v: DateTime<Utc>) {
        self.msg
            .body_mut()
            .set_field(tags::ORIG_ORD_MOD_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `OrigOrdModTime` (tag 586, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn orig_ord_mod_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::ORIG_ORD_MOD_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::ORIG_ORD_MOD_TIME)
    }

    /// Whether `OrigOrdModTime` (tag 586) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_orig_ord_mod_time(&self) -> bool {
        self.msg.body().has_field(tags::ORIG_ORD_MOD_TIME)
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

    // --- DayBookingInst (tag 589, CHAR, optional) ---

    /// Set `DayBookingInst` (tag 589, optional).
    pub fn set_day_booking_inst(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::DAY_BOOKING_INST, v.to_string());
    }

    /// `DayBookingInst` (tag 589, optional).
    pub fn day_booking_inst(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::DAY_BOOKING_INST)
    }

    /// Whether `DayBookingInst` (tag 589) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_day_booking_inst(&self) -> bool {
        self.msg.body().has_field(tags::DAY_BOOKING_INST)
    }

    // --- BookingUnit (tag 590, CHAR, optional) ---

    /// Set `BookingUnit` (tag 590, optional).
    pub fn set_booking_unit(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::BOOKING_UNIT, v.to_string());
    }

    /// `BookingUnit` (tag 590, optional).
    pub fn booking_unit(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::BOOKING_UNIT)
    }

    /// Whether `BookingUnit` (tag 590) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_booking_unit(&self) -> bool {
        self.msg.body().has_field(tags::BOOKING_UNIT)
    }

    // --- PreallocMethod (tag 591, CHAR, optional) ---

    /// Set `PreallocMethod` (tag 591, optional).
    pub fn set_prealloc_method(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::PREALLOC_METHOD, v.to_string());
    }

    /// `PreallocMethod` (tag 591, optional).
    pub fn prealloc_method(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::PREALLOC_METHOD)
    }

    /// Whether `PreallocMethod` (tag 591) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_prealloc_method(&self) -> bool {
        self.msg.body().has_field(tags::PREALLOC_METHOD)
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

    // --- CashMargin (tag 544, CHAR, optional) ---

    /// Set `CashMargin` (tag 544, optional).
    pub fn set_cash_margin(&mut self, v: fields::CashMargin) {
        self.msg.body_mut().set_field(tags::CASH_MARGIN, v.to_fix().to_string());
    }

    /// `CashMargin` (tag 544, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::CashMargin`]'s.
    pub fn cash_margin(&self) -> Result<fields::CashMargin, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::CASH_MARGIN)?;
        fields::CashMargin::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::CASH_MARGIN,
        })
    }

    /// Whether `CashMargin` (tag 544) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_margin(&self) -> bool {
        self.msg.body().has_field(tags::CASH_MARGIN)
    }

    // --- ClearingFeeIndicator (tag 635, STRING, optional) ---

    /// Set `ClearingFeeIndicator` (tag 635, optional).
    pub fn set_clearing_fee_indicator(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CLEARING_FEE_INDICATOR, v.to_string());
    }

    /// `ClearingFeeIndicator` (tag 635, optional).
    pub fn clearing_fee_indicator(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CLEARING_FEE_INDICATOR)
    }

    /// Whether `ClearingFeeIndicator` (tag 635) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_clearing_fee_indicator(&self) -> bool {
        self.msg.body().has_field(tags::CLEARING_FEE_INDICATOR)
    }

    // --- HandlInst (tag 21, CHAR, required) ---

    /// Set `HandlInst` (tag 21, required).
    pub fn set_handl_inst(&mut self, v: fields::HandlInst) {
        self.msg.body_mut().set_field(tags::HANDL_INST, v.to_fix().to_string());
    }

    /// `HandlInst` (tag 21, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::HandlInst`]'s.
    pub fn handl_inst(&self) -> Result<fields::HandlInst, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::HANDL_INST)?;
        fields::HandlInst::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::HANDL_INST,
        })
    }

    /// Whether `HandlInst` (tag 21) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_handl_inst(&self) -> bool {
        self.msg.body().has_field(tags::HANDL_INST)
    }

    // --- ExecInst (tag 18, MULTIPLEVALUESTRING, optional) ---

    /// Set `ExecInst` (tag 18, optional).
    pub fn set_exec_inst(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::EXEC_INST, v.to_string());
    }

    /// `ExecInst` (tag 18, optional).
    pub fn exec_inst(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::EXEC_INST)
    }

    /// Whether `ExecInst` (tag 18) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exec_inst(&self) -> bool {
        self.msg.body().has_field(tags::EXEC_INST)
    }

    // --- MinQty (tag 110, QTY, optional) ---

    /// Set `MinQty` (tag 110, optional); written in FIX `QTY` format.
    pub fn set_min_qty(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MIN_QTY, convert::format_fix_decimal(v));
    }

    /// `MinQty` (tag 110, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn min_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MIN_QTY)?;
        convert::parse_fix_decimal(&raw, tags::MIN_QTY)
    }

    /// Whether `MinQty` (tag 110) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_min_qty(&self) -> bool {
        self.msg.body().has_field(tags::MIN_QTY)
    }

    // --- MaxFloor (tag 111, QTY, optional) ---

    /// Set `MaxFloor` (tag 111, optional); written in FIX `QTY` format.
    pub fn set_max_floor(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MAX_FLOOR, convert::format_fix_decimal(v));
    }

    /// `MaxFloor` (tag 111, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn max_floor(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MAX_FLOOR)?;
        convert::parse_fix_decimal(&raw, tags::MAX_FLOOR)
    }

    /// Whether `MaxFloor` (tag 111) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_max_floor(&self) -> bool {
        self.msg.body().has_field(tags::MAX_FLOOR)
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

    // --- ProcessCode (tag 81, CHAR, optional) ---

    /// Set `ProcessCode` (tag 81, optional).
    pub fn set_process_code(&mut self, v: fields::ProcessCode) {
        self.msg.body_mut().set_field(tags::PROCESS_CODE, v.to_fix().to_string());
    }

    /// `ProcessCode` (tag 81, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ProcessCode`]'s.
    pub fn process_code(&self) -> Result<fields::ProcessCode, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::PROCESS_CODE)?;
        fields::ProcessCode::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PROCESS_CODE,
        })
    }

    /// Whether `ProcessCode` (tag 81) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_process_code(&self) -> bool {
        self.msg.body().has_field(tags::PROCESS_CODE)
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

    // --- PrevClosePx (tag 140, PRICE, optional) ---

    /// Set `PrevClosePx` (tag 140, optional); written in FIX `PRICE` format.
    pub fn set_prev_close_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::PREV_CLOSE_PX, convert::format_fix_decimal(v));
    }

    /// `PrevClosePx` (tag 140, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn prev_close_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::PREV_CLOSE_PX)?;
        convert::parse_fix_decimal(&raw, tags::PREV_CLOSE_PX)
    }

    /// Whether `PrevClosePx` (tag 140) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_prev_close_px(&self) -> bool {
        self.msg.body().has_field(tags::PREV_CLOSE_PX)
    }

    // --- LocateReqd (tag 114, BOOLEAN, optional) ---

    /// Set `LocateReqd` (tag 114, optional); written in FIX `BOOLEAN` format.
    pub fn set_locate_reqd(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::LOCATE_REQD, convert::format_fix_bool(v));
    }

    /// `LocateReqd` (tag 114, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn locate_reqd(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::LOCATE_REQD)?;
        convert::parse_fix_bool(&raw, tags::LOCATE_REQD)
    }

    /// Whether `LocateReqd` (tag 114) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_locate_reqd(&self) -> bool {
        self.msg.body().has_field(tags::LOCATE_REQD)
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

    // --- OrdType (tag 40, CHAR, required) ---

    /// Set `OrdType` (tag 40, required).
    pub fn set_ord_type(&mut self, v: fields::OrdType) {
        self.msg.body_mut().set_field(tags::ORD_TYPE, v.to_fix().to_string());
    }

    /// `OrdType` (tag 40, required). `Err(InvalidFormat)` if the value isn't one
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

    // --- StopPx (tag 99, PRICE, optional) ---

    /// Set `StopPx` (tag 99, optional); written in FIX `PRICE` format.
    pub fn set_stop_px(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::STOP_PX, convert::format_fix_decimal(v));
    }

    /// `StopPx` (tag 99, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn stop_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::STOP_PX)?;
        convert::parse_fix_decimal(&raw, tags::STOP_PX)
    }

    /// Whether `StopPx` (tag 99) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_stop_px(&self) -> bool {
        self.msg.body().has_field(tags::STOP_PX)
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

    // --- ComplianceID (tag 376, STRING, optional) ---

    /// Set `ComplianceID` (tag 376, optional).
    pub fn set_compliance_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::COMPLIANCE_ID, v.to_string());
    }

    /// `ComplianceID` (tag 376, optional).
    pub fn compliance_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::COMPLIANCE_ID)
    }

    /// Whether `ComplianceID` (tag 376) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_compliance_id(&self) -> bool {
        self.msg.body().has_field(tags::COMPLIANCE_ID)
    }

    // --- SolicitedFlag (tag 377, BOOLEAN, optional) ---

    /// Set `SolicitedFlag` (tag 377, optional); written in FIX `BOOLEAN` format.
    pub fn set_solicited_flag(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::SOLICITED_FLAG, convert::format_fix_bool(v));
    }

    /// `SolicitedFlag` (tag 377, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn solicited_flag(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SOLICITED_FLAG)?;
        convert::parse_fix_bool(&raw, tags::SOLICITED_FLAG)
    }

    /// Whether `SolicitedFlag` (tag 377) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_solicited_flag(&self) -> bool {
        self.msg.body().has_field(tags::SOLICITED_FLAG)
    }

    // --- IOIID (tag 23, STRING, optional) ---

    /// Set `IOIID` (tag 23, optional).
    pub fn set_ioi_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::IOI_ID, v.to_string());
    }

    /// `IOIID` (tag 23, optional).
    pub fn ioi_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::IOI_ID)
    }

    /// Whether `IOIID` (tag 23) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ioi_id(&self) -> bool {
        self.msg.body().has_field(tags::IOI_ID)
    }

    // --- QuoteID (tag 117, STRING, optional) ---

    /// Set `QuoteID` (tag 117, optional).
    pub fn set_quote_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::QUOTE_ID, v.to_string());
    }

    /// `QuoteID` (tag 117, optional).
    pub fn quote_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::QUOTE_ID)
    }

    /// Whether `QuoteID` (tag 117) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_id(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_ID)
    }

    // --- TimeInForce (tag 59, CHAR, optional) ---

    /// Set `TimeInForce` (tag 59, optional).
    pub fn set_time_in_force(&mut self, v: fields::TimeInForce) {
        self.msg.body_mut().set_field(tags::TIME_IN_FORCE, v.to_fix().to_string());
    }

    /// `TimeInForce` (tag 59, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TimeInForce`]'s.
    pub fn time_in_force(&self) -> Result<fields::TimeInForce, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::TIME_IN_FORCE)?;
        fields::TimeInForce::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TIME_IN_FORCE,
        })
    }

    /// Whether `TimeInForce` (tag 59) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_time_in_force(&self) -> bool {
        self.msg.body().has_field(tags::TIME_IN_FORCE)
    }

    // --- EffectiveTime (tag 168, UTCTIMESTAMP, optional) ---

    /// Set `EffectiveTime` (tag 168, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_effective_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::EFFECTIVE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `EffectiveTime` (tag 168, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn effective_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::EFFECTIVE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::EFFECTIVE_TIME)
    }

    /// Whether `EffectiveTime` (tag 168) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_effective_time(&self) -> bool {
        self.msg.body().has_field(tags::EFFECTIVE_TIME)
    }

    // --- ExpireDate (tag 432, LOCALMKTDATE, optional) ---

    /// Set `ExpireDate` (tag 432, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_expire_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::EXPIRE_DATE, convert::format_fix_date(v));
    }

    /// `ExpireDate` (tag 432, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn expire_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::EXPIRE_DATE)?;
        convert::parse_fix_date(&raw, tags::EXPIRE_DATE)
    }

    /// Whether `ExpireDate` (tag 432) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_expire_date(&self) -> bool {
        self.msg.body().has_field(tags::EXPIRE_DATE)
    }

    // --- ExpireTime (tag 126, UTCTIMESTAMP, optional) ---

    /// Set `ExpireTime` (tag 126, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_expire_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::EXPIRE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `ExpireTime` (tag 126, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn expire_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::EXPIRE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::EXPIRE_TIME)
    }

    /// Whether `ExpireTime` (tag 126) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_expire_time(&self) -> bool {
        self.msg.body().has_field(tags::EXPIRE_TIME)
    }

    // --- GTBookingInst (tag 427, INT, optional) ---

    /// Set `GTBookingInst` (tag 427, optional).
    pub fn set_gt_booking_inst(&mut self, v: fields::GtBookingInst) {
        self.msg.body_mut().set_field(tags::GT_BOOKING_INST, v.to_fix().to_string());
    }

    /// `GTBookingInst` (tag 427, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::GtBookingInst`]'s.
    pub fn gt_booking_inst(&self) -> Result<fields::GtBookingInst, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::GT_BOOKING_INST)?;
        fields::GtBookingInst::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::GT_BOOKING_INST,
        })
    }

    /// Whether `GTBookingInst` (tag 427) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_gt_booking_inst(&self) -> bool {
        self.msg.body().has_field(tags::GT_BOOKING_INST)
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

    // --- CommCurrency (tag 479, CURRENCY, optional) ---

    /// Set `CommCurrency` (tag 479, optional).
    pub fn set_comm_currency(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::COMM_CURRENCY, v.to_string());
    }

    /// `CommCurrency` (tag 479, optional).
    pub fn comm_currency(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::COMM_CURRENCY)
    }

    /// Whether `CommCurrency` (tag 479) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_comm_currency(&self) -> bool {
        self.msg.body().has_field(tags::COMM_CURRENCY)
    }

    // --- FundRenewWaiv (tag 497, CHAR, optional) ---

    /// Set `FundRenewWaiv` (tag 497, optional).
    pub fn set_fund_renew_waiv(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::FUND_RENEW_WAIV, v.to_string());
    }

    /// `FundRenewWaiv` (tag 497, optional).
    pub fn fund_renew_waiv(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::FUND_RENEW_WAIV)
    }

    /// Whether `FundRenewWaiv` (tag 497) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fund_renew_waiv(&self) -> bool {
        self.msg.body().has_field(tags::FUND_RENEW_WAIV)
    }

    // --- OrderCapacity (tag 528, CHAR, optional) ---

    /// Set `OrderCapacity` (tag 528, optional).
    pub fn set_order_capacity(&mut self, v: fields::OrderCapacity) {
        self.msg.body_mut().set_field(tags::ORDER_CAPACITY, v.to_fix().to_string());
    }

    /// `OrderCapacity` (tag 528, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrderCapacity`]'s.
    pub fn order_capacity(&self) -> Result<fields::OrderCapacity, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::ORDER_CAPACITY)?;
        fields::OrderCapacity::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORDER_CAPACITY,
        })
    }

    /// Whether `OrderCapacity` (tag 528) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_capacity(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_CAPACITY)
    }

    // --- OrderRestrictions (tag 529, MULTIPLEVALUESTRING, optional) ---

    /// Set `OrderRestrictions` (tag 529, optional).
    pub fn set_order_restrictions(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ORDER_RESTRICTIONS, v.to_string());
    }

    /// `OrderRestrictions` (tag 529, optional).
    pub fn order_restrictions(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ORDER_RESTRICTIONS)
    }

    /// Whether `OrderRestrictions` (tag 529) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_restrictions(&self) -> bool {
        self.msg.body().has_field(tags::ORDER_RESTRICTIONS)
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

    // --- ForexReq (tag 121, BOOLEAN, optional) ---

    /// Set `ForexReq` (tag 121, optional); written in FIX `BOOLEAN` format.
    pub fn set_forex_req(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::FOREX_REQ, convert::format_fix_bool(v));
    }

    /// `ForexReq` (tag 121, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn forex_req(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::FOREX_REQ)?;
        convert::parse_fix_bool(&raw, tags::FOREX_REQ)
    }

    /// Whether `ForexReq` (tag 121) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_forex_req(&self) -> bool {
        self.msg.body().has_field(tags::FOREX_REQ)
    }

    // --- SettlCurrency (tag 120, CURRENCY, optional) ---

    /// Set `SettlCurrency` (tag 120, optional).
    pub fn set_settl_currency(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SETTL_CURRENCY, v.to_string());
    }

    /// `SettlCurrency` (tag 120, optional).
    pub fn settl_currency(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SETTL_CURRENCY)
    }

    /// Whether `SettlCurrency` (tag 120) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_currency(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_CURRENCY)
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

    // --- CoveredOrUncovered (tag 203, INT, optional) ---

    /// Set `CoveredOrUncovered` (tag 203, optional).
    pub fn set_covered_or_uncovered(&mut self, v: fields::CoveredOrUncovered) {
        self.msg.body_mut().set_field(tags::COVERED_OR_UNCOVERED, v.to_fix().to_string());
    }

    /// `CoveredOrUncovered` (tag 203, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::CoveredOrUncovered`]'s.
    pub fn covered_or_uncovered(&self) -> Result<fields::CoveredOrUncovered, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::COVERED_OR_UNCOVERED)?;
        fields::CoveredOrUncovered::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::COVERED_OR_UNCOVERED,
        })
    }

    /// Whether `CoveredOrUncovered` (tag 203) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_covered_or_uncovered(&self) -> bool {
        self.msg.body().has_field(tags::COVERED_OR_UNCOVERED)
    }

    // --- MaxShow (tag 210, QTY, optional) ---

    /// Set `MaxShow` (tag 210, optional); written in FIX `QTY` format.
    pub fn set_max_show(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::MAX_SHOW, convert::format_fix_decimal(v));
    }

    /// `MaxShow` (tag 210, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn max_show(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::MAX_SHOW)?;
        convert::parse_fix_decimal(&raw, tags::MAX_SHOW)
    }

    /// Whether `MaxShow` (tag 210) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_max_show(&self) -> bool {
        self.msg.body().has_field(tags::MAX_SHOW)
    }

    // --- PegDifference (tag 211, PRICEOFFSET, optional) ---

    /// Set `PegDifference` (tag 211, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_peg_difference(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::PEG_DIFFERENCE, convert::format_fix_decimal(v));
    }

    /// `PegDifference` (tag 211, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn peg_difference(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::PEG_DIFFERENCE)?;
        convert::parse_fix_decimal(&raw, tags::PEG_DIFFERENCE)
    }

    /// Whether `PegDifference` (tag 211) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_peg_difference(&self) -> bool {
        self.msg.body().has_field(tags::PEG_DIFFERENCE)
    }

    // --- DiscretionInst (tag 388, CHAR, optional) ---

    /// Set `DiscretionInst` (tag 388, optional).
    pub fn set_discretion_inst(&mut self, v: fields::DiscretionInst) {
        self.msg.body_mut().set_field(tags::DISCRETION_INST, v.to_fix().to_string());
    }

    /// `DiscretionInst` (tag 388, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::DiscretionInst`]'s.
    pub fn discretion_inst(&self) -> Result<fields::DiscretionInst, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::DISCRETION_INST)?;
        fields::DiscretionInst::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::DISCRETION_INST,
        })
    }

    /// Whether `DiscretionInst` (tag 388) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_discretion_inst(&self) -> bool {
        self.msg.body().has_field(tags::DISCRETION_INST)
    }

    // --- DiscretionOffset (tag 389, PRICEOFFSET, optional) ---

    /// Set `DiscretionOffset` (tag 389, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_discretion_offset(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::DISCRETION_OFFSET, convert::format_fix_decimal(v));
    }

    /// `DiscretionOffset` (tag 389, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn discretion_offset(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::DISCRETION_OFFSET)?;
        convert::parse_fix_decimal(&raw, tags::DISCRETION_OFFSET)
    }

    /// Whether `DiscretionOffset` (tag 389) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_discretion_offset(&self) -> bool {
        self.msg.body().has_field(tags::DISCRETION_OFFSET)
    }

    // --- CancellationRights (tag 480, CHAR, optional) ---

    /// Set `CancellationRights` (tag 480, optional).
    pub fn set_cancellation_rights(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::CANCELLATION_RIGHTS, v.to_string());
    }

    /// `CancellationRights` (tag 480, optional).
    pub fn cancellation_rights(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::CANCELLATION_RIGHTS)
    }

    /// Whether `CancellationRights` (tag 480) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cancellation_rights(&self) -> bool {
        self.msg.body().has_field(tags::CANCELLATION_RIGHTS)
    }

    // --- MoneyLaunderingStatus (tag 481, CHAR, optional) ---

    /// Set `MoneyLaunderingStatus` (tag 481, optional).
    pub fn set_money_laundering_status(&mut self, v: fields::MoneyLaunderingStatus) {
        self.msg.body_mut().set_field(tags::MONEY_LAUNDERING_STATUS, v.to_fix().to_string());
    }

    /// `MoneyLaunderingStatus` (tag 481, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MoneyLaunderingStatus`]'s.
    pub fn money_laundering_status(&self) -> Result<fields::MoneyLaunderingStatus, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::MONEY_LAUNDERING_STATUS)?;
        fields::MoneyLaunderingStatus::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MONEY_LAUNDERING_STATUS,
        })
    }

    /// Whether `MoneyLaunderingStatus` (tag 481) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_money_laundering_status(&self) -> bool {
        self.msg.body().has_field(tags::MONEY_LAUNDERING_STATUS)
    }

    // --- RegistID (tag 513, STRING, optional) ---

    /// Set `RegistID` (tag 513, optional).
    pub fn set_regist_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REGIST_ID, v.to_string());
    }

    /// `RegistID` (tag 513, optional).
    pub fn regist_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REGIST_ID)
    }

    /// Whether `RegistID` (tag 513) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_id(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_ID)
    }

    // --- Designation (tag 494, STRING, optional) ---

    /// Set `Designation` (tag 494, optional).
    pub fn set_designation(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::DESIGNATION, v.to_string());
    }

    /// `Designation` (tag 494, optional).
    pub fn designation(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::DESIGNATION)
    }

    /// Whether `Designation` (tag 494) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_designation(&self) -> bool {
        self.msg.body().has_field(tags::DESIGNATION)
    }

    // --- MultiLegRptTypeReq (tag 563, INT, optional) ---

    /// Set `MultiLegRptTypeReq` (tag 563, optional).
    pub fn set_multi_leg_rpt_type_req(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::MULTI_LEG_RPT_TYPE_REQ, v.to_string());
    }

    /// `MultiLegRptTypeReq` (tag 563, optional).
    pub fn multi_leg_rpt_type_req(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::MULTI_LEG_RPT_TYPE_REQ)
    }

    /// Whether `MultiLegRptTypeReq` (tag 563) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_multi_leg_rpt_type_req(&self) -> bool {
        self.msg.body().has_field(tags::MULTI_LEG_RPT_TYPE_REQ)
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

    /// The `NoAllocs` group (count tag `NO_ALLOCS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_allocs(&self) -> Option<NoAllocs<'_>> {
        self.msg.body().get_group(tags::NO_ALLOCS).map(|group| NoAllocs { group })
    }

    /// The `NoAllocs` group (count tag `NO_ALLOCS`, optional) for editing, or `None` if
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

    // --- NoTradingSessions group (count tag NO_TRADING_SESSIONS) ---

    /// The `NoTradingSessions` group (count tag `NO_TRADING_SESSIONS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_trading_sessions(&self) -> Option<NoTradingSessions<'_>> {
        self.msg
            .body()
            .get_group(tags::NO_TRADING_SESSIONS)
            .map(|group| NoTradingSessions { group })
    }

    /// The `NoTradingSessions` group (count tag `NO_TRADING_SESSIONS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_trading_sessions_mut(&mut self) -> Option<NoTradingSessionsMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_TRADING_SESSIONS)
            .map(|group| NoTradingSessionsMut { group })
    }

    /// Create the `NoTradingSessions` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_trading_sessions_entry`] on the same group.
    pub fn set_no_trading_sessions(&mut self, count: u32) -> NoTradingSessionsMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_TRADING_SESSIONS,
            count,
            NO_TRADING_SESSIONS_FIELD_ORDER,
        );
        NoTradingSessionsMut { group }
    }

    /// Append one `NoTradingSessions` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_trading_sessions`] on the same group.
    pub fn add_no_trading_sessions_entry(
        &mut self,
        fill: impl FnOnce(&mut NoTradingSessionsEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_TRADING_SESSIONS, NO_TRADING_SESSIONS_FIELD_ORDER);
        fill(&mut NoTradingSessionsEntryMut { inner });
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
    tags::INDIVIDUAL_ALLOC_ID,
    tags::ALLOC_QTY,
];

/// One `NoAllocs` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoAllocsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoAllocsEntry<'_> {
    /// `AllocAccount` (tag 79, optional).
    pub fn alloc_account(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ALLOC_ACCOUNT)
    }

    /// Whether `AllocAccount` (tag 79) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_account(&self) -> bool {
        self.inner.has_field(tags::ALLOC_ACCOUNT)
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

    /// `AllocQty` (tag 80, optional). `Err(InvalidFormat)` unless a valid FIX
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

    /// Set `AllocAccount` (tag 79, optional).
    pub fn set_alloc_account(&mut self, v: &str) {
        self.inner.set_field(tags::ALLOC_ACCOUNT, v.to_string());
    }

    /// Set `IndividualAllocID` (tag 467, optional).
    pub fn set_individual_alloc_id(&mut self, v: &str) {
        self.inner.set_field(tags::INDIVIDUAL_ALLOC_ID, v.to_string());
    }

    /// Set `AllocQty` (tag 80, optional); written in FIX `QTY` format.
    pub fn set_alloc_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::ALLOC_QTY, convert::format_fix_decimal(v));
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

/// Wire order of a `NoTradingSessions` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_TRADING_SESSIONS_FIELD_ORDER: &[u32] =
    &[tags::TRADING_SESSION_ID, tags::TRADING_SESSION_SUB_ID];

/// One `NoTradingSessions` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoTradingSessionsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoTradingSessionsEntry<'_> {
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
}

/// One `NoTradingSessions` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoTradingSessionsEntryMut::as_ref`].
pub struct NoTradingSessionsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoTradingSessionsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoTradingSessionsEntry<'_> {
        NoTradingSessionsEntry { inner: self.inner }
    }

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
    }
}

/// The `NoTradingSessions` repeating group (count tag `NO_TRADING_SESSIONS`), read-only: its
/// instances as [`NoTradingSessionsEntry`] views.
pub struct NoTradingSessions<'a> {
    group: &'a Group,
}

impl<'a> NoTradingSessions<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoTradingSessionsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoTradingSessionsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoTradingSessionsEntry<'a> {
        self.get(i).expect("NoTradingSessions index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoTradingSessionsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoTradingSessionsEntry { inner })
    }
}

/// The `NoTradingSessions` repeating group (count tag `NO_TRADING_SESSIONS`), mutable: its
/// instances as [`NoTradingSessionsEntryMut`] views.
pub struct NoTradingSessionsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoTradingSessionsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoTradingSessionsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoTradingSessionsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoTradingSessionsEntryMut<'_> {
        self.get_mut(i).expect("NoTradingSessions index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoTradingSessionsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoTradingSessionsEntryMut { inner })
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
    tags::LEG_POSITION_EFFECT,
    tags::LEG_COVERED_OR_UNCOVERED,
    tags::NO_NESTED_PARTY_IDS,
    tags::LEG_REF_ID,
    tags::LEG_PRICE,
    tags::LEG_SETTLMNT_TYP,
    tags::LEG_FUT_SETT_DATE,
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

    /// `LegPositionEffect` (tag 564, optional).
    pub fn leg_position_effect(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::LEG_POSITION_EFFECT)
    }

    /// Whether `LegPositionEffect` (tag 564) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_position_effect(&self) -> bool {
        self.inner.has_field(tags::LEG_POSITION_EFFECT)
    }

    /// `LegCoveredOrUncovered` (tag 565, optional).
    pub fn leg_covered_or_uncovered(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::LEG_COVERED_OR_UNCOVERED)
    }

    /// Whether `LegCoveredOrUncovered` (tag 565) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_covered_or_uncovered(&self) -> bool {
        self.inner.has_field(tags::LEG_COVERED_OR_UNCOVERED)
    }

    /// `LegRefID` (tag 654, optional).
    pub fn leg_ref_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LEG_REF_ID)
    }

    /// Whether `LegRefID` (tag 654) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_ref_id(&self) -> bool {
        self.inner.has_field(tags::LEG_REF_ID)
    }

    /// `LegPrice` (tag 566, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn leg_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::LEG_PRICE)
    }

    /// Whether `LegPrice` (tag 566) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_price(&self) -> bool {
        self.inner.has_field(tags::LEG_PRICE)
    }

    /// `LegSettlmntTyp` (tag 587, optional).
    pub fn leg_settlmnt_typ(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::LEG_SETTLMNT_TYP)
    }

    /// Whether `LegSettlmntTyp` (tag 587) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_settlmnt_typ(&self) -> bool {
        self.inner.has_field(tags::LEG_SETTLMNT_TYP)
    }

    /// `LegFutSettDate` (tag 588, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn leg_fut_sett_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEG_FUT_SETT_DATE)?;
        convert::parse_fix_date(&raw, tags::LEG_FUT_SETT_DATE)
    }

    /// Whether `LegFutSettDate` (tag 588) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leg_fut_sett_date(&self) -> bool {
        self.inner.has_field(tags::LEG_FUT_SETT_DATE)
    }

    /// The `NoLegSecurityAltId` group (count tag `NO_LEG_SECURITY_ALT_ID`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_leg_security_alt_id(&self) -> Option<NoLegSecurityAltId<'_>> {
        self.inner.get_group(tags::NO_LEG_SECURITY_ALT_ID).map(|group| NoLegSecurityAltId { group })
    }

    /// The `NoNestedPartyIds` group (count tag `NO_NESTED_PARTY_IDS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_nested_party_ids(&self) -> Option<NoNestedPartyIds<'_>> {
        self.inner.get_group(tags::NO_NESTED_PARTY_IDS).map(|group| NoNestedPartyIds { group })
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

    /// Set `LegPositionEffect` (tag 564, optional).
    pub fn set_leg_position_effect(&mut self, v: char) {
        self.inner.set_field(tags::LEG_POSITION_EFFECT, v.to_string());
    }

    /// Set `LegCoveredOrUncovered` (tag 565, optional).
    pub fn set_leg_covered_or_uncovered(&mut self, v: i32) {
        self.inner.set_field(tags::LEG_COVERED_OR_UNCOVERED, v.to_string());
    }

    /// Set `LegRefID` (tag 654, optional).
    pub fn set_leg_ref_id(&mut self, v: &str) {
        self.inner.set_field(tags::LEG_REF_ID, v.to_string());
    }

    /// Set `LegPrice` (tag 566, optional); written in FIX `PRICE` format.
    pub fn set_leg_price(&mut self, v: Decimal) {
        self.inner.set_field(tags::LEG_PRICE, convert::format_fix_decimal(v));
    }

    /// Set `LegSettlmntTyp` (tag 587, optional).
    pub fn set_leg_settlmnt_typ(&mut self, v: char) {
        self.inner.set_field(tags::LEG_SETTLMNT_TYP, v.to_string());
    }

    /// Set `LegFutSettDate` (tag 588, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_leg_fut_sett_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::LEG_FUT_SETT_DATE, convert::format_fix_date(v));
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
