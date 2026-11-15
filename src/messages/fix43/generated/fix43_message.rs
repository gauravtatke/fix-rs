//! `Fix43Message`: every message of this FIX version as one enum, each variant holding its
//! typed facade.
//!
//! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen`; `cargo test -p codegen`
//! fails if this file drifts from the generator's output.

use super::advertisement::Advertisement;
use super::allocation::Allocation;
use super::allocation_ack::AllocationACK;
use super::bid_request::BidRequest;
use super::bid_response::BidResponse;
use super::business_message_reject::BusinessMessageReject;
use super::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest;
use super::cross_order_cancel_request::CrossOrderCancelRequest;
use super::derivative_security_list::DerivativeSecurityList;
use super::derivative_security_list_request::DerivativeSecurityListRequest;
use super::dont_know_trade::DontKnowTrade;
use super::email::Email;
use super::execution_report::ExecutionReport;
use super::heartbeat::Heartbeat;
use super::indication_of_interest::IndicationOfInterest;
use super::list_cancel_request::ListCancelRequest;
use super::list_execute::ListExecute;
use super::list_status::ListStatus;
use super::list_status_request::ListStatusRequest;
use super::list_strike_price::ListStrikePrice;
use super::logon::Logon;
use super::logout::Logout;
use super::market_data_incremental_refresh::MarketDataIncrementalRefresh;
use super::market_data_request::MarketDataRequest;
use super::market_data_request_reject::MarketDataRequestReject;
use super::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh;
use super::mass_quote::MassQuote;
use super::mass_quote_acknowledgement::MassQuoteAcknowledgement;
use super::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest;
use super::new_order_cross::NewOrderCross;
use super::new_order_list::NewOrderList;
use super::new_order_multileg::NewOrderMultileg;
use super::new_order_single::NewOrderSingle;
use super::news::News;
use super::order_cancel_reject::OrderCancelReject;
use super::order_cancel_replace_request::OrderCancelReplaceRequest;
use super::order_cancel_request::OrderCancelRequest;
use super::order_mass_cancel_report::OrderMassCancelReport;
use super::order_mass_cancel_request::OrderMassCancelRequest;
use super::order_mass_status_request::OrderMassStatusRequest;
use super::order_status_request::OrderStatusRequest;
use super::quote::Quote;
use super::quote_cancel::QuoteCancel;
use super::quote_request::QuoteRequest;
use super::quote_request_reject::QuoteRequestReject;
use super::quote_status_report::QuoteStatusReport;
use super::quote_status_request::QuoteStatusRequest;
use super::registration_instructions::RegistrationInstructions;
use super::registration_instructions_response::RegistrationInstructionsResponse;
use super::reject::Reject;
use super::resend_request::ResendRequest;
use super::rfq_request::RFQRequest;
use super::security_definition::SecurityDefinition;
use super::security_definition_request::SecurityDefinitionRequest;
use super::security_list::SecurityList;
use super::security_list_request::SecurityListRequest;
use super::security_status::SecurityStatus;
use super::security_status_request::SecurityStatusRequest;
use super::security_type_request::SecurityTypeRequest;
use super::security_types::SecurityTypes;
use super::sequence_reset::SequenceReset;
use super::settlement_instructions::SettlementInstructions;
use super::test_request::TestRequest;
use super::trade_capture_report::TradeCaptureReport;
use super::trade_capture_report_request::TradeCaptureReportRequest;
use super::trading_session_status::TradingSessionStatus;
use super::trading_session_status_request::TradingSessionStatusRequest;
use crate::core::message::Message;

/// A received message, typed by its MsgType (35). Build one with `Fix43Message::from(msg)` and
/// `match` on it; `Message::from(..)` gives the raw message back.
#[derive(Debug, Clone)]
pub enum Fix43Message {
    Heartbeat(Heartbeat),
    Logon(Logon),
    TestRequest(TestRequest),
    ResendRequest(ResendRequest),
    Reject(Reject),
    SequenceReset(SequenceReset),
    Logout(Logout),
    BusinessMessageReject(BusinessMessageReject),
    Advertisement(Advertisement),
    IndicationOfInterest(IndicationOfInterest),
    News(News),
    Email(Email),
    QuoteRequest(QuoteRequest),
    QuoteRequestReject(QuoteRequestReject),
    RFQRequest(RFQRequest),
    Quote(Quote),
    QuoteCancel(QuoteCancel),
    QuoteStatusRequest(QuoteStatusRequest),
    QuoteStatusReport(QuoteStatusReport),
    MassQuote(MassQuote),
    MassQuoteAcknowledgement(MassQuoteAcknowledgement),
    MarketDataRequest(MarketDataRequest),
    MarketDataSnapshotFullRefresh(MarketDataSnapshotFullRefresh),
    MarketDataIncrementalRefresh(MarketDataIncrementalRefresh),
    MarketDataRequestReject(MarketDataRequestReject),
    SecurityDefinitionRequest(SecurityDefinitionRequest),
    SecurityDefinition(SecurityDefinition),
    SecurityTypeRequest(SecurityTypeRequest),
    SecurityTypes(SecurityTypes),
    SecurityListRequest(SecurityListRequest),
    SecurityList(SecurityList),
    DerivativeSecurityListRequest(DerivativeSecurityListRequest),
    DerivativeSecurityList(DerivativeSecurityList),
    SecurityStatusRequest(SecurityStatusRequest),
    SecurityStatus(SecurityStatus),
    TradingSessionStatusRequest(TradingSessionStatusRequest),
    TradingSessionStatus(TradingSessionStatus),
    NewOrderSingle(NewOrderSingle),
    ExecutionReport(ExecutionReport),
    DontKnowTrade(DontKnowTrade),
    OrderCancelReplaceRequest(OrderCancelReplaceRequest),
    OrderCancelRequest(OrderCancelRequest),
    OrderCancelReject(OrderCancelReject),
    OrderStatusRequest(OrderStatusRequest),
    OrderMassCancelRequest(OrderMassCancelRequest),
    OrderMassCancelReport(OrderMassCancelReport),
    OrderMassStatusRequest(OrderMassStatusRequest),
    NewOrderCross(NewOrderCross),
    CrossOrderCancelReplaceRequest(CrossOrderCancelReplaceRequest),
    CrossOrderCancelRequest(CrossOrderCancelRequest),
    NewOrderMultileg(NewOrderMultileg),
    MultilegOrderCancelReplaceRequest(MultilegOrderCancelReplaceRequest),
    BidRequest(BidRequest),
    BidResponse(BidResponse),
    NewOrderList(NewOrderList),
    ListStrikePrice(ListStrikePrice),
    ListExecute(ListExecute),
    ListCancelRequest(ListCancelRequest),
    ListStatusRequest(ListStatusRequest),
    ListStatus(ListStatus),
    Allocation(Allocation),
    AllocationACK(AllocationACK),
    SettlementInstructions(SettlementInstructions),
    TradeCaptureReportRequest(TradeCaptureReportRequest),
    TradeCaptureReport(TradeCaptureReport),
    RegistrationInstructions(RegistrationInstructions),
    RegistrationInstructionsResponse(RegistrationInstructionsResponse),
    /// A MsgType this dictionary doesn't define, or no MsgType at all: the raw message, unchanged.
    Unknown(Message),
}

/// Inbound seam: never fails. MsgType is matched against each facade's `MSG_TYPE` once, so the
/// facade is built with the unchecked `wrap`; anything else becomes `Unknown`.
impl From<Message> for Fix43Message {
    fn from(msg: Message) -> Self {
        match msg.get_msg_type().as_deref() {
            Ok(Heartbeat::MSG_TYPE) => Self::Heartbeat(Heartbeat::wrap(msg)),
            Ok(Logon::MSG_TYPE) => Self::Logon(Logon::wrap(msg)),
            Ok(TestRequest::MSG_TYPE) => Self::TestRequest(TestRequest::wrap(msg)),
            Ok(ResendRequest::MSG_TYPE) => Self::ResendRequest(ResendRequest::wrap(msg)),
            Ok(Reject::MSG_TYPE) => Self::Reject(Reject::wrap(msg)),
            Ok(SequenceReset::MSG_TYPE) => Self::SequenceReset(SequenceReset::wrap(msg)),
            Ok(Logout::MSG_TYPE) => Self::Logout(Logout::wrap(msg)),
            Ok(BusinessMessageReject::MSG_TYPE) => {
                Self::BusinessMessageReject(BusinessMessageReject::wrap(msg))
            }
            Ok(Advertisement::MSG_TYPE) => Self::Advertisement(Advertisement::wrap(msg)),
            Ok(IndicationOfInterest::MSG_TYPE) => {
                Self::IndicationOfInterest(IndicationOfInterest::wrap(msg))
            }
            Ok(News::MSG_TYPE) => Self::News(News::wrap(msg)),
            Ok(Email::MSG_TYPE) => Self::Email(Email::wrap(msg)),
            Ok(QuoteRequest::MSG_TYPE) => Self::QuoteRequest(QuoteRequest::wrap(msg)),
            Ok(QuoteRequestReject::MSG_TYPE) => {
                Self::QuoteRequestReject(QuoteRequestReject::wrap(msg))
            }
            Ok(RFQRequest::MSG_TYPE) => Self::RFQRequest(RFQRequest::wrap(msg)),
            Ok(Quote::MSG_TYPE) => Self::Quote(Quote::wrap(msg)),
            Ok(QuoteCancel::MSG_TYPE) => Self::QuoteCancel(QuoteCancel::wrap(msg)),
            Ok(QuoteStatusRequest::MSG_TYPE) => {
                Self::QuoteStatusRequest(QuoteStatusRequest::wrap(msg))
            }
            Ok(QuoteStatusReport::MSG_TYPE) => {
                Self::QuoteStatusReport(QuoteStatusReport::wrap(msg))
            }
            Ok(MassQuote::MSG_TYPE) => Self::MassQuote(MassQuote::wrap(msg)),
            Ok(MassQuoteAcknowledgement::MSG_TYPE) => {
                Self::MassQuoteAcknowledgement(MassQuoteAcknowledgement::wrap(msg))
            }
            Ok(MarketDataRequest::MSG_TYPE) => {
                Self::MarketDataRequest(MarketDataRequest::wrap(msg))
            }
            Ok(MarketDataSnapshotFullRefresh::MSG_TYPE) => {
                Self::MarketDataSnapshotFullRefresh(MarketDataSnapshotFullRefresh::wrap(msg))
            }
            Ok(MarketDataIncrementalRefresh::MSG_TYPE) => {
                Self::MarketDataIncrementalRefresh(MarketDataIncrementalRefresh::wrap(msg))
            }
            Ok(MarketDataRequestReject::MSG_TYPE) => {
                Self::MarketDataRequestReject(MarketDataRequestReject::wrap(msg))
            }
            Ok(SecurityDefinitionRequest::MSG_TYPE) => {
                Self::SecurityDefinitionRequest(SecurityDefinitionRequest::wrap(msg))
            }
            Ok(SecurityDefinition::MSG_TYPE) => {
                Self::SecurityDefinition(SecurityDefinition::wrap(msg))
            }
            Ok(SecurityTypeRequest::MSG_TYPE) => {
                Self::SecurityTypeRequest(SecurityTypeRequest::wrap(msg))
            }
            Ok(SecurityTypes::MSG_TYPE) => Self::SecurityTypes(SecurityTypes::wrap(msg)),
            Ok(SecurityListRequest::MSG_TYPE) => {
                Self::SecurityListRequest(SecurityListRequest::wrap(msg))
            }
            Ok(SecurityList::MSG_TYPE) => Self::SecurityList(SecurityList::wrap(msg)),
            Ok(DerivativeSecurityListRequest::MSG_TYPE) => {
                Self::DerivativeSecurityListRequest(DerivativeSecurityListRequest::wrap(msg))
            }
            Ok(DerivativeSecurityList::MSG_TYPE) => {
                Self::DerivativeSecurityList(DerivativeSecurityList::wrap(msg))
            }
            Ok(SecurityStatusRequest::MSG_TYPE) => {
                Self::SecurityStatusRequest(SecurityStatusRequest::wrap(msg))
            }
            Ok(SecurityStatus::MSG_TYPE) => Self::SecurityStatus(SecurityStatus::wrap(msg)),
            Ok(TradingSessionStatusRequest::MSG_TYPE) => {
                Self::TradingSessionStatusRequest(TradingSessionStatusRequest::wrap(msg))
            }
            Ok(TradingSessionStatus::MSG_TYPE) => {
                Self::TradingSessionStatus(TradingSessionStatus::wrap(msg))
            }
            Ok(NewOrderSingle::MSG_TYPE) => Self::NewOrderSingle(NewOrderSingle::wrap(msg)),
            Ok(ExecutionReport::MSG_TYPE) => Self::ExecutionReport(ExecutionReport::wrap(msg)),
            Ok(DontKnowTrade::MSG_TYPE) => Self::DontKnowTrade(DontKnowTrade::wrap(msg)),
            Ok(OrderCancelReplaceRequest::MSG_TYPE) => {
                Self::OrderCancelReplaceRequest(OrderCancelReplaceRequest::wrap(msg))
            }
            Ok(OrderCancelRequest::MSG_TYPE) => {
                Self::OrderCancelRequest(OrderCancelRequest::wrap(msg))
            }
            Ok(OrderCancelReject::MSG_TYPE) => {
                Self::OrderCancelReject(OrderCancelReject::wrap(msg))
            }
            Ok(OrderStatusRequest::MSG_TYPE) => {
                Self::OrderStatusRequest(OrderStatusRequest::wrap(msg))
            }
            Ok(OrderMassCancelRequest::MSG_TYPE) => {
                Self::OrderMassCancelRequest(OrderMassCancelRequest::wrap(msg))
            }
            Ok(OrderMassCancelReport::MSG_TYPE) => {
                Self::OrderMassCancelReport(OrderMassCancelReport::wrap(msg))
            }
            Ok(OrderMassStatusRequest::MSG_TYPE) => {
                Self::OrderMassStatusRequest(OrderMassStatusRequest::wrap(msg))
            }
            Ok(NewOrderCross::MSG_TYPE) => Self::NewOrderCross(NewOrderCross::wrap(msg)),
            Ok(CrossOrderCancelReplaceRequest::MSG_TYPE) => {
                Self::CrossOrderCancelReplaceRequest(CrossOrderCancelReplaceRequest::wrap(msg))
            }
            Ok(CrossOrderCancelRequest::MSG_TYPE) => {
                Self::CrossOrderCancelRequest(CrossOrderCancelRequest::wrap(msg))
            }
            Ok(NewOrderMultileg::MSG_TYPE) => Self::NewOrderMultileg(NewOrderMultileg::wrap(msg)),
            Ok(MultilegOrderCancelReplaceRequest::MSG_TYPE) => {
                Self::MultilegOrderCancelReplaceRequest(MultilegOrderCancelReplaceRequest::wrap(
                    msg,
                ))
            }
            Ok(BidRequest::MSG_TYPE) => Self::BidRequest(BidRequest::wrap(msg)),
            Ok(BidResponse::MSG_TYPE) => Self::BidResponse(BidResponse::wrap(msg)),
            Ok(NewOrderList::MSG_TYPE) => Self::NewOrderList(NewOrderList::wrap(msg)),
            Ok(ListStrikePrice::MSG_TYPE) => Self::ListStrikePrice(ListStrikePrice::wrap(msg)),
            Ok(ListExecute::MSG_TYPE) => Self::ListExecute(ListExecute::wrap(msg)),
            Ok(ListCancelRequest::MSG_TYPE) => {
                Self::ListCancelRequest(ListCancelRequest::wrap(msg))
            }
            Ok(ListStatusRequest::MSG_TYPE) => {
                Self::ListStatusRequest(ListStatusRequest::wrap(msg))
            }
            Ok(ListStatus::MSG_TYPE) => Self::ListStatus(ListStatus::wrap(msg)),
            Ok(Allocation::MSG_TYPE) => Self::Allocation(Allocation::wrap(msg)),
            Ok(AllocationACK::MSG_TYPE) => Self::AllocationACK(AllocationACK::wrap(msg)),
            Ok(SettlementInstructions::MSG_TYPE) => {
                Self::SettlementInstructions(SettlementInstructions::wrap(msg))
            }
            Ok(TradeCaptureReportRequest::MSG_TYPE) => {
                Self::TradeCaptureReportRequest(TradeCaptureReportRequest::wrap(msg))
            }
            Ok(TradeCaptureReport::MSG_TYPE) => {
                Self::TradeCaptureReport(TradeCaptureReport::wrap(msg))
            }
            Ok(RegistrationInstructions::MSG_TYPE) => {
                Self::RegistrationInstructions(RegistrationInstructions::wrap(msg))
            }
            Ok(RegistrationInstructionsResponse::MSG_TYPE) => {
                Self::RegistrationInstructionsResponse(RegistrationInstructionsResponse::wrap(msg))
            }
            _ => Self::Unknown(msg),
        }
    }
}

/// Outbound seam: the underlying raw `Message`, whichever variant holds it.
impl From<Fix43Message> for Message {
    fn from(msg: Fix43Message) -> Self {
        match msg {
            Fix43Message::Heartbeat(m) => m.into(),
            Fix43Message::Logon(m) => m.into(),
            Fix43Message::TestRequest(m) => m.into(),
            Fix43Message::ResendRequest(m) => m.into(),
            Fix43Message::Reject(m) => m.into(),
            Fix43Message::SequenceReset(m) => m.into(),
            Fix43Message::Logout(m) => m.into(),
            Fix43Message::BusinessMessageReject(m) => m.into(),
            Fix43Message::Advertisement(m) => m.into(),
            Fix43Message::IndicationOfInterest(m) => m.into(),
            Fix43Message::News(m) => m.into(),
            Fix43Message::Email(m) => m.into(),
            Fix43Message::QuoteRequest(m) => m.into(),
            Fix43Message::QuoteRequestReject(m) => m.into(),
            Fix43Message::RFQRequest(m) => m.into(),
            Fix43Message::Quote(m) => m.into(),
            Fix43Message::QuoteCancel(m) => m.into(),
            Fix43Message::QuoteStatusRequest(m) => m.into(),
            Fix43Message::QuoteStatusReport(m) => m.into(),
            Fix43Message::MassQuote(m) => m.into(),
            Fix43Message::MassQuoteAcknowledgement(m) => m.into(),
            Fix43Message::MarketDataRequest(m) => m.into(),
            Fix43Message::MarketDataSnapshotFullRefresh(m) => m.into(),
            Fix43Message::MarketDataIncrementalRefresh(m) => m.into(),
            Fix43Message::MarketDataRequestReject(m) => m.into(),
            Fix43Message::SecurityDefinitionRequest(m) => m.into(),
            Fix43Message::SecurityDefinition(m) => m.into(),
            Fix43Message::SecurityTypeRequest(m) => m.into(),
            Fix43Message::SecurityTypes(m) => m.into(),
            Fix43Message::SecurityListRequest(m) => m.into(),
            Fix43Message::SecurityList(m) => m.into(),
            Fix43Message::DerivativeSecurityListRequest(m) => m.into(),
            Fix43Message::DerivativeSecurityList(m) => m.into(),
            Fix43Message::SecurityStatusRequest(m) => m.into(),
            Fix43Message::SecurityStatus(m) => m.into(),
            Fix43Message::TradingSessionStatusRequest(m) => m.into(),
            Fix43Message::TradingSessionStatus(m) => m.into(),
            Fix43Message::NewOrderSingle(m) => m.into(),
            Fix43Message::ExecutionReport(m) => m.into(),
            Fix43Message::DontKnowTrade(m) => m.into(),
            Fix43Message::OrderCancelReplaceRequest(m) => m.into(),
            Fix43Message::OrderCancelRequest(m) => m.into(),
            Fix43Message::OrderCancelReject(m) => m.into(),
            Fix43Message::OrderStatusRequest(m) => m.into(),
            Fix43Message::OrderMassCancelRequest(m) => m.into(),
            Fix43Message::OrderMassCancelReport(m) => m.into(),
            Fix43Message::OrderMassStatusRequest(m) => m.into(),
            Fix43Message::NewOrderCross(m) => m.into(),
            Fix43Message::CrossOrderCancelReplaceRequest(m) => m.into(),
            Fix43Message::CrossOrderCancelRequest(m) => m.into(),
            Fix43Message::NewOrderMultileg(m) => m.into(),
            Fix43Message::MultilegOrderCancelReplaceRequest(m) => m.into(),
            Fix43Message::BidRequest(m) => m.into(),
            Fix43Message::BidResponse(m) => m.into(),
            Fix43Message::NewOrderList(m) => m.into(),
            Fix43Message::ListStrikePrice(m) => m.into(),
            Fix43Message::ListExecute(m) => m.into(),
            Fix43Message::ListCancelRequest(m) => m.into(),
            Fix43Message::ListStatusRequest(m) => m.into(),
            Fix43Message::ListStatus(m) => m.into(),
            Fix43Message::Allocation(m) => m.into(),
            Fix43Message::AllocationACK(m) => m.into(),
            Fix43Message::SettlementInstructions(m) => m.into(),
            Fix43Message::TradeCaptureReportRequest(m) => m.into(),
            Fix43Message::TradeCaptureReport(m) => m.into(),
            Fix43Message::RegistrationInstructions(m) => m.into(),
            Fix43Message::RegistrationInstructionsResponse(m) => m.into(),
            Fix43Message::Unknown(m) => m,
        }
    }
}
