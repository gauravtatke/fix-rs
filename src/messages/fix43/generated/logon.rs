//! `Logon` (admin message, MsgType `35=A`): typed facade over the raw
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

/// Typed facade over a `Logon` [`Message`] (35=A). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct Logon {
    msg: Message,
}

impl Default for Logon {
    fn default() -> Self {
        Logon::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `Logon`, checking the msg type.
impl TryFrom<Message> for Logon {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a Logon" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<Logon> for Message {
    fn from(typed_msg: Logon) -> Self {
        typed_msg.msg
    }
}

impl Logon {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "A";

    /// An empty `Logon` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- EncryptMethod (tag 98, INT, required) ---

    /// Set `EncryptMethod` (tag 98, required).
    pub fn set_encrypt_method(&mut self, v: fields::EncryptMethod) {
        self.msg.body_mut().set_field(tags::ENCRYPT_METHOD, v.to_fix().to_string());
    }

    /// `EncryptMethod` (tag 98, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::EncryptMethod`]'s.
    pub fn encrypt_method(&self) -> Result<fields::EncryptMethod, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::ENCRYPT_METHOD)?;
        fields::EncryptMethod::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ENCRYPT_METHOD,
        })
    }

    /// Whether `EncryptMethod` (tag 98) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encrypt_method(&self) -> bool {
        self.msg.body().has_field(tags::ENCRYPT_METHOD)
    }

    // --- HeartBtInt (tag 108, INT, required) ---

    /// Set `HeartBtInt` (tag 108, required).
    pub fn set_heart_bt_int(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::HEART_BT_INT, v.to_string());
    }

    /// `HeartBtInt` (tag 108, required).
    pub fn heart_bt_int(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::HEART_BT_INT)
    }

    /// Whether `HeartBtInt` (tag 108) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_heart_bt_int(&self) -> bool {
        self.msg.body().has_field(tags::HEART_BT_INT)
    }

    // --- RawDataLength (tag 95, LENGTH, optional) ---

    /// Set `RawDataLength` (tag 95, optional).
    pub fn set_raw_data_length(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::RAW_DATA_LENGTH, v.to_string());
    }

    /// `RawDataLength` (tag 95, optional).
    pub fn raw_data_length(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::RAW_DATA_LENGTH)
    }

    /// Whether `RawDataLength` (tag 95) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_raw_data_length(&self) -> bool {
        self.msg.body().has_field(tags::RAW_DATA_LENGTH)
    }

    // --- RawData (tag 96, DATA, optional) ---

    /// Set `RawData` (tag 96, optional).
    pub fn set_raw_data(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::RAW_DATA, v.to_string());
    }

    /// `RawData` (tag 96, optional).
    pub fn raw_data(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::RAW_DATA)
    }

    /// Whether `RawData` (tag 96) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_raw_data(&self) -> bool {
        self.msg.body().has_field(tags::RAW_DATA)
    }

    // --- ResetSeqNumFlag (tag 141, BOOLEAN, optional) ---

    /// Set `ResetSeqNumFlag` (tag 141, optional); written as `Y`/`N`.
    pub fn set_reset_seq_num_flag(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::RESET_SEQ_NUM_FLAG, convert::fix_bool_str(v));
    }

    /// `ResetSeqNumFlag` (tag 141, optional). `Err(InvalidFormat)` unless `Y`/`N`.
    pub fn reset_seq_num_flag(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::RESET_SEQ_NUM_FLAG)?;
        convert::parse_fix_bool(&raw, tags::RESET_SEQ_NUM_FLAG)
    }

    /// Whether `ResetSeqNumFlag` (tag 141) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_reset_seq_num_flag(&self) -> bool {
        self.msg.body().has_field(tags::RESET_SEQ_NUM_FLAG)
    }

    // --- MaxMessageSize (tag 383, LENGTH, optional) ---

    /// Set `MaxMessageSize` (tag 383, optional).
    pub fn set_max_message_size(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::MAX_MESSAGE_SIZE, v.to_string());
    }

    /// `MaxMessageSize` (tag 383, optional).
    pub fn max_message_size(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::MAX_MESSAGE_SIZE)
    }

    /// Whether `MaxMessageSize` (tag 383) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_max_message_size(&self) -> bool {
        self.msg.body().has_field(tags::MAX_MESSAGE_SIZE)
    }

    // --- TestMessageIndicator (tag 464, BOOLEAN, optional) ---

    /// Set `TestMessageIndicator` (tag 464, optional); written as `Y`/`N`.
    pub fn set_test_message_indicator(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::TEST_MESSAGE_INDICATOR, convert::fix_bool_str(v));
    }

    /// `TestMessageIndicator` (tag 464, optional). `Err(InvalidFormat)` unless `Y`/`N`.
    pub fn test_message_indicator(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TEST_MESSAGE_INDICATOR)?;
        convert::parse_fix_bool(&raw, tags::TEST_MESSAGE_INDICATOR)
    }

    /// Whether `TestMessageIndicator` (tag 464) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_test_message_indicator(&self) -> bool {
        self.msg.body().has_field(tags::TEST_MESSAGE_INDICATOR)
    }

    // --- Username (tag 553, STRING, optional) ---

    /// Set `Username` (tag 553, optional).
    pub fn set_username(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::USERNAME, v.to_string());
    }

    /// `Username` (tag 553, optional).
    pub fn username(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::USERNAME)
    }

    /// Whether `Username` (tag 553) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_username(&self) -> bool {
        self.msg.body().has_field(tags::USERNAME)
    }

    // --- Password (tag 554, STRING, optional) ---

    /// Set `Password` (tag 554, optional).
    pub fn set_password(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::PASSWORD, v.to_string());
    }

    /// `Password` (tag 554, optional).
    pub fn password(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::PASSWORD)
    }

    /// Whether `Password` (tag 554) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_password(&self) -> bool {
        self.msg.body().has_field(tags::PASSWORD)
    }

    // --- NoMsgTypes group (count tag NO_MSG_TYPES) ---

    /// The `NoMsgTypes` group (count tag `NO_MSG_TYPES`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_msg_types(&self) -> Option<NoMsgTypes<'_>> {
        self.msg.body().get_group(tags::NO_MSG_TYPES).map(|group| NoMsgTypes { group })
    }

    /// The `NoMsgTypes` group (count tag `NO_MSG_TYPES`, optional) for editing, or `None` if
    /// absent.
    pub fn no_msg_types_mut(&mut self) -> Option<NoMsgTypesMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_MSG_TYPES).map(|group| NoMsgTypesMut { group })
    }

    /// Create the `NoMsgTypes` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_msg_types_entry`] on the same group.
    pub fn set_no_msg_types(&mut self, count: u32) -> NoMsgTypesMut<'_> {
        let group =
            self.msg.body_mut().set_group(tags::NO_MSG_TYPES, count, NO_MSG_TYPES_FIELD_ORDER);
        NoMsgTypesMut { group }
    }

    /// Append one `NoMsgTypes` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_msg_types`] on the same group.
    pub fn add_no_msg_types_entry(&mut self, fill: impl FnOnce(&mut NoMsgTypesEntryMut<'_>)) {
        let inner =
            self.msg.body_mut().add_group_instance(tags::NO_MSG_TYPES, NO_MSG_TYPES_FIELD_ORDER);
        fill(&mut NoMsgTypesEntryMut { inner });
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

/// Wire order of a `NoMsgTypes` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_MSG_TYPES_FIELD_ORDER: &[u32] = &[tags::REF_MSG_TYPE, tags::MSG_DIRECTION];

/// One `NoMsgTypes` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoMsgTypesEntry<'a> {
    inner: &'a FieldMap,
}

impl NoMsgTypesEntry<'_> {
    /// `RefMsgType` (tag 372, optional).
    pub fn ref_msg_type(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::REF_MSG_TYPE)
    }

    /// Whether `RefMsgType` (tag 372) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ref_msg_type(&self) -> bool {
        self.inner.has_field(tags::REF_MSG_TYPE)
    }

    /// `MsgDirection` (tag 385, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MsgDirection`]'s.
    pub fn msg_direction(&self) -> Result<fields::MsgDirection, FieldError> {
        let n = self.inner.get_field::<char>(tags::MSG_DIRECTION)?;
        fields::MsgDirection::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MSG_DIRECTION,
        })
    }

    /// Whether `MsgDirection` (tag 385) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_msg_direction(&self) -> bool {
        self.inner.has_field(tags::MSG_DIRECTION)
    }
}

/// One `NoMsgTypes` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoMsgTypesEntryMut::as_ref`].
pub struct NoMsgTypesEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoMsgTypesEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoMsgTypesEntry<'_> {
        NoMsgTypesEntry { inner: self.inner }
    }

    /// Set `RefMsgType` (tag 372, optional).
    pub fn set_ref_msg_type(&mut self, v: &str) {
        self.inner.set_field(tags::REF_MSG_TYPE, v.to_string());
    }

    /// Set `MsgDirection` (tag 385, optional).
    pub fn set_msg_direction(&mut self, v: fields::MsgDirection) {
        self.inner.set_field(tags::MSG_DIRECTION, v.to_fix().to_string());
    }
}

/// The `NoMsgTypes` repeating group (count tag `NO_MSG_TYPES`), read-only: its
/// instances as [`NoMsgTypesEntry`] views.
pub struct NoMsgTypes<'a> {
    group: &'a Group,
}

impl<'a> NoMsgTypes<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoMsgTypesEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoMsgTypesEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoMsgTypesEntry<'a> {
        self.get(i).expect("NoMsgTypes index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoMsgTypesEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoMsgTypesEntry { inner })
    }
}

/// The `NoMsgTypes` repeating group (count tag `NO_MSG_TYPES`), mutable: its
/// instances as [`NoMsgTypesEntryMut`] views.
pub struct NoMsgTypesMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoMsgTypesMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoMsgTypesEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoMsgTypesEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoMsgTypesEntryMut<'_> {
        self.get_mut(i).expect("NoMsgTypes index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoMsgTypesEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoMsgTypesEntryMut { inner })
    }
}
