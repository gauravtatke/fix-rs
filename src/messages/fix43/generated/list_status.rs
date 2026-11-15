//! `ListStatus` (app message, MsgType `35=N`): typed facade over the raw
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

/// Typed facade over a `ListStatus` [`Message`] (35=N). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct ListStatus {
    msg: Message,
}

impl Default for ListStatus {
    fn default() -> Self {
        ListStatus::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `ListStatus`, checking the msg type.
impl TryFrom<Message> for ListStatus {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a ListStatus" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<ListStatus> for Message {
    fn from(typed_msg: ListStatus) -> Self {
        typed_msg.msg
    }
}

impl ListStatus {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "N";

    /// An empty `ListStatus` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- ListID (tag 66, STRING, required) ---

    /// Set `ListID` (tag 66, required).
    pub fn set_list_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LIST_ID, v.to_string());
    }

    /// `ListID` (tag 66, required).
    pub fn list_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LIST_ID)
    }

    /// Whether `ListID` (tag 66) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_id(&self) -> bool {
        self.msg.body().has_field(tags::LIST_ID)
    }

    // --- ListStatusType (tag 429, INT, required) ---

    /// Set `ListStatusType` (tag 429, required).
    pub fn set_list_status_type(&mut self, v: fields::ListStatusType) {
        self.msg.body_mut().set_field(tags::LIST_STATUS_TYPE, v.to_fix().to_string());
    }

    /// `ListStatusType` (tag 429, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ListStatusType`]'s.
    pub fn list_status_type(&self) -> Result<fields::ListStatusType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::LIST_STATUS_TYPE)?;
        fields::ListStatusType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::LIST_STATUS_TYPE,
        })
    }

    /// Whether `ListStatusType` (tag 429) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_status_type(&self) -> bool {
        self.msg.body().has_field(tags::LIST_STATUS_TYPE)
    }

    // --- NoRpts (tag 82, NUMINGROUP, required) ---

    /// Set `NoRpts` (tag 82, required).
    pub fn set_no_rpts(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::NO_RPTS, v.to_string());
    }

    /// `NoRpts` (tag 82, required).
    pub fn no_rpts(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::NO_RPTS)
    }

    /// Whether `NoRpts` (tag 82) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_no_rpts(&self) -> bool {
        self.msg.body().has_field(tags::NO_RPTS)
    }

    // --- ListOrderStatus (tag 431, INT, required) ---

    /// Set `ListOrderStatus` (tag 431, required).
    pub fn set_list_order_status(&mut self, v: fields::ListOrderStatus) {
        self.msg.body_mut().set_field(tags::LIST_ORDER_STATUS, v.to_fix().to_string());
    }

    /// `ListOrderStatus` (tag 431, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ListOrderStatus`]'s.
    pub fn list_order_status(&self) -> Result<fields::ListOrderStatus, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::LIST_ORDER_STATUS)?;
        fields::ListOrderStatus::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::LIST_ORDER_STATUS,
        })
    }

    /// Whether `ListOrderStatus` (tag 431) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_order_status(&self) -> bool {
        self.msg.body().has_field(tags::LIST_ORDER_STATUS)
    }

    // --- RptSeq (tag 83, INT, required) ---

    /// Set `RptSeq` (tag 83, required).
    pub fn set_rpt_seq(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::RPT_SEQ, v.to_string());
    }

    /// `RptSeq` (tag 83, required).
    pub fn rpt_seq(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::RPT_SEQ)
    }

    /// Whether `RptSeq` (tag 83) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_rpt_seq(&self) -> bool {
        self.msg.body().has_field(tags::RPT_SEQ)
    }

    // --- ListStatusText (tag 444, STRING, optional) ---

    /// Set `ListStatusText` (tag 444, optional).
    pub fn set_list_status_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LIST_STATUS_TEXT, v.to_string());
    }

    /// `ListStatusText` (tag 444, optional).
    pub fn list_status_text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LIST_STATUS_TEXT)
    }

    /// Whether `ListStatusText` (tag 444) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_status_text(&self) -> bool {
        self.msg.body().has_field(tags::LIST_STATUS_TEXT)
    }

    // --- EncodedListStatusTextLen (tag 445, LENGTH, optional) ---

    /// Set `EncodedListStatusTextLen` (tag 445, optional).
    pub fn set_encoded_list_status_text_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_LIST_STATUS_TEXT_LEN, v.to_string());
    }

    /// `EncodedListStatusTextLen` (tag 445, optional).
    pub fn encoded_list_status_text_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_LIST_STATUS_TEXT_LEN)
    }

    /// Whether `EncodedListStatusTextLen` (tag 445) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_list_status_text_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_LIST_STATUS_TEXT_LEN)
    }

    // --- EncodedListStatusText (tag 446, DATA, optional) ---

    /// Set `EncodedListStatusText` (tag 446, optional).
    pub fn set_encoded_list_status_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_LIST_STATUS_TEXT, v.to_string());
    }

    /// `EncodedListStatusText` (tag 446, optional).
    pub fn encoded_list_status_text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_LIST_STATUS_TEXT)
    }

    /// Whether `EncodedListStatusText` (tag 446) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_list_status_text(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_LIST_STATUS_TEXT)
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

    // --- TotNoOrders (tag 68, INT, required) ---

    /// Set `TotNoOrders` (tag 68, required).
    pub fn set_tot_no_orders(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::TOT_NO_ORDERS, v.to_string());
    }

    /// `TotNoOrders` (tag 68, required).
    pub fn tot_no_orders(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::TOT_NO_ORDERS)
    }

    /// Whether `TotNoOrders` (tag 68) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_tot_no_orders(&self) -> bool {
        self.msg.body().has_field(tags::TOT_NO_ORDERS)
    }

    // --- NoOrders group (count tag NO_ORDERS) ---

    /// The `NoOrders` group (count tag `NO_ORDERS`, required), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_orders(&self) -> Option<NoOrders<'_>> {
        self.msg.body().get_group(tags::NO_ORDERS).map(|group| NoOrders { group })
    }

    /// The `NoOrders` group (count tag `NO_ORDERS`, required) for editing, or `None` if
    /// absent.
    pub fn no_orders_mut(&mut self) -> Option<NoOrdersMut<'_>> {
        self.msg.body_mut().get_group_mut(tags::NO_ORDERS).map(|group| NoOrdersMut { group })
    }

    /// Create the `NoOrders` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_orders_entry`] on the same group.
    pub fn set_no_orders(&mut self, count: u32) -> NoOrdersMut<'_> {
        let group = self.msg.body_mut().set_group(tags::NO_ORDERS, count, NO_ORDERS_FIELD_ORDER);
        NoOrdersMut { group }
    }

    /// Append one `NoOrders` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_orders`] on the same group.
    pub fn add_no_orders_entry(&mut self, fill: impl FnOnce(&mut NoOrdersEntryMut<'_>)) {
        let inner = self.msg.body_mut().add_group_instance(tags::NO_ORDERS, NO_ORDERS_FIELD_ORDER);
        fill(&mut NoOrdersEntryMut { inner });
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

/// Wire order of a `NoOrders` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_ORDERS_FIELD_ORDER: &[u32] = &[
    tags::CL_ORD_ID,
    tags::SECONDARY_CL_ORD_ID,
    tags::CUM_QTY,
    tags::ORD_STATUS,
    tags::WORKING_INDICATOR,
    tags::LEAVES_QTY,
    tags::CXL_QTY,
    tags::AVG_PX,
    tags::ORD_REJ_REASON,
    tags::TEXT,
    tags::ENCODED_TEXT_LEN,
    tags::ENCODED_TEXT,
];

/// One `NoOrders` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoOrdersEntry<'a> {
    inner: &'a FieldMap,
}

impl NoOrdersEntry<'_> {
    /// `ClOrdID` (tag 11, required).
    pub fn cl_ord_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::CL_ORD_ID)
    }

    /// Whether `ClOrdID` (tag 11) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cl_ord_id(&self) -> bool {
        self.inner.has_field(tags::CL_ORD_ID)
    }

    /// `SecondaryClOrdID` (tag 526, optional).
    pub fn secondary_cl_ord_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::SECONDARY_CL_ORD_ID)
    }

    /// Whether `SecondaryClOrdID` (tag 526) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_secondary_cl_ord_id(&self) -> bool {
        self.inner.has_field(tags::SECONDARY_CL_ORD_ID)
    }

    /// `CumQty` (tag 14, required). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn cum_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::CUM_QTY)?;
        convert::parse_fix_decimal(&raw, tags::CUM_QTY)
    }

    /// Whether `CumQty` (tag 14) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cum_qty(&self) -> bool {
        self.inner.has_field(tags::CUM_QTY)
    }

    /// `OrdStatus` (tag 39, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrdStatus`]'s.
    pub fn ord_status(&self) -> Result<fields::OrdStatus, FieldError> {
        let n = self.inner.get_field::<char>(tags::ORD_STATUS)?;
        fields::OrdStatus::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORD_STATUS,
        })
    }

    /// Whether `OrdStatus` (tag 39) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ord_status(&self) -> bool {
        self.inner.has_field(tags::ORD_STATUS)
    }

    /// `WorkingIndicator` (tag 636, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn working_indicator(&self) -> Result<bool, FieldError> {
        let raw = self.inner.get_field::<String>(tags::WORKING_INDICATOR)?;
        convert::parse_fix_bool(&raw, tags::WORKING_INDICATOR)
    }

    /// Whether `WorkingIndicator` (tag 636) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_working_indicator(&self) -> bool {
        self.inner.has_field(tags::WORKING_INDICATOR)
    }

    /// `LeavesQty` (tag 151, required). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn leaves_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LEAVES_QTY)?;
        convert::parse_fix_decimal(&raw, tags::LEAVES_QTY)
    }

    /// Whether `LeavesQty` (tag 151) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_leaves_qty(&self) -> bool {
        self.inner.has_field(tags::LEAVES_QTY)
    }

    /// `CxlQty` (tag 84, required). `Err(InvalidFormat)` unless a valid FIX
    /// `QTY`.
    pub fn cxl_qty(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::CXL_QTY)?;
        convert::parse_fix_decimal(&raw, tags::CXL_QTY)
    }

    /// Whether `CxlQty` (tag 84) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cxl_qty(&self) -> bool {
        self.inner.has_field(tags::CXL_QTY)
    }

    /// `AvgPx` (tag 6, required). `Err(InvalidFormat)` unless a valid FIX
    /// `PRICE`.
    pub fn avg_px(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::AVG_PX)?;
        convert::parse_fix_decimal(&raw, tags::AVG_PX)
    }

    /// Whether `AvgPx` (tag 6) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_avg_px(&self) -> bool {
        self.inner.has_field(tags::AVG_PX)
    }

    /// `OrdRejReason` (tag 103, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::OrdRejReason`]'s.
    pub fn ord_rej_reason(&self) -> Result<fields::OrdRejReason, FieldError> {
        let n = self.inner.get_field::<i32>(tags::ORD_REJ_REASON)?;
        fields::OrdRejReason::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ORD_REJ_REASON,
        })
    }

    /// Whether `OrdRejReason` (tag 103) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ord_rej_reason(&self) -> bool {
        self.inner.has_field(tags::ORD_REJ_REASON)
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

/// One `NoOrders` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoOrdersEntryMut::as_ref`].
pub struct NoOrdersEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoOrdersEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoOrdersEntry<'_> {
        NoOrdersEntry { inner: self.inner }
    }

    /// Set `ClOrdID` (tag 11, required).
    pub fn set_cl_ord_id(&mut self, v: &str) {
        self.inner.set_field(tags::CL_ORD_ID, v.to_string());
    }

    /// Set `SecondaryClOrdID` (tag 526, optional).
    pub fn set_secondary_cl_ord_id(&mut self, v: &str) {
        self.inner.set_field(tags::SECONDARY_CL_ORD_ID, v.to_string());
    }

    /// Set `CumQty` (tag 14, required); written in FIX `QTY` format.
    pub fn set_cum_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::CUM_QTY, convert::format_fix_decimal(v));
    }

    /// Set `OrdStatus` (tag 39, required).
    pub fn set_ord_status(&mut self, v: fields::OrdStatus) {
        self.inner.set_field(tags::ORD_STATUS, v.to_fix().to_string());
    }

    /// Set `WorkingIndicator` (tag 636, optional); written in FIX `BOOLEAN` format.
    pub fn set_working_indicator(&mut self, v: bool) {
        self.inner.set_field(tags::WORKING_INDICATOR, convert::format_fix_bool(v));
    }

    /// Set `LeavesQty` (tag 151, required); written in FIX `QTY` format.
    pub fn set_leaves_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::LEAVES_QTY, convert::format_fix_decimal(v));
    }

    /// Set `CxlQty` (tag 84, required); written in FIX `QTY` format.
    pub fn set_cxl_qty(&mut self, v: Decimal) {
        self.inner.set_field(tags::CXL_QTY, convert::format_fix_decimal(v));
    }

    /// Set `AvgPx` (tag 6, required); written in FIX `PRICE` format.
    pub fn set_avg_px(&mut self, v: Decimal) {
        self.inner.set_field(tags::AVG_PX, convert::format_fix_decimal(v));
    }

    /// Set `OrdRejReason` (tag 103, optional).
    pub fn set_ord_rej_reason(&mut self, v: fields::OrdRejReason) {
        self.inner.set_field(tags::ORD_REJ_REASON, v.to_fix().to_string());
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

/// The `NoOrders` repeating group (count tag `NO_ORDERS`), read-only: its
/// instances as [`NoOrdersEntry`] views.
pub struct NoOrders<'a> {
    group: &'a Group,
}

impl<'a> NoOrders<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoOrdersEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoOrdersEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoOrdersEntry<'a> {
        self.get(i).expect("NoOrders index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoOrdersEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoOrdersEntry { inner })
    }
}

/// The `NoOrders` repeating group (count tag `NO_ORDERS`), mutable: its
/// instances as [`NoOrdersEntryMut`] views.
pub struct NoOrdersMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoOrdersMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoOrdersEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoOrdersEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoOrdersEntryMut<'_> {
        self.get_mut(i).expect("NoOrders index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoOrdersEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoOrdersEntryMut { inner })
    }
}
