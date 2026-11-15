//! `SecurityTypes` (app message, MsgType `35=w`): typed facade over the raw
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

/// Typed facade over a `SecurityTypes` [`Message`] (35=w). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct SecurityTypes {
    msg: Message,
}

impl Default for SecurityTypes {
    fn default() -> Self {
        SecurityTypes::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `SecurityTypes`, checking the msg type.
impl TryFrom<Message> for SecurityTypes {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a SecurityTypes" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<SecurityTypes> for Message {
    fn from(typed_msg: SecurityTypes) -> Self {
        typed_msg.msg
    }
}

impl SecurityTypes {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "w";

    /// An empty `SecurityTypes` with only MsgType (35) set.
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

    // --- TotalNumSecurityTypes (tag 557, INT, optional) ---

    /// Set `TotalNumSecurityTypes` (tag 557, optional).
    pub fn set_total_num_security_types(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::TOTAL_NUM_SECURITY_TYPES, v.to_string());
    }

    /// `TotalNumSecurityTypes` (tag 557, optional).
    pub fn total_num_security_types(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::TOTAL_NUM_SECURITY_TYPES)
    }

    /// Whether `TotalNumSecurityTypes` (tag 557) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_num_security_types(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_NUM_SECURITY_TYPES)
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

    // --- NoSecurityTypes group (count tag NO_SECURITY_TYPES) ---

    /// The `NoSecurityTypes` group (count tag `NO_SECURITY_TYPES`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_security_types(&self) -> Option<NoSecurityTypes<'_>> {
        self.msg.body().get_group(tags::NO_SECURITY_TYPES).map(|group| NoSecurityTypes { group })
    }

    /// The `NoSecurityTypes` group (count tag `NO_SECURITY_TYPES`, optional) for editing, or `None` if
    /// absent.
    pub fn no_security_types_mut(&mut self) -> Option<NoSecurityTypesMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_SECURITY_TYPES)
            .map(|group| NoSecurityTypesMut { group })
    }

    /// Create the `NoSecurityTypes` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_security_types_entry`] on the same group.
    pub fn set_no_security_types(&mut self, count: u32) -> NoSecurityTypesMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_SECURITY_TYPES,
            count,
            NO_SECURITY_TYPES_FIELD_ORDER,
        );
        NoSecurityTypesMut { group }
    }

    /// Append one `NoSecurityTypes` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_security_types`] on the same group.
    pub fn add_no_security_types_entry(
        &mut self,
        fill: impl FnOnce(&mut NoSecurityTypesEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_SECURITY_TYPES, NO_SECURITY_TYPES_FIELD_ORDER);
        fill(&mut NoSecurityTypesEntryMut { inner });
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

/// Wire order of a `NoSecurityTypes` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_SECURITY_TYPES_FIELD_ORDER: &[u32] = &[tags::SECURITY_TYPE, tags::PRODUCT, tags::CFI_CODE];

/// One `NoSecurityTypes` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoSecurityTypesEntry<'a> {
    inner: &'a FieldMap,
}

impl NoSecurityTypesEntry<'_> {
    /// `SecurityType` (tag 167, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityType`]'s.
    pub fn security_type(&self) -> Result<fields::SecurityType, FieldError> {
        let raw = self.inner.get_field::<String>(tags::SECURITY_TYPE)?;
        fields::SecurityType::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_TYPE,
        })
    }

    /// Whether `SecurityType` (tag 167) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_type(&self) -> bool {
        self.inner.has_field(tags::SECURITY_TYPE)
    }

    /// `Product` (tag 460, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Product`]'s.
    pub fn product(&self) -> Result<fields::Product, FieldError> {
        let n = self.inner.get_field::<i32>(tags::PRODUCT)?;
        fields::Product::from_fix(n).ok_or(FieldError::InvalidFormat { tag: tags::PRODUCT })
    }

    /// Whether `Product` (tag 460) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_product(&self) -> bool {
        self.inner.has_field(tags::PRODUCT)
    }

    /// `CFICode` (tag 461, optional).
    pub fn cfi_code(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CFI_CODE)
    }

    /// Whether `CFICode` (tag 461) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cfi_code(&self) -> bool {
        self.inner.has_field(tags::CFI_CODE)
    }
}

/// One `NoSecurityTypes` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoSecurityTypesEntryMut::as_ref`].
pub struct NoSecurityTypesEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoSecurityTypesEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoSecurityTypesEntry<'_> {
        NoSecurityTypesEntry { inner: self.inner }
    }

    /// Set `SecurityType` (tag 167, optional).
    pub fn set_security_type(&mut self, v: fields::SecurityType) {
        self.inner.set_field(tags::SECURITY_TYPE, v.to_fix().to_string());
    }

    /// Set `Product` (tag 460, optional).
    pub fn set_product(&mut self, v: fields::Product) {
        self.inner.set_field(tags::PRODUCT, v.to_fix().to_string());
    }

    /// Set `CFICode` (tag 461, optional).
    pub fn set_cfi_code(&mut self, v: &str) {
        self.inner.set_field(tags::CFI_CODE, v.to_string());
    }
}

/// The `NoSecurityTypes` repeating group (count tag `NO_SECURITY_TYPES`), read-only: its
/// instances as [`NoSecurityTypesEntry`] views.
pub struct NoSecurityTypes<'a> {
    group: &'a Group,
}

impl<'a> NoSecurityTypes<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoSecurityTypesEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoSecurityTypesEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoSecurityTypesEntry<'a> {
        self.get(i).expect("NoSecurityTypes index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoSecurityTypesEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoSecurityTypesEntry { inner })
    }
}

/// The `NoSecurityTypes` repeating group (count tag `NO_SECURITY_TYPES`), mutable: its
/// instances as [`NoSecurityTypesEntryMut`] views.
pub struct NoSecurityTypesMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoSecurityTypesMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoSecurityTypesEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoSecurityTypesEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoSecurityTypesEntryMut<'_> {
        self.get_mut(i).expect("NoSecurityTypes index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoSecurityTypesEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoSecurityTypesEntryMut { inner })
    }
}
