//! `RegistrationInstructions` (app message, MsgType `35=o`): typed facade over the raw
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

/// Typed facade over a `RegistrationInstructions` [`Message`] (35=o). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct RegistrationInstructions {
    msg: Message,
}

impl Default for RegistrationInstructions {
    fn default() -> Self {
        RegistrationInstructions::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `RegistrationInstructions`, checking the msg type.
impl TryFrom<Message> for RegistrationInstructions {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a RegistrationInstructions" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<RegistrationInstructions> for Message {
    fn from(typed_msg: RegistrationInstructions) -> Self {
        typed_msg.msg
    }
}

impl RegistrationInstructions {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "o";

    /// An empty `RegistrationInstructions` with only MsgType (35) set.
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

    // --- RegistAcctType (tag 493, STRING, optional) ---

    /// Set `RegistAcctType` (tag 493, optional).
    pub fn set_regist_acct_type(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REGIST_ACCT_TYPE, v.to_string());
    }

    /// `RegistAcctType` (tag 493, optional).
    pub fn regist_acct_type(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REGIST_ACCT_TYPE)
    }

    /// Whether `RegistAcctType` (tag 493) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_acct_type(&self) -> bool {
        self.msg.body().has_field(tags::REGIST_ACCT_TYPE)
    }

    // --- TaxAdvantageType (tag 495, INT, optional) ---

    /// Set `TaxAdvantageType` (tag 495, optional).
    pub fn set_tax_advantage_type(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::TAX_ADVANTAGE_TYPE, v.to_string());
    }

    /// `TaxAdvantageType` (tag 495, optional).
    pub fn tax_advantage_type(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::TAX_ADVANTAGE_TYPE)
    }

    /// Whether `TaxAdvantageType` (tag 495) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_tax_advantage_type(&self) -> bool {
        self.msg.body().has_field(tags::TAX_ADVANTAGE_TYPE)
    }

    // --- OwnershipType (tag 517, CHAR, optional) ---

    /// Set `OwnershipType` (tag 517, optional).
    pub fn set_ownership_type(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::OWNERSHIP_TYPE, v.to_string());
    }

    /// `OwnershipType` (tag 517, optional).
    pub fn ownership_type(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::OWNERSHIP_TYPE)
    }

    /// Whether `OwnershipType` (tag 517) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ownership_type(&self) -> bool {
        self.msg.body().has_field(tags::OWNERSHIP_TYPE)
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

    // --- NoRegistDtls group (count tag NO_REGIST_DTLS) ---

    /// The `NoRegistDtls` group (count tag `NO_REGIST_DTLS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_regist_dtls(&self) -> Option<NoRegistDtls<'_>> {
        self.msg.body().get_group(tags::NO_REGIST_DTLS).map(|group| NoRegistDtls { group })
    }

    /// The `NoRegistDtls` group (count tag `NO_REGIST_DTLS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_regist_dtls_mut(&mut self) -> Option<NoRegistDtlsMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_REGIST_DTLS)
            .map(|group| NoRegistDtlsMut { group })
    }

    /// Create the `NoRegistDtls` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_regist_dtls_entry`] on the same group.
    pub fn set_no_regist_dtls(&mut self, count: u32) -> NoRegistDtlsMut<'_> {
        let group =
            self.msg.body_mut().set_group(tags::NO_REGIST_DTLS, count, NO_REGIST_DTLS_FIELD_ORDER);
        NoRegistDtlsMut { group }
    }

    /// Append one `NoRegistDtls` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_regist_dtls`] on the same group.
    pub fn add_no_regist_dtls_entry(&mut self, fill: impl FnOnce(&mut NoRegistDtlsEntryMut<'_>)) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_REGIST_DTLS, NO_REGIST_DTLS_FIELD_ORDER);
        fill(&mut NoRegistDtlsEntryMut { inner });
    }

    // --- NoDistribInsts group (count tag NO_DISTRIB_INSTS) ---

    /// The `NoDistribInsts` group (count tag `NO_DISTRIB_INSTS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_distrib_insts(&self) -> Option<NoDistribInsts<'_>> {
        self.msg.body().get_group(tags::NO_DISTRIB_INSTS).map(|group| NoDistribInsts { group })
    }

    /// The `NoDistribInsts` group (count tag `NO_DISTRIB_INSTS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_distrib_insts_mut(&mut self) -> Option<NoDistribInstsMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_DISTRIB_INSTS)
            .map(|group| NoDistribInstsMut { group })
    }

    /// Create the `NoDistribInsts` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_distrib_insts_entry`] on the same group.
    pub fn set_no_distrib_insts(&mut self, count: u32) -> NoDistribInstsMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_DISTRIB_INSTS,
            count,
            NO_DISTRIB_INSTS_FIELD_ORDER,
        );
        NoDistribInstsMut { group }
    }

    /// Append one `NoDistribInsts` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_distrib_insts`] on the same group.
    pub fn add_no_distrib_insts_entry(
        &mut self,
        fill: impl FnOnce(&mut NoDistribInstsEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_DISTRIB_INSTS, NO_DISTRIB_INSTS_FIELD_ORDER);
        fill(&mut NoDistribInstsEntryMut { inner });
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

/// Wire order of a `NoRegistDtls` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_REGIST_DTLS_FIELD_ORDER: &[u32] = &[
    tags::REGIST_DETLS,
    tags::REGIST_EMAIL,
    tags::MAILING_DTLS,
    tags::MAILING_INST,
    tags::NO_NESTED_PARTY_IDS,
    tags::OWNER_TYPE,
    tags::DATE_OF_BIRTH,
    tags::INVESTOR_COUNTRY_OF_RESIDENCE,
];

/// One `NoRegistDtls` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoRegistDtlsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoRegistDtlsEntry<'_> {
    /// `RegistDetls` (tag 509, optional).
    pub fn regist_detls(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::REGIST_DETLS)
    }

    /// Whether `RegistDetls` (tag 509) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_detls(&self) -> bool {
        self.inner.has_field(tags::REGIST_DETLS)
    }

    /// `RegistEmail` (tag 511, optional).
    pub fn regist_email(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::REGIST_EMAIL)
    }

    /// Whether `RegistEmail` (tag 511) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_regist_email(&self) -> bool {
        self.inner.has_field(tags::REGIST_EMAIL)
    }

    /// `MailingDtls` (tag 474, optional).
    pub fn mailing_dtls(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MAILING_DTLS)
    }

    /// Whether `MailingDtls` (tag 474) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mailing_dtls(&self) -> bool {
        self.inner.has_field(tags::MAILING_DTLS)
    }

    /// `MailingInst` (tag 482, optional).
    pub fn mailing_inst(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MAILING_INST)
    }

    /// Whether `MailingInst` (tag 482) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mailing_inst(&self) -> bool {
        self.inner.has_field(tags::MAILING_INST)
    }

    /// `OwnerType` (tag 522, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OwnerType`]'s.
    pub fn owner_type(&self) -> Result<fields::OwnerType, FieldError> {
        let n = self.inner.get_field::<i32>(tags::OWNER_TYPE)?;
        fields::OwnerType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::OWNER_TYPE,
        })
    }

    /// Whether `OwnerType` (tag 522) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_owner_type(&self) -> bool {
        self.inner.has_field(tags::OWNER_TYPE)
    }

    /// `DateOfBirth` (tag 486, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn date_of_birth(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::DATE_OF_BIRTH)?;
        convert::parse_fix_date(&raw, tags::DATE_OF_BIRTH)
    }

    /// Whether `DateOfBirth` (tag 486) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_date_of_birth(&self) -> bool {
        self.inner.has_field(tags::DATE_OF_BIRTH)
    }

    /// `InvestorCountryOfResidence` (tag 475, optional).
    pub fn investor_country_of_residence(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::INVESTOR_COUNTRY_OF_RESIDENCE)
    }

    /// Whether `InvestorCountryOfResidence` (tag 475) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_investor_country_of_residence(&self) -> bool {
        self.inner.has_field(tags::INVESTOR_COUNTRY_OF_RESIDENCE)
    }

    /// The `NoNestedPartyIds` group (count tag `NO_NESTED_PARTY_IDS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_nested_party_ids(&self) -> Option<NoNestedPartyIds<'_>> {
        self.inner.get_group(tags::NO_NESTED_PARTY_IDS).map(|group| NoNestedPartyIds { group })
    }
}

/// One `NoRegistDtls` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoRegistDtlsEntryMut::as_ref`].
pub struct NoRegistDtlsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoRegistDtlsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoRegistDtlsEntry<'_> {
        NoRegistDtlsEntry { inner: self.inner }
    }

    /// Set `RegistDetls` (tag 509, optional).
    pub fn set_regist_detls(&mut self, v: &str) {
        self.inner.set_field(tags::REGIST_DETLS, v.to_string());
    }

    /// Set `RegistEmail` (tag 511, optional).
    pub fn set_regist_email(&mut self, v: &str) {
        self.inner.set_field(tags::REGIST_EMAIL, v.to_string());
    }

    /// Set `MailingDtls` (tag 474, optional).
    pub fn set_mailing_dtls(&mut self, v: &str) {
        self.inner.set_field(tags::MAILING_DTLS, v.to_string());
    }

    /// Set `MailingInst` (tag 482, optional).
    pub fn set_mailing_inst(&mut self, v: &str) {
        self.inner.set_field(tags::MAILING_INST, v.to_string());
    }

    /// Set `OwnerType` (tag 522, optional).
    pub fn set_owner_type(&mut self, v: fields::OwnerType) {
        self.inner.set_field(tags::OWNER_TYPE, v.to_fix().to_string());
    }

    /// Set `DateOfBirth` (tag 486, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_date_of_birth(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::DATE_OF_BIRTH, convert::format_fix_date(v));
    }

    /// Set `InvestorCountryOfResidence` (tag 475, optional).
    pub fn set_investor_country_of_residence(&mut self, v: &str) {
        self.inner.set_field(tags::INVESTOR_COUNTRY_OF_RESIDENCE, v.to_string());
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

/// The `NoRegistDtls` repeating group (count tag `NO_REGIST_DTLS`), read-only: its
/// instances as [`NoRegistDtlsEntry`] views.
pub struct NoRegistDtls<'a> {
    group: &'a Group,
}

impl<'a> NoRegistDtls<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoRegistDtlsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoRegistDtlsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoRegistDtlsEntry<'a> {
        self.get(i).expect("NoRegistDtls index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoRegistDtlsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoRegistDtlsEntry { inner })
    }
}

/// The `NoRegistDtls` repeating group (count tag `NO_REGIST_DTLS`), mutable: its
/// instances as [`NoRegistDtlsEntryMut`] views.
pub struct NoRegistDtlsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoRegistDtlsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoRegistDtlsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoRegistDtlsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoRegistDtlsEntryMut<'_> {
        self.get_mut(i).expect("NoRegistDtls index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoRegistDtlsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoRegistDtlsEntryMut { inner })
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

/// Wire order of a `NoDistribInsts` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_DISTRIB_INSTS_FIELD_ORDER: &[u32] = &[
    tags::DISTRIB_PAYMENT_METHOD,
    tags::DISTRIB_PERCENTAGE,
    tags::CASH_DISTRIB_CURR,
    tags::CASH_DISTRIB_AGENT_NAME,
    tags::CASH_DISTRIB_AGENT_CODE,
    tags::CASH_DISTRIB_AGENT_ACCT_NUMBER,
    tags::CASH_DISTRIB_PAY_REF,
    tags::CASH_DISTRIB_AGENT_ACCT_NAME,
];

/// One `NoDistribInsts` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoDistribInstsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoDistribInstsEntry<'_> {
    /// `DistribPaymentMethod` (tag 477, optional).
    pub fn distrib_payment_method(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::DISTRIB_PAYMENT_METHOD)
    }

    /// Whether `DistribPaymentMethod` (tag 477) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_distrib_payment_method(&self) -> bool {
        self.inner.has_field(tags::DISTRIB_PAYMENT_METHOD)
    }

    /// `DistribPercentage` (tag 512, optional).
    pub fn distrib_percentage(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::DISTRIB_PERCENTAGE)
    }

    /// Whether `DistribPercentage` (tag 512) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_distrib_percentage(&self) -> bool {
        self.inner.has_field(tags::DISTRIB_PERCENTAGE)
    }

    /// `CashDistribCurr` (tag 478, optional).
    pub fn cash_distrib_curr(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CASH_DISTRIB_CURR)
    }

    /// Whether `CashDistribCurr` (tag 478) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_distrib_curr(&self) -> bool {
        self.inner.has_field(tags::CASH_DISTRIB_CURR)
    }

    /// `CashDistribAgentName` (tag 498, optional).
    pub fn cash_distrib_agent_name(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CASH_DISTRIB_AGENT_NAME)
    }

    /// Whether `CashDistribAgentName` (tag 498) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_distrib_agent_name(&self) -> bool {
        self.inner.has_field(tags::CASH_DISTRIB_AGENT_NAME)
    }

    /// `CashDistribAgentCode` (tag 499, optional).
    pub fn cash_distrib_agent_code(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CASH_DISTRIB_AGENT_CODE)
    }

    /// Whether `CashDistribAgentCode` (tag 499) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_distrib_agent_code(&self) -> bool {
        self.inner.has_field(tags::CASH_DISTRIB_AGENT_CODE)
    }

    /// `CashDistribAgentAcctNumber` (tag 500, optional).
    pub fn cash_distrib_agent_acct_number(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CASH_DISTRIB_AGENT_ACCT_NUMBER)
    }

    /// Whether `CashDistribAgentAcctNumber` (tag 500) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_distrib_agent_acct_number(&self) -> bool {
        self.inner.has_field(tags::CASH_DISTRIB_AGENT_ACCT_NUMBER)
    }

    /// `CashDistribPayRef` (tag 501, optional).
    pub fn cash_distrib_pay_ref(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CASH_DISTRIB_PAY_REF)
    }

    /// Whether `CashDistribPayRef` (tag 501) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_distrib_pay_ref(&self) -> bool {
        self.inner.has_field(tags::CASH_DISTRIB_PAY_REF)
    }

    /// `CashDistribAgentAcctName` (tag 502, optional).
    pub fn cash_distrib_agent_acct_name(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CASH_DISTRIB_AGENT_ACCT_NAME)
    }

    /// Whether `CashDistribAgentAcctName` (tag 502) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_distrib_agent_acct_name(&self) -> bool {
        self.inner.has_field(tags::CASH_DISTRIB_AGENT_ACCT_NAME)
    }
}

/// One `NoDistribInsts` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoDistribInstsEntryMut::as_ref`].
pub struct NoDistribInstsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoDistribInstsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoDistribInstsEntry<'_> {
        NoDistribInstsEntry { inner: self.inner }
    }

    /// Set `DistribPaymentMethod` (tag 477, optional).
    pub fn set_distrib_payment_method(&mut self, v: i32) {
        self.inner.set_field(tags::DISTRIB_PAYMENT_METHOD, v.to_string());
    }

    /// Set `DistribPercentage` (tag 512, optional).
    pub fn set_distrib_percentage(&mut self, v: f64) {
        self.inner.set_field(tags::DISTRIB_PERCENTAGE, v.to_string());
    }

    /// Set `CashDistribCurr` (tag 478, optional).
    pub fn set_cash_distrib_curr(&mut self, v: &str) {
        self.inner.set_field(tags::CASH_DISTRIB_CURR, v.to_string());
    }

    /// Set `CashDistribAgentName` (tag 498, optional).
    pub fn set_cash_distrib_agent_name(&mut self, v: &str) {
        self.inner.set_field(tags::CASH_DISTRIB_AGENT_NAME, v.to_string());
    }

    /// Set `CashDistribAgentCode` (tag 499, optional).
    pub fn set_cash_distrib_agent_code(&mut self, v: &str) {
        self.inner.set_field(tags::CASH_DISTRIB_AGENT_CODE, v.to_string());
    }

    /// Set `CashDistribAgentAcctNumber` (tag 500, optional).
    pub fn set_cash_distrib_agent_acct_number(&mut self, v: &str) {
        self.inner.set_field(tags::CASH_DISTRIB_AGENT_ACCT_NUMBER, v.to_string());
    }

    /// Set `CashDistribPayRef` (tag 501, optional).
    pub fn set_cash_distrib_pay_ref(&mut self, v: &str) {
        self.inner.set_field(tags::CASH_DISTRIB_PAY_REF, v.to_string());
    }

    /// Set `CashDistribAgentAcctName` (tag 502, optional).
    pub fn set_cash_distrib_agent_acct_name(&mut self, v: &str) {
        self.inner.set_field(tags::CASH_DISTRIB_AGENT_ACCT_NAME, v.to_string());
    }
}

/// The `NoDistribInsts` repeating group (count tag `NO_DISTRIB_INSTS`), read-only: its
/// instances as [`NoDistribInstsEntry`] views.
pub struct NoDistribInsts<'a> {
    group: &'a Group,
}

impl<'a> NoDistribInsts<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoDistribInstsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoDistribInstsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoDistribInstsEntry<'a> {
        self.get(i).expect("NoDistribInsts index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoDistribInstsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoDistribInstsEntry { inner })
    }
}

/// The `NoDistribInsts` repeating group (count tag `NO_DISTRIB_INSTS`), mutable: its
/// instances as [`NoDistribInstsEntryMut`] views.
pub struct NoDistribInstsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoDistribInstsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoDistribInstsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoDistribInstsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoDistribInstsEntryMut<'_> {
        self.get_mut(i).expect("NoDistribInsts index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoDistribInstsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoDistribInstsEntryMut { inner })
    }
}
