//! `TradingSessionStatusRequest` (app message, MsgType `35=g`): typed facade over the raw
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

/// Typed facade over a `TradingSessionStatusRequest` [`Message`] (35=g). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct TradingSessionStatusRequest {
    msg: Message,
}

impl Default for TradingSessionStatusRequest {
    fn default() -> Self {
        TradingSessionStatusRequest::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `TradingSessionStatusRequest`, checking the msg type.
impl TryFrom<Message> for TradingSessionStatusRequest {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a TradingSessionStatusRequest" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<TradingSessionStatusRequest> for Message {
    fn from(typed_msg: TradingSessionStatusRequest) -> Self {
        typed_msg.msg
    }
}

impl TradingSessionStatusRequest {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "g";

    /// An empty `TradingSessionStatusRequest` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- TradSesReqID (tag 335, STRING, required) ---

    /// Set `TradSesReqID` (tag 335, required).
    pub fn set_trad_ses_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TRAD_SES_REQ_ID, v.to_string());
    }

    /// `TradSesReqID` (tag 335, required).
    pub fn trad_ses_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TRAD_SES_REQ_ID)
    }

    /// Whether `TradSesReqID` (tag 335) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trad_ses_req_id(&self) -> bool {
        self.msg.body().has_field(tags::TRAD_SES_REQ_ID)
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

    // --- SubscriptionRequestType (tag 263, CHAR, required) ---

    /// Set `SubscriptionRequestType` (tag 263, required).
    pub fn set_subscription_request_type(&mut self, v: fields::SubscriptionRequestType) {
        self.msg.body_mut().set_field(tags::SUBSCRIPTION_REQUEST_TYPE, v.to_fix().to_string());
    }

    /// `SubscriptionRequestType` (tag 263, required). `Err(InvalidFormat)` if the value isn't one
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

    /// The underlying raw message (e.g. for fields this facade has no accessor for).
    pub fn message(&self) -> &Message {
        &self.msg
    }

    /// Mutable access to the underlying raw message (e.g. to set a custom field, D1a).
    pub fn message_mut(&mut self) -> &mut Message {
        &mut self.msg
    }
}
