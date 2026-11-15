//! `MassQuote` (app message, MsgType `35=i`): typed facade over the raw
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

/// Typed facade over a `MassQuote` [`Message`] (35=i). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct MassQuote {
    msg: Message,
}

impl Default for MassQuote {
    fn default() -> Self {
        MassQuote::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `MassQuote`, checking the msg type.
impl TryFrom<Message> for MassQuote {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a MassQuote" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<MassQuote> for Message {
    fn from(typed_msg: MassQuote) -> Self {
        typed_msg.msg
    }
}

impl MassQuote {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "i";

    /// An empty `MassQuote` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- QuoteReqID (tag 131, STRING, optional) ---

    /// Set `QuoteReqID` (tag 131, optional).
    pub fn set_quote_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::QUOTE_REQ_ID, v.to_string());
    }

    /// `QuoteReqID` (tag 131, optional).
    pub fn quote_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::QUOTE_REQ_ID)
    }

    /// Whether `QuoteReqID` (tag 131) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_req_id(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_REQ_ID)
    }

    // --- QuoteID (tag 117, STRING, required) ---

    /// Set `QuoteID` (tag 117, required).
    pub fn set_quote_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::QUOTE_ID, v.to_string());
    }

    /// `QuoteID` (tag 117, required).
    pub fn quote_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::QUOTE_ID)
    }

    /// Whether `QuoteID` (tag 117) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_id(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_ID)
    }

    // --- QuoteType (tag 537, INT, optional) ---

    /// Set `QuoteType` (tag 537, optional).
    pub fn set_quote_type(&mut self, v: fields::QuoteType) {
        self.msg.body_mut().set_field(tags::QUOTE_TYPE, v.to_fix().to_string());
    }

    /// `QuoteType` (tag 537, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::QuoteType`]'s.
    pub fn quote_type(&self) -> Result<fields::QuoteType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::QUOTE_TYPE)?;
        fields::QuoteType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::QUOTE_TYPE,
        })
    }

    /// Whether `QuoteType` (tag 537) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_type(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_TYPE)
    }

    // --- QuoteResponseLevel (tag 301, INT, optional) ---

    /// Set `QuoteResponseLevel` (tag 301, optional).
    pub fn set_quote_response_level(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::QUOTE_RESPONSE_LEVEL, v.to_string());
    }

    /// `QuoteResponseLevel` (tag 301, optional).
    pub fn quote_response_level(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::QUOTE_RESPONSE_LEVEL)
    }

    /// Whether `QuoteResponseLevel` (tag 301) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_response_level(&self) -> bool {
        self.msg.body().has_field(tags::QUOTE_RESPONSE_LEVEL)
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

    // --- DefBidSize (tag 293, QTY, optional) ---

    /// Set `DefBidSize` (tag 293, optional); written in FIX `QTY` format.
    pub fn set_def_bid_size(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::DEF_BID_SIZE, convert::format_fix_decimal(v));
    }

    /// `DefBidSize` (tag 293, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn def_bid_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::DEF_BID_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::DEF_BID_SIZE)
    }

    /// Whether `DefBidSize` (tag 293) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_def_bid_size(&self) -> bool {
        self.msg.body().has_field(tags::DEF_BID_SIZE)
    }

    // --- DefOfferSize (tag 294, QTY, optional) ---

    /// Set `DefOfferSize` (tag 294, optional); written in FIX `QTY` format.
    pub fn set_def_offer_size(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::DEF_OFFER_SIZE, convert::format_fix_decimal(v));
    }

    /// `DefOfferSize` (tag 294, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn def_offer_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::DEF_OFFER_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::DEF_OFFER_SIZE)
    }

    /// Whether `DefOfferSize` (tag 294) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_def_offer_size(&self) -> bool {
        self.msg.body().has_field(tags::DEF_OFFER_SIZE)
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

    // --- NoQuoteSets group (count tag NO_QUOTE_SETS) ---

    /// The `NoQuoteSets` group (count tag `NO_QUOTE_SETS`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_quote_sets(&self) -> Option<NoQuoteSets<'_>> {
        self.msg.body().get_group(tags::NO_QUOTE_SETS).map(|group| NoQuoteSets { group })
    }

    /// The `NoQuoteSets` group (count tag `NO_QUOTE_SETS`, required) for editing, or `None` if
    /// absent.
    pub fn no_quote_sets_mut(&mut self) -> Option<NoQuoteSetsMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_QUOTE_SETS).map(|group| NoQuoteSetsMut { group })
    }

    /// Create the `NoQuoteSets` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_quote_sets_entry`] on the same group.
    pub fn set_no_quote_sets(&mut self, count: u32) -> NoQuoteSetsMut<'_> {
        let group =
            self.msg.body_mut().set_group(tags::NO_QUOTE_SETS, count, NO_QUOTE_SETS_FIELD_ORDER);
        NoQuoteSetsMut { group }
    }

    /// Append one `NoQuoteSets` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_quote_sets`] on the same group.
    pub fn add_no_quote_sets_entry(&mut self, fill: impl FnOnce(&mut NoQuoteSetsEntryMut<'_>)) {
        let inner =
            self.msg.body_mut().add_group_instance(tags::NO_QUOTE_SETS, NO_QUOTE_SETS_FIELD_ORDER);
        fill(&mut NoQuoteSetsEntryMut { inner });
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

/// Wire order of a `NoQuoteSets` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_QUOTE_SETS_FIELD_ORDER: &[u32] = &[
    tags::QUOTE_SET_ID,
    tags::UNDERLYING_SYMBOL,
    tags::UNDERLYING_SYMBOL_SFX,
    tags::UNDERLYING_SECURITY_ID,
    tags::UNDERLYING_SECURITY_ID_SOURCE,
    tags::NO_UNDERLYING_SECURITY_ALT_ID,
    tags::UNDERLYING_PRODUCT,
    tags::UNDERLYING_CFI_CODE,
    tags::UNDERLYING_SECURITY_TYPE,
    tags::UNDERLYING_MATURITY_MONTH_YEAR,
    tags::UNDERLYING_MATURITY_DATE,
    tags::UNDERLYING_COUPON_PAYMENT_DATE,
    tags::UNDERLYING_ISSUE_DATE,
    tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE,
    tags::UNDERLYING_REPURCHASE_TERM,
    tags::UNDERLYING_REPURCHASE_RATE,
    tags::UNDERLYING_FACTOR,
    tags::UNDERLYING_CREDIT_RATING,
    tags::UNDERLYING_INSTR_REGISTRY,
    tags::UNDERLYING_COUNTRY_OF_ISSUE,
    tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE,
    tags::UNDERLYING_LOCALE_OF_ISSUE,
    tags::UNDERLYING_REDEMPTION_DATE,
    tags::UNDERLYING_PUT_OR_CALL,
    tags::UNDERLYING_STRIKE_PRICE,
    tags::UNDERLYING_OPT_ATTRIBUTE,
    tags::UNDERLYING_CONTRACT_MULTIPLIER,
    tags::UNDERLYING_COUPON_RATE,
    tags::UNDERLYING_SECURITY_EXCHANGE,
    tags::UNDERLYING_ISSUER,
    tags::ENCODED_UNDERLYING_ISSUER_LEN,
    tags::ENCODED_UNDERLYING_ISSUER,
    tags::UNDERLYING_SECURITY_DESC,
    tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN,
    tags::ENCODED_UNDERLYING_SECURITY_DESC,
    tags::QUOTE_SET_VALID_UNTIL_TIME,
    tags::TOT_QUOTE_ENTRIES,
    tags::NO_QUOTE_ENTRIES,
];

/// One `NoQuoteSets` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoQuoteSetsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoQuoteSetsEntry<'_> {
    /// `QuoteSetID` (tag 302, required).
    pub fn quote_set_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::QUOTE_SET_ID)
    }

    /// Whether `QuoteSetID` (tag 302) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_set_id(&self) -> bool {
        self.inner.has_field(tags::QUOTE_SET_ID)
    }

    /// `UnderlyingSymbol` (tag 311, optional).
    pub fn underlying_symbol(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SYMBOL)
    }

    /// Whether `UnderlyingSymbol` (tag 311) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_symbol(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SYMBOL)
    }

    /// `UnderlyingSymbolSfx` (tag 312, optional).
    pub fn underlying_symbol_sfx(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SYMBOL_SFX)
    }

    /// Whether `UnderlyingSymbolSfx` (tag 312) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_symbol_sfx(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SYMBOL_SFX)
    }

    /// `UnderlyingSecurityID` (tag 309, optional).
    pub fn underlying_security_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_ID)
    }

    /// Whether `UnderlyingSecurityID` (tag 309) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_id(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_ID)
    }

    /// `UnderlyingSecurityIDSource` (tag 305, optional).
    pub fn underlying_security_id_source(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_ID_SOURCE)
    }

    /// Whether `UnderlyingSecurityIDSource` (tag 305) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_id_source(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_ID_SOURCE)
    }

    /// `UnderlyingProduct` (tag 462, optional).
    pub fn underlying_product(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::UNDERLYING_PRODUCT)
    }

    /// Whether `UnderlyingProduct` (tag 462) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_product(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_PRODUCT)
    }

    /// `UnderlyingCFICode` (tag 463, optional).
    pub fn underlying_cfi_code(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_CFI_CODE)
    }

    /// Whether `UnderlyingCFICode` (tag 463) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_cfi_code(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_CFI_CODE)
    }

    /// `UnderlyingSecurityType` (tag 310, optional).
    pub fn underlying_security_type(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_TYPE)
    }

    /// Whether `UnderlyingSecurityType` (tag 310) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_type(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_TYPE)
    }

    /// `UnderlyingMaturityMonthYear` (tag 313, optional).
    pub fn underlying_maturity_month_year(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_MATURITY_MONTH_YEAR)
    }

    /// Whether `UnderlyingMaturityMonthYear` (tag 313) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_maturity_month_year(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_MATURITY_MONTH_YEAR)
    }

    /// `UnderlyingMaturityDate` (tag 542, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn underlying_maturity_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::UNDERLYING_MATURITY_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_MATURITY_DATE)
    }

    /// Whether `UnderlyingMaturityDate` (tag 542) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_maturity_date(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_MATURITY_DATE)
    }

    /// `UnderlyingCouponPaymentDate` (tag 241, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn underlying_coupon_payment_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::UNDERLYING_COUPON_PAYMENT_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_COUPON_PAYMENT_DATE)
    }

    /// Whether `UnderlyingCouponPaymentDate` (tag 241) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_coupon_payment_date(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_COUPON_PAYMENT_DATE)
    }

    /// `UnderlyingIssueDate` (tag 242, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn underlying_issue_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::UNDERLYING_ISSUE_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_ISSUE_DATE)
    }

    /// Whether `UnderlyingIssueDate` (tag 242) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_issue_date(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_ISSUE_DATE)
    }

    /// `UnderlyingRepoCollateralSecurityType` (tag 243, optional).
    pub fn underlying_repo_collateral_security_type(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// Whether `UnderlyingRepoCollateralSecurityType` (tag 243) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_repo_collateral_security_type(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// `UnderlyingRepurchaseTerm` (tag 244, optional).
    pub fn underlying_repurchase_term(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::UNDERLYING_REPURCHASE_TERM)
    }

    /// Whether `UnderlyingRepurchaseTerm` (tag 244) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_repurchase_term(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_REPURCHASE_TERM)
    }

    /// `UnderlyingRepurchaseRate` (tag 245, optional).
    pub fn underlying_repurchase_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::UNDERLYING_REPURCHASE_RATE)
    }

    /// Whether `UnderlyingRepurchaseRate` (tag 245) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_repurchase_rate(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_REPURCHASE_RATE)
    }

    /// `UnderlyingFactor` (tag 246, optional).
    pub fn underlying_factor(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::UNDERLYING_FACTOR)
    }

    /// Whether `UnderlyingFactor` (tag 246) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_factor(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_FACTOR)
    }

    /// `UnderlyingCreditRating` (tag 256, optional).
    pub fn underlying_credit_rating(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_CREDIT_RATING)
    }

    /// Whether `UnderlyingCreditRating` (tag 256) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_credit_rating(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_CREDIT_RATING)
    }

    /// `UnderlyingInstrRegistry` (tag 595, optional).
    pub fn underlying_instr_registry(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_INSTR_REGISTRY)
    }

    /// Whether `UnderlyingInstrRegistry` (tag 595) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_instr_registry(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_INSTR_REGISTRY)
    }

    /// `UnderlyingCountryOfIssue` (tag 592, optional).
    pub fn underlying_country_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_COUNTRY_OF_ISSUE)
    }

    /// Whether `UnderlyingCountryOfIssue` (tag 592) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_country_of_issue(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_COUNTRY_OF_ISSUE)
    }

    /// `UnderlyingStateOrProvinceOfIssue` (tag 593, optional).
    pub fn underlying_state_or_province_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// Whether `UnderlyingStateOrProvinceOfIssue` (tag 593) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_state_or_province_of_issue(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// `UnderlyingLocaleOfIssue` (tag 594, optional).
    pub fn underlying_locale_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_LOCALE_OF_ISSUE)
    }

    /// Whether `UnderlyingLocaleOfIssue` (tag 594) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_locale_of_issue(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_LOCALE_OF_ISSUE)
    }

    /// `UnderlyingRedemptionDate` (tag 247, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn underlying_redemption_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::UNDERLYING_REDEMPTION_DATE)?;
        convert::parse_fix_date(&raw, tags::UNDERLYING_REDEMPTION_DATE)
    }

    /// Whether `UnderlyingRedemptionDate` (tag 247) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_redemption_date(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_REDEMPTION_DATE)
    }

    /// `UnderlyingPutOrCall` (tag 315, optional).
    pub fn underlying_put_or_call(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::UNDERLYING_PUT_OR_CALL)
    }

    /// Whether `UnderlyingPutOrCall` (tag 315) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_put_or_call(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_PUT_OR_CALL)
    }

    /// `UnderlyingStrikePrice` (tag 316, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn underlying_strike_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::UNDERLYING_STRIKE_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::UNDERLYING_STRIKE_PRICE)
    }

    /// Whether `UnderlyingStrikePrice` (tag 316) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_strike_price(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_STRIKE_PRICE)
    }

    /// `UnderlyingOptAttribute` (tag 317, optional).
    pub fn underlying_opt_attribute(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::UNDERLYING_OPT_ATTRIBUTE)
    }

    /// Whether `UnderlyingOptAttribute` (tag 317) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_opt_attribute(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_OPT_ATTRIBUTE)
    }

    /// `UnderlyingContractMultiplier` (tag 436, optional).
    pub fn underlying_contract_multiplier(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::UNDERLYING_CONTRACT_MULTIPLIER)
    }

    /// Whether `UnderlyingContractMultiplier` (tag 436) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_contract_multiplier(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_CONTRACT_MULTIPLIER)
    }

    /// `UnderlyingCouponRate` (tag 435, optional).
    pub fn underlying_coupon_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::UNDERLYING_COUPON_RATE)
    }

    /// Whether `UnderlyingCouponRate` (tag 435) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_coupon_rate(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_COUPON_RATE)
    }

    /// `UnderlyingSecurityExchange` (tag 308, optional).
    pub fn underlying_security_exchange(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_EXCHANGE)
    }

    /// Whether `UnderlyingSecurityExchange` (tag 308) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_exchange(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_EXCHANGE)
    }

    /// `UnderlyingIssuer` (tag 306, optional).
    pub fn underlying_issuer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_ISSUER)
    }

    /// Whether `UnderlyingIssuer` (tag 306) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_issuer(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_ISSUER)
    }

    /// `EncodedUnderlyingIssuerLen` (tag 362, optional).
    pub fn encoded_underlying_issuer_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_UNDERLYING_ISSUER_LEN)
    }

    /// Whether `EncodedUnderlyingIssuerLen` (tag 362) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_issuer_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_UNDERLYING_ISSUER_LEN)
    }

    /// `EncodedUnderlyingIssuer` (tag 363, optional).
    pub fn encoded_underlying_issuer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_UNDERLYING_ISSUER)
    }

    /// Whether `EncodedUnderlyingIssuer` (tag 363) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_issuer(&self) -> bool {
        self.inner.has_field(tags::ENCODED_UNDERLYING_ISSUER)
    }

    /// `UnderlyingSecurityDesc` (tag 307, optional).
    pub fn underlying_security_desc(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::UNDERLYING_SECURITY_DESC)
    }

    /// Whether `UnderlyingSecurityDesc` (tag 307) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_underlying_security_desc(&self) -> bool {
        self.inner.has_field(tags::UNDERLYING_SECURITY_DESC)
    }

    /// `EncodedUnderlyingSecurityDescLen` (tag 364, optional).
    pub fn encoded_underlying_security_desc_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN)
    }

    /// Whether `EncodedUnderlyingSecurityDescLen` (tag 364) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_security_desc_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN)
    }

    /// `EncodedUnderlyingSecurityDesc` (tag 365, optional).
    pub fn encoded_underlying_security_desc(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_UNDERLYING_SECURITY_DESC)
    }

    /// Whether `EncodedUnderlyingSecurityDesc` (tag 365) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_underlying_security_desc(&self) -> bool {
        self.inner.has_field(tags::ENCODED_UNDERLYING_SECURITY_DESC)
    }

    /// `QuoteSetValidUntilTime` (tag 367, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn quote_set_valid_until_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.inner.get_field::<String>(tags::QUOTE_SET_VALID_UNTIL_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::QUOTE_SET_VALID_UNTIL_TIME)
    }

    /// Whether `QuoteSetValidUntilTime` (tag 367) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_set_valid_until_time(&self) -> bool {
        self.inner.has_field(tags::QUOTE_SET_VALID_UNTIL_TIME)
    }

    /// `TotQuoteEntries` (tag 304, required).
    pub fn tot_quote_entries(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::TOT_QUOTE_ENTRIES)
    }

    /// Whether `TotQuoteEntries` (tag 304) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_tot_quote_entries(&self) -> bool {
        self.inner.has_field(tags::TOT_QUOTE_ENTRIES)
    }

    /// The `NoUnderlyingSecurityAltId` group (count tag `NO_UNDERLYING_SECURITY_ALT_ID`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_underlying_security_alt_id(&self) -> Option<NoUnderlyingSecurityAltId<'_>> {
        self.inner
            .get_group(tags::NO_UNDERLYING_SECURITY_ALT_ID)
            .map(|group| NoUnderlyingSecurityAltId { group })
    }

    /// The `NoQuoteEntries` group (count tag `NO_QUOTE_ENTRIES`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_quote_entries(&self) -> Option<NoQuoteEntries<'_>> {
        self.inner.get_group(tags::NO_QUOTE_ENTRIES).map(|group| NoQuoteEntries { group })
    }
}

/// One `NoQuoteSets` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoQuoteSetsEntryMut::as_ref`].
pub struct NoQuoteSetsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoQuoteSetsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoQuoteSetsEntry<'_> {
        NoQuoteSetsEntry { inner: self.inner }
    }

    /// Set `QuoteSetID` (tag 302, required).
    pub fn set_quote_set_id(&mut self, v: &str) {
        self.inner.set_field(tags::QUOTE_SET_ID, v.to_string());
    }

    /// Set `UnderlyingSymbol` (tag 311, optional).
    pub fn set_underlying_symbol(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SYMBOL, v.to_string());
    }

    /// Set `UnderlyingSymbolSfx` (tag 312, optional).
    pub fn set_underlying_symbol_sfx(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SYMBOL_SFX, v.to_string());
    }

    /// Set `UnderlyingSecurityID` (tag 309, optional).
    pub fn set_underlying_security_id(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_ID, v.to_string());
    }

    /// Set `UnderlyingSecurityIDSource` (tag 305, optional).
    pub fn set_underlying_security_id_source(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_ID_SOURCE, v.to_string());
    }

    /// Set `UnderlyingProduct` (tag 462, optional).
    pub fn set_underlying_product(&mut self, v: i32) {
        self.inner.set_field(tags::UNDERLYING_PRODUCT, v.to_string());
    }

    /// Set `UnderlyingCFICode` (tag 463, optional).
    pub fn set_underlying_cfi_code(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_CFI_CODE, v.to_string());
    }

    /// Set `UnderlyingSecurityType` (tag 310, optional).
    pub fn set_underlying_security_type(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_TYPE, v.to_string());
    }

    /// Set `UnderlyingMaturityMonthYear` (tag 313, optional).
    pub fn set_underlying_maturity_month_year(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_MATURITY_MONTH_YEAR, v.to_string());
    }

    /// Set `UnderlyingMaturityDate` (tag 542, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_underlying_maturity_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::UNDERLYING_MATURITY_DATE, convert::format_fix_date(v));
    }

    /// Set `UnderlyingCouponPaymentDate` (tag 241, optional); written in FIX `UTCDATE` format.
    pub fn set_underlying_coupon_payment_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::UNDERLYING_COUPON_PAYMENT_DATE, convert::format_fix_date(v));
    }

    /// Set `UnderlyingIssueDate` (tag 242, optional); written in FIX `UTCDATE` format.
    pub fn set_underlying_issue_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::UNDERLYING_ISSUE_DATE, convert::format_fix_date(v));
    }

    /// Set `UnderlyingRepoCollateralSecurityType` (tag 243, optional).
    pub fn set_underlying_repo_collateral_security_type(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE, v.to_string());
    }

    /// Set `UnderlyingRepurchaseTerm` (tag 244, optional).
    pub fn set_underlying_repurchase_term(&mut self, v: i32) {
        self.inner.set_field(tags::UNDERLYING_REPURCHASE_TERM, v.to_string());
    }

    /// Set `UnderlyingRepurchaseRate` (tag 245, optional).
    pub fn set_underlying_repurchase_rate(&mut self, v: f64) {
        self.inner.set_field(tags::UNDERLYING_REPURCHASE_RATE, v.to_string());
    }

    /// Set `UnderlyingFactor` (tag 246, optional).
    pub fn set_underlying_factor(&mut self, v: f64) {
        self.inner.set_field(tags::UNDERLYING_FACTOR, v.to_string());
    }

    /// Set `UnderlyingCreditRating` (tag 256, optional).
    pub fn set_underlying_credit_rating(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_CREDIT_RATING, v.to_string());
    }

    /// Set `UnderlyingInstrRegistry` (tag 595, optional).
    pub fn set_underlying_instr_registry(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_INSTR_REGISTRY, v.to_string());
    }

    /// Set `UnderlyingCountryOfIssue` (tag 592, optional).
    pub fn set_underlying_country_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_COUNTRY_OF_ISSUE, v.to_string());
    }

    /// Set `UnderlyingStateOrProvinceOfIssue` (tag 593, optional).
    pub fn set_underlying_state_or_province_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE, v.to_string());
    }

    /// Set `UnderlyingLocaleOfIssue` (tag 594, optional).
    pub fn set_underlying_locale_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_LOCALE_OF_ISSUE, v.to_string());
    }

    /// Set `UnderlyingRedemptionDate` (tag 247, optional); written in FIX `UTCDATE` format.
    pub fn set_underlying_redemption_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::UNDERLYING_REDEMPTION_DATE, convert::format_fix_date(v));
    }

    /// Set `UnderlyingPutOrCall` (tag 315, optional).
    pub fn set_underlying_put_or_call(&mut self, v: i32) {
        self.inner.set_field(tags::UNDERLYING_PUT_OR_CALL, v.to_string());
    }

    /// Set `UnderlyingStrikePrice` (tag 316, optional); written in FIX `PRICE` format.
    pub fn set_underlying_strike_price(&mut self, v: Decimal) {
        self.inner.set_field(tags::UNDERLYING_STRIKE_PRICE, convert::format_fix_decimal(v));
    }

    /// Set `UnderlyingOptAttribute` (tag 317, optional).
    pub fn set_underlying_opt_attribute(&mut self, v: char) {
        self.inner.set_field(tags::UNDERLYING_OPT_ATTRIBUTE, v.to_string());
    }

    /// Set `UnderlyingContractMultiplier` (tag 436, optional).
    pub fn set_underlying_contract_multiplier(&mut self, v: f64) {
        self.inner.set_field(tags::UNDERLYING_CONTRACT_MULTIPLIER, v.to_string());
    }

    /// Set `UnderlyingCouponRate` (tag 435, optional).
    pub fn set_underlying_coupon_rate(&mut self, v: f64) {
        self.inner.set_field(tags::UNDERLYING_COUPON_RATE, v.to_string());
    }

    /// Set `UnderlyingSecurityExchange` (tag 308, optional).
    pub fn set_underlying_security_exchange(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_EXCHANGE, v.to_string());
    }

    /// Set `UnderlyingIssuer` (tag 306, optional).
    pub fn set_underlying_issuer(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_ISSUER, v.to_string());
    }

    /// Set `EncodedUnderlyingIssuerLen` (tag 362, optional).
    pub fn set_encoded_underlying_issuer_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_UNDERLYING_ISSUER_LEN, v.to_string());
    }

    /// Set `EncodedUnderlyingIssuer` (tag 363, optional).
    pub fn set_encoded_underlying_issuer(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_UNDERLYING_ISSUER, v.to_string());
    }

    /// Set `UnderlyingSecurityDesc` (tag 307, optional).
    pub fn set_underlying_security_desc(&mut self, v: &str) {
        self.inner.set_field(tags::UNDERLYING_SECURITY_DESC, v.to_string());
    }

    /// Set `EncodedUnderlyingSecurityDescLen` (tag 364, optional).
    pub fn set_encoded_underlying_security_desc_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_UNDERLYING_SECURITY_DESC_LEN, v.to_string());
    }

    /// Set `EncodedUnderlyingSecurityDesc` (tag 365, optional).
    pub fn set_encoded_underlying_security_desc(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_UNDERLYING_SECURITY_DESC, v.to_string());
    }

    /// Set `QuoteSetValidUntilTime` (tag 367, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_quote_set_valid_until_time(&mut self, v: DateTime<Utc>) {
        self.inner
            .set_field(tags::QUOTE_SET_VALID_UNTIL_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// Set `TotQuoteEntries` (tag 304, required).
    pub fn set_tot_quote_entries(&mut self, v: i32) {
        self.inner.set_field(tags::TOT_QUOTE_ENTRIES, v.to_string());
    }

    /// The `NoUnderlyingSecurityAltId` group (count tag `NO_UNDERLYING_SECURITY_ALT_ID`, optional) for editing, or `None` if
    /// absent.
    pub fn no_underlying_security_alt_id_mut(
        &mut self,
    ) -> Option<NoUnderlyingSecurityAltIdMut<'_>> {
        self.inner
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
        let group = self.inner.set_group(
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
        let inner = self.inner.add_group_instance(
            tags::NO_UNDERLYING_SECURITY_ALT_ID,
            NO_UNDERLYING_SECURITY_ALT_ID_FIELD_ORDER,
        );
        fill(&mut NoUnderlyingSecurityAltIdEntryMut { inner });
    }

    /// The `NoQuoteEntries` group (count tag `NO_QUOTE_ENTRIES`, optional) for editing, or `None` if
    /// absent.
    pub fn no_quote_entries_mut(&mut self) -> Option<NoQuoteEntriesMut<'_>> {
        self.inner.get_group_mut(tags::NO_QUOTE_ENTRIES).map(|group| NoQuoteEntriesMut { group })
    }

    /// Create the `NoQuoteEntries` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_quote_entries_entry`] on the same group.
    pub fn set_no_quote_entries(&mut self, count: u32) -> NoQuoteEntriesMut<'_> {
        let group =
            self.inner.set_group(tags::NO_QUOTE_ENTRIES, count, NO_QUOTE_ENTRIES_FIELD_ORDER);
        NoQuoteEntriesMut { group }
    }

    /// Append one `NoQuoteEntries` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_quote_entries`] on the same group.
    pub fn add_no_quote_entries_entry(
        &mut self,
        fill: impl FnOnce(&mut NoQuoteEntriesEntryMut<'_>),
    ) {
        let inner =
            self.inner.add_group_instance(tags::NO_QUOTE_ENTRIES, NO_QUOTE_ENTRIES_FIELD_ORDER);
        fill(&mut NoQuoteEntriesEntryMut { inner });
    }
}

/// The `NoQuoteSets` repeating group (count tag `NO_QUOTE_SETS`), read-only: its
/// instances as [`NoQuoteSetsEntry`] views.
pub struct NoQuoteSets<'a> {
    group: &'a Group,
}

impl<'a> NoQuoteSets<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoQuoteSetsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoQuoteSetsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoQuoteSetsEntry<'a> {
        self.get(i).expect("NoQuoteSets index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoQuoteSetsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoQuoteSetsEntry { inner })
    }
}

/// The `NoQuoteSets` repeating group (count tag `NO_QUOTE_SETS`), mutable: its
/// instances as [`NoQuoteSetsEntryMut`] views.
pub struct NoQuoteSetsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoQuoteSetsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoQuoteSetsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoQuoteSetsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoQuoteSetsEntryMut<'_> {
        self.get_mut(i).expect("NoQuoteSets index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoQuoteSetsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoQuoteSetsEntryMut { inner })
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

/// Wire order of a `NoQuoteEntries` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_QUOTE_ENTRIES_FIELD_ORDER: &[u32] = &[
    tags::QUOTE_ENTRY_ID,
    tags::SYMBOL,
    tags::SYMBOL_SFX,
    tags::SECURITY_ID,
    tags::SECURITY_ID_SOURCE,
    tags::NO_SECURITY_ALT_ID,
    tags::PRODUCT,
    tags::CFI_CODE,
    tags::SECURITY_TYPE,
    tags::MATURITY_MONTH_YEAR,
    tags::MATURITY_DATE,
    tags::COUPON_PAYMENT_DATE,
    tags::ISSUE_DATE,
    tags::REPO_COLLATERAL_SECURITY_TYPE,
    tags::REPURCHASE_TERM,
    tags::REPURCHASE_RATE,
    tags::FACTOR,
    tags::CREDIT_RATING,
    tags::INSTR_REGISTRY,
    tags::COUNTRY_OF_ISSUE,
    tags::STATE_OR_PROVINCE_OF_ISSUE,
    tags::LOCALE_OF_ISSUE,
    tags::REDEMPTION_DATE,
    tags::STRIKE_PRICE,
    tags::OPT_ATTRIBUTE,
    tags::CONTRACT_MULTIPLIER,
    tags::COUPON_RATE,
    tags::SECURITY_EXCHANGE,
    tags::ISSUER,
    tags::ENCODED_ISSUER_LEN,
    tags::ENCODED_ISSUER,
    tags::SECURITY_DESC,
    tags::ENCODED_SECURITY_DESC_LEN,
    tags::ENCODED_SECURITY_DESC,
    tags::BID_PX,
    tags::OFFER_PX,
    tags::BID_SIZE,
    tags::OFFER_SIZE,
    tags::VALID_UNTIL_TIME,
    tags::BID_SPOT_RATE,
    tags::OFFER_SPOT_RATE,
    tags::BID_FORWARD_POINTS,
    tags::OFFER_FORWARD_POINTS,
    tags::MID_PX,
    tags::BID_YIELD,
    tags::MID_YIELD,
    tags::OFFER_YIELD,
    tags::TRANSACT_TIME,
    tags::TRADING_SESSION_ID,
    tags::TRADING_SESSION_SUB_ID,
    tags::FUT_SETT_DATE,
    tags::ORD_TYPE,
    tags::FUT_SETT_DATE2,
    tags::ORDER_QTY2,
    tags::BID_FORWARD_POINTS2,
    tags::OFFER_FORWARD_POINTS2,
    tags::CURRENCY,
];

/// One `NoQuoteEntries` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoQuoteEntriesEntry<'a> {
    inner: &'a FieldMap,
}

impl NoQuoteEntriesEntry<'_> {
    /// `QuoteEntryID` (tag 299, optional).
    pub fn quote_entry_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::QUOTE_ENTRY_ID)
    }

    /// Whether `QuoteEntryID` (tag 299) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_quote_entry_id(&self) -> bool {
        self.inner.has_field(tags::QUOTE_ENTRY_ID)
    }

    /// `Symbol` (tag 55, optional).
    pub fn symbol(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SYMBOL)
    }

    /// Whether `Symbol` (tag 55) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_symbol(&self) -> bool {
        self.inner.has_field(tags::SYMBOL)
    }

    /// `SymbolSfx` (tag 65, optional).
    pub fn symbol_sfx(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SYMBOL_SFX)
    }

    /// Whether `SymbolSfx` (tag 65) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_symbol_sfx(&self) -> bool {
        self.inner.has_field(tags::SYMBOL_SFX)
    }

    /// `SecurityID` (tag 48, optional).
    pub fn security_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECURITY_ID)
    }

    /// Whether `SecurityID` (tag 48) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_id(&self) -> bool {
        self.inner.has_field(tags::SECURITY_ID)
    }

    /// `SecurityIDSource` (tag 22, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityIdSource`]'s.
    pub fn security_id_source(&self) -> Result<fields::SecurityIdSource, FieldError> {
        let raw = self.inner.get_field::<String>(tags::SECURITY_ID_SOURCE)?;
        fields::SecurityIdSource::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_ID_SOURCE,
        })
    }

    /// Whether `SecurityIDSource` (tag 22) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_id_source(&self) -> bool {
        self.inner.has_field(tags::SECURITY_ID_SOURCE)
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

    /// `MaturityMonthYear` (tag 200, optional).
    pub fn maturity_month_year(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::MATURITY_MONTH_YEAR)
    }

    /// Whether `MaturityMonthYear` (tag 200) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_maturity_month_year(&self) -> bool {
        self.inner.has_field(tags::MATURITY_MONTH_YEAR)
    }

    /// `MaturityDate` (tag 541, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn maturity_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MATURITY_DATE)?;
        convert::parse_fix_date(&raw, tags::MATURITY_DATE)
    }

    /// Whether `MaturityDate` (tag 541) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_maturity_date(&self) -> bool {
        self.inner.has_field(tags::MATURITY_DATE)
    }

    /// `CouponPaymentDate` (tag 224, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn coupon_payment_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::COUPON_PAYMENT_DATE)?;
        convert::parse_fix_date(&raw, tags::COUPON_PAYMENT_DATE)
    }

    /// Whether `CouponPaymentDate` (tag 224) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_coupon_payment_date(&self) -> bool {
        self.inner.has_field(tags::COUPON_PAYMENT_DATE)
    }

    /// `IssueDate` (tag 225, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn issue_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ISSUE_DATE)?;
        convert::parse_fix_date(&raw, tags::ISSUE_DATE)
    }

    /// Whether `IssueDate` (tag 225) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_issue_date(&self) -> bool {
        self.inner.has_field(tags::ISSUE_DATE)
    }

    /// `RepoCollateralSecurityType` (tag 239, optional).
    pub fn repo_collateral_security_type(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// Whether `RepoCollateralSecurityType` (tag 239) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_repo_collateral_security_type(&self) -> bool {
        self.inner.has_field(tags::REPO_COLLATERAL_SECURITY_TYPE)
    }

    /// `RepurchaseTerm` (tag 226, optional).
    pub fn repurchase_term(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::REPURCHASE_TERM)
    }

    /// Whether `RepurchaseTerm` (tag 226) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_repurchase_term(&self) -> bool {
        self.inner.has_field(tags::REPURCHASE_TERM)
    }

    /// `RepurchaseRate` (tag 227, optional).
    pub fn repurchase_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::REPURCHASE_RATE)
    }

    /// Whether `RepurchaseRate` (tag 227) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_repurchase_rate(&self) -> bool {
        self.inner.has_field(tags::REPURCHASE_RATE)
    }

    /// `Factor` (tag 228, optional).
    pub fn factor(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::FACTOR)
    }

    /// Whether `Factor` (tag 228) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_factor(&self) -> bool {
        self.inner.has_field(tags::FACTOR)
    }

    /// `CreditRating` (tag 255, optional).
    pub fn credit_rating(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CREDIT_RATING)
    }

    /// Whether `CreditRating` (tag 255) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_credit_rating(&self) -> bool {
        self.inner.has_field(tags::CREDIT_RATING)
    }

    /// `InstrRegistry` (tag 543, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::InstrRegistry`]'s.
    pub fn instr_registry(&self) -> Result<fields::InstrRegistry, FieldError> {
        let raw = self.inner.get_field::<String>(tags::INSTR_REGISTRY)?;
        fields::InstrRegistry::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::INSTR_REGISTRY,
        })
    }

    /// Whether `InstrRegistry` (tag 543) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_instr_registry(&self) -> bool {
        self.inner.has_field(tags::INSTR_REGISTRY)
    }

    /// `CountryOfIssue` (tag 470, optional).
    pub fn country_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::COUNTRY_OF_ISSUE)
    }

    /// Whether `CountryOfIssue` (tag 470) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_country_of_issue(&self) -> bool {
        self.inner.has_field(tags::COUNTRY_OF_ISSUE)
    }

    /// `StateOrProvinceOfIssue` (tag 471, optional).
    pub fn state_or_province_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// Whether `StateOrProvinceOfIssue` (tag 471) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_state_or_province_of_issue(&self) -> bool {
        self.inner.has_field(tags::STATE_OR_PROVINCE_OF_ISSUE)
    }

    /// `LocaleOfIssue` (tag 472, optional).
    pub fn locale_of_issue(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LOCALE_OF_ISSUE)
    }

    /// Whether `LocaleOfIssue` (tag 472) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_locale_of_issue(&self) -> bool {
        self.inner.has_field(tags::LOCALE_OF_ISSUE)
    }

    /// `RedemptionDate` (tag 240, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCDATE`.
    pub fn redemption_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::REDEMPTION_DATE)?;
        convert::parse_fix_date(&raw, tags::REDEMPTION_DATE)
    }

    /// Whether `RedemptionDate` (tag 240) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_redemption_date(&self) -> bool {
        self.inner.has_field(tags::REDEMPTION_DATE)
    }

    /// `StrikePrice` (tag 202, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn strike_price(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::STRIKE_PRICE)?;
        convert::parse_fix_decimal(&raw, tags::STRIKE_PRICE)
    }

    /// Whether `StrikePrice` (tag 202) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_strike_price(&self) -> bool {
        self.inner.has_field(tags::STRIKE_PRICE)
    }

    /// `OptAttribute` (tag 206, optional).
    pub fn opt_attribute(&self) -> Result<char, FieldError> {
        self.inner.get_field::<char>(tags::OPT_ATTRIBUTE)
    }

    /// Whether `OptAttribute` (tag 206) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_opt_attribute(&self) -> bool {
        self.inner.has_field(tags::OPT_ATTRIBUTE)
    }

    /// `ContractMultiplier` (tag 231, optional).
    pub fn contract_multiplier(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::CONTRACT_MULTIPLIER)
    }

    /// Whether `ContractMultiplier` (tag 231) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_contract_multiplier(&self) -> bool {
        self.inner.has_field(tags::CONTRACT_MULTIPLIER)
    }

    /// `CouponRate` (tag 223, optional).
    pub fn coupon_rate(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::COUPON_RATE)
    }

    /// Whether `CouponRate` (tag 223) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_coupon_rate(&self) -> bool {
        self.inner.has_field(tags::COUPON_RATE)
    }

    /// `SecurityExchange` (tag 207, optional).
    pub fn security_exchange(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECURITY_EXCHANGE)
    }

    /// Whether `SecurityExchange` (tag 207) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_exchange(&self) -> bool {
        self.inner.has_field(tags::SECURITY_EXCHANGE)
    }

    /// `Issuer` (tag 106, optional).
    pub fn issuer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ISSUER)
    }

    /// Whether `Issuer` (tag 106) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_issuer(&self) -> bool {
        self.inner.has_field(tags::ISSUER)
    }

    /// `EncodedIssuerLen` (tag 348, optional).
    pub fn encoded_issuer_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_ISSUER_LEN)
    }

    /// Whether `EncodedIssuerLen` (tag 348) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_issuer_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_ISSUER_LEN)
    }

    /// `EncodedIssuer` (tag 349, optional).
    pub fn encoded_issuer(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_ISSUER)
    }

    /// Whether `EncodedIssuer` (tag 349) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_issuer(&self) -> bool {
        self.inner.has_field(tags::ENCODED_ISSUER)
    }

    /// `SecurityDesc` (tag 107, optional).
    pub fn security_desc(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECURITY_DESC)
    }

    /// Whether `SecurityDesc` (tag 107) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_desc(&self) -> bool {
        self.inner.has_field(tags::SECURITY_DESC)
    }

    /// `EncodedSecurityDescLen` (tag 350, optional).
    pub fn encoded_security_desc_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_SECURITY_DESC_LEN)
    }

    /// Whether `EncodedSecurityDescLen` (tag 350) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_security_desc_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_SECURITY_DESC_LEN)
    }

    /// `EncodedSecurityDesc` (tag 351, optional).
    pub fn encoded_security_desc(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_SECURITY_DESC)
    }

    /// Whether `EncodedSecurityDesc` (tag 351) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_security_desc(&self) -> bool {
        self.inner.has_field(tags::ENCODED_SECURITY_DESC)
    }

    /// `BidPx` (tag 132, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn bid_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::BID_PX)?;
        convert::parse_fix_decimal(&raw, tags::BID_PX)
    }

    /// Whether `BidPx` (tag 132) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_px(&self) -> bool {
        self.inner.has_field(tags::BID_PX)
    }

    /// `OfferPx` (tag 133, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn offer_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::OFFER_PX)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_PX)
    }

    /// Whether `OfferPx` (tag 133) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_px(&self) -> bool {
        self.inner.has_field(tags::OFFER_PX)
    }

    /// `BidSize` (tag 134, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn bid_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::BID_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::BID_SIZE)
    }

    /// Whether `BidSize` (tag 134) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_size(&self) -> bool {
        self.inner.has_field(tags::BID_SIZE)
    }

    /// `OfferSize` (tag 135, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn offer_size(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::OFFER_SIZE)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_SIZE)
    }

    /// Whether `OfferSize` (tag 135) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_size(&self) -> bool {
        self.inner.has_field(tags::OFFER_SIZE)
    }

    /// `ValidUntilTime` (tag 62, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn valid_until_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.inner.get_field::<String>(tags::VALID_UNTIL_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::VALID_UNTIL_TIME)
    }

    /// Whether `ValidUntilTime` (tag 62) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_valid_until_time(&self) -> bool {
        self.inner.has_field(tags::VALID_UNTIL_TIME)
    }

    /// `BidSpotRate` (tag 188, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn bid_spot_rate(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::BID_SPOT_RATE)?;
        convert::parse_fix_decimal(&raw, tags::BID_SPOT_RATE)
    }

    /// Whether `BidSpotRate` (tag 188) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_spot_rate(&self) -> bool {
        self.inner.has_field(tags::BID_SPOT_RATE)
    }

    /// `OfferSpotRate` (tag 190, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn offer_spot_rate(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::OFFER_SPOT_RATE)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_SPOT_RATE)
    }

    /// Whether `OfferSpotRate` (tag 190) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_spot_rate(&self) -> bool {
        self.inner.has_field(tags::OFFER_SPOT_RATE)
    }

    /// `BidForwardPoints` (tag 189, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn bid_forward_points(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::BID_FORWARD_POINTS)?;
        convert::parse_fix_decimal(&raw, tags::BID_FORWARD_POINTS)
    }

    /// Whether `BidForwardPoints` (tag 189) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_forward_points(&self) -> bool {
        self.inner.has_field(tags::BID_FORWARD_POINTS)
    }

    /// `OfferForwardPoints` (tag 191, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn offer_forward_points(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::OFFER_FORWARD_POINTS)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_FORWARD_POINTS)
    }

    /// Whether `OfferForwardPoints` (tag 191) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_forward_points(&self) -> bool {
        self.inner.has_field(tags::OFFER_FORWARD_POINTS)
    }

    /// `MidPx` (tag 631, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn mid_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::MID_PX)?;
        convert::parse_fix_decimal(&raw, tags::MID_PX)
    }

    /// Whether `MidPx` (tag 631) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mid_px(&self) -> bool {
        self.inner.has_field(tags::MID_PX)
    }

    /// `BidYield` (tag 632, optional).
    pub fn bid_yield(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::BID_YIELD)
    }

    /// Whether `BidYield` (tag 632) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_yield(&self) -> bool {
        self.inner.has_field(tags::BID_YIELD)
    }

    /// `MidYield` (tag 633, optional).
    pub fn mid_yield(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::MID_YIELD)
    }

    /// Whether `MidYield` (tag 633) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_mid_yield(&self) -> bool {
        self.inner.has_field(tags::MID_YIELD)
    }

    /// `OfferYield` (tag 634, optional).
    pub fn offer_yield(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::OFFER_YIELD)
    }

    /// Whether `OfferYield` (tag 634) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_yield(&self) -> bool {
        self.inner.has_field(tags::OFFER_YIELD)
    }

    /// `TransactTime` (tag 60, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn transact_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.inner.get_field::<String>(tags::TRANSACT_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRANSACT_TIME)
    }

    /// Whether `TransactTime` (tag 60) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_transact_time(&self) -> bool {
        self.inner.has_field(tags::TRANSACT_TIME)
    }

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

    /// `FutSettDate` (tag 64, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn fut_sett_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::FUT_SETT_DATE)?;
        convert::parse_fix_date(&raw, tags::FUT_SETT_DATE)
    }

    /// Whether `FutSettDate` (tag 64) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fut_sett_date(&self) -> bool {
        self.inner.has_field(tags::FUT_SETT_DATE)
    }

    /// `OrdType` (tag 40, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrdType`]'s.
    pub fn ord_type(&self) -> Result<fields::OrdType, FieldError> {
        let n = self.inner.get_field::<char>(tags::ORD_TYPE)?;
        fields::OrdType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORD_TYPE,
        })
    }

    /// Whether `OrdType` (tag 40) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ord_type(&self) -> bool {
        self.inner.has_field(tags::ORD_TYPE)
    }

    /// `FutSettDate2` (tag 193, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn fut_sett_date2(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.inner.get_field::<String>(tags::FUT_SETT_DATE2)?;
        convert::parse_fix_date(&raw, tags::FUT_SETT_DATE2)
    }

    /// Whether `FutSettDate2` (tag 193) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fut_sett_date2(&self) -> bool {
        self.inner.has_field(tags::FUT_SETT_DATE2)
    }

    /// `OrderQty2` (tag 192, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn order_qty2(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::ORDER_QTY2)?;
        convert::parse_fix_decimal(&raw, tags::ORDER_QTY2)
    }

    /// Whether `OrderQty2` (tag 192) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_order_qty2(&self) -> bool {
        self.inner.has_field(tags::ORDER_QTY2)
    }

    /// `BidForwardPoints2` (tag 642, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn bid_forward_points2(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::BID_FORWARD_POINTS2)?;
        convert::parse_fix_decimal(&raw, tags::BID_FORWARD_POINTS2)
    }

    /// Whether `BidForwardPoints2` (tag 642) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_forward_points2(&self) -> bool {
        self.inner.has_field(tags::BID_FORWARD_POINTS2)
    }

    /// `OfferForwardPoints2` (tag 643, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICEOFFSET`.
    pub fn offer_forward_points2(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::OFFER_FORWARD_POINTS2)?;
        convert::parse_fix_decimal(&raw, tags::OFFER_FORWARD_POINTS2)
    }

    /// Whether `OfferForwardPoints2` (tag 643) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_offer_forward_points2(&self) -> bool {
        self.inner.has_field(tags::OFFER_FORWARD_POINTS2)
    }

    /// `Currency` (tag 15, optional).
    pub fn currency(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CURRENCY)
    }

    /// Whether `Currency` (tag 15) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_currency(&self) -> bool {
        self.inner.has_field(tags::CURRENCY)
    }

    /// The `NoSecurityAltId` group (count tag `NO_SECURITY_ALT_ID`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_security_alt_id(&self) -> Option<NoSecurityAltId<'_>> {
        self.inner.get_group(tags::NO_SECURITY_ALT_ID).map(|group| NoSecurityAltId { group })
    }
}

/// One `NoQuoteEntries` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoQuoteEntriesEntryMut::as_ref`].
pub struct NoQuoteEntriesEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoQuoteEntriesEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoQuoteEntriesEntry<'_> {
        NoQuoteEntriesEntry { inner: self.inner }
    }

    /// Set `QuoteEntryID` (tag 299, optional).
    pub fn set_quote_entry_id(&mut self, v: &str) {
        self.inner.set_field(tags::QUOTE_ENTRY_ID, v.to_string());
    }

    /// Set `Symbol` (tag 55, optional).
    pub fn set_symbol(&mut self, v: &str) {
        self.inner.set_field(tags::SYMBOL, v.to_string());
    }

    /// Set `SymbolSfx` (tag 65, optional).
    pub fn set_symbol_sfx(&mut self, v: &str) {
        self.inner.set_field(tags::SYMBOL_SFX, v.to_string());
    }

    /// Set `SecurityID` (tag 48, optional).
    pub fn set_security_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECURITY_ID, v.to_string());
    }

    /// Set `SecurityIDSource` (tag 22, optional).
    pub fn set_security_id_source(&mut self, v: fields::SecurityIdSource) {
        self.inner.set_field(tags::SECURITY_ID_SOURCE, v.to_fix().to_string());
    }

    /// Set `Product` (tag 460, optional).
    pub fn set_product(&mut self, v: fields::Product) {
        self.inner.set_field(tags::PRODUCT, v.to_fix().to_string());
    }

    /// Set `CFICode` (tag 461, optional).
    pub fn set_cfi_code(&mut self, v: &str) {
        self.inner.set_field(tags::CFI_CODE, v.to_string());
    }

    /// Set `SecurityType` (tag 167, optional).
    pub fn set_security_type(&mut self, v: fields::SecurityType) {
        self.inner.set_field(tags::SECURITY_TYPE, v.to_fix().to_string());
    }

    /// Set `MaturityMonthYear` (tag 200, optional).
    pub fn set_maturity_month_year(&mut self, v: &str) {
        self.inner.set_field(tags::MATURITY_MONTH_YEAR, v.to_string());
    }

    /// Set `MaturityDate` (tag 541, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_maturity_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::MATURITY_DATE, convert::format_fix_date(v));
    }

    /// Set `CouponPaymentDate` (tag 224, optional); written in FIX `UTCDATE` format.
    pub fn set_coupon_payment_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::COUPON_PAYMENT_DATE, convert::format_fix_date(v));
    }

    /// Set `IssueDate` (tag 225, optional); written in FIX `UTCDATE` format.
    pub fn set_issue_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::ISSUE_DATE, convert::format_fix_date(v));
    }

    /// Set `RepoCollateralSecurityType` (tag 239, optional).
    pub fn set_repo_collateral_security_type(&mut self, v: &str) {
        self.inner.set_field(tags::REPO_COLLATERAL_SECURITY_TYPE, v.to_string());
    }

    /// Set `RepurchaseTerm` (tag 226, optional).
    pub fn set_repurchase_term(&mut self, v: i32) {
        self.inner.set_field(tags::REPURCHASE_TERM, v.to_string());
    }

    /// Set `RepurchaseRate` (tag 227, optional).
    pub fn set_repurchase_rate(&mut self, v: f64) {
        self.inner.set_field(tags::REPURCHASE_RATE, v.to_string());
    }

    /// Set `Factor` (tag 228, optional).
    pub fn set_factor(&mut self, v: f64) {
        self.inner.set_field(tags::FACTOR, v.to_string());
    }

    /// Set `CreditRating` (tag 255, optional).
    pub fn set_credit_rating(&mut self, v: &str) {
        self.inner.set_field(tags::CREDIT_RATING, v.to_string());
    }

    /// Set `InstrRegistry` (tag 543, optional).
    pub fn set_instr_registry(&mut self, v: fields::InstrRegistry) {
        self.inner.set_field(tags::INSTR_REGISTRY, v.to_fix().to_string());
    }

    /// Set `CountryOfIssue` (tag 470, optional).
    pub fn set_country_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::COUNTRY_OF_ISSUE, v.to_string());
    }

    /// Set `StateOrProvinceOfIssue` (tag 471, optional).
    pub fn set_state_or_province_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::STATE_OR_PROVINCE_OF_ISSUE, v.to_string());
    }

    /// Set `LocaleOfIssue` (tag 472, optional).
    pub fn set_locale_of_issue(&mut self, v: &str) {
        self.inner.set_field(tags::LOCALE_OF_ISSUE, v.to_string());
    }

    /// Set `RedemptionDate` (tag 240, optional); written in FIX `UTCDATE` format.
    pub fn set_redemption_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::REDEMPTION_DATE, convert::format_fix_date(v));
    }

    /// Set `StrikePrice` (tag 202, optional); written in FIX `PRICE` format.
    pub fn set_strike_price(&mut self, v: Decimal) {
        self.inner.set_field(tags::STRIKE_PRICE, convert::format_fix_decimal(v));
    }

    /// Set `OptAttribute` (tag 206, optional).
    pub fn set_opt_attribute(&mut self, v: char) {
        self.inner.set_field(tags::OPT_ATTRIBUTE, v.to_string());
    }

    /// Set `ContractMultiplier` (tag 231, optional).
    pub fn set_contract_multiplier(&mut self, v: f64) {
        self.inner.set_field(tags::CONTRACT_MULTIPLIER, v.to_string());
    }

    /// Set `CouponRate` (tag 223, optional).
    pub fn set_coupon_rate(&mut self, v: f64) {
        self.inner.set_field(tags::COUPON_RATE, v.to_string());
    }

    /// Set `SecurityExchange` (tag 207, optional).
    pub fn set_security_exchange(&mut self, v: &str) {
        self.inner.set_field(tags::SECURITY_EXCHANGE, v.to_string());
    }

    /// Set `Issuer` (tag 106, optional).
    pub fn set_issuer(&mut self, v: &str) {
        self.inner.set_field(tags::ISSUER, v.to_string());
    }

    /// Set `EncodedIssuerLen` (tag 348, optional).
    pub fn set_encoded_issuer_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_ISSUER_LEN, v.to_string());
    }

    /// Set `EncodedIssuer` (tag 349, optional).
    pub fn set_encoded_issuer(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_ISSUER, v.to_string());
    }

    /// Set `SecurityDesc` (tag 107, optional).
    pub fn set_security_desc(&mut self, v: &str) {
        self.inner.set_field(tags::SECURITY_DESC, v.to_string());
    }

    /// Set `EncodedSecurityDescLen` (tag 350, optional).
    pub fn set_encoded_security_desc_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_SECURITY_DESC_LEN, v.to_string());
    }

    /// Set `EncodedSecurityDesc` (tag 351, optional).
    pub fn set_encoded_security_desc(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_SECURITY_DESC, v.to_string());
    }

    /// Set `BidPx` (tag 132, optional); written in FIX `PRICE` format.
    pub fn set_bid_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::BID_PX, convert::format_fix_decimal(v));
    }

    /// Set `OfferPx` (tag 133, optional); written in FIX `PRICE` format.
    pub fn set_offer_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::OFFER_PX, convert::format_fix_decimal(v));
    }

    /// Set `BidSize` (tag 134, optional); written in FIX `QTY` format.
    pub fn set_bid_size(&mut self, v: Decimal) {
        self.inner.set_field(tags::BID_SIZE, convert::format_fix_decimal(v));
    }

    /// Set `OfferSize` (tag 135, optional); written in FIX `QTY` format.
    pub fn set_offer_size(&mut self, v: Decimal) {
        self.inner.set_field(tags::OFFER_SIZE, convert::format_fix_decimal(v));
    }

    /// Set `ValidUntilTime` (tag 62, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_valid_until_time(&mut self, v: DateTime<Utc>) {
        self.inner.set_field(tags::VALID_UNTIL_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// Set `BidSpotRate` (tag 188, optional); written in FIX `PRICE` format.
    pub fn set_bid_spot_rate(&mut self, v: Decimal) {
        self.inner.set_field(tags::BID_SPOT_RATE, convert::format_fix_decimal(v));
    }

    /// Set `OfferSpotRate` (tag 190, optional); written in FIX `PRICE` format.
    pub fn set_offer_spot_rate(&mut self, v: Decimal) {
        self.inner.set_field(tags::OFFER_SPOT_RATE, convert::format_fix_decimal(v));
    }

    /// Set `BidForwardPoints` (tag 189, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_bid_forward_points(&mut self, v: Decimal) {
        self.inner.set_field(tags::BID_FORWARD_POINTS, convert::format_fix_decimal(v));
    }

    /// Set `OfferForwardPoints` (tag 191, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_offer_forward_points(&mut self, v: Decimal) {
        self.inner.set_field(tags::OFFER_FORWARD_POINTS, convert::format_fix_decimal(v));
    }

    /// Set `MidPx` (tag 631, optional); written in FIX `PRICE` format.
    pub fn set_mid_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::MID_PX, convert::format_fix_decimal(v));
    }

    /// Set `BidYield` (tag 632, optional).
    pub fn set_bid_yield(&mut self, v: f64) {
        self.inner.set_field(tags::BID_YIELD, v.to_string());
    }

    /// Set `MidYield` (tag 633, optional).
    pub fn set_mid_yield(&mut self, v: f64) {
        self.inner.set_field(tags::MID_YIELD, v.to_string());
    }

    /// Set `OfferYield` (tag 634, optional).
    pub fn set_offer_yield(&mut self, v: f64) {
        self.inner.set_field(tags::OFFER_YIELD, v.to_string());
    }

    /// Set `TransactTime` (tag 60, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_transact_time(&mut self, v: DateTime<Utc>) {
        self.inner.set_field(tags::TRANSACT_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
    }

    /// Set `FutSettDate` (tag 64, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_fut_sett_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::FUT_SETT_DATE, convert::format_fix_date(v));
    }

    /// Set `OrdType` (tag 40, optional).
    pub fn set_ord_type(&mut self, v: fields::OrdType) {
        self.inner.set_field(tags::ORD_TYPE, v.to_fix().to_string());
    }

    /// Set `FutSettDate2` (tag 193, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_fut_sett_date2(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::FUT_SETT_DATE2, convert::format_fix_date(v));
    }

    /// Set `OrderQty2` (tag 192, optional); written in FIX `QTY` format.
    pub fn set_order_qty2(&mut self, v: Decimal) {
        self.inner.set_field(tags::ORDER_QTY2, convert::format_fix_decimal(v));
    }

    /// Set `BidForwardPoints2` (tag 642, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_bid_forward_points2(&mut self, v: Decimal) {
        self.inner.set_field(tags::BID_FORWARD_POINTS2, convert::format_fix_decimal(v));
    }

    /// Set `OfferForwardPoints2` (tag 643, optional); written in FIX `PRICEOFFSET` format.
    pub fn set_offer_forward_points2(&mut self, v: Decimal) {
        self.inner.set_field(tags::OFFER_FORWARD_POINTS2, convert::format_fix_decimal(v));
    }

    /// Set `Currency` (tag 15, optional).
    pub fn set_currency(&mut self, v: &str) {
        self.inner.set_field(tags::CURRENCY, v.to_string());
    }

    /// The `NoSecurityAltId` group (count tag `NO_SECURITY_ALT_ID`, optional) for editing, or `None` if
    /// absent.
    pub fn no_security_alt_id_mut(&mut self) -> Option<NoSecurityAltIdMut<'_>> {
        self.inner.get_group_mut(tags::NO_SECURITY_ALT_ID).map(|group| NoSecurityAltIdMut { group })
    }

    /// Create the `NoSecurityAltId` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_security_alt_id_entry`] on the same group.
    pub fn set_no_security_alt_id(&mut self, count: u32) -> NoSecurityAltIdMut<'_> {
        let group =
            self.inner.set_group(tags::NO_SECURITY_ALT_ID, count, NO_SECURITY_ALT_ID_FIELD_ORDER);
        NoSecurityAltIdMut { group }
    }

    /// Append one `NoSecurityAltId` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_security_alt_id`] on the same group.
    pub fn add_no_security_alt_id_entry(
        &mut self,
        fill: impl FnOnce(&mut NoSecurityAltIdEntryMut<'_>),
    ) {
        let inner =
            self.inner.add_group_instance(tags::NO_SECURITY_ALT_ID, NO_SECURITY_ALT_ID_FIELD_ORDER);
        fill(&mut NoSecurityAltIdEntryMut { inner });
    }
}

/// The `NoQuoteEntries` repeating group (count tag `NO_QUOTE_ENTRIES`), read-only: its
/// instances as [`NoQuoteEntriesEntry`] views.
pub struct NoQuoteEntries<'a> {
    group: &'a Group,
}

impl<'a> NoQuoteEntries<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoQuoteEntriesEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoQuoteEntriesEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoQuoteEntriesEntry<'a> {
        self.get(i).expect("NoQuoteEntries index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoQuoteEntriesEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoQuoteEntriesEntry { inner })
    }
}

/// The `NoQuoteEntries` repeating group (count tag `NO_QUOTE_ENTRIES`), mutable: its
/// instances as [`NoQuoteEntriesEntryMut`] views.
pub struct NoQuoteEntriesMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoQuoteEntriesMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoQuoteEntriesEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoQuoteEntriesEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoQuoteEntriesEntryMut<'_> {
        self.get_mut(i).expect("NoQuoteEntries index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoQuoteEntriesEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoQuoteEntriesEntryMut { inner })
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
