//! `SettlementInstructions` (app message, MsgType `35=T`): typed facade over the raw
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

/// Typed facade over a `SettlementInstructions` [`Message`] (35=T). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct SettlementInstructions {
    msg: Message,
}

impl Default for SettlementInstructions {
    fn default() -> Self {
        SettlementInstructions::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `SettlementInstructions`, checking the msg type.
impl TryFrom<Message> for SettlementInstructions {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a SettlementInstructions" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<SettlementInstructions> for Message {
    fn from(typed_msg: SettlementInstructions) -> Self {
        typed_msg.msg
    }
}

impl SettlementInstructions {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "T";

    /// An empty `SettlementInstructions` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- SettlInstID (tag 162, STRING, required) ---

    /// Set `SettlInstID` (tag 162, required).
    pub fn set_settl_inst_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SETTL_INST_ID, v.to_string());
    }

    /// `SettlInstID` (tag 162, required).
    pub fn settl_inst_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SETTL_INST_ID)
    }

    /// Whether `SettlInstID` (tag 162) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_id(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_INST_ID)
    }

    // --- SettlInstTransType (tag 163, CHAR, required) ---

    /// Set `SettlInstTransType` (tag 163, required).
    pub fn set_settl_inst_trans_type(&mut self, v: fields::SettlInstTransType) {
        self.msg.body_mut().set_field(tags::SETTL_INST_TRANS_TYPE, v.to_fix().to_string());
    }

    /// `SettlInstTransType` (tag 163, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SettlInstTransType`]'s.
    pub fn settl_inst_trans_type(&self) -> Result<fields::SettlInstTransType, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SETTL_INST_TRANS_TYPE)?;
        fields::SettlInstTransType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SETTL_INST_TRANS_TYPE,
        })
    }

    /// Whether `SettlInstTransType` (tag 163) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_trans_type(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_INST_TRANS_TYPE)
    }

    // --- SettlInstRefID (tag 214, STRING, required) ---

    /// Set `SettlInstRefID` (tag 214, required).
    pub fn set_settl_inst_ref_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SETTL_INST_REF_ID, v.to_string());
    }

    /// `SettlInstRefID` (tag 214, required).
    pub fn settl_inst_ref_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SETTL_INST_REF_ID)
    }

    /// Whether `SettlInstRefID` (tag 214) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_ref_id(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_INST_REF_ID)
    }

    // --- SettlInstMode (tag 160, CHAR, required) ---

    /// Set `SettlInstMode` (tag 160, required).
    pub fn set_settl_inst_mode(&mut self, v: fields::SettlInstMode) {
        self.msg.body_mut().set_field(tags::SETTL_INST_MODE, v.to_fix().to_string());
    }

    /// `SettlInstMode` (tag 160, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SettlInstMode`]'s.
    pub fn settl_inst_mode(&self) -> Result<fields::SettlInstMode, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SETTL_INST_MODE)?;
        fields::SettlInstMode::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SETTL_INST_MODE,
        })
    }

    /// Whether `SettlInstMode` (tag 160) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_mode(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_INST_MODE)
    }

    // --- SettlInstSource (tag 165, CHAR, required) ---

    /// Set `SettlInstSource` (tag 165, required).
    pub fn set_settl_inst_source(&mut self, v: fields::SettlInstSource) {
        self.msg.body_mut().set_field(tags::SETTL_INST_SOURCE, v.to_fix().to_string());
    }

    /// `SettlInstSource` (tag 165, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SettlInstSource`]'s.
    pub fn settl_inst_source(&self) -> Result<fields::SettlInstSource, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SETTL_INST_SOURCE)?;
        fields::SettlInstSource::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::SETTL_INST_SOURCE,
        })
    }

    /// Whether `SettlInstSource` (tag 165) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_source(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_INST_SOURCE)
    }

    // --- AllocAccount (tag 79, STRING, required) ---

    /// Set `AllocAccount` (tag 79, required).
    pub fn set_alloc_account(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ALLOC_ACCOUNT, v.to_string());
    }

    /// `AllocAccount` (tag 79, required).
    pub fn alloc_account(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ALLOC_ACCOUNT)
    }

    /// Whether `AllocAccount` (tag 79) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_account(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_ACCOUNT)
    }

    // --- IndividualAllocID (tag 467, STRING, optional) ---

    /// Set `IndividualAllocID` (tag 467, optional).
    pub fn set_individual_alloc_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::INDIVIDUAL_ALLOC_ID, v.to_string());
    }

    /// `IndividualAllocID` (tag 467, optional).
    pub fn individual_alloc_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::INDIVIDUAL_ALLOC_ID)
    }

    /// Whether `IndividualAllocID` (tag 467) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_individual_alloc_id(&self) -> bool {
        self.msg.body().has_field(tags::INDIVIDUAL_ALLOC_ID)
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

    // --- AllocID (tag 70, STRING, optional) ---

    /// Set `AllocID` (tag 70, optional).
    pub fn set_alloc_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ALLOC_ID, v.to_string());
    }

    /// `AllocID` (tag 70, optional).
    pub fn alloc_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ALLOC_ID)
    }

    /// Whether `AllocID` (tag 70) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_alloc_id(&self) -> bool {
        self.msg.body().has_field(tags::ALLOC_ID)
    }

    // --- LastMkt (tag 30, EXCHANGE, optional) ---

    /// Set `LastMkt` (tag 30, optional).
    pub fn set_last_mkt(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LAST_MKT, v.to_string());
    }

    /// `LastMkt` (tag 30, optional).
    pub fn last_mkt(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LAST_MKT)
    }

    /// Whether `LastMkt` (tag 30) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_last_mkt(&self) -> bool {
        self.msg.body().has_field(tags::LAST_MKT)
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

    // --- Side (tag 54, CHAR, optional) ---

    /// Set `Side` (tag 54, optional).
    pub fn set_side(&mut self, v: fields::Side) {
        self.msg.body_mut().set_field(tags::SIDE, v.to_fix().to_string());
    }

    /// `Side` (tag 54, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::Side`]'s.
    pub fn side(&self) -> Result<fields::Side, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::SIDE)?;
        fields::Side::from_fix(n).ok_or(FieldError::InvalidFormat { tag: tags::SIDE })
    }

    /// Whether `Side` (tag 54) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_side(&self) -> bool {
        self.msg.body().has_field(tags::SIDE)
    }

    // --- SecurityType (tag 167, STRING, optional) ---

    /// Set `SecurityType` (tag 167, optional).
    pub fn set_security_type(&mut self, v: fields::SecurityType) {
        self.msg.body_mut().set_field(tags::SECURITY_TYPE, v.to_fix().to_string());
    }

    /// `SecurityType` (tag 167, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::SecurityType`]'s.
    pub fn security_type(&self) -> Result<fields::SecurityType, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::SECURITY_TYPE)?;
        fields::SecurityType::from_fix(&raw).ok_or(FieldError::InvalidFormat {
            tag: tags::SECURITY_TYPE,
        })
    }

    /// Whether `SecurityType` (tag 167) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_type(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_TYPE)
    }

    // --- EffectiveTime (tag 168, UTCTIMESTAMP, optional) ---

    /// Set `EffectiveTime` (tag 168, optional); written in FIX `UTCTIMESTAMP` format.
    pub fn set_effective_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::EFFECTIVE_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `EffectiveTime` (tag 168, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn effective_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::EFFECTIVE_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::EFFECTIVE_TIME)
    }

    /// Whether `EffectiveTime` (tag 168) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_effective_time(&self) -> bool {
        self.msg.body().has_field(tags::EFFECTIVE_TIME)
    }

    // --- TransactTime (tag 60, UTCTIMESTAMP, required) ---

    /// Set `TransactTime` (tag 60, required); written in FIX `UTCTIMESTAMP` format.
    pub fn set_transact_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::TRANSACT_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TransactTime` (tag 60, required). `Err(InvalidFormat)` unless a valid FIX
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

    // --- StandInstDbType (tag 169, INT, optional) ---

    /// Set `StandInstDbType` (tag 169, optional).
    pub fn set_stand_inst_db_type(&mut self, v: fields::StandInstDbType) {
        self.msg.body_mut().set_field(tags::STAND_INST_DB_TYPE, v.to_fix().to_string());
    }

    /// `StandInstDbType` (tag 169, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::StandInstDbType`]'s.
    pub fn stand_inst_db_type(&self) -> Result<fields::StandInstDbType, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::STAND_INST_DB_TYPE)?;
        fields::StandInstDbType::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::STAND_INST_DB_TYPE,
        })
    }

    /// Whether `StandInstDbType` (tag 169) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_stand_inst_db_type(&self) -> bool {
        self.msg.body().has_field(tags::STAND_INST_DB_TYPE)
    }

    // --- StandInstDbName (tag 170, STRING, optional) ---

    /// Set `StandInstDbName` (tag 170, optional).
    pub fn set_stand_inst_db_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::STAND_INST_DB_NAME, v.to_string());
    }

    /// `StandInstDbName` (tag 170, optional).
    pub fn stand_inst_db_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::STAND_INST_DB_NAME)
    }

    /// Whether `StandInstDbName` (tag 170) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_stand_inst_db_name(&self) -> bool {
        self.msg.body().has_field(tags::STAND_INST_DB_NAME)
    }

    // --- StandInstDbID (tag 171, STRING, optional) ---

    /// Set `StandInstDbID` (tag 171, optional).
    pub fn set_stand_inst_db_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::STAND_INST_DB_ID, v.to_string());
    }

    /// `StandInstDbID` (tag 171, optional).
    pub fn stand_inst_db_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::STAND_INST_DB_ID)
    }

    /// Whether `StandInstDbID` (tag 171) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_stand_inst_db_id(&self) -> bool {
        self.msg.body().has_field(tags::STAND_INST_DB_ID)
    }

    // --- SettlDeliveryType (tag 172, INT, optional) ---

    /// Set `SettlDeliveryType` (tag 172, optional).
    pub fn set_settl_delivery_type(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::SETTL_DELIVERY_TYPE, v.to_string());
    }

    /// `SettlDeliveryType` (tag 172, optional).
    pub fn settl_delivery_type(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::SETTL_DELIVERY_TYPE)
    }

    /// Whether `SettlDeliveryType` (tag 172) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_delivery_type(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_DELIVERY_TYPE)
    }

    // --- SettlDepositoryCode (tag 173, STRING, optional) ---

    /// Set `SettlDepositoryCode` (tag 173, optional).
    pub fn set_settl_depository_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SETTL_DEPOSITORY_CODE, v.to_string());
    }

    /// `SettlDepositoryCode` (tag 173, optional).
    pub fn settl_depository_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SETTL_DEPOSITORY_CODE)
    }

    /// Whether `SettlDepositoryCode` (tag 173) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_depository_code(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_DEPOSITORY_CODE)
    }

    // --- SettlBrkrCode (tag 174, STRING, optional) ---

    /// Set `SettlBrkrCode` (tag 174, optional).
    pub fn set_settl_brkr_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SETTL_BRKR_CODE, v.to_string());
    }

    /// `SettlBrkrCode` (tag 174, optional).
    pub fn settl_brkr_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SETTL_BRKR_CODE)
    }

    /// Whether `SettlBrkrCode` (tag 174) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_brkr_code(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_BRKR_CODE)
    }

    // --- SettlInstCode (tag 175, STRING, optional) ---

    /// Set `SettlInstCode` (tag 175, optional).
    pub fn set_settl_inst_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SETTL_INST_CODE, v.to_string());
    }

    /// `SettlInstCode` (tag 175, optional).
    pub fn settl_inst_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SETTL_INST_CODE)
    }

    /// Whether `SettlInstCode` (tag 175) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_settl_inst_code(&self) -> bool {
        self.msg.body().has_field(tags::SETTL_INST_CODE)
    }

    // --- SecuritySettlAgentName (tag 176, STRING, optional) ---

    /// Set `SecuritySettlAgentName` (tag 176, optional).
    pub fn set_security_settl_agent_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_SETTL_AGENT_NAME, v.to_string());
    }

    /// `SecuritySettlAgentName` (tag 176, optional).
    pub fn security_settl_agent_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_SETTL_AGENT_NAME)
    }

    /// Whether `SecuritySettlAgentName` (tag 176) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_settl_agent_name(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_SETTL_AGENT_NAME)
    }

    // --- SecuritySettlAgentCode (tag 177, STRING, optional) ---

    /// Set `SecuritySettlAgentCode` (tag 177, optional).
    pub fn set_security_settl_agent_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_SETTL_AGENT_CODE, v.to_string());
    }

    /// `SecuritySettlAgentCode` (tag 177, optional).
    pub fn security_settl_agent_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_SETTL_AGENT_CODE)
    }

    /// Whether `SecuritySettlAgentCode` (tag 177) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_settl_agent_code(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_SETTL_AGENT_CODE)
    }

    // --- SecuritySettlAgentAcctNum (tag 178, STRING, optional) ---

    /// Set `SecuritySettlAgentAcctNum` (tag 178, optional).
    pub fn set_security_settl_agent_acct_num(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_SETTL_AGENT_ACCT_NUM, v.to_string());
    }

    /// `SecuritySettlAgentAcctNum` (tag 178, optional).
    pub fn security_settl_agent_acct_num(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_SETTL_AGENT_ACCT_NUM)
    }

    /// Whether `SecuritySettlAgentAcctNum` (tag 178) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_settl_agent_acct_num(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_SETTL_AGENT_ACCT_NUM)
    }

    // --- SecuritySettlAgentAcctName (tag 179, STRING, optional) ---

    /// Set `SecuritySettlAgentAcctName` (tag 179, optional).
    pub fn set_security_settl_agent_acct_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_SETTL_AGENT_ACCT_NAME, v.to_string());
    }

    /// `SecuritySettlAgentAcctName` (tag 179, optional).
    pub fn security_settl_agent_acct_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_SETTL_AGENT_ACCT_NAME)
    }

    /// Whether `SecuritySettlAgentAcctName` (tag 179) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_settl_agent_acct_name(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_SETTL_AGENT_ACCT_NAME)
    }

    // --- SecuritySettlAgentContactName (tag 180, STRING, optional) ---

    /// Set `SecuritySettlAgentContactName` (tag 180, optional).
    pub fn set_security_settl_agent_contact_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_SETTL_AGENT_CONTACT_NAME, v.to_string());
    }

    /// `SecuritySettlAgentContactName` (tag 180, optional).
    pub fn security_settl_agent_contact_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_SETTL_AGENT_CONTACT_NAME)
    }

    /// Whether `SecuritySettlAgentContactName` (tag 180) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_settl_agent_contact_name(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_SETTL_AGENT_CONTACT_NAME)
    }

    // --- SecuritySettlAgentContactPhone (tag 181, STRING, optional) ---

    /// Set `SecuritySettlAgentContactPhone` (tag 181, optional).
    pub fn set_security_settl_agent_contact_phone(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::SECURITY_SETTL_AGENT_CONTACT_PHONE, v.to_string());
    }

    /// `SecuritySettlAgentContactPhone` (tag 181, optional).
    pub fn security_settl_agent_contact_phone(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::SECURITY_SETTL_AGENT_CONTACT_PHONE)
    }

    /// Whether `SecuritySettlAgentContactPhone` (tag 181) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_security_settl_agent_contact_phone(&self) -> bool {
        self.msg.body().has_field(tags::SECURITY_SETTL_AGENT_CONTACT_PHONE)
    }

    // --- CashSettlAgentName (tag 182, STRING, optional) ---

    /// Set `CashSettlAgentName` (tag 182, optional).
    pub fn set_cash_settl_agent_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CASH_SETTL_AGENT_NAME, v.to_string());
    }

    /// `CashSettlAgentName` (tag 182, optional).
    pub fn cash_settl_agent_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CASH_SETTL_AGENT_NAME)
    }

    /// Whether `CashSettlAgentName` (tag 182) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_settl_agent_name(&self) -> bool {
        self.msg.body().has_field(tags::CASH_SETTL_AGENT_NAME)
    }

    // --- CashSettlAgentCode (tag 183, STRING, optional) ---

    /// Set `CashSettlAgentCode` (tag 183, optional).
    pub fn set_cash_settl_agent_code(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CASH_SETTL_AGENT_CODE, v.to_string());
    }

    /// `CashSettlAgentCode` (tag 183, optional).
    pub fn cash_settl_agent_code(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CASH_SETTL_AGENT_CODE)
    }

    /// Whether `CashSettlAgentCode` (tag 183) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_settl_agent_code(&self) -> bool {
        self.msg.body().has_field(tags::CASH_SETTL_AGENT_CODE)
    }

    // --- CashSettlAgentAcctNum (tag 184, STRING, optional) ---

    /// Set `CashSettlAgentAcctNum` (tag 184, optional).
    pub fn set_cash_settl_agent_acct_num(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CASH_SETTL_AGENT_ACCT_NUM, v.to_string());
    }

    /// `CashSettlAgentAcctNum` (tag 184, optional).
    pub fn cash_settl_agent_acct_num(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CASH_SETTL_AGENT_ACCT_NUM)
    }

    /// Whether `CashSettlAgentAcctNum` (tag 184) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_settl_agent_acct_num(&self) -> bool {
        self.msg.body().has_field(tags::CASH_SETTL_AGENT_ACCT_NUM)
    }

    // --- CashSettlAgentAcctName (tag 185, STRING, optional) ---

    /// Set `CashSettlAgentAcctName` (tag 185, optional).
    pub fn set_cash_settl_agent_acct_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CASH_SETTL_AGENT_ACCT_NAME, v.to_string());
    }

    /// `CashSettlAgentAcctName` (tag 185, optional).
    pub fn cash_settl_agent_acct_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CASH_SETTL_AGENT_ACCT_NAME)
    }

    /// Whether `CashSettlAgentAcctName` (tag 185) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_settl_agent_acct_name(&self) -> bool {
        self.msg.body().has_field(tags::CASH_SETTL_AGENT_ACCT_NAME)
    }

    // --- CashSettlAgentContactName (tag 186, STRING, optional) ---

    /// Set `CashSettlAgentContactName` (tag 186, optional).
    pub fn set_cash_settl_agent_contact_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CASH_SETTL_AGENT_CONTACT_NAME, v.to_string());
    }

    /// `CashSettlAgentContactName` (tag 186, optional).
    pub fn cash_settl_agent_contact_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CASH_SETTL_AGENT_CONTACT_NAME)
    }

    /// Whether `CashSettlAgentContactName` (tag 186) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_settl_agent_contact_name(&self) -> bool {
        self.msg.body().has_field(tags::CASH_SETTL_AGENT_CONTACT_NAME)
    }

    // --- CashSettlAgentContactPhone (tag 187, STRING, optional) ---

    /// Set `CashSettlAgentContactPhone` (tag 187, optional).
    pub fn set_cash_settl_agent_contact_phone(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CASH_SETTL_AGENT_CONTACT_PHONE, v.to_string());
    }

    /// `CashSettlAgentContactPhone` (tag 187, optional).
    pub fn cash_settl_agent_contact_phone(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CASH_SETTL_AGENT_CONTACT_PHONE)
    }

    /// Whether `CashSettlAgentContactPhone` (tag 187) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_cash_settl_agent_contact_phone(&self) -> bool {
        self.msg.body().has_field(tags::CASH_SETTL_AGENT_CONTACT_PHONE)
    }

    // --- PaymentMethod (tag 492, INT, optional) ---

    /// Set `PaymentMethod` (tag 492, optional).
    pub fn set_payment_method(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::PAYMENT_METHOD, v.to_string());
    }

    /// `PaymentMethod` (tag 492, optional).
    pub fn payment_method(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::PAYMENT_METHOD)
    }

    /// Whether `PaymentMethod` (tag 492) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_payment_method(&self) -> bool {
        self.msg.body().has_field(tags::PAYMENT_METHOD)
    }

    // --- PaymentRef (tag 476, STRING, optional) ---

    /// Set `PaymentRef` (tag 476, optional).
    pub fn set_payment_ref(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::PAYMENT_REF, v.to_string());
    }

    /// `PaymentRef` (tag 476, optional).
    pub fn payment_ref(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::PAYMENT_REF)
    }

    /// Whether `PaymentRef` (tag 476) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_payment_ref(&self) -> bool {
        self.msg.body().has_field(tags::PAYMENT_REF)
    }

    // --- CardHolderName (tag 488, STRING, optional) ---

    /// Set `CardHolderName` (tag 488, optional).
    pub fn set_card_holder_name(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CARD_HOLDER_NAME, v.to_string());
    }

    /// `CardHolderName` (tag 488, optional).
    pub fn card_holder_name(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CARD_HOLDER_NAME)
    }

    /// Whether `CardHolderName` (tag 488) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_card_holder_name(&self) -> bool {
        self.msg.body().has_field(tags::CARD_HOLDER_NAME)
    }

    // --- CardNumber (tag 489, STRING, optional) ---

    /// Set `CardNumber` (tag 489, optional).
    pub fn set_card_number(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CARD_NUMBER, v.to_string());
    }

    /// `CardNumber` (tag 489, optional).
    pub fn card_number(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CARD_NUMBER)
    }

    /// Whether `CardNumber` (tag 489) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_card_number(&self) -> bool {
        self.msg.body().has_field(tags::CARD_NUMBER)
    }

    // --- CardStartDate (tag 503, LOCALMKTDATE, optional) ---

    /// Set `CardStartDate` (tag 503, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_card_start_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::CARD_START_DATE, convert::format_fix_date(v));
    }

    /// `CardStartDate` (tag 503, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn card_start_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::CARD_START_DATE)?;
        convert::parse_fix_date(&raw, tags::CARD_START_DATE)
    }

    /// Whether `CardStartDate` (tag 503) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_card_start_date(&self) -> bool {
        self.msg.body().has_field(tags::CARD_START_DATE)
    }

    // --- CardExpDate (tag 490, LOCALMKTDATE, optional) ---

    /// Set `CardExpDate` (tag 490, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_card_exp_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::CARD_EXP_DATE, convert::format_fix_date(v));
    }

    /// `CardExpDate` (tag 490, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn card_exp_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::CARD_EXP_DATE)?;
        convert::parse_fix_date(&raw, tags::CARD_EXP_DATE)
    }

    /// Whether `CardExpDate` (tag 490) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_card_exp_date(&self) -> bool {
        self.msg.body().has_field(tags::CARD_EXP_DATE)
    }

    // --- CardIssNo (tag 491, STRING, optional) ---

    /// Set `CardIssNo` (tag 491, optional).
    pub fn set_card_iss_no(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CARD_ISS_NO, v.to_string());
    }

    /// `CardIssNo` (tag 491, optional).
    pub fn card_iss_no(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CARD_ISS_NO)
    }

    /// Whether `CardIssNo` (tag 491) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_card_iss_no(&self) -> bool {
        self.msg.body().has_field(tags::CARD_ISS_NO)
    }

    // --- PaymentDate (tag 504, LOCALMKTDATE, optional) ---

    /// Set `PaymentDate` (tag 504, optional); written in FIX `LOCALMKTDATE` format.
    pub fn set_payment_date(&mut self, v: NaiveDate) {
        self.msg.body_mut().set_field(tags::PAYMENT_DATE, convert::format_fix_date(v));
    }

    /// `PaymentDate` (tag 504, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `LOCALMKTDATE`.
    pub fn payment_date(&self) -> Result<NaiveDate, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::PAYMENT_DATE)?;
        convert::parse_fix_date(&raw, tags::PAYMENT_DATE)
    }

    /// Whether `PaymentDate` (tag 504) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_payment_date(&self) -> bool {
        self.msg.body().has_field(tags::PAYMENT_DATE)
    }

    // --- PaymentRemitterID (tag 505, STRING, optional) ---

    /// Set `PaymentRemitterID` (tag 505, optional).
    pub fn set_payment_remitter_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::PAYMENT_REMITTER_ID, v.to_string());
    }

    /// `PaymentRemitterID` (tag 505, optional).
    pub fn payment_remitter_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::PAYMENT_REMITTER_ID)
    }

    /// Whether `PaymentRemitterID` (tag 505) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_payment_remitter_id(&self) -> bool {
        self.msg.body().has_field(tags::PAYMENT_REMITTER_ID)
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
