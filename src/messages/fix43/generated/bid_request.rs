//! `BidRequest` (app message, MsgType `35=k`): typed facade over the raw
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

/// Typed facade over a `BidRequest` [`Message`] (35=k). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct BidRequest {
    msg: Message,
}

impl Default for BidRequest {
    fn default() -> Self {
        BidRequest::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `BidRequest`, checking the msg type.
impl TryFrom<Message> for BidRequest {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a BidRequest" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<BidRequest> for Message {
    fn from(typed_msg: BidRequest) -> Self {
        typed_msg.msg
    }
}

impl BidRequest {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "k";

    /// An empty `BidRequest` with only MsgType (35) set.
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

    // --- ClientBidID (tag 391, STRING, required) ---

    /// Set `ClientBidID` (tag 391, required).
    pub fn set_client_bid_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CLIENT_BID_ID, v.to_string());
    }

    /// `ClientBidID` (tag 391, required).
    pub fn client_bid_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CLIENT_BID_ID)
    }

    /// Whether `ClientBidID` (tag 391) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_client_bid_id(&self) -> bool {
        self.msg.body().has_field(tags::CLIENT_BID_ID)
    }

    // --- BidRequestTransType (tag 374, CHAR, required) ---

    /// Set `BidRequestTransType` (tag 374, required).
    pub fn set_bid_request_trans_type(&mut self, v: fields::BidRequestTransType) {
        self.msg.body_mut().set_field(tags::BID_REQUEST_TRANS_TYPE, v.to_fix().to_string());
    }

    /// `BidRequestTransType` (tag 374, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::BidRequestTransType`]'s.
    pub fn bid_request_trans_type(&self) -> Result<fields::BidRequestTransType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::BID_REQUEST_TRANS_TYPE)?;
        fields::BidRequestTransType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::BID_REQUEST_TRANS_TYPE,
        })
    }

    /// Whether `BidRequestTransType` (tag 374) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_request_trans_type(&self) -> bool {
        self.msg.body().has_field(tags::BID_REQUEST_TRANS_TYPE)
    }

    // --- ListName (tag 392, STRING, optional) ---

    /// Set `ListName` (tag 392, optional).
    pub fn set_list_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LIST_NAME, v.to_string());
    }

    /// `ListName` (tag 392, optional).
    pub fn list_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LIST_NAME)
    }

    /// Whether `ListName` (tag 392) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_name(&self) -> bool {
        self.msg.body().has_field(tags::LIST_NAME)
    }

    // --- TotalNumSecurities (tag 393, INT, required) ---

    /// Set `TotalNumSecurities` (tag 393, required).
    pub fn set_total_num_securities(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::TOTAL_NUM_SECURITIES, v.to_string());
    }

    /// `TotalNumSecurities` (tag 393, required).
    pub fn total_num_securities(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::TOTAL_NUM_SECURITIES)
    }

    /// Whether `TotalNumSecurities` (tag 393) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_total_num_securities(&self) -> bool {
        self.msg.body().has_field(tags::TOTAL_NUM_SECURITIES)
    }

    // --- BidType (tag 394, INT, required) ---

    /// Set `BidType` (tag 394, required).
    pub fn set_bid_type(&mut self, v: fields::BidType) {
        self.msg.body_mut().set_field(tags::BID_TYPE, v.to_fix().to_string());
    }

    /// `BidType` (tag 394, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::BidType`]'s.
    pub fn bid_type(&self) -> Result<fields::BidType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::BID_TYPE)?;
        fields::BidType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::BID_TYPE,
        })
    }

    /// Whether `BidType` (tag 394) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_type(&self) -> bool {
        self.msg.body().has_field(tags::BID_TYPE)
    }

    // --- NumTickets (tag 395, INT, optional) ---

    /// Set `NumTickets` (tag 395, optional).
    pub fn set_num_tickets(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::NUM_TICKETS, v.to_string());
    }

    /// `NumTickets` (tag 395, optional).
    pub fn num_tickets(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::NUM_TICKETS)
    }

    /// Whether `NumTickets` (tag 395) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_num_tickets(&self) -> bool {
        self.msg.body().has_field(tags::NUM_TICKETS)
    }

    // --- Currency (tag 15, CURRENCY, optional) ---

    /// Set `Currency` (tag 15, optional).
    pub fn set_currency(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CURRENCY, v.to_string());
    }

    /// `Currency` (tag 15, optional).
    pub fn currency(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CURRENCY)
    }

    /// Whether `Currency` (tag 15) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_currency(&self) -> bool {
        self.msg.body().has_field(tags::CURRENCY)
    }

    // --- SideValue1 (tag 396, AMT, optional) ---

    /// Set `SideValue1` (tag 396, optional); written in FIX `AMT` format.
    pub fn set_side_value1(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::SIDE_VALUE1, convert::format_fix_decimal(v));
    }

    /// `SideValue1` (tag 396, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn side_value1(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SIDE_VALUE1)?;
        convert::parse_fix_decimal(&raw, tags::SIDE_VALUE1)
    }

    /// Whether `SideValue1` (tag 396) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side_value1(&self) -> bool {
        self.msg.body().has_field(tags::SIDE_VALUE1)
    }

    // --- SideValue2 (tag 397, AMT, optional) ---

    /// Set `SideValue2` (tag 397, optional); written in FIX `AMT` format.
    pub fn set_side_value2(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::SIDE_VALUE2, convert::format_fix_decimal(v));
    }

    /// `SideValue2` (tag 397, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn side_value2(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SIDE_VALUE2)?;
        convert::parse_fix_decimal(&raw, tags::SIDE_VALUE2)
    }

    /// Whether `SideValue2` (tag 397) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side_value2(&self) -> bool {
        self.msg.body().has_field(tags::SIDE_VALUE2)
    }

    // --- LiquidityIndType (tag 409, INT, optional) ---

    /// Set `LiquidityIndType` (tag 409, optional).
    pub fn set_liquidity_ind_type(&mut self, v: fields::LiquidityIndType) {
        self.msg.body_mut().set_field(tags::LIQUIDITY_IND_TYPE, v.to_fix().to_string());
    }

    /// `LiquidityIndType` (tag 409, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::LiquidityIndType`]'s.
    pub fn liquidity_ind_type(&self) -> Result<fields::LiquidityIndType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::LIQUIDITY_IND_TYPE)?;
        fields::LiquidityIndType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::LIQUIDITY_IND_TYPE,
        })
    }

    /// Whether `LiquidityIndType` (tag 409) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_liquidity_ind_type(&self) -> bool {
        self.msg.body().has_field(tags::LIQUIDITY_IND_TYPE)
    }

    // --- WtAverageLiquidity (tag 410, PERCENTAGE, optional) ---

    /// Set `WtAverageLiquidity` (tag 410, optional).
    pub fn set_wt_average_liquidity(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::WT_AVERAGE_LIQUIDITY, v.to_string());
    }

    /// `WtAverageLiquidity` (tag 410, optional).
    pub fn wt_average_liquidity(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::WT_AVERAGE_LIQUIDITY)
    }

    /// Whether `WtAverageLiquidity` (tag 410) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_wt_average_liquidity(&self) -> bool {
        self.msg.body().has_field(tags::WT_AVERAGE_LIQUIDITY)
    }

    // --- ExchangeForPhysical (tag 411, BOOLEAN, optional) ---

    /// Set `ExchangeForPhysical` (tag 411, optional); written in FIX `BOOLEAN` format.
    pub fn set_exchange_for_physical(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::EXCHANGE_FOR_PHYSICAL, convert::format_fix_bool(v));
    }

    /// `ExchangeForPhysical` (tag 411, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn exchange_for_physical(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::EXCHANGE_FOR_PHYSICAL)?;
        convert::parse_fix_bool(&raw, tags::EXCHANGE_FOR_PHYSICAL)
    }

    /// Whether `ExchangeForPhysical` (tag 411) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_exchange_for_physical(&self) -> bool {
        self.msg.body().has_field(tags::EXCHANGE_FOR_PHYSICAL)
    }

    // --- OutMainCntryUIndex (tag 412, AMT, optional) ---

    /// Set `OutMainCntryUIndex` (tag 412, optional); written in FIX `AMT` format.
    pub fn set_out_main_cntry_u_index(&mut self, v: Decimal) {
        self.msg.body_mut().set_field(tags::OUT_MAIN_CNTRY_U_INDEX, convert::format_fix_decimal(v));
    }

    /// `OutMainCntryUIndex` (tag 412, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn out_main_cntry_u_index(&self) -> Result<Decimal, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::OUT_MAIN_CNTRY_U_INDEX)?;
        convert::parse_fix_decimal(&raw, tags::OUT_MAIN_CNTRY_U_INDEX)
    }

    /// Whether `OutMainCntryUIndex` (tag 412) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_out_main_cntry_u_index(&self) -> bool {
        self.msg.body().has_field(tags::OUT_MAIN_CNTRY_U_INDEX)
    }

    // --- CrossPercent (tag 413, PERCENTAGE, optional) ---

    /// Set `CrossPercent` (tag 413, optional).
    pub fn set_cross_percent(&mut self, v: f64) {
        self.msg.body_mut().set_field(tags::CROSS_PERCENT, v.to_string());
    }

    /// `CrossPercent` (tag 413, optional).
    pub fn cross_percent(&self) -> Result<f64, FieldError> {
        self.msg.body().get_field::<f64>(tags::CROSS_PERCENT)
    }

    /// Whether `CrossPercent` (tag 413) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cross_percent(&self) -> bool {
        self.msg.body().has_field(tags::CROSS_PERCENT)
    }

    // --- ProgRptReqs (tag 414, INT, optional) ---

    /// Set `ProgRptReqs` (tag 414, optional).
    pub fn set_prog_rpt_reqs(&mut self, v: fields::ProgRptReqs) {
        self.msg.body_mut().set_field(tags::PROG_RPT_REQS, v.to_fix().to_string());
    }

    /// `ProgRptReqs` (tag 414, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::ProgRptReqs`]'s.
    pub fn prog_rpt_reqs(&self) -> Result<fields::ProgRptReqs, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::PROG_RPT_REQS)?;
        fields::ProgRptReqs::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::PROG_RPT_REQS,
        })
    }

    /// Whether `ProgRptReqs` (tag 414) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_prog_rpt_reqs(&self) -> bool {
        self.msg.body().has_field(tags::PROG_RPT_REQS)
    }

    // --- ProgPeriodInterval (tag 415, INT, optional) ---

    /// Set `ProgPeriodInterval` (tag 415, optional).
    pub fn set_prog_period_interval(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::PROG_PERIOD_INTERVAL, v.to_string());
    }

    /// `ProgPeriodInterval` (tag 415, optional).
    pub fn prog_period_interval(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::PROG_PERIOD_INTERVAL)
    }

    /// Whether `ProgPeriodInterval` (tag 415) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_prog_period_interval(&self) -> bool {
        self.msg.body().has_field(tags::PROG_PERIOD_INTERVAL)
    }

    // --- IncTaxInd (tag 416, INT, optional) ---

    /// Set `IncTaxInd` (tag 416, optional).
    pub fn set_inc_tax_ind(&mut self, v: fields::IncTaxInd) {
        self.msg.body_mut().set_field(tags::INC_TAX_IND, v.to_fix().to_string());
    }

    /// `IncTaxInd` (tag 416, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::IncTaxInd`]'s.
    pub fn inc_tax_ind(&self) -> Result<fields::IncTaxInd, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::INC_TAX_IND)?;
        fields::IncTaxInd::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::INC_TAX_IND,
        })
    }

    /// Whether `IncTaxInd` (tag 416) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_inc_tax_ind(&self) -> bool {
        self.msg.body().has_field(tags::INC_TAX_IND)
    }

    // --- ForexReq (tag 121, BOOLEAN, optional) ---

    /// Set `ForexReq` (tag 121, optional); written in FIX `BOOLEAN` format.
    pub fn set_forex_req(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::FOREX_REQ, convert::format_fix_bool(v));
    }

    /// `ForexReq` (tag 121, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn forex_req(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::FOREX_REQ)?;
        convert::parse_fix_bool(&raw, tags::FOREX_REQ)
    }

    /// Whether `ForexReq` (tag 121) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_forex_req(&self) -> bool {
        self.msg.body().has_field(tags::FOREX_REQ)
    }

    // --- NumBidders (tag 417, INT, optional) ---

    /// Set `NumBidders` (tag 417, optional).
    pub fn set_num_bidders(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::NUM_BIDDERS, v.to_string());
    }

    /// `NumBidders` (tag 417, optional).
    pub fn num_bidders(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::NUM_BIDDERS)
    }

    /// Whether `NumBidders` (tag 417) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_num_bidders(&self) -> bool {
        self.msg.body().has_field(tags::NUM_BIDDERS)
    }

    // --- TradeDate (tag 75, LOCALMKTDATE, optional) ---

    /// Set `TradeDate` (tag 75, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_trade_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::TRADE_DATE, convert::format_fix_date(v));
    }

    /// `TradeDate` (tag 75, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn trade_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRADE_DATE)?;
        convert::parse_fix_date(&raw, tags::TRADE_DATE)
    }

    /// Whether `TradeDate` (tag 75) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_date(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_DATE)
    }

    // --- TradeType (tag 418, CHAR, required) ---

    /// Set `TradeType` (tag 418, required).
    pub fn set_trade_type(&mut self, v: char) {
        self.msg.body_mut().set_field(tags::TRADE_TYPE, v.to_string());
    }

    /// `TradeType` (tag 418, required).
    pub fn trade_type(&self) -> Result<char, FieldError> {
        self.msg.body().get_field::<char>(tags::TRADE_TYPE)
    }

    /// Whether `TradeType` (tag 418) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_trade_type(&self) -> bool {
        self.msg.body().has_field(tags::TRADE_TYPE)
    }

    // --- BasisPxType (tag 419, CHAR, required) ---

    /// Set `BasisPxType` (tag 419, required).
    pub fn set_basis_px_type(&mut self, v: fields::BasisPxType) {
        self.msg.body_mut().set_field(tags::BASIS_PX_TYPE, v.to_fix().to_string());
    }

    /// `BasisPxType` (tag 419, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::BasisPxType`]'s.
    pub fn basis_px_type(&self) -> Result<fields::BasisPxType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::BASIS_PX_TYPE)?;
        fields::BasisPxType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::BASIS_PX_TYPE,
        })
    }

    /// Whether `BasisPxType` (tag 419) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_basis_px_type(&self) -> bool {
        self.msg.body().has_field(tags::BASIS_PX_TYPE)
    }

    // --- StrikeTime (tag 443, UTCTIMESTAMP, optional) ---

    /// Set `StrikeTime` (tag 443, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_strike_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::STRIKE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `StrikeTime` (tag 443, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn strike_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::STRIKE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::STRIKE_TIME)
    }

    /// Whether `StrikeTime` (tag 443) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_strike_time(&self) -> bool {
        self.msg.body().has_field(tags::STRIKE_TIME)
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

    // --- NoBidDescriptors group (count tag NO_BID_DESCRIPTORS) ---

    /// The `NoBidDescriptors` group (count tag `NO_BID_DESCRIPTORS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_bid_descriptors(&self) -> Option<NoBidDescriptors<'_>> {
        self.msg.body().get_group(tags::NO_BID_DESCRIPTORS).map(|group| NoBidDescriptors { group })
    }

    /// The `NoBidDescriptors` group (count tag `NO_BID_DESCRIPTORS`, optional) for editing, or `None` if
    /// absent.
    pub fn no_bid_descriptors_mut(&mut self) -> Option<NoBidDescriptorsMut<'_>> {
        self.msg
            .body_mut()
            .get_group_mut(tags::NO_BID_DESCRIPTORS)
            .map(|group| NoBidDescriptorsMut { group })
    }

    /// Create the `NoBidDescriptors` group with `count` empty instances to fill by index. Each
    /// instance serializes in dictionary order.
    ///
    /// Additive, not idempotent: on an existing group it appends `count` more instances while the
    /// count field says `count`. Call it once per group, and don't mix it with
    /// [`Self::add_no_bid_descriptors_entry`] on the same group.
    pub fn set_no_bid_descriptors(&mut self, count: u32) -> NoBidDescriptorsMut<'_> {
        let group = self.msg.body_mut().set_group(
            tags::NO_BID_DESCRIPTORS,
            count,
            NO_BID_DESCRIPTORS_FIELD_ORDER,
        );
        NoBidDescriptorsMut { group }
    }

    /// Append one `NoBidDescriptors` instance, filled by `fill`; creates the group if absent and
    /// keeps its count in sync. The instance serializes in dictionary order whatever order
    /// `fill` sets its fields in. Don't mix with [`Self::set_no_bid_descriptors`] on the same group.
    pub fn add_no_bid_descriptors_entry(
        &mut self,
        fill: impl FnOnce(&mut NoBidDescriptorsEntryMut<'_>),
    ) {
        let inner = self
            .msg
            .body_mut()
            .add_group_instance(tags::NO_BID_DESCRIPTORS, NO_BID_DESCRIPTORS_FIELD_ORDER);
        fill(&mut NoBidDescriptorsEntryMut { inner });
    }

    // --- NoBidComponents group (count tag NO_BID_COMPONENTS) ---

    /// The `NoBidComponents` group (count tag `NO_BID_COMPONENTS`, optional), or `None` if absent. An
    /// existing group may still have zero instances.
    pub fn no_bid_components(&self) -> Option<NoBidComponents<'_>> {
        self.msg.body().get_group(tags::NO_BID_COMPONENTS).map(|group| NoBidComponents { group })
    }

    /// The `NoBidComponents` group (count tag `NO_BID_COMPONENTS`, optional) for editing, or `None` if
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
}

/// Wire order of a `NoBidDescriptors` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_BID_DESCRIPTORS_FIELD_ORDER: &[u32] = &[
    tags::BID_DESCRIPTOR_TYPE,
    tags::BID_DESCRIPTOR,
    tags::SIDE_VALUE_IND,
    tags::LIQUIDITY_VALUE,
    tags::LIQUIDITY_NUM_SECURITIES,
    tags::LIQUIDITY_PCT_LOW,
    tags::LIQUIDITY_PCT_HIGH,
    tags::EFP_TRACKING_ERROR,
    tags::FAIR_VALUE,
    tags::OUTSIDE_INDEX_PCT,
    tags::VALUE_OF_FUTURES,
];

/// One `NoBidDescriptors` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoBidDescriptorsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoBidDescriptorsEntry<'_> {
    /// `BidDescriptorType` (tag 399, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::BidDescriptorType`]'s.
    pub fn bid_descriptor_type(&self) -> Result<fields::BidDescriptorType, FieldError> {
        let n = self.inner.get_field::<i32>(tags::BID_DESCRIPTOR_TYPE)?;
        fields::BidDescriptorType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::BID_DESCRIPTOR_TYPE,
        })
    }

    /// Whether `BidDescriptorType` (tag 399) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_descriptor_type(&self) -> bool {
        self.inner.has_field(tags::BID_DESCRIPTOR_TYPE)
    }

    /// `BidDescriptor` (tag 400, optional).
    pub fn bid_descriptor(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::BID_DESCRIPTOR)
    }

    /// Whether `BidDescriptor` (tag 400) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_descriptor(&self) -> bool {
        self.inner.has_field(tags::BID_DESCRIPTOR)
    }

    /// `SideValueInd` (tag 401, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SideValueInd`]'s.
    pub fn side_value_ind(&self) -> Result<fields::SideValueInd, FieldError> {
        let n = self.inner.get_field::<i32>(tags::SIDE_VALUE_IND)?;
        fields::SideValueInd::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SIDE_VALUE_IND,
        })
    }

    /// Whether `SideValueInd` (tag 401) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side_value_ind(&self) -> bool {
        self.inner.has_field(tags::SIDE_VALUE_IND)
    }

    /// `LiquidityValue` (tag 404, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn liquidity_value(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::LIQUIDITY_VALUE)?;
        convert::parse_fix_decimal(&raw, tags::LIQUIDITY_VALUE)
    }

    /// Whether `LiquidityValue` (tag 404) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_liquidity_value(&self) -> bool {
        self.inner.has_field(tags::LIQUIDITY_VALUE)
    }

    /// `LiquidityNumSecurities` (tag 441, optional).
    pub fn liquidity_num_securities(&self) -> Result<i32, FieldError> {
        self.inner.get_field::<i32>(tags::LIQUIDITY_NUM_SECURITIES)
    }

    /// Whether `LiquidityNumSecurities` (tag 441) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_liquidity_num_securities(&self) -> bool {
        self.inner.has_field(tags::LIQUIDITY_NUM_SECURITIES)
    }

    /// `LiquidityPctLow` (tag 402, optional).
    pub fn liquidity_pct_low(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LIQUIDITY_PCT_LOW)
    }

    /// Whether `LiquidityPctLow` (tag 402) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_liquidity_pct_low(&self) -> bool {
        self.inner.has_field(tags::LIQUIDITY_PCT_LOW)
    }

    /// `LiquidityPctHigh` (tag 403, optional).
    pub fn liquidity_pct_high(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::LIQUIDITY_PCT_HIGH)
    }

    /// Whether `LiquidityPctHigh` (tag 403) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_liquidity_pct_high(&self) -> bool {
        self.inner.has_field(tags::LIQUIDITY_PCT_HIGH)
    }

    /// `EFPTrackingError` (tag 405, optional).
    pub fn efp_tracking_error(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::EFP_TRACKING_ERROR)
    }

    /// Whether `EFPTrackingError` (tag 405) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_efp_tracking_error(&self) -> bool {
        self.inner.has_field(tags::EFP_TRACKING_ERROR)
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

    /// `OutsideIndexPct` (tag 407, optional).
    pub fn outside_index_pct(&self) -> Result<f64, FieldError> {
        self.inner.get_field::<f64>(tags::OUTSIDE_INDEX_PCT)
    }

    /// Whether `OutsideIndexPct` (tag 407) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_outside_index_pct(&self) -> bool {
        self.inner.has_field(tags::OUTSIDE_INDEX_PCT)
    }

    /// `ValueOfFutures` (tag 408, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `AMT`.
    pub fn value_of_futures(&self) -> Result<Decimal, FieldError> {
        let raw = self.inner.get_field::<String>(tags::VALUE_OF_FUTURES)?;
        convert::parse_fix_decimal(&raw, tags::VALUE_OF_FUTURES)
    }

    /// Whether `ValueOfFutures` (tag 408) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_value_of_futures(&self) -> bool {
        self.inner.has_field(tags::VALUE_OF_FUTURES)
    }
}

/// One `NoBidDescriptors` instance, mutable: field setters, plus write access to nested groups.
/// Read through [`NoBidDescriptorsEntryMut::as_ref`].
pub struct NoBidDescriptorsEntryMut<'a> {
    inner: &'a mut FieldMap,
}

impl NoBidDescriptorsEntryMut<'_> {
    /// Read-only view of this instance (its getters, `has_*` and nested-group readers).
    pub fn as_ref(&self) -> NoBidDescriptorsEntry<'_> {
        NoBidDescriptorsEntry { inner: self.inner }
    }

    /// Set `BidDescriptorType` (tag 399, optional).
    pub fn set_bid_descriptor_type(&mut self, v: fields::BidDescriptorType) {
        self.inner.set_field(tags::BID_DESCRIPTOR_TYPE, v.to_fix().to_string());
    }

    /// Set `BidDescriptor` (tag 400, optional).
    pub fn set_bid_descriptor(&mut self, v: &str) {
        self.inner.set_field(tags::BID_DESCRIPTOR, v.to_string());
    }

    /// Set `SideValueInd` (tag 401, optional).
    pub fn set_side_value_ind(&mut self, v: fields::SideValueInd) {
        self.inner.set_field(tags::SIDE_VALUE_IND, v.to_fix().to_string());
    }

    /// Set `LiquidityValue` (tag 404, optional); written in FIX `AMT` format.
    pub fn set_liquidity_value(&mut self, v: Decimal) {
        self.inner.set_field(tags::LIQUIDITY_VALUE, convert::format_fix_decimal(v));
    }

    /// Set `LiquidityNumSecurities` (tag 441, optional).
    pub fn set_liquidity_num_securities(&mut self, v: i32) {
        self.inner.set_field(tags::LIQUIDITY_NUM_SECURITIES, v.to_string());
    }

    /// Set `LiquidityPctLow` (tag 402, optional).
    pub fn set_liquidity_pct_low(&mut self, v: f64) {
        self.inner.set_field(tags::LIQUIDITY_PCT_LOW, v.to_string());
    }

    /// Set `LiquidityPctHigh` (tag 403, optional).
    pub fn set_liquidity_pct_high(&mut self, v: f64) {
        self.inner.set_field(tags::LIQUIDITY_PCT_HIGH, v.to_string());
    }

    /// Set `EFPTrackingError` (tag 405, optional).
    pub fn set_efp_tracking_error(&mut self, v: f64) {
        self.inner.set_field(tags::EFP_TRACKING_ERROR, v.to_string());
    }

    /// Set `FairValue` (tag 406, optional); written in FIX `AMT` format.
    pub fn set_fair_value(&mut self, v: Decimal) {
        self.inner.set_field(tags::FAIR_VALUE, convert::format_fix_decimal(v));
    }

    /// Set `OutsideIndexPct` (tag 407, optional).
    pub fn set_outside_index_pct(&mut self, v: f64) {
        self.inner.set_field(tags::OUTSIDE_INDEX_PCT, v.to_string());
    }

    /// Set `ValueOfFutures` (tag 408, optional); written in FIX `AMT` format.
    pub fn set_value_of_futures(&mut self, v: Decimal) {
        self.inner.set_field(tags::VALUE_OF_FUTURES, convert::format_fix_decimal(v));
    }
}

/// The `NoBidDescriptors` repeating group (count tag `NO_BID_DESCRIPTORS`), read-only: its
/// instances as [`NoBidDescriptorsEntry`] views.
pub struct NoBidDescriptors<'a> {
    group: &'a Group,
}

impl<'a> NoBidDescriptors<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<NoBidDescriptorsEntry<'a>> {
        self.group.as_slice().get(i).map(|inner| NoBidDescriptorsEntry { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`. A parsed message's instance
    /// count is validated against its count field, so use `len()` / `iter()` / `get()` to stay
    /// in range.
    pub fn index(&self, i: usize) -> NoBidDescriptorsEntry<'a> {
        self.get(i).expect("NoBidDescriptors index out of range")
    }

    /// The instances, in order.
    pub fn iter(&self) -> impl Iterator<Item = NoBidDescriptorsEntry<'a>> {
        self.group.as_slice().iter().map(|inner| NoBidDescriptorsEntry { inner })
    }
}

/// The `NoBidDescriptors` repeating group (count tag `NO_BID_DESCRIPTORS`), mutable: its
/// instances as [`NoBidDescriptorsEntryMut`] views.
pub struct NoBidDescriptorsMut<'a> {
    group: &'a mut Group,
}

impl<'a> NoBidDescriptorsMut<'a> {
    /// Number of instances.
    pub fn len(&self) -> usize {
        self.group.as_slice().len()
    }

    /// Whether the group has no instances.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Instance `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<NoBidDescriptorsEntryMut<'_>> {
        self.group.as_mut_slice().get_mut(i).map(|inner| NoBidDescriptorsEntryMut { inner })
    }

    /// Instance `i`. **Panics** if out of range, like `vec[i]`.
    pub fn index_mut(&mut self, i: usize) -> NoBidDescriptorsEntryMut<'_> {
        self.get_mut(i).expect("NoBidDescriptors index out of range")
    }

    /// The instances, in order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = NoBidDescriptorsEntryMut<'_>> {
        self.group.as_mut_slice().iter_mut().map(|inner| NoBidDescriptorsEntryMut { inner })
    }
}

/// Wire order of a `NoBidComponents` instance (FIX 4.3 Vol 1 §4: dictionary order, delimiter first):
/// each field's tag, and a nested group's count tag where that group sits.
const NO_BID_COMPONENTS_FIELD_ORDER: &[u32] = &[
    tags::LIST_ID,
    tags::SIDE,
    tags::TRADING_SESSION_ID,
    tags::TRADING_SESSION_SUB_ID,
    tags::NET_GROSS_IND,
    tags::SETTLMNT_TYP,
    tags::FUT_SETT_DATE,
    tags::ACCOUNT,
];

/// One `NoBidComponents` instance, read-only: field getters and `has_*`, plus read access to
/// nested groups. A view over the instance's [`FieldMap`]; nothing is copied.
pub struct NoBidComponentsEntry<'a> {
    inner: &'a FieldMap,
}

impl NoBidComponentsEntry<'_> {
    /// `ListID` (tag 66, optional).
    pub fn list_id(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::LIST_ID)
    }

    /// Whether `ListID` (tag 66) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_id(&self) -> bool {
        self.inner.has_field(tags::LIST_ID)
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

    /// `Account` (tag 1, optional).
    pub fn account(&self) -> Result<String, FieldError> {
        self.inner.get_field::<String>(tags::ACCOUNT)
    }

    /// Whether `Account` (tag 1) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_account(&self) -> bool {
        self.inner.has_field(tags::ACCOUNT)
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

    /// Set `ListID` (tag 66, optional).
    pub fn set_list_id(&mut self, v: &str) {
        self.inner.set_field(tags::LIST_ID, v.to_string());
    }

    /// Set `Side` (tag 54, optional).
    pub fn set_side(&mut self, v: fields::Side) {
        self.inner.set_field(tags::SIDE, v.to_fix().to_string());
    }

    /// Set `TradingSessionID` (tag 336, optional).
    pub fn set_trading_session_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_ID, v.to_string());
    }

    /// Set `TradingSessionSubID` (tag 625, optional).
    pub fn set_trading_session_sub_id(&mut self, v: &str) {
        self.inner.set_field(tags::TRADING_SESSION_SUB_ID, v.to_string());
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

    /// Set `Account` (tag 1, optional).
    pub fn set_account(&mut self, v: &str) {
        self.inner.set_field(tags::ACCOUNT, v.to_string());
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
