//! `TradingSessionStatus` (app message, MsgType `35=h`): typed facade over the raw
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

/// Typed facade over a `TradingSessionStatus` [`Message`] (35=h). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct TradingSessionStatus {
    msg: Message,
}

impl Default for TradingSessionStatus {
    fn default() -> Self {
        TradingSessionStatus::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `TradingSessionStatus`, checking the msg type.
impl TryFrom<Message> for TradingSessionStatus {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a TradingSessionStatus" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<TradingSessionStatus> for Message {
    fn from(typed_msg: TradingSessionStatus) -> Self {
        typed_msg.msg
    }
}

impl TradingSessionStatus {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "h";

    /// An empty `TradingSessionStatus` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- TradSesReqID (tag 335, STRING, optional) ---

    /// Set `TradSesReqID` (tag 335, optional).
    pub fn set_trad_ses_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRAD_SES_REQ_ID, v.to_string());
    }

    /// `TradSesReqID` (tag 335, optional).
    pub fn trad_ses_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRAD_SES_REQ_ID)
    }

    /// Whether `TradSesReqID` (tag 335) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_req_id(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_REQ_ID)
    }

    // --- TradingSessionID (tag 336, STRING, required) ---

    /// Set `TradingSessionID` (tag 336, required).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// `TradingSessionID` (tag 336, required).
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

    // --- TradSesMethod (tag 338, INT, optional) ---

    /// Set `TradSesMethod` (tag 338, optional).
    pub fn set_trad_ses_method(&mut self, v: fields::TradSesMethod) {
        self.msg.body_mut().set_field(tags::TRAD_SES_METHOD, v.to_fix().to_string());
    }

    /// `TradSesMethod` (tag 338, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TradSesMethod`]'s.
    pub fn trad_ses_method(&self) -> Result<fields::TradSesMethod, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::TRAD_SES_METHOD)?;
        fields::TradSesMethod::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TRAD_SES_METHOD,
        })
    }

    /// Whether `TradSesMethod` (tag 338) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_method(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_METHOD)
    }

    // --- TradSesMode (tag 339, INT, optional) ---

    /// Set `TradSesMode` (tag 339, optional).
    pub fn set_trad_ses_mode(&mut self, v: fields::TradSesMode) {
        self.msg.body_mut().set_field(tags::TRAD_SES_MODE, v.to_fix().to_string());
    }

    /// `TradSesMode` (tag 339, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TradSesMode`]'s.
    pub fn trad_ses_mode(&self) -> Result<fields::TradSesMode, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::TRAD_SES_MODE)?;
        fields::TradSesMode::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TRAD_SES_MODE,
        })
    }

    /// Whether `TradSesMode` (tag 339) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_mode(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_MODE)
    }

    // --- UnsolicitedIndicator (tag 325, BOOLEAN, optional) ---

    /// Set `UnsolicitedIndicator` (tag 325, optional); written in FIX `BOOLEAN` format.
    pub fn set_unsolicited_indicator(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::UNSOLICITED_INDICATOR, convert::format_fix_bool(v));
    }

    /// `UnsolicitedIndicator` (tag 325, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn unsolicited_indicator(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::UNSOLICITED_INDICATOR)?;
        convert::parse_fix_bool(&raw, tags::UNSOLICITED_INDICATOR)
    }

    /// Whether `UnsolicitedIndicator` (tag 325) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_unsolicited_indicator(&self) -> bool {
        self.msg.body().has_field(tags::UNSOLICITED_INDICATOR)
    }

    // --- TradSesStatus (tag 340, INT, required) ---

    /// Set `TradSesStatus` (tag 340, required).
    pub fn set_trad_ses_status(&mut self, v: fields::TradSesStatus) {
        self.msg.body_mut().set_field(tags::TRAD_SES_STATUS, v.to_fix().to_string());
    }

    /// `TradSesStatus` (tag 340, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TradSesStatus`]'s.
    pub fn trad_ses_status(&self) -> Result<fields::TradSesStatus, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::TRAD_SES_STATUS)?;
        fields::TradSesStatus::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TRAD_SES_STATUS,
        })
    }

    /// Whether `TradSesStatus` (tag 340) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_status(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_STATUS)
    }

    // --- TradSesStatusRejReason (tag 567, INT, optional) ---

    /// Set `TradSesStatusRejReason` (tag 567, optional).
    pub fn set_trad_ses_status_rej_reason(&mut self, v: fields::TradSesStatusRejReason) {
        self.msg.body_mut().set_field(tags::TRAD_SES_STATUS_REJ_REASON, v.to_fix().to_string());
    }

    /// `TradSesStatusRejReason` (tag 567, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::TradSesStatusRejReason`]'s.
    pub fn trad_ses_status_rej_reason(&self) -> Result<fields::TradSesStatusRejReason, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::TRAD_SES_STATUS_REJ_REASON)?;
        fields::TradSesStatusRejReason::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::TRAD_SES_STATUS_REJ_REASON,
        })
    }

    /// Whether `TradSesStatusRejReason` (tag 567) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_status_rej_reason(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_STATUS_REJ_REASON)
    }

    // --- TradSesStartTime (tag 341, UTCTIMESTAMP, optional) ---

    /// Set `TradSesStartTime` (tag 341, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_trad_ses_start_time(&mut self, v: DateTime<Utc>) {
        self.msg
            .body_mut()
            .set_field(tags::TRAD_SES_START_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TradSesStartTime` (tag 341, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn trad_ses_start_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRAD_SES_START_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRAD_SES_START_TIME)
    }

    /// Whether `TradSesStartTime` (tag 341) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_start_time(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_START_TIME)
    }

    // --- TradSesOpenTime (tag 342, UTCTIMESTAMP, optional) ---

    /// Set `TradSesOpenTime` (tag 342, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_trad_ses_open_time(&mut self, v: DateTime<Utc>) {
        self.msg
            .body_mut()
            .set_field(tags::TRAD_SES_OPEN_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TradSesOpenTime` (tag 342, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn trad_ses_open_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRAD_SES_OPEN_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRAD_SES_OPEN_TIME)
    }

    /// Whether `TradSesOpenTime` (tag 342) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_open_time(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_OPEN_TIME)
    }

    // --- TradSesPreCloseTime (tag 343, UTCTIMESTAMP, optional) ---

    /// Set `TradSesPreCloseTime` (tag 343, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_trad_ses_pre_close_time(&mut self, v: DateTime<Utc>) {
        self.msg
            .body_mut()
            .set_field(tags::TRAD_SES_PRE_CLOSE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TradSesPreCloseTime` (tag 343, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn trad_ses_pre_close_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRAD_SES_PRE_CLOSE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRAD_SES_PRE_CLOSE_TIME)
    }

    /// Whether `TradSesPreCloseTime` (tag 343) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_pre_close_time(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_PRE_CLOSE_TIME)
    }

    // --- TradSesCloseTime (tag 344, UTCTIMESTAMP, optional) ---

    /// Set `TradSesCloseTime` (tag 344, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_trad_ses_close_time(&mut self, v: DateTime<Utc>) {
        self.msg
            .body_mut()
            .set_field(tags::TRAD_SES_CLOSE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TradSesCloseTime` (tag 344, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn trad_ses_close_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRAD_SES_CLOSE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRAD_SES_CLOSE_TIME)
    }

    /// Whether `TradSesCloseTime` (tag 344) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_close_time(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_CLOSE_TIME)
    }

    // --- TradSesEndTime (tag 345, UTCTIMESTAMP, optional) ---

    /// Set `TradSesEndTime` (tag 345, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_trad_ses_end_time(&mut self, v: DateTime<Utc>) {
        self.msg
            .body_mut()
            .set_field(tags::TRAD_SES_END_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TradSesEndTime` (tag 345, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn trad_ses_end_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRAD_SES_END_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRAD_SES_END_TIME)
    }

    /// Whether `TradSesEndTime` (tag 345) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_end_time(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_END_TIME)
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
