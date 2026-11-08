//! `RegistrationInstructionsResponse` (app message, MsgType `35=p`): typed facade over the raw
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

/// Typed facade over a `RegistrationInstructionsResponse` [`Message`] (35=p). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct RegistrationInstructionsResponse {
    msg: Message,
}

impl Default for RegistrationInstructionsResponse {
    fn default() -> Self {
        RegistrationInstructionsResponse::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `RegistrationInstructionsResponse`, checking the msg type.
impl TryFrom<Message> for RegistrationInstructionsResponse {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a RegistrationInstructionsResponse" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<RegistrationInstructionsResponse> for Message {
    fn from(typed_msg: RegistrationInstructionsResponse) -> Self {
        typed_msg.msg
    }
}

impl RegistrationInstructionsResponse {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "p";

    /// An empty `RegistrationInstructionsResponse` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- RegistID (tag 513, STRING, required) ---

    /// Set `RegistID` (tag 513, required).
    pub fn set_regist_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REGIST_ID, v.to_string());
    }

    /// `RegistID` (tag 513, required).
    pub fn regist_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REGIST_ID)
    }

    /// Whether `RegistID` (tag 513) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_id(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_ID)
    }

    // --- RegistTransType (tag 514, CHAR, required) ---

    /// Set `RegistTransType` (tag 514, required).
    pub fn set_regist_trans_type(&mut self, v: fields::RegistTransType) {
        self.msg.body_mut().set_field(tags::REGIST_TRANS_TYPE, v.to_fix().to_string());
    }

    /// `RegistTransType` (tag 514, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::RegistTransType`]'s.
    pub fn regist_trans_type(&self) -> Result<fields::RegistTransType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::REGIST_TRANS_TYPE)?;
        fields::RegistTransType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::REGIST_TRANS_TYPE,
        })
    }

    /// Whether `RegistTransType` (tag 514) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_trans_type(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_TRANS_TYPE)
    }

    // --- RegistRefID (tag 508, STRING, required) ---

    /// Set `RegistRefID` (tag 508, required).
    pub fn set_regist_ref_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REGIST_REF_ID, v.to_string());
    }

    /// `RegistRefID` (tag 508, required).
    pub fn regist_ref_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REGIST_REF_ID)
    }

    /// Whether `RegistRefID` (tag 508) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_ref_id(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_REF_ID)
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

    // --- RegistStatus (tag 506, CHAR, required) ---

    /// Set `RegistStatus` (tag 506, required).
    pub fn set_regist_status(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::REGIST_STATUS, v.to_string());
    }

    /// `RegistStatus` (tag 506, required).
    pub fn regist_status(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::REGIST_STATUS)
    }

    /// Whether `RegistStatus` (tag 506) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_status(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_STATUS)
    }

    // --- RegistRejReasonCode (tag 507, INT, optional) ---

    /// Set `RegistRejReasonCode` (tag 507, optional).
    pub fn set_regist_rej_reason_code(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::REGIST_REJ_REASON_CODE, v.to_string());
    }

    /// `RegistRejReasonCode` (tag 507, optional).
    pub fn regist_rej_reason_code(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::REGIST_REJ_REASON_CODE)
    }

    /// Whether `RegistRejReasonCode` (tag 507) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_rej_reason_code(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_REJ_REASON_CODE)
    }

    // --- RegistRejReasonText (tag 496, STRING, optional) ---

    /// Set `RegistRejReasonText` (tag 496, optional).
    pub fn set_regist_rej_reason_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REGIST_REJ_REASON_TEXT, v.to_string());
    }

    /// `RegistRejReasonText` (tag 496, optional).
    pub fn regist_rej_reason_text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REGIST_REJ_REASON_TEXT)
    }

    /// Whether `RegistRejReasonText` (tag 496) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_rej_reason_text(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_REJ_REASON_TEXT)
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
