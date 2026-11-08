//! `OrderCancelReject` (app message, MsgType `35=9`): typed facade over the raw
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

/// Typed facade over a `OrderCancelReject` [`Message`] (35=9). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct OrderCancelReject {
    msg: Message,
}

impl Default for OrderCancelReject {
    fn default() -> Self {
        OrderCancelReject::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `OrderCancelReject`, checking the msg type.
impl TryFrom<Message> for OrderCancelReject {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a OrderCancelReject" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<OrderCancelReject> for Message {
    fn from(typed_msg: OrderCancelReject) -> Self {
        typed_msg.msg
    }
}

impl OrderCancelReject {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "9";

    /// An empty `OrderCancelReject` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
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

    // --- OrdStatus (tag 39, CHAR, required) ---

    /// Set `OrdStatus` (tag 39, required).
    pub fn set_ord_status(&mut self, v: fields::OrdStatus) {
        self.msg.body_mut().set_field(tags::ORD_STATUS, v.to_fix().to_string());
    }

    /// `OrdStatus` (tag 39, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrdStatus`]'s.
    pub fn ord_status(&self) -> Result<fields::OrdStatus, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::ORD_STATUS)?;
        fields::OrdStatus::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORD_STATUS,
        })
    }

    /// Whether `OrdStatus` (tag 39) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ord_status(&self) -> bool {
        self.msg.body().has_field(tags::ORD_STATUS)
    }

    // --- WorkingIndicator (tag 636, BOOLEAN, optional) ---

    /// Set `WorkingIndicator` (tag 636, optional); written in FIX `BOOLEAN` format.
    pub fn set_working_indicator(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::WORKING_INDICATOR, convert::format_fix_bool(v));
    }

    /// `WorkingIndicator` (tag 636, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn working_indicator(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::WORKING_INDICATOR)?;
        convert::parse_fix_bool(&raw, tags::WORKING_INDICATOR)
    }

    /// Whether `WorkingIndicator` (tag 636) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_working_indicator(&self) -> bool {
        self.msg.body().has_field(tags::WORKING_INDICATOR)
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

    // --- ListID (tag 66, STRING, optional) ---

    /// Set `ListID` (tag 66, optional).
    pub fn set_list_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LIST_ID, v.to_string());
    }

    /// `ListID` (tag 66, optional).
    pub fn list_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LIST_ID)
    }

    /// Whether `ListID` (tag 66) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_id(&self) -> bool {
        self.msg.body().has_field(tags::LIST_ID)
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

    // --- CxlRejResponseTo (tag 434, CHAR, required) ---

    /// Set `CxlRejResponseTo` (tag 434, required).
    pub fn set_cxl_rej_response_to(&mut self, v: fields::CxlRejResponseTo) {
        self.msg.body_mut().set_field(tags::CXL_REJ_RESPONSE_TO, v.to_fix().to_string());
    }

    /// `CxlRejResponseTo` (tag 434, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::CxlRejResponseTo`]'s.
    pub fn cxl_rej_response_to(&self) -> Result<fields::CxlRejResponseTo, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::CXL_REJ_RESPONSE_TO)?;
        fields::CxlRejResponseTo::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::CXL_REJ_RESPONSE_TO,
        })
    }

    /// Whether `CxlRejResponseTo` (tag 434) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cxl_rej_response_to(&self) -> bool {
        self.msg.body().has_field(tags::CXL_REJ_RESPONSE_TO)
    }

    // --- CxlRejReason (tag 102, INT, optional) ---

    /// Set `CxlRejReason` (tag 102, optional).
    pub fn set_cxl_rej_reason(&mut self, v: fields::CxlRejReason) {
        self.msg.body_mut().set_field(tags::CXL_REJ_REASON, v.to_fix().to_string());
    }

    /// `CxlRejReason` (tag 102, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::CxlRejReason`]'s.
    pub fn cxl_rej_reason(&self) -> Result<fields::CxlRejReason, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::CXL_REJ_REASON)?;
        fields::CxlRejReason::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::CXL_REJ_REASON,
        })
    }

    /// Whether `CxlRejReason` (tag 102) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cxl_rej_reason(&self) -> bool {
        self.msg.body().has_field(tags::CXL_REJ_REASON)
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

    /// The underlying raw message (e.g. for fields this facade has no accessor for).
    pub fn message(&self) -> &Message {
        &self.msg
    }

    /// Mutable access to the underlying raw message (e.g. to set a custom field, D1a).
    pub fn message_mut(&mut self) -> &mut Message {
        &mut self.msg
    }
}
