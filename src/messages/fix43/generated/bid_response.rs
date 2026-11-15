//! `BidResponse` (app message, MsgType `35=l`): typed facade over the raw
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

/// Typed facade over a `BidResponse` [`Message`] (35=l). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct BidResponse {
    msg: Message,
}

impl Default for BidResponse {
    fn default() -> Self {
        BidResponse::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `BidResponse`, checking the msg type.
impl TryFrom<Message> for BidResponse {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a BidResponse" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<BidResponse> for Message {
    fn from(typed_msg: BidResponse) -> Self {
        typed_msg.msg
    }
}

impl BidResponse {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "l";

    /// An empty `BidResponse` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- BidID (tag 390, STRING, optional) ---

    /// Set `BidID` (tag 390, optional).
    pub fn set_bid_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BID_ID, v.to_string());
    }

    /// `BidID` (tag 390, optional).
    pub fn bid_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BID_ID)
    }

    /// Whether `BidID` (tag 390) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_id(&self) -> bool {
        self.msg.body().has_field(tags::BID_ID)
    }

    // --- ClientBidID (tag 391, STRING, optional) ---

    /// Set `ClientBidID` (tag 391, optional).
    pub fn set_client_bid_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CLIENT_BID_ID, v.to_string());
    }

    /// `ClientBidID` (tag 391, optional).
    pub fn client_bid_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CLIENT_BID_ID)
    }

    /// Whether `ClientBidID` (tag 391) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_client_bid_id(&self) -> bool {
        self.msg.body().has_field(tags::CLIENT_BID_ID)
    }

    // --- NoBidComponents group (count tag NO_BID_COMPONENTS) ---

    /// The `NoBidComponents` group (count tag `NO_BID_COMPONENTS`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_bid_components(&self) -> Option<NoBidComponents<'_>> {
        self.msg.body().get_group(tags::NO_BID_COMPONENTS).map(|group| NoBidComponents { group })
    }

    /// The `NoBidComponents` group (count tag `NO_BID_COMPONENTS`, required) for editing, or `None` if
    /// absent.
    pub fn no_bid_components_mut(&mut self) -> Option<NoBidComponentsMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_BID_COMPONENTS)
            .map(|group| NoBidComponentsMut { group })
    }

    /// Create the `NoBidComponents` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_bid_components_entry`] on the same group.
    pub fn set_no_bid_components(&mut self, count: u32) -> NoBidComponentsMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_BID_COMPONENTS,
            count,
            NO_BID_COMPONENTS_FIELD_ORDER,
        );
        NoBidComponentsMut { group }
    }

    /// Append one `NoBidComponents` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_bid_components`] on the same group.
    pub fn add_no_bid_components_entry(
        &mut self,
        fill: impl FnOnce(&mut NoBidComponentsEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_BID_COMPONENTS, NO_BID_COMPONENTS_FIELD_ORDER);
        fill(&mut NoBidComponentsEntryMut { inner });
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

/// Wire order of a `NoBidComponents` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_BID_COMPONENTS_FIELD_ORDER: &[u32] = &[
    tags::COMMISSION,
    tags::COMM_TYPE,
    tags::COMM_CURRENCY,
    tags::FUND_RENEW_WAIV,
    tags::LIST_ID,
    tags::COUNTRY,
    tags::SIDE,
    tags::PRICE,
    tags::PRICE_TYPE,
    tags::FAIR_VALUE,
    tags::NET_GROSS_IND,
    tags::SETTLMNT_TYP,
    tags::FUT_SETT_DATE,
    tags::TRADING_SESSION_ID,
    tags::TRADING_SESSION_SUB_ID,
    tags::TEXT,
    tags::ENCODED_TEXT_LEN,
    tags::ENCODED_TEXT,
];

/// One `NoBidComponents` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoBidComponentsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoBidComponentsEntry<'_> {
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

    /// `ListID` (tag 66, optional).
    pub fn list_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LIST_ID)
    }

    /// Whether `ListID` (tag 66) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_id(&self) -> bool {
        self.inner.has_field(tags::LIST_ID)
    }

    /// `Country` (tag 421, optional).
    pub fn country(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::COUNTRY)
    }

    /// Whether `Country` (tag 421) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_country(&self) -> bool {
        self.inner.has_field(tags::COUNTRY)
    }

    /// `Side` (tag 54, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Side`]'s.
    pub fn side(&self) -> Result<fields::Side, FieldError> {
        let n = self.inner.get_field::<char>(tags::SIDE)?;
        fields::Side::from_fix(n).ok_or(FieldError::InvalidFormat { tag: tags::SIDE })
    }

    /// Whether `Side` (tag 54) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side(&self) -> bool {
        self.inner.has_field(tags::SIDE)
    }

    /// `Price` (tag 44, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn price(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::PRICE)?;
        convert::parse_fix_decimal(&raw, tags::PRICE)
    }

    /// Whether `Price` (tag 44) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_price(&self) -> bool {
        self.inner.has_field(tags::PRICE)
    }

    /// `PriceType` (tag 423, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::PriceType`]'s.
    pub fn price_type(&self) -> Result<fields::PriceType, FieldError> {
        let n = self.inner.get_field::<i32>(tags::PRICE_TYPE)?;
        fields::PriceType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PRICE_TYPE,
        })
    }

    /// Whether `PriceType` (tag 423) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_price_type(&self) -> bool {
        self.inner.has_field(tags::PRICE_TYPE)
    }

    /// `FairValue` (tag 406, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn fair_value(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::FAIR_VALUE)?;
        convert::parse_fix_decimal(&raw, tags::FAIR_VALUE)
    }

    /// Whether `FairValue` (tag 406) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_fair_value(&self) -> bool {
        self.inner.has_field(tags::FAIR_VALUE)
    }

    /// `NetGrossInd` (tag 430, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::NetGrossInd`]'s.
    pub fn net_gross_ind(&self) -> Result<fields::NetGrossInd, FieldError> {
        let n = self.inner.get_field::<i32>(tags::NET_GROSS_IND)?;
        fields::NetGrossInd::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::NET_GROSS_IND,
        })
    }

    /// Whether `NetGrossInd` (tag 430) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_net_gross_ind(&self) -> bool {
        self.inner.has_field(tags::NET_GROSS_IND)
    }

    /// `SettlmntTyp` (tag 63, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SettlmntTyp`]'s.
    pub fn settlmnt_typ(&self) -> Result<fields::SettlmntTyp, FieldError> {
        let n = self.inner.get_field::<char>(tags::SETTLMNT_TYP)?;
        fields::SettlmntTyp::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SETTLMNT_TYP,
        })
    }

    /// Whether `SettlmntTyp` (tag 63) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settlmnt_typ(&self) -> bool {
        self.inner.has_field(tags::SETTLMNT_TYP)
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

    /// `Text` (tag 58, optional).
    pub fn text(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::TEXT)
    }

    /// Whether `Text` (tag 58) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_text(&self) -> bool {
        self.inner.has_field(tags::TEXT)
    }

    /// `EncodedTextLen` (tag 354, optional).
    pub fn encoded_text_len(&self) -> Result<u32, FieldError> {
        self.inner.get_field::<u32>(tags::ENCODED_TEXT_LEN)
    }

    /// Whether `EncodedTextLen` (tag 354) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text_len(&self) -> bool {
        self.inner.has_field(tags::ENCODED_TEXT_LEN)
    }

    /// `EncodedText` (tag 355, optional).
    pub fn encoded_text(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ENCODED_TEXT)
    }

    /// Whether `EncodedText` (tag 355) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text(&self) -> bool {
        self.inner.has_field(tags::ENCODED_TEXT)
    }
}

/// One `NoBidComponents` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoBidComponentsEntryMut::as_ref`].
pub struct NoBidComponentsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoBidComponentsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoBidComponentsEntry<'_> {
        NoBidComponentsEntry { inner: self.inner }
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

    /// Set `ListID` (tag 66, optional).
    pub fn set_list_id(&mut self, v: &str) {
        self.inner.set_field(tags::LIST_ID, v.to_string());
    }

    /// Set `Country` (tag 421, optional).
    pub fn set_country(&mut self, v: &str) {
        self.inner.set_field(tags::COUNTRY, v.to_string());
    }

    /// Set `Side` (tag 54, optional).
    pub fn set_side(&mut self, v: fields::Side) {
        self.inner.set_field(tags::SIDE, v.to_fix().to_string());
    }

    /// Set `Price` (tag 44, optional); written in FIX `PRICE` format.
    pub fn set_price(&mut self, v: Decimal) {
        self.inner.set_field(tags::PRICE, convert::format_fix_decimal(v));
    }

    /// Set `PriceType` (tag 423, optional).
    pub fn set_price_type(&mut self, v: fields::PriceType) {
        self.inner.set_field(tags::PRICE_TYPE, v.to_fix().to_string());
    }

    /// Set `FairValue` (tag 406, optional); written in FIX `AMT` format.
    pub fn set_fair_value(&mut self, v: Decimal) {
        self.inner.set_field(tags::FAIR_VALUE, convert::format_fix_decimal(v));
    }

    /// Set `NetGrossInd` (tag 430, optional).
    pub fn set_net_gross_ind(&mut self, v: fields::NetGrossInd) {
        self.inner.set_field(tags::NET_GROSS_IND, v.to_fix().to_string());
    }

    /// Set `SettlmntTyp` (tag 63, optional).
    pub fn set_settlmnt_typ(&mut self, v: fields::SettlmntTyp) {
        self.inner.set_field(tags::SETTLMNT_TYP, v.to_fix().to_string());
    }

    /// Set `FutSettDate` (tag 64, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_fut_sett_date(&mut self, v: NaiveDate) {
        self.inner.set_field(tags::FUT_SETT_DATE, convert::format_fix_date(v));
    }

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
    }

    /// Set `Text` (tag 58, optional).
    pub fn set_text(&mut self, v: &str) {
        self.inner.set_field(tags::TEXT, v.to_string());
    }

    /// Set `EncodedTextLen` (tag 354, optional).
    pub fn set_encoded_text_len(&mut self, v: u32) {
        self.inner.set_field(tags::ENCODED_TEXT_LEN, v.to_string());
    }

    /// Set `EncodedText` (tag 355, optional).
    pub fn set_encoded_text(&mut self, v: &str) {
        self.inner.set_field(tags::ENCODED_TEXT, v.to_string());
    }
}

/// The `NoBidComponents` repeating group (count tag `NO_BID_COMPONENTS`), read-only: its
/// instances as [`NoBidComponentsEntry`] views.
pub struct NoBidComponents<'a> {
    group: &'a Group,
}

impl<'a> NoBidComponents<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoBidComponentsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoBidComponentsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoBidComponentsEntry<'a> {
        self.get(i).expect("NoBidComponents index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoBidComponentsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoBidComponentsEntry { inner })
    }
}

/// The `NoBidComponents` repeating group (count tag `NO_BID_COMPONENTS`), mutable: its
/// instances as [`NoBidComponentsEntryMut`] views.
pub struct NoBidComponentsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoBidComponentsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoBidComponentsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoBidComponentsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoBidComponentsEntryMut<'_> {
        self.get_mut(i).expect("NoBidComponents index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoBidComponentsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoBidComponentsEntryMut { inner })
    }
}
