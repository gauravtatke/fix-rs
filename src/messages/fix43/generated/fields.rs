//! FIX value enums for FIX.4.3: one enum per value-constrained field.
//!
//! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen` from the FIX.4.3
//! data dictionary; `cargo test -p codegen` fails if this file drifts from the generator's output.
//!
//! Value-constrained FIX fields become real Rust **enums** (D3) rather than loose char/int
//! constants, so an invalid value is unrepresentable. Each enum owns its on-wire conversion:
//! `to_fix` (variant -> wire value) and `from_fix` (wire value -> variant, `None` if unknown).

/// `AdvSide` (tag 4, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvSide {
    /// Wire value `B`.
    Buy,
    /// Wire value `S`.
    Sell,
    /// Wire value `T`.
    Trade,
    /// Wire value `X`.
    Cross,
}

impl AdvSide {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Buy => 'B',
            Self::Sell => 'S',
            Self::Trade => 'T',
            Self::Cross => 'X',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'B' => Some(Self::Buy),
            'S' => Some(Self::Sell),
            'T' => Some(Self::Trade),
            'X' => Some(Self::Cross),
            _ => None,
        }
    }
}

/// `AdvTransType` (tag 5, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvTransType {
    /// Wire value `C`.
    Cancel,
    /// Wire value `N`.
    New,
    /// Wire value `R`.
    Replace,
}

impl AdvTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Cancel => "C",
            Self::New => "N",
            Self::Replace => "R",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "C" => Some(Self::Cancel),
            "N" => Some(Self::New),
            "R" => Some(Self::Replace),
            _ => None,
        }
    }
}

/// `CommType` (tag 13, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType {
    /// Wire value `1`.
    PerShare,
    /// Wire value `2`.
    Percentage,
    /// Wire value `3`.
    Absolute,
    /// Wire value `4`.
    PercentageWaivedCashDiscount,
    /// Wire value `5`.
    PercentageWaivedEnhancedUnits,
    /// Wire value `6`.
    PerBond,
}

impl CommType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::PerShare => '1',
            Self::Percentage => '2',
            Self::Absolute => '3',
            Self::PercentageWaivedCashDiscount => '4',
            Self::PercentageWaivedEnhancedUnits => '5',
            Self::PerBond => '6',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::PerShare),
            '2' => Some(Self::Percentage),
            '3' => Some(Self::Absolute),
            '4' => Some(Self::PercentageWaivedCashDiscount),
            '5' => Some(Self::PercentageWaivedEnhancedUnits),
            '6' => Some(Self::PerBond),
            _ => None,
        }
    }
}

/// `ExecInst` (tag 18, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecInst {
    /// Wire value `0`.
    StayOnOfferside,
    /// Wire value `1`.
    NotHeld,
    /// Wire value `2`.
    Work,
    /// Wire value `3`.
    GoAlong,
    /// Wire value `4`.
    OverTheDay,
    /// Wire value `5`.
    Held,
    /// Wire value `6`.
    ParticipateDontInitiate,
    /// Wire value `7`.
    StrictScale,
    /// Wire value `8`.
    TryToScale,
    /// Wire value `9`.
    StayOnBidside,
    /// Wire value `A`.
    NoCross,
    /// Wire value `B`.
    OkToCross,
    /// Wire value `C`.
    CallFirst,
    /// Wire value `D`.
    PercentOfVolume,
    /// Wire value `E`.
    DoNotIncrease,
    /// Wire value `F`.
    DoNotReduce,
    /// Wire value `G`.
    AllOrNone,
    /// Wire value `H`.
    ReinstateOnSystemFailure,
    /// Wire value `I`.
    InstitutionsOnly,
    /// Wire value `J`.
    ReinstateOnTradingHalt,
    /// Wire value `K`.
    CancelOnTradingHalt,
    /// Wire value `L`.
    LastPeg,
    /// Wire value `M`.
    MidPricePeg,
    /// Wire value `N`.
    NonNegotiable,
    /// Wire value `O`.
    OpeningPeg,
    /// Wire value `P`.
    MarketPeg,
    /// Wire value `Q`.
    CancelOnSystemFailure,
    /// Wire value `R`.
    PrimaryPeg,
    /// Wire value `S`.
    Suspend,
    /// Wire value `T`.
    FixedPeg,
    /// Wire value `U`.
    CustomerDisplayInstruction,
    /// Wire value `V`.
    Netting,
    /// Wire value `W`.
    PegToVwap,
    /// Wire value `X`.
    TradeAlong,
    /// Wire value `Y`.
    TryToStop,
}

impl ExecInst {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::StayOnOfferside => "0",
            Self::NotHeld => "1",
            Self::Work => "2",
            Self::GoAlong => "3",
            Self::OverTheDay => "4",
            Self::Held => "5",
            Self::ParticipateDontInitiate => "6",
            Self::StrictScale => "7",
            Self::TryToScale => "8",
            Self::StayOnBidside => "9",
            Self::NoCross => "A",
            Self::OkToCross => "B",
            Self::CallFirst => "C",
            Self::PercentOfVolume => "D",
            Self::DoNotIncrease => "E",
            Self::DoNotReduce => "F",
            Self::AllOrNone => "G",
            Self::ReinstateOnSystemFailure => "H",
            Self::InstitutionsOnly => "I",
            Self::ReinstateOnTradingHalt => "J",
            Self::CancelOnTradingHalt => "K",
            Self::LastPeg => "L",
            Self::MidPricePeg => "M",
            Self::NonNegotiable => "N",
            Self::OpeningPeg => "O",
            Self::MarketPeg => "P",
            Self::CancelOnSystemFailure => "Q",
            Self::PrimaryPeg => "R",
            Self::Suspend => "S",
            Self::FixedPeg => "T",
            Self::CustomerDisplayInstruction => "U",
            Self::Netting => "V",
            Self::PegToVwap => "W",
            Self::TradeAlong => "X",
            Self::TryToStop => "Y",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "0" => Some(Self::StayOnOfferside),
            "1" => Some(Self::NotHeld),
            "2" => Some(Self::Work),
            "3" => Some(Self::GoAlong),
            "4" => Some(Self::OverTheDay),
            "5" => Some(Self::Held),
            "6" => Some(Self::ParticipateDontInitiate),
            "7" => Some(Self::StrictScale),
            "8" => Some(Self::TryToScale),
            "9" => Some(Self::StayOnBidside),
            "A" => Some(Self::NoCross),
            "B" => Some(Self::OkToCross),
            "C" => Some(Self::CallFirst),
            "D" => Some(Self::PercentOfVolume),
            "E" => Some(Self::DoNotIncrease),
            "F" => Some(Self::DoNotReduce),
            "G" => Some(Self::AllOrNone),
            "H" => Some(Self::ReinstateOnSystemFailure),
            "I" => Some(Self::InstitutionsOnly),
            "J" => Some(Self::ReinstateOnTradingHalt),
            "K" => Some(Self::CancelOnTradingHalt),
            "L" => Some(Self::LastPeg),
            "M" => Some(Self::MidPricePeg),
            "N" => Some(Self::NonNegotiable),
            "O" => Some(Self::OpeningPeg),
            "P" => Some(Self::MarketPeg),
            "Q" => Some(Self::CancelOnSystemFailure),
            "R" => Some(Self::PrimaryPeg),
            "S" => Some(Self::Suspend),
            "T" => Some(Self::FixedPeg),
            "U" => Some(Self::CustomerDisplayInstruction),
            "V" => Some(Self::Netting),
            "W" => Some(Self::PegToVwap),
            "X" => Some(Self::TradeAlong),
            "Y" => Some(Self::TryToStop),
            _ => None,
        }
    }
}

/// `ExecTransType` (tag 20, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecTransType {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    Cancel,
    /// Wire value `2`.
    Correct,
    /// Wire value `3`.
    Status,
}

impl ExecTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::Cancel => '1',
            Self::Correct => '2',
            Self::Status => '3',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::Cancel),
            '2' => Some(Self::Correct),
            '3' => Some(Self::Status),
            _ => None,
        }
    }
}

/// `HandlInst` (tag 21, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlInst {
    /// Wire value `1`.
    AutomatedExecutionOrderPrivate,
    /// Wire value `2`.
    AutomatedExecutionOrderPublic,
    /// Wire value `3`.
    ManualOrder,
}

impl HandlInst {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::AutomatedExecutionOrderPrivate => '1',
            Self::AutomatedExecutionOrderPublic => '2',
            Self::ManualOrder => '3',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::AutomatedExecutionOrderPrivate),
            '2' => Some(Self::AutomatedExecutionOrderPublic),
            '3' => Some(Self::ManualOrder),
            _ => None,
        }
    }
}

/// `SecurityIdSource` (tag 22, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityIdSource {
    /// Wire value `1`.
    Cusip,
    /// Wire value `2`.
    Sedol,
    /// Wire value `3`.
    Quik,
    /// Wire value `4`.
    IsinNumber,
    /// Wire value `5`.
    RicCode,
    /// Wire value `6`.
    IsoCurrencyCode,
    /// Wire value `7`.
    IsoCountryCode,
    /// Wire value `8`.
    ExchangeSymbol,
    /// Wire value `9`.
    ConsolidatedTapeAssociation,
    /// Wire value `A`.
    BloombergSymbol,
    /// Wire value `B`.
    Wertpapier,
    /// Wire value `C`.
    Dutch,
    /// Wire value `D`.
    Valoren,
    /// Wire value `E`.
    Sicovam,
    /// Wire value `F`.
    Belgian,
    /// Wire value `G`.
    Common,
}

impl SecurityIdSource {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Cusip => "1",
            Self::Sedol => "2",
            Self::Quik => "3",
            Self::IsinNumber => "4",
            Self::RicCode => "5",
            Self::IsoCurrencyCode => "6",
            Self::IsoCountryCode => "7",
            Self::ExchangeSymbol => "8",
            Self::ConsolidatedTapeAssociation => "9",
            Self::BloombergSymbol => "A",
            Self::Wertpapier => "B",
            Self::Dutch => "C",
            Self::Valoren => "D",
            Self::Sicovam => "E",
            Self::Belgian => "F",
            Self::Common => "G",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "1" => Some(Self::Cusip),
            "2" => Some(Self::Sedol),
            "3" => Some(Self::Quik),
            "4" => Some(Self::IsinNumber),
            "5" => Some(Self::RicCode),
            "6" => Some(Self::IsoCurrencyCode),
            "7" => Some(Self::IsoCountryCode),
            "8" => Some(Self::ExchangeSymbol),
            "9" => Some(Self::ConsolidatedTapeAssociation),
            "A" => Some(Self::BloombergSymbol),
            "B" => Some(Self::Wertpapier),
            "C" => Some(Self::Dutch),
            "D" => Some(Self::Valoren),
            "E" => Some(Self::Sicovam),
            "F" => Some(Self::Belgian),
            "G" => Some(Self::Common),
            _ => None,
        }
    }
}

/// `IoiQltyInd` (tag 25, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoiQltyInd {
    /// Wire value `H`.
    High,
    /// Wire value `L`.
    Low,
    /// Wire value `M`.
    Medium,
}

impl IoiQltyInd {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::High => 'H',
            Self::Low => 'L',
            Self::Medium => 'M',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'H' => Some(Self::High),
            'L' => Some(Self::Low),
            'M' => Some(Self::Medium),
            _ => None,
        }
    }
}

/// `IoiTransType` (tag 28, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoiTransType {
    /// Wire value `C`.
    Cancel,
    /// Wire value `N`.
    New,
    /// Wire value `R`.
    Replace,
}

impl IoiTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Cancel => 'C',
            Self::New => 'N',
            Self::Replace => 'R',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'C' => Some(Self::Cancel),
            'N' => Some(Self::New),
            'R' => Some(Self::Replace),
            _ => None,
        }
    }
}

/// `LastCapacity` (tag 29, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastCapacity {
    /// Wire value `1`.
    Agent,
    /// Wire value `2`.
    CrossAsAgent,
    /// Wire value `3`.
    CrossAsPrincipal,
    /// Wire value `4`.
    Principal,
}

impl LastCapacity {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Agent => '1',
            Self::CrossAsAgent => '2',
            Self::CrossAsPrincipal => '3',
            Self::Principal => '4',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Agent),
            '2' => Some(Self::CrossAsAgent),
            '3' => Some(Self::CrossAsPrincipal),
            '4' => Some(Self::Principal),
            _ => None,
        }
    }
}

/// `MsgType` (tag 35, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    /// Wire value `0`.
    Heartbeat,
    /// Wire value `1`.
    TestRequest,
    /// Wire value `2`.
    ResendRequest,
    /// Wire value `3`.
    Reject,
    /// Wire value `4`.
    SequenceReset,
    /// Wire value `5`.
    Logout,
    /// Wire value `6`.
    IndicationOfInterest,
    /// Wire value `7`.
    Advertisement,
    /// Wire value `8`.
    ExecutionReport,
    /// Wire value `9`.
    OrderCancelReject,
    /// Wire value `A`.
    Logon,
    /// Wire value `AA`.
    DerivativeSecurityList,
    /// Wire value `AB`.
    NewOrderMultileg,
    /// Wire value `AC`.
    MultilegOrderCancel,
    /// Wire value `AD`.
    TradeCaptureReportRequest,
    /// Wire value `AE`.
    TradeCaptureReport,
    /// Wire value `AF`.
    OrderMassStatusRequest,
    /// Wire value `AG`.
    QuoteRequestReject,
    /// Wire value `AH`.
    RfqRequest,
    /// Wire value `AI`.
    QuoteStatusReport,
    /// Wire value `B`.
    News,
    /// Wire value `C`.
    Email,
    /// Wire value `D`.
    OrderSingle,
    /// Wire value `E`.
    OrderList,
    /// Wire value `F`.
    OrderCancelRequest,
    /// Wire value `G`.
    OrderCancelReplaceRequest,
    /// Wire value `H`.
    OrderStatusRequest,
    /// Wire value `J`.
    Allocation,
    /// Wire value `K`.
    ListCancelRequest,
    /// Wire value `L`.
    ListExecute,
    /// Wire value `M`.
    ListStatusRequest,
    /// Wire value `N`.
    ListStatus,
    /// Wire value `P`.
    AllocationAck,
    /// Wire value `Q`.
    DontKnowTrade,
    /// Wire value `R`.
    QuoteRequest,
    /// Wire value `S`.
    Quote,
    /// Wire value `T`.
    SettlementInstructions,
    /// Wire value `V`.
    MarketDataRequest,
    /// Wire value `W`.
    MarketDataSnapshot,
    /// Wire value `X`.
    MarketDataIncrementalRefresh,
    /// Wire value `Y`.
    MarketDataRequestReject,
    /// Wire value `Z`.
    QuoteCancel,
    /// Wire value `a`.
    QuoteStatusRequest,
    /// Wire value `b`.
    MassQuoteAcknowledgement,
    /// Wire value `c`.
    SecurityDefinitionRequest,
    /// Wire value `d`.
    SecurityDefinition,
    /// Wire value `e`.
    SecurityStatusRequest,
    /// Wire value `f`.
    SecurityStatus,
    /// Wire value `g`.
    TradingSessionStatusRequest,
    /// Wire value `h`.
    TradingSessionStatus,
    /// Wire value `i`.
    MassQuote,
    /// Wire value `j`.
    BusinessMessageReject,
    /// Wire value `k`.
    BidRequest,
    /// Wire value `l`.
    BidResponse,
    /// Wire value `m`.
    ListStrikePrice,
    /// Wire value `n`.
    XmlMessage,
    /// Wire value `o`.
    RegistrationInstructions,
    /// Wire value `p`.
    RegistrationInstructionsResponse,
    /// Wire value `q`.
    OrderMassCancelRequest,
    /// Wire value `r`.
    OrderMassCancelReport,
    /// Wire value `s`.
    NewOrderCross,
    /// Wire value `t`.
    CrossOrderCancel,
    /// Wire value `u`.
    CrossOrderCancelRequest,
    /// Wire value `v`.
    SecurityTypeRequest,
    /// Wire value `w`.
    SecurityTypes,
    /// Wire value `x`.
    SecurityListRequest,
    /// Wire value `y`.
    SecurityList,
    /// Wire value `z`.
    DerivativeSecurityListRequest,
}

impl MsgType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Heartbeat => "0",
            Self::TestRequest => "1",
            Self::ResendRequest => "2",
            Self::Reject => "3",
            Self::SequenceReset => "4",
            Self::Logout => "5",
            Self::IndicationOfInterest => "6",
            Self::Advertisement => "7",
            Self::ExecutionReport => "8",
            Self::OrderCancelReject => "9",
            Self::Logon => "A",
            Self::DerivativeSecurityList => "AA",
            Self::NewOrderMultileg => "AB",
            Self::MultilegOrderCancel => "AC",
            Self::TradeCaptureReportRequest => "AD",
            Self::TradeCaptureReport => "AE",
            Self::OrderMassStatusRequest => "AF",
            Self::QuoteRequestReject => "AG",
            Self::RfqRequest => "AH",
            Self::QuoteStatusReport => "AI",
            Self::News => "B",
            Self::Email => "C",
            Self::OrderSingle => "D",
            Self::OrderList => "E",
            Self::OrderCancelRequest => "F",
            Self::OrderCancelReplaceRequest => "G",
            Self::OrderStatusRequest => "H",
            Self::Allocation => "J",
            Self::ListCancelRequest => "K",
            Self::ListExecute => "L",
            Self::ListStatusRequest => "M",
            Self::ListStatus => "N",
            Self::AllocationAck => "P",
            Self::DontKnowTrade => "Q",
            Self::QuoteRequest => "R",
            Self::Quote => "S",
            Self::SettlementInstructions => "T",
            Self::MarketDataRequest => "V",
            Self::MarketDataSnapshot => "W",
            Self::MarketDataIncrementalRefresh => "X",
            Self::MarketDataRequestReject => "Y",
            Self::QuoteCancel => "Z",
            Self::QuoteStatusRequest => "a",
            Self::MassQuoteAcknowledgement => "b",
            Self::SecurityDefinitionRequest => "c",
            Self::SecurityDefinition => "d",
            Self::SecurityStatusRequest => "e",
            Self::SecurityStatus => "f",
            Self::TradingSessionStatusRequest => "g",
            Self::TradingSessionStatus => "h",
            Self::MassQuote => "i",
            Self::BusinessMessageReject => "j",
            Self::BidRequest => "k",
            Self::BidResponse => "l",
            Self::ListStrikePrice => "m",
            Self::XmlMessage => "n",
            Self::RegistrationInstructions => "o",
            Self::RegistrationInstructionsResponse => "p",
            Self::OrderMassCancelRequest => "q",
            Self::OrderMassCancelReport => "r",
            Self::NewOrderCross => "s",
            Self::CrossOrderCancel => "t",
            Self::CrossOrderCancelRequest => "u",
            Self::SecurityTypeRequest => "v",
            Self::SecurityTypes => "w",
            Self::SecurityListRequest => "x",
            Self::SecurityList => "y",
            Self::DerivativeSecurityListRequest => "z",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "0" => Some(Self::Heartbeat),
            "1" => Some(Self::TestRequest),
            "2" => Some(Self::ResendRequest),
            "3" => Some(Self::Reject),
            "4" => Some(Self::SequenceReset),
            "5" => Some(Self::Logout),
            "6" => Some(Self::IndicationOfInterest),
            "7" => Some(Self::Advertisement),
            "8" => Some(Self::ExecutionReport),
            "9" => Some(Self::OrderCancelReject),
            "A" => Some(Self::Logon),
            "AA" => Some(Self::DerivativeSecurityList),
            "AB" => Some(Self::NewOrderMultileg),
            "AC" => Some(Self::MultilegOrderCancel),
            "AD" => Some(Self::TradeCaptureReportRequest),
            "AE" => Some(Self::TradeCaptureReport),
            "AF" => Some(Self::OrderMassStatusRequest),
            "AG" => Some(Self::QuoteRequestReject),
            "AH" => Some(Self::RfqRequest),
            "AI" => Some(Self::QuoteStatusReport),
            "B" => Some(Self::News),
            "C" => Some(Self::Email),
            "D" => Some(Self::OrderSingle),
            "E" => Some(Self::OrderList),
            "F" => Some(Self::OrderCancelRequest),
            "G" => Some(Self::OrderCancelReplaceRequest),
            "H" => Some(Self::OrderStatusRequest),
            "J" => Some(Self::Allocation),
            "K" => Some(Self::ListCancelRequest),
            "L" => Some(Self::ListExecute),
            "M" => Some(Self::ListStatusRequest),
            "N" => Some(Self::ListStatus),
            "P" => Some(Self::AllocationAck),
            "Q" => Some(Self::DontKnowTrade),
            "R" => Some(Self::QuoteRequest),
            "S" => Some(Self::Quote),
            "T" => Some(Self::SettlementInstructions),
            "V" => Some(Self::MarketDataRequest),
            "W" => Some(Self::MarketDataSnapshot),
            "X" => Some(Self::MarketDataIncrementalRefresh),
            "Y" => Some(Self::MarketDataRequestReject),
            "Z" => Some(Self::QuoteCancel),
            "a" => Some(Self::QuoteStatusRequest),
            "b" => Some(Self::MassQuoteAcknowledgement),
            "c" => Some(Self::SecurityDefinitionRequest),
            "d" => Some(Self::SecurityDefinition),
            "e" => Some(Self::SecurityStatusRequest),
            "f" => Some(Self::SecurityStatus),
            "g" => Some(Self::TradingSessionStatusRequest),
            "h" => Some(Self::TradingSessionStatus),
            "i" => Some(Self::MassQuote),
            "j" => Some(Self::BusinessMessageReject),
            "k" => Some(Self::BidRequest),
            "l" => Some(Self::BidResponse),
            "m" => Some(Self::ListStrikePrice),
            "n" => Some(Self::XmlMessage),
            "o" => Some(Self::RegistrationInstructions),
            "p" => Some(Self::RegistrationInstructionsResponse),
            "q" => Some(Self::OrderMassCancelRequest),
            "r" => Some(Self::OrderMassCancelReport),
            "s" => Some(Self::NewOrderCross),
            "t" => Some(Self::CrossOrderCancel),
            "u" => Some(Self::CrossOrderCancelRequest),
            "v" => Some(Self::SecurityTypeRequest),
            "w" => Some(Self::SecurityTypes),
            "x" => Some(Self::SecurityListRequest),
            "y" => Some(Self::SecurityList),
            "z" => Some(Self::DerivativeSecurityListRequest),
            _ => None,
        }
    }
}

/// `OrdStatus` (tag 39, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdStatus {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    PartiallyFilled,
    /// Wire value `2`.
    Filled,
    /// Wire value `3`.
    DoneForDay,
    /// Wire value `4`.
    Canceled,
    /// Wire value `5`.
    Replaced,
    /// Wire value `6`.
    PendingCancel,
    /// Wire value `7`.
    Stopped,
    /// Wire value `8`.
    Rejected,
    /// Wire value `9`.
    Suspended,
    /// Wire value `A`.
    PendingNew,
    /// Wire value `B`.
    Calculated,
    /// Wire value `C`.
    Expired,
    /// Wire value `D`.
    AcceptedForBidding,
    /// Wire value `E`.
    PendingReplace,
}

impl OrdStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::PartiallyFilled => '1',
            Self::Filled => '2',
            Self::DoneForDay => '3',
            Self::Canceled => '4',
            Self::Replaced => '5',
            Self::PendingCancel => '6',
            Self::Stopped => '7',
            Self::Rejected => '8',
            Self::Suspended => '9',
            Self::PendingNew => 'A',
            Self::Calculated => 'B',
            Self::Expired => 'C',
            Self::AcceptedForBidding => 'D',
            Self::PendingReplace => 'E',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::PartiallyFilled),
            '2' => Some(Self::Filled),
            '3' => Some(Self::DoneForDay),
            '4' => Some(Self::Canceled),
            '5' => Some(Self::Replaced),
            '6' => Some(Self::PendingCancel),
            '7' => Some(Self::Stopped),
            '8' => Some(Self::Rejected),
            '9' => Some(Self::Suspended),
            'A' => Some(Self::PendingNew),
            'B' => Some(Self::Calculated),
            'C' => Some(Self::Expired),
            'D' => Some(Self::AcceptedForBidding),
            'E' => Some(Self::PendingReplace),
            _ => None,
        }
    }
}

/// `OrdType` (tag 40, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdType {
    /// Wire value `1`.
    Market,
    /// Wire value `2`.
    Limit,
    /// Wire value `3`.
    Stop,
    /// Wire value `4`.
    StopLimit,
    /// Wire value `5`.
    MarketOnClose,
    /// Wire value `6`.
    WithOrWithout,
    /// Wire value `7`.
    LimitOrBetter,
    /// Wire value `8`.
    LimitWithOrWithout,
    /// Wire value `9`.
    OnBasis,
    /// Wire value `A`.
    OnClose,
    /// Wire value `B`.
    LimitOnClose,
    /// Wire value `C`.
    ForexMarket,
    /// Wire value `D`.
    PreviouslyQuoted,
    /// Wire value `E`.
    PreviouslyIndicated,
    /// Wire value `F`.
    ForexLimit,
    /// Wire value `G`.
    ForexSwap,
    /// Wire value `H`.
    ForexPreviouslyQuoted,
    /// Wire value `I`.
    Funari,
    /// Wire value `J`.
    MarketIfTouched,
    /// Wire value `K`.
    MarketWithLeftoverAsLimit,
    /// Wire value `L`.
    PreviousFundValuationPoint,
    /// Wire value `M`.
    NextFundValuationPoint,
    /// Wire value `P`.
    Pegged,
}

impl OrdType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Market => '1',
            Self::Limit => '2',
            Self::Stop => '3',
            Self::StopLimit => '4',
            Self::MarketOnClose => '5',
            Self::WithOrWithout => '6',
            Self::LimitOrBetter => '7',
            Self::LimitWithOrWithout => '8',
            Self::OnBasis => '9',
            Self::OnClose => 'A',
            Self::LimitOnClose => 'B',
            Self::ForexMarket => 'C',
            Self::PreviouslyQuoted => 'D',
            Self::PreviouslyIndicated => 'E',
            Self::ForexLimit => 'F',
            Self::ForexSwap => 'G',
            Self::ForexPreviouslyQuoted => 'H',
            Self::Funari => 'I',
            Self::MarketIfTouched => 'J',
            Self::MarketWithLeftoverAsLimit => 'K',
            Self::PreviousFundValuationPoint => 'L',
            Self::NextFundValuationPoint => 'M',
            Self::Pegged => 'P',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Market),
            '2' => Some(Self::Limit),
            '3' => Some(Self::Stop),
            '4' => Some(Self::StopLimit),
            '5' => Some(Self::MarketOnClose),
            '6' => Some(Self::WithOrWithout),
            '7' => Some(Self::LimitOrBetter),
            '8' => Some(Self::LimitWithOrWithout),
            '9' => Some(Self::OnBasis),
            'A' => Some(Self::OnClose),
            'B' => Some(Self::LimitOnClose),
            'C' => Some(Self::ForexMarket),
            'D' => Some(Self::PreviouslyQuoted),
            'E' => Some(Self::PreviouslyIndicated),
            'F' => Some(Self::ForexLimit),
            'G' => Some(Self::ForexSwap),
            'H' => Some(Self::ForexPreviouslyQuoted),
            'I' => Some(Self::Funari),
            'J' => Some(Self::MarketIfTouched),
            'K' => Some(Self::MarketWithLeftoverAsLimit),
            'L' => Some(Self::PreviousFundValuationPoint),
            'M' => Some(Self::NextFundValuationPoint),
            'P' => Some(Self::Pegged),
            _ => None,
        }
    }
}

/// `Rule80A` (tag 47, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule80A {
    /// Wire value `A`.
    AgencySingleOrder,
    /// Wire value `B`.
    ShortExemptTransactionReferToAType,
    /// Wire value `C`.
    ProgramOrderNonIndexArbForMemberFirm,
    /// Wire value `D`.
    ProgramOrderIndexArbForMemberFirm,
    /// Wire value `E`.
    ShortExemptTransactionForPrincipal,
    /// Wire value `F`.
    ShortExemptTransactionReferToWType,
    /// Wire value `H`.
    ShortExemptTransactionReferToIType,
    /// Wire value `I`.
    IndividualInvestor,
    /// Wire value `J`.
    ProgramOrderIndexArbForIndividualCustomer,
    /// Wire value `K`.
    ProgramOrderNonIndexArbForIndividualCustomer,
    /// Wire value `L`.
    ShortExemptAffiliated,
    /// Wire value `M`.
    ProgramOrderIndexArbForOtherMember,
    /// Wire value `N`.
    ProgramOrderNonIndexArbForOtherMember,
    /// Wire value `O`.
    ProprietaryAffiliated,
    /// Wire value `P`.
    Principal,
    /// Wire value `R`.
    TransactionsNonMember,
    /// Wire value `S`.
    SpecialistTrades,
    /// Wire value `T`.
    TransactionsUnaffiliatedMember,
    /// Wire value `U`.
    ProgramOrderIndexArbForOtherAgency,
    /// Wire value `W`.
    AllOtherOrdersAsAgentForOtherMember,
    /// Wire value `X`.
    ShortExemptNotAffiliated,
    /// Wire value `Y`.
    ProgramOrderNonIndexArbForOtherAgency,
    /// Wire value `Z`.
    ShortExemptNonmember,
}

impl Rule80A {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::AgencySingleOrder => 'A',
            Self::ShortExemptTransactionReferToAType => 'B',
            Self::ProgramOrderNonIndexArbForMemberFirm => 'C',
            Self::ProgramOrderIndexArbForMemberFirm => 'D',
            Self::ShortExemptTransactionForPrincipal => 'E',
            Self::ShortExemptTransactionReferToWType => 'F',
            Self::ShortExemptTransactionReferToIType => 'H',
            Self::IndividualInvestor => 'I',
            Self::ProgramOrderIndexArbForIndividualCustomer => 'J',
            Self::ProgramOrderNonIndexArbForIndividualCustomer => 'K',
            Self::ShortExemptAffiliated => 'L',
            Self::ProgramOrderIndexArbForOtherMember => 'M',
            Self::ProgramOrderNonIndexArbForOtherMember => 'N',
            Self::ProprietaryAffiliated => 'O',
            Self::Principal => 'P',
            Self::TransactionsNonMember => 'R',
            Self::SpecialistTrades => 'S',
            Self::TransactionsUnaffiliatedMember => 'T',
            Self::ProgramOrderIndexArbForOtherAgency => 'U',
            Self::AllOtherOrdersAsAgentForOtherMember => 'W',
            Self::ShortExemptNotAffiliated => 'X',
            Self::ProgramOrderNonIndexArbForOtherAgency => 'Y',
            Self::ShortExemptNonmember => 'Z',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'A' => Some(Self::AgencySingleOrder),
            'B' => Some(Self::ShortExemptTransactionReferToAType),
            'C' => Some(Self::ProgramOrderNonIndexArbForMemberFirm),
            'D' => Some(Self::ProgramOrderIndexArbForMemberFirm),
            'E' => Some(Self::ShortExemptTransactionForPrincipal),
            'F' => Some(Self::ShortExemptTransactionReferToWType),
            'H' => Some(Self::ShortExemptTransactionReferToIType),
            'I' => Some(Self::IndividualInvestor),
            'J' => Some(Self::ProgramOrderIndexArbForIndividualCustomer),
            'K' => Some(Self::ProgramOrderNonIndexArbForIndividualCustomer),
            'L' => Some(Self::ShortExemptAffiliated),
            'M' => Some(Self::ProgramOrderIndexArbForOtherMember),
            'N' => Some(Self::ProgramOrderNonIndexArbForOtherMember),
            'O' => Some(Self::ProprietaryAffiliated),
            'P' => Some(Self::Principal),
            'R' => Some(Self::TransactionsNonMember),
            'S' => Some(Self::SpecialistTrades),
            'T' => Some(Self::TransactionsUnaffiliatedMember),
            'U' => Some(Self::ProgramOrderIndexArbForOtherAgency),
            'W' => Some(Self::AllOtherOrdersAsAgentForOtherMember),
            'X' => Some(Self::ShortExemptNotAffiliated),
            'Y' => Some(Self::ProgramOrderNonIndexArbForOtherAgency),
            'Z' => Some(Self::ShortExemptNonmember),
            _ => None,
        }
    }
}

/// `Side` (tag 54, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// Wire value `1`.
    Buy,
    /// Wire value `2`.
    Sell,
    /// Wire value `3`.
    BuyMinus,
    /// Wire value `4`.
    SellPlus,
    /// Wire value `5`.
    SellShort,
    /// Wire value `6`.
    SellShortExempt,
    /// Wire value `7`.
    Undisclosed,
    /// Wire value `8`.
    Cross,
    /// Wire value `9`.
    CrossShort,
    /// Wire value `A`.
    CrossShortExempt,
    /// Wire value `B`.
    AsDefined,
    /// Wire value `C`.
    Opposite,
}

impl Side {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Buy => '1',
            Self::Sell => '2',
            Self::BuyMinus => '3',
            Self::SellPlus => '4',
            Self::SellShort => '5',
            Self::SellShortExempt => '6',
            Self::Undisclosed => '7',
            Self::Cross => '8',
            Self::CrossShort => '9',
            Self::CrossShortExempt => 'A',
            Self::AsDefined => 'B',
            Self::Opposite => 'C',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Buy),
            '2' => Some(Self::Sell),
            '3' => Some(Self::BuyMinus),
            '4' => Some(Self::SellPlus),
            '5' => Some(Self::SellShort),
            '6' => Some(Self::SellShortExempt),
            '7' => Some(Self::Undisclosed),
            '8' => Some(Self::Cross),
            '9' => Some(Self::CrossShort),
            'A' => Some(Self::CrossShortExempt),
            'B' => Some(Self::AsDefined),
            'C' => Some(Self::Opposite),
            _ => None,
        }
    }
}

/// `TimeInForce` (tag 59, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    /// Wire value `0`.
    Day,
    /// Wire value `1`.
    GoodTillCancel,
    /// Wire value `2`.
    AtTheOpening,
    /// Wire value `3`.
    ImmediateOrCancel,
    /// Wire value `4`.
    FillOrKill,
    /// Wire value `5`.
    GoodTillCrossing,
    /// Wire value `6`.
    GoodTillDate,
    /// Wire value `7`.
    AtTheClose,
}

impl TimeInForce {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Day => '0',
            Self::GoodTillCancel => '1',
            Self::AtTheOpening => '2',
            Self::ImmediateOrCancel => '3',
            Self::FillOrKill => '4',
            Self::GoodTillCrossing => '5',
            Self::GoodTillDate => '6',
            Self::AtTheClose => '7',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Day),
            '1' => Some(Self::GoodTillCancel),
            '2' => Some(Self::AtTheOpening),
            '3' => Some(Self::ImmediateOrCancel),
            '4' => Some(Self::FillOrKill),
            '5' => Some(Self::GoodTillCrossing),
            '6' => Some(Self::GoodTillDate),
            '7' => Some(Self::AtTheClose),
            _ => None,
        }
    }
}

/// `Urgency` (tag 61, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Urgency {
    /// Wire value `0`.
    Normal,
    /// Wire value `1`.
    Flash,
    /// Wire value `2`.
    Background,
}

impl Urgency {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Normal => '0',
            Self::Flash => '1',
            Self::Background => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Normal),
            '1' => Some(Self::Flash),
            '2' => Some(Self::Background),
            _ => None,
        }
    }
}

/// `SettlmntTyp` (tag 63, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlmntTyp {
    /// Wire value `0`.
    Regular,
    /// Wire value `1`.
    Cash,
    /// Wire value `2`.
    NextDay,
    /// Wire value `3`.
    TPlus2,
    /// Wire value `4`.
    TPlus3,
    /// Wire value `5`.
    TPlus4,
    /// Wire value `6`.
    Future,
    /// Wire value `7`.
    WhenAndIfIssued,
    /// Wire value `8`.
    SellersOption,
    /// Wire value `9`.
    TPlus5,
    /// Wire value `A`.
    TPlus1,
}

impl SettlmntTyp {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Regular => '0',
            Self::Cash => '1',
            Self::NextDay => '2',
            Self::TPlus2 => '3',
            Self::TPlus3 => '4',
            Self::TPlus4 => '5',
            Self::Future => '6',
            Self::WhenAndIfIssued => '7',
            Self::SellersOption => '8',
            Self::TPlus5 => '9',
            Self::TPlus1 => 'A',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Regular),
            '1' => Some(Self::Cash),
            '2' => Some(Self::NextDay),
            '3' => Some(Self::TPlus2),
            '4' => Some(Self::TPlus3),
            '5' => Some(Self::TPlus4),
            '6' => Some(Self::Future),
            '7' => Some(Self::WhenAndIfIssued),
            '8' => Some(Self::SellersOption),
            '9' => Some(Self::TPlus5),
            'A' => Some(Self::TPlus1),
            _ => None,
        }
    }
}

/// `AllocTransType` (tag 71, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocTransType {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    Replace,
    /// Wire value `2`.
    Cancel,
    /// Wire value `3`.
    Preliminary,
    /// Wire value `4`.
    Calculated,
    /// Wire value `5`.
    CalculatedWithoutPreliminary,
}

impl AllocTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::Replace => '1',
            Self::Cancel => '2',
            Self::Preliminary => '3',
            Self::Calculated => '4',
            Self::CalculatedWithoutPreliminary => '5',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::Replace),
            '2' => Some(Self::Cancel),
            '3' => Some(Self::Preliminary),
            '4' => Some(Self::Calculated),
            '5' => Some(Self::CalculatedWithoutPreliminary),
            _ => None,
        }
    }
}

/// `ProcessCode` (tag 81, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessCode {
    /// Wire value `0`.
    Regular,
    /// Wire value `1`.
    SoftDollar,
    /// Wire value `2`.
    StepIn,
    /// Wire value `3`.
    StepOut,
    /// Wire value `4`.
    SoftDollarStepIn,
    /// Wire value `5`.
    SoftDollarStepOut,
    /// Wire value `6`.
    PlanSponsor,
}

impl ProcessCode {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Regular => '0',
            Self::SoftDollar => '1',
            Self::StepIn => '2',
            Self::StepOut => '3',
            Self::SoftDollarStepIn => '4',
            Self::SoftDollarStepOut => '5',
            Self::PlanSponsor => '6',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Regular),
            '1' => Some(Self::SoftDollar),
            '2' => Some(Self::StepIn),
            '3' => Some(Self::StepOut),
            '4' => Some(Self::SoftDollarStepIn),
            '5' => Some(Self::SoftDollarStepOut),
            '6' => Some(Self::PlanSponsor),
            _ => None,
        }
    }
}

/// `AllocStatus` (tag 87, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocStatus {
    /// Wire value `0`.
    Accepted,
    /// Wire value `1`.
    Rejected,
    /// Wire value `2`.
    PartialAccept,
    /// Wire value `3`.
    Received,
}

impl AllocStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Accepted => 0,
            Self::Rejected => 1,
            Self::PartialAccept => 2,
            Self::Received => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Accepted),
            1 => Some(Self::Rejected),
            2 => Some(Self::PartialAccept),
            3 => Some(Self::Received),
            _ => None,
        }
    }
}

/// `AllocRejCode` (tag 88, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocRejCode {
    /// Wire value `0`.
    UnknownAccount,
    /// Wire value `1`.
    IncorrectQuantity,
    /// Wire value `2`.
    IncorrectAveragePrice,
    /// Wire value `3`.
    UnknownExecutingBrokerMnemonic,
    /// Wire value `4`.
    CommissionDifference,
    /// Wire value `5`.
    UnknownOrderid,
    /// Wire value `6`.
    UnknownListid,
    /// Wire value `7`.
    Other,
}

impl AllocRejCode {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::UnknownAccount => 0,
            Self::IncorrectQuantity => 1,
            Self::IncorrectAveragePrice => 2,
            Self::UnknownExecutingBrokerMnemonic => 3,
            Self::CommissionDifference => 4,
            Self::UnknownOrderid => 5,
            Self::UnknownListid => 6,
            Self::Other => 7,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::UnknownAccount),
            1 => Some(Self::IncorrectQuantity),
            2 => Some(Self::IncorrectAveragePrice),
            3 => Some(Self::UnknownExecutingBrokerMnemonic),
            4 => Some(Self::CommissionDifference),
            5 => Some(Self::UnknownOrderid),
            6 => Some(Self::UnknownListid),
            7 => Some(Self::Other),
            _ => None,
        }
    }
}

/// `EmailType` (tag 94, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailType {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    Reply,
    /// Wire value `2`.
    AdminReply,
}

impl EmailType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::Reply => '1',
            Self::AdminReply => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::Reply),
            '2' => Some(Self::AdminReply),
            _ => None,
        }
    }
}

/// `EncryptMethod` (tag 98, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptMethod {
    /// Wire value `0`.
    None,
    /// Wire value `1`.
    PkcsProprietary,
    /// Wire value `2`.
    Des,
    /// Wire value `3`.
    PkcsDes,
    /// Wire value `4`.
    PgpDes,
    /// Wire value `5`.
    PgpDesMd5,
    /// Wire value `6`.
    Pem,
}

impl EncryptMethod {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::None => 0,
            Self::PkcsProprietary => 1,
            Self::Des => 2,
            Self::PkcsDes => 3,
            Self::PgpDes => 4,
            Self::PgpDesMd5 => 5,
            Self::Pem => 6,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::PkcsProprietary),
            2 => Some(Self::Des),
            3 => Some(Self::PkcsDes),
            4 => Some(Self::PgpDes),
            5 => Some(Self::PgpDesMd5),
            6 => Some(Self::Pem),
            _ => None,
        }
    }
}

/// `CxlRejReason` (tag 102, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CxlRejReason {
    /// Wire value `0`.
    TooLateToCancel,
    /// Wire value `1`.
    UnknownOrder,
    /// Wire value `2`.
    Broker,
    /// Wire value `3`.
    AlreadyPending,
    /// Wire value `4`.
    UnableToProcessOrderMassCancelRequest,
    /// Wire value `5`.
    OrigordmodtimeDidNotMatchLastTransacttimeOfOrder,
    /// Wire value `6`.
    DuplicateClordidReceived,
}

impl CxlRejReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::TooLateToCancel => 0,
            Self::UnknownOrder => 1,
            Self::Broker => 2,
            Self::AlreadyPending => 3,
            Self::UnableToProcessOrderMassCancelRequest => 4,
            Self::OrigordmodtimeDidNotMatchLastTransacttimeOfOrder => 5,
            Self::DuplicateClordidReceived => 6,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::TooLateToCancel),
            1 => Some(Self::UnknownOrder),
            2 => Some(Self::Broker),
            3 => Some(Self::AlreadyPending),
            4 => Some(Self::UnableToProcessOrderMassCancelRequest),
            5 => Some(Self::OrigordmodtimeDidNotMatchLastTransacttimeOfOrder),
            6 => Some(Self::DuplicateClordidReceived),
            _ => None,
        }
    }
}

/// `OrdRejReason` (tag 103, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdRejReason {
    /// Wire value `0`.
    BrokerOption,
    /// Wire value `1`.
    UnknownSymbol,
    /// Wire value `2`.
    ExchangeClosed,
    /// Wire value `3`.
    OrderExceedsLimit,
    /// Wire value `4`.
    TooLateToEnter,
    /// Wire value `5`.
    UnknownOrder,
    /// Wire value `6`.
    DuplicateOrder,
    /// Wire value `7`.
    DuplicateVerbal,
    /// Wire value `8`.
    StaleOrder,
    /// Wire value `9`.
    TradeAlongRequired,
    /// Wire value `10`.
    InvalidInvestorId,
    /// Wire value `11`.
    UnsupportedOrderCharacteristic,
    /// Wire value `12`.
    SurveillenceOption,
}

impl OrdRejReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::BrokerOption => 0,
            Self::UnknownSymbol => 1,
            Self::ExchangeClosed => 2,
            Self::OrderExceedsLimit => 3,
            Self::TooLateToEnter => 4,
            Self::UnknownOrder => 5,
            Self::DuplicateOrder => 6,
            Self::DuplicateVerbal => 7,
            Self::StaleOrder => 8,
            Self::TradeAlongRequired => 9,
            Self::InvalidInvestorId => 10,
            Self::UnsupportedOrderCharacteristic => 11,
            Self::SurveillenceOption => 12,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::BrokerOption),
            1 => Some(Self::UnknownSymbol),
            2 => Some(Self::ExchangeClosed),
            3 => Some(Self::OrderExceedsLimit),
            4 => Some(Self::TooLateToEnter),
            5 => Some(Self::UnknownOrder),
            6 => Some(Self::DuplicateOrder),
            7 => Some(Self::DuplicateVerbal),
            8 => Some(Self::StaleOrder),
            9 => Some(Self::TradeAlongRequired),
            10 => Some(Self::InvalidInvestorId),
            11 => Some(Self::UnsupportedOrderCharacteristic),
            12 => Some(Self::SurveillenceOption),
            _ => None,
        }
    }
}

/// `IoiQualifier` (tag 104, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoiQualifier {
    /// Wire value `A`.
    AllOrNone,
    /// Wire value `B`.
    MarketOnClose,
    /// Wire value `C`.
    AtTheClose,
    /// Wire value `D`.
    Vwap,
    /// Wire value `I`.
    InTouchWith,
    /// Wire value `L`.
    Limit,
    /// Wire value `M`.
    MoreBehind,
    /// Wire value `O`.
    AtTheOpen,
    /// Wire value `P`.
    TakingAPosition,
    /// Wire value `Q`.
    AtTheMarket,
    /// Wire value `R`.
    ReadyToTrade,
    /// Wire value `S`.
    PortfolioShown,
    /// Wire value `T`.
    ThroughTheDay,
    /// Wire value `V`.
    Versus,
    /// Wire value `W`.
    Indication,
    /// Wire value `X`.
    CrossingOpportunity,
    /// Wire value `Y`.
    AtTheMidpoint,
    /// Wire value `Z`.
    PreOpen,
}

impl IoiQualifier {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::AllOrNone => 'A',
            Self::MarketOnClose => 'B',
            Self::AtTheClose => 'C',
            Self::Vwap => 'D',
            Self::InTouchWith => 'I',
            Self::Limit => 'L',
            Self::MoreBehind => 'M',
            Self::AtTheOpen => 'O',
            Self::TakingAPosition => 'P',
            Self::AtTheMarket => 'Q',
            Self::ReadyToTrade => 'R',
            Self::PortfolioShown => 'S',
            Self::ThroughTheDay => 'T',
            Self::Versus => 'V',
            Self::Indication => 'W',
            Self::CrossingOpportunity => 'X',
            Self::AtTheMidpoint => 'Y',
            Self::PreOpen => 'Z',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'A' => Some(Self::AllOrNone),
            'B' => Some(Self::MarketOnClose),
            'C' => Some(Self::AtTheClose),
            'D' => Some(Self::Vwap),
            'I' => Some(Self::InTouchWith),
            'L' => Some(Self::Limit),
            'M' => Some(Self::MoreBehind),
            'O' => Some(Self::AtTheOpen),
            'P' => Some(Self::TakingAPosition),
            'Q' => Some(Self::AtTheMarket),
            'R' => Some(Self::ReadyToTrade),
            'S' => Some(Self::PortfolioShown),
            'T' => Some(Self::ThroughTheDay),
            'V' => Some(Self::Versus),
            'W' => Some(Self::Indication),
            'X' => Some(Self::CrossingOpportunity),
            'Y' => Some(Self::AtTheMidpoint),
            'Z' => Some(Self::PreOpen),
            _ => None,
        }
    }
}

/// `DkReason` (tag 127, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkReason {
    /// Wire value `A`.
    UnknownSymbol,
    /// Wire value `B`.
    WrongSide,
    /// Wire value `C`.
    QuantityExceedsOrder,
    /// Wire value `D`.
    NoMatchingOrder,
    /// Wire value `E`.
    PriceExceedsLimit,
    /// Wire value `Z`.
    Other,
}

impl DkReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::UnknownSymbol => 'A',
            Self::WrongSide => 'B',
            Self::QuantityExceedsOrder => 'C',
            Self::NoMatchingOrder => 'D',
            Self::PriceExceedsLimit => 'E',
            Self::Other => 'Z',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'A' => Some(Self::UnknownSymbol),
            'B' => Some(Self::WrongSide),
            'C' => Some(Self::QuantityExceedsOrder),
            'D' => Some(Self::NoMatchingOrder),
            'E' => Some(Self::PriceExceedsLimit),
            'Z' => Some(Self::Other),
            _ => None,
        }
    }
}

/// `MiscFeeType` (tag 139, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscFeeType {
    /// Wire value `1`.
    Regulatory,
    /// Wire value `2`.
    Tax,
    /// Wire value `3`.
    LocalCommission,
    /// Wire value `4`.
    ExchangeFees,
    /// Wire value `5`.
    Stamp,
    /// Wire value `6`.
    Levy,
    /// Wire value `7`.
    Other,
    /// Wire value `8`.
    Markup,
    /// Wire value `9`.
    ConsumptionTax,
}

impl MiscFeeType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Regulatory => '1',
            Self::Tax => '2',
            Self::LocalCommission => '3',
            Self::ExchangeFees => '4',
            Self::Stamp => '5',
            Self::Levy => '6',
            Self::Other => '7',
            Self::Markup => '8',
            Self::ConsumptionTax => '9',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Regulatory),
            '2' => Some(Self::Tax),
            '3' => Some(Self::LocalCommission),
            '4' => Some(Self::ExchangeFees),
            '5' => Some(Self::Stamp),
            '6' => Some(Self::Levy),
            '7' => Some(Self::Other),
            '8' => Some(Self::Markup),
            '9' => Some(Self::ConsumptionTax),
            _ => None,
        }
    }
}

/// `ExecType` (tag 150, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecType {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    PartialFill,
    /// Wire value `2`.
    Fill,
    /// Wire value `3`.
    DoneForDay,
    /// Wire value `4`.
    Canceled,
    /// Wire value `5`.
    Replace,
    /// Wire value `6`.
    PendingCancel,
    /// Wire value `7`.
    Stopped,
    /// Wire value `8`.
    Rejected,
    /// Wire value `9`.
    Suspended,
    /// Wire value `A`.
    PendingNew,
    /// Wire value `B`.
    Calculated,
    /// Wire value `C`.
    Expired,
    /// Wire value `D`.
    Restated,
    /// Wire value `E`.
    PendingReplace,
    /// Wire value `F`.
    Trade,
    /// Wire value `G`.
    TradeCorrect,
    /// Wire value `H`.
    TradeCancel,
    /// Wire value `I`.
    OrderStatus,
}

impl ExecType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::PartialFill => '1',
            Self::Fill => '2',
            Self::DoneForDay => '3',
            Self::Canceled => '4',
            Self::Replace => '5',
            Self::PendingCancel => '6',
            Self::Stopped => '7',
            Self::Rejected => '8',
            Self::Suspended => '9',
            Self::PendingNew => 'A',
            Self::Calculated => 'B',
            Self::Expired => 'C',
            Self::Restated => 'D',
            Self::PendingReplace => 'E',
            Self::Trade => 'F',
            Self::TradeCorrect => 'G',
            Self::TradeCancel => 'H',
            Self::OrderStatus => 'I',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::PartialFill),
            '2' => Some(Self::Fill),
            '3' => Some(Self::DoneForDay),
            '4' => Some(Self::Canceled),
            '5' => Some(Self::Replace),
            '6' => Some(Self::PendingCancel),
            '7' => Some(Self::Stopped),
            '8' => Some(Self::Rejected),
            '9' => Some(Self::Suspended),
            'A' => Some(Self::PendingNew),
            'B' => Some(Self::Calculated),
            'C' => Some(Self::Expired),
            'D' => Some(Self::Restated),
            'E' => Some(Self::PendingReplace),
            'F' => Some(Self::Trade),
            'G' => Some(Self::TradeCorrect),
            'H' => Some(Self::TradeCancel),
            'I' => Some(Self::OrderStatus),
            _ => None,
        }
    }
}

/// `SettlInstMode` (tag 160, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstMode {
    /// Wire value `0`.
    Default,
    /// Wire value `1`.
    StandingInstructionsProvided,
    /// Wire value `2`.
    SpecificAllocationAccountOverriding,
    /// Wire value `3`.
    SpecificAllocationAccountStanding,
    /// Wire value `4`.
    SpecificOrderForASingleAccount,
}

impl SettlInstMode {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Default => '0',
            Self::StandingInstructionsProvided => '1',
            Self::SpecificAllocationAccountOverriding => '2',
            Self::SpecificAllocationAccountStanding => '3',
            Self::SpecificOrderForASingleAccount => '4',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Default),
            '1' => Some(Self::StandingInstructionsProvided),
            '2' => Some(Self::SpecificAllocationAccountOverriding),
            '3' => Some(Self::SpecificAllocationAccountStanding),
            '4' => Some(Self::SpecificOrderForASingleAccount),
            _ => None,
        }
    }
}

/// `SettlInstTransType` (tag 163, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstTransType {
    /// Wire value `C`.
    Cancel,
    /// Wire value `N`.
    New,
    /// Wire value `R`.
    Replace,
}

impl SettlInstTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Cancel => 'C',
            Self::New => 'N',
            Self::Replace => 'R',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'C' => Some(Self::Cancel),
            'N' => Some(Self::New),
            'R' => Some(Self::Replace),
            _ => None,
        }
    }
}

/// `SettlInstSource` (tag 165, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstSource {
    /// Wire value `1`.
    BrokersInstructions,
    /// Wire value `2`.
    InstitutionsInstructions,
    /// Wire value `3`.
    Investor,
}

impl SettlInstSource {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::BrokersInstructions => '1',
            Self::InstitutionsInstructions => '2',
            Self::Investor => '3',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::BrokersInstructions),
            '2' => Some(Self::InstitutionsInstructions),
            '3' => Some(Self::Investor),
            _ => None,
        }
    }
}

/// `SettlLocation` (tag 166, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlLocation {
    /// Wire value `CED`.
    Cedel,
    /// Wire value `DTC`.
    DepositoryTrustCompany,
    /// Wire value `EUR`.
    Euroclear,
    /// Wire value `FED`.
    FederalBookEntry,
    /// Wire value `PED`.
    Physical,
    /// Wire value `PTC`.
    ParticipantTrustCompanyIsoCountry,
}

impl SettlLocation {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Cedel => "CED",
            Self::DepositoryTrustCompany => "DTC",
            Self::Euroclear => "EUR",
            Self::FederalBookEntry => "FED",
            Self::Physical => "PED",
            Self::ParticipantTrustCompanyIsoCountry => "PTC",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "CED" => Some(Self::Cedel),
            "DTC" => Some(Self::DepositoryTrustCompany),
            "EUR" => Some(Self::Euroclear),
            "FED" => Some(Self::FederalBookEntry),
            "PED" => Some(Self::Physical),
            "PTC" => Some(Self::ParticipantTrustCompanyIsoCountry),
            _ => None,
        }
    }
}

/// `SecurityType` (tag 167, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    /// Wire value `?`.
    Wildcard,
    /// Wire value `ABS`.
    AssetBackedSecurities,
    /// Wire value `AMENDED`.
    AmendedAndRestated,
    /// Wire value `AN`.
    OtherAnticipationNotes,
    /// Wire value `BA`.
    BankersAcceptance,
    /// Wire value `BN`.
    BankNotes,
    /// Wire value `BOX`.
    BillOfExchanges,
    /// Wire value `BRADY`.
    BradyBond,
    /// Wire value `BRIDGE`.
    BridgeLoan,
    /// Wire value `CB`.
    ConvertableBond,
    /// Wire value `CD`.
    CertificateOfDeposit,
    /// Wire value `CL`.
    CallLoans,
    /// Wire value `CMBS`.
    CorpMortgageBackedSecurities,
    /// Wire value `CMO`.
    CollateralizedMortgageObligation,
    /// Wire value `COFO`.
    CertificateOfObligation,
    /// Wire value `COFP`.
    CertificateOfParticipation,
    /// Wire value `CORP`.
    CorporateBond,
    /// Wire value `CP`.
    CommercialPaper,
    /// Wire value `CPP`.
    CorporatePrivatePlacement,
    /// Wire value `CS`.
    CommonStock,
    /// Wire value `DEFLTED`.
    Defaulted,
    /// Wire value `DINP`.
    DebtorInPossession,
    /// Wire value `DP`.
    DepositNotes,
    /// Wire value `DUAL`.
    DualCurrency,
    /// Wire value `FOR`.
    ForeignExchangeContract,
    /// Wire value `GO`.
    GeneralObligationBonds,
    /// Wire value `IET`.
    IoetteMortgage,
    /// Wire value `LOFC`.
    LetterOfCredit,
    /// Wire value `LQN`.
    LiquidityNotes,
    /// Wire value `MATURED`.
    Matured,
    /// Wire value `MBS`.
    MortgageBackedSecurities,
    /// Wire value `MF`.
    MutualFund,
    /// Wire value `MIO`.
    MortgageInterestOnly,
    /// Wire value `MLEG`.
    MultiLegInstrument,
    /// Wire value `MPO`.
    MortgagePrincipalOnly,
    /// Wire value `MPP`.
    MortgagePrivatePlacement,
    /// Wire value `MPT`.
    MiscellaneousPassThrough,
    /// Wire value `MT`.
    MandatoryTender,
    /// Wire value `MTN`.
    MediumTermNotes,
    /// Wire value `NONE`.
    NoSecurityType,
    /// Wire value `ONITE`.
    Overnite,
    /// Wire value `PN`.
    PromissoryNotes,
    /// Wire value `POOL`.
    AgencyPools,
    /// Wire value `PS`.
    PreferedStock,
    /// Wire value `PZFJ`.
    PlazosFijos,
    /// Wire value `RAN`.
    RevenueAnticipationNote,
    /// Wire value `REPLACD`.
    Replaced,
    /// Wire value `RETIRED`.
    Retired,
    /// Wire value `REV`.
    RevenueBonds,
    /// Wire value `RP`.
    RepurchaseAgreement,
    /// Wire value `RVLV`.
    RevolverLoan,
    /// Wire value `RVLVTRM`.
    RevolverTermLoan,
    /// Wire value `RVRP`.
    ReverseRepurchaseAgreement,
    /// Wire value `SPCLA`.
    SpecialAssessment,
    /// Wire value `SPCLO`.
    SpecialObligation,
    /// Wire value `SPCLT`.
    SpecialTax,
    /// Wire value `STN`.
    ShortTermLoanNote,
    /// Wire value `STRUCT`.
    StructuredNotes,
    /// Wire value `SWING`.
    SwingLineFacility,
    /// Wire value `TAN`.
    TaxAnticipationNote,
    /// Wire value `TAXA`.
    TaxAllocation,
    /// Wire value `TBOND`.
    UsTreasuryBond,
    /// Wire value `TCAL`.
    PrincipalStripOfACallableBondOrNote,
    /// Wire value `TD`.
    TimeDeposit,
    /// Wire value `TECP`.
    TaxExemptCommercialPaper,
    /// Wire value `TERM`.
    TermLoan,
    /// Wire value `TINT`.
    InterestStripFromAnyBondOrNote,
    /// Wire value `TIPS`.
    TreasuryInflationProtectedSecurities,
    /// Wire value `TPRN`.
    PrincipalStripFromANonCallableBondOrNote,
    /// Wire value `TRAN`.
    TaxAndRevenueAnticipationNote,
    /// Wire value `UST`.
    UsTreasuryNoteBond,
    /// Wire value `USTB`.
    UsTreasuryBill,
    /// Wire value `VRDN`.
    VariableRateDemandNote,
    /// Wire value `WAR`.
    Warrant,
    /// Wire value `WITHDRN`.
    Withdrawn,
    /// Wire value `XCN`.
    ExtendedCommNote,
    /// Wire value `XLINKD`.
    IndexLinked,
    /// Wire value `YANK`.
    YankeeCorporateBond,
}

impl SecurityType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Wildcard => "?",
            Self::AssetBackedSecurities => "ABS",
            Self::AmendedAndRestated => "AMENDED",
            Self::OtherAnticipationNotes => "AN",
            Self::BankersAcceptance => "BA",
            Self::BankNotes => "BN",
            Self::BillOfExchanges => "BOX",
            Self::BradyBond => "BRADY",
            Self::BridgeLoan => "BRIDGE",
            Self::ConvertableBond => "CB",
            Self::CertificateOfDeposit => "CD",
            Self::CallLoans => "CL",
            Self::CorpMortgageBackedSecurities => "CMBS",
            Self::CollateralizedMortgageObligation => "CMO",
            Self::CertificateOfObligation => "COFO",
            Self::CertificateOfParticipation => "COFP",
            Self::CorporateBond => "CORP",
            Self::CommercialPaper => "CP",
            Self::CorporatePrivatePlacement => "CPP",
            Self::CommonStock => "CS",
            Self::Defaulted => "DEFLTED",
            Self::DebtorInPossession => "DINP",
            Self::DepositNotes => "DP",
            Self::DualCurrency => "DUAL",
            Self::ForeignExchangeContract => "FOR",
            Self::GeneralObligationBonds => "GO",
            Self::IoetteMortgage => "IET",
            Self::LetterOfCredit => "LOFC",
            Self::LiquidityNotes => "LQN",
            Self::Matured => "MATURED",
            Self::MortgageBackedSecurities => "MBS",
            Self::MutualFund => "MF",
            Self::MortgageInterestOnly => "MIO",
            Self::MultiLegInstrument => "MLEG",
            Self::MortgagePrincipalOnly => "MPO",
            Self::MortgagePrivatePlacement => "MPP",
            Self::MiscellaneousPassThrough => "MPT",
            Self::MandatoryTender => "MT",
            Self::MediumTermNotes => "MTN",
            Self::NoSecurityType => "NONE",
            Self::Overnite => "ONITE",
            Self::PromissoryNotes => "PN",
            Self::AgencyPools => "POOL",
            Self::PreferedStock => "PS",
            Self::PlazosFijos => "PZFJ",
            Self::RevenueAnticipationNote => "RAN",
            Self::Replaced => "REPLACD",
            Self::Retired => "RETIRED",
            Self::RevenueBonds => "REV",
            Self::RepurchaseAgreement => "RP",
            Self::RevolverLoan => "RVLV",
            Self::RevolverTermLoan => "RVLVTRM",
            Self::ReverseRepurchaseAgreement => "RVRP",
            Self::SpecialAssessment => "SPCLA",
            Self::SpecialObligation => "SPCLO",
            Self::SpecialTax => "SPCLT",
            Self::ShortTermLoanNote => "STN",
            Self::StructuredNotes => "STRUCT",
            Self::SwingLineFacility => "SWING",
            Self::TaxAnticipationNote => "TAN",
            Self::TaxAllocation => "TAXA",
            Self::UsTreasuryBond => "TBOND",
            Self::PrincipalStripOfACallableBondOrNote => "TCAL",
            Self::TimeDeposit => "TD",
            Self::TaxExemptCommercialPaper => "TECP",
            Self::TermLoan => "TERM",
            Self::InterestStripFromAnyBondOrNote => "TINT",
            Self::TreasuryInflationProtectedSecurities => "TIPS",
            Self::PrincipalStripFromANonCallableBondOrNote => "TPRN",
            Self::TaxAndRevenueAnticipationNote => "TRAN",
            Self::UsTreasuryNoteBond => "UST",
            Self::UsTreasuryBill => "USTB",
            Self::VariableRateDemandNote => "VRDN",
            Self::Warrant => "WAR",
            Self::Withdrawn => "WITHDRN",
            Self::ExtendedCommNote => "XCN",
            Self::IndexLinked => "XLINKD",
            Self::YankeeCorporateBond => "YANK",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "?" => Some(Self::Wildcard),
            "ABS" => Some(Self::AssetBackedSecurities),
            "AMENDED" => Some(Self::AmendedAndRestated),
            "AN" => Some(Self::OtherAnticipationNotes),
            "BA" => Some(Self::BankersAcceptance),
            "BN" => Some(Self::BankNotes),
            "BOX" => Some(Self::BillOfExchanges),
            "BRADY" => Some(Self::BradyBond),
            "BRIDGE" => Some(Self::BridgeLoan),
            "CB" => Some(Self::ConvertableBond),
            "CD" => Some(Self::CertificateOfDeposit),
            "CL" => Some(Self::CallLoans),
            "CMBS" => Some(Self::CorpMortgageBackedSecurities),
            "CMO" => Some(Self::CollateralizedMortgageObligation),
            "COFO" => Some(Self::CertificateOfObligation),
            "COFP" => Some(Self::CertificateOfParticipation),
            "CORP" => Some(Self::CorporateBond),
            "CP" => Some(Self::CommercialPaper),
            "CPP" => Some(Self::CorporatePrivatePlacement),
            "CS" => Some(Self::CommonStock),
            "DEFLTED" => Some(Self::Defaulted),
            "DINP" => Some(Self::DebtorInPossession),
            "DP" => Some(Self::DepositNotes),
            "DUAL" => Some(Self::DualCurrency),
            "FOR" => Some(Self::ForeignExchangeContract),
            "GO" => Some(Self::GeneralObligationBonds),
            "IET" => Some(Self::IoetteMortgage),
            "LOFC" => Some(Self::LetterOfCredit),
            "LQN" => Some(Self::LiquidityNotes),
            "MATURED" => Some(Self::Matured),
            "MBS" => Some(Self::MortgageBackedSecurities),
            "MF" => Some(Self::MutualFund),
            "MIO" => Some(Self::MortgageInterestOnly),
            "MLEG" => Some(Self::MultiLegInstrument),
            "MPO" => Some(Self::MortgagePrincipalOnly),
            "MPP" => Some(Self::MortgagePrivatePlacement),
            "MPT" => Some(Self::MiscellaneousPassThrough),
            "MT" => Some(Self::MandatoryTender),
            "MTN" => Some(Self::MediumTermNotes),
            "NONE" => Some(Self::NoSecurityType),
            "ONITE" => Some(Self::Overnite),
            "PN" => Some(Self::PromissoryNotes),
            "POOL" => Some(Self::AgencyPools),
            "PS" => Some(Self::PreferedStock),
            "PZFJ" => Some(Self::PlazosFijos),
            "RAN" => Some(Self::RevenueAnticipationNote),
            "REPLACD" => Some(Self::Replaced),
            "RETIRED" => Some(Self::Retired),
            "REV" => Some(Self::RevenueBonds),
            "RP" => Some(Self::RepurchaseAgreement),
            "RVLV" => Some(Self::RevolverLoan),
            "RVLVTRM" => Some(Self::RevolverTermLoan),
            "RVRP" => Some(Self::ReverseRepurchaseAgreement),
            "SPCLA" => Some(Self::SpecialAssessment),
            "SPCLO" => Some(Self::SpecialObligation),
            "SPCLT" => Some(Self::SpecialTax),
            "STN" => Some(Self::ShortTermLoanNote),
            "STRUCT" => Some(Self::StructuredNotes),
            "SWING" => Some(Self::SwingLineFacility),
            "TAN" => Some(Self::TaxAnticipationNote),
            "TAXA" => Some(Self::TaxAllocation),
            "TBOND" => Some(Self::UsTreasuryBond),
            "TCAL" => Some(Self::PrincipalStripOfACallableBondOrNote),
            "TD" => Some(Self::TimeDeposit),
            "TECP" => Some(Self::TaxExemptCommercialPaper),
            "TERM" => Some(Self::TermLoan),
            "TINT" => Some(Self::InterestStripFromAnyBondOrNote),
            "TIPS" => Some(Self::TreasuryInflationProtectedSecurities),
            "TPRN" => Some(Self::PrincipalStripFromANonCallableBondOrNote),
            "TRAN" => Some(Self::TaxAndRevenueAnticipationNote),
            "UST" => Some(Self::UsTreasuryNoteBond),
            "USTB" => Some(Self::UsTreasuryBill),
            "VRDN" => Some(Self::VariableRateDemandNote),
            "WAR" => Some(Self::Warrant),
            "WITHDRN" => Some(Self::Withdrawn),
            "XCN" => Some(Self::ExtendedCommNote),
            "XLINKD" => Some(Self::IndexLinked),
            "YANK" => Some(Self::YankeeCorporateBond),
            _ => None,
        }
    }
}

/// `StandInstDbType` (tag 169, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandInstDbType {
    /// Wire value `0`.
    Other,
    /// Wire value `1`.
    DtcSid,
    /// Wire value `2`.
    ThomsonAlert,
    /// Wire value `3`.
    AGlobalCustodian,
}

impl StandInstDbType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Other => 0,
            Self::DtcSid => 1,
            Self::ThomsonAlert => 2,
            Self::AGlobalCustodian => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Other),
            1 => Some(Self::DtcSid),
            2 => Some(Self::ThomsonAlert),
            3 => Some(Self::AGlobalCustodian),
            _ => None,
        }
    }
}

/// `AllocLinkType` (tag 197, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocLinkType {
    /// Wire value `0`.
    FXNetting,
    /// Wire value `1`.
    FXSwap,
}

impl AllocLinkType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::FXNetting => 0,
            Self::FXSwap => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::FXNetting),
            1 => Some(Self::FXSwap),
            _ => None,
        }
    }
}

/// `PutOrCall` (tag 201, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutOrCall {
    /// Wire value `0`.
    Put,
    /// Wire value `1`.
    Call,
}

impl PutOrCall {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Put => 0,
            Self::Call => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Put),
            1 => Some(Self::Call),
            _ => None,
        }
    }
}

/// `CoveredOrUncovered` (tag 203, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoveredOrUncovered {
    /// Wire value `0`.
    Covered,
    /// Wire value `1`.
    Uncovered,
}

impl CoveredOrUncovered {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Covered => 0,
            Self::Uncovered => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Covered),
            1 => Some(Self::Uncovered),
            _ => None,
        }
    }
}

/// `CustomerOrFirm` (tag 204, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerOrFirm {
    /// Wire value `0`.
    Customer,
    /// Wire value `1`.
    Firm,
}

impl CustomerOrFirm {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Customer => 0,
            Self::Firm => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Customer),
            1 => Some(Self::Firm),
            _ => None,
        }
    }
}

/// `AllocHandlInst` (tag 209, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocHandlInst {
    /// Wire value `1`.
    Match,
    /// Wire value `2`.
    Forward,
    /// Wire value `3`.
    ForwardAndMatch,
}

impl AllocHandlInst {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Match => 1,
            Self::Forward => 2,
            Self::ForwardAndMatch => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Match),
            2 => Some(Self::Forward),
            3 => Some(Self::ForwardAndMatch),
            _ => None,
        }
    }
}

/// `RoutingType` (tag 216, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingType {
    /// Wire value `1`.
    TargetFirm,
    /// Wire value `2`.
    TargetList,
    /// Wire value `3`.
    BlockFirm,
    /// Wire value `4`.
    BlockList,
}

impl RoutingType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::TargetFirm => 1,
            Self::TargetList => 2,
            Self::BlockFirm => 3,
            Self::BlockList => 4,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::TargetFirm),
            2 => Some(Self::TargetList),
            3 => Some(Self::BlockFirm),
            4 => Some(Self::BlockList),
            _ => None,
        }
    }
}

/// `Benchmark` (tag 219, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Benchmark {
    /// Wire value `1`.
    Curve,
    /// Wire value `2`.
    Fiveyr,
    /// Wire value `3`.
    Old5,
    /// Wire value `4`.
    Tenyr,
    /// Wire value `5`.
    Old10,
    /// Wire value `6`.
    Thirtyyr,
    /// Wire value `7`.
    Old30,
    /// Wire value `8`.
    Threemolibor,
    /// Wire value `9`.
    Sixmolibor,
}

impl Benchmark {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Curve => '1',
            Self::Fiveyr => '2',
            Self::Old5 => '3',
            Self::Tenyr => '4',
            Self::Old10 => '5',
            Self::Thirtyyr => '6',
            Self::Old30 => '7',
            Self::Threemolibor => '8',
            Self::Sixmolibor => '9',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Curve),
            '2' => Some(Self::Fiveyr),
            '3' => Some(Self::Old5),
            '4' => Some(Self::Tenyr),
            '5' => Some(Self::Old10),
            '6' => Some(Self::Thirtyyr),
            '7' => Some(Self::Old30),
            '8' => Some(Self::Threemolibor),
            '9' => Some(Self::Sixmolibor),
            _ => None,
        }
    }
}

/// `YieldType` (tag 235, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YieldType {
    /// Wire value `AFTERTAX`.
    AfterTaxYield,
    /// Wire value `ANNUAL`.
    AnnualYield,
    /// Wire value `ATISSUE`.
    YieldAtIssue,
    /// Wire value `AVGLIFE`.
    YieldToAverageLife,
    /// Wire value `AVGMATURITY`.
    YieldToAverageMaturity,
    /// Wire value `BOOK`.
    BookYield,
    /// Wire value `CALL`.
    YieldToNextCall,
    /// Wire value `CHANGE`.
    YieldChangeSinceClose,
    /// Wire value `CLOSE`.
    ClosingYield,
    /// Wire value `COMPOUND`.
    CompoundYield,
    /// Wire value `CURRENT`.
    CurrentYield,
    /// Wire value `GOVTEQUIV`.
    GovernmentEquivalentYield,
    /// Wire value `GROSS`.
    TrueGrossYield,
    /// Wire value `INFLATION`.
    YieldWithInflationAssumption,
    /// Wire value `INVERSEFLOATER`.
    InverseFloaterBondYield,
    /// Wire value `LASTCLOSE`.
    MostRecentClosingYield,
    /// Wire value `LASTMONTH`.
    ClosingYieldMostRecentMonth,
    /// Wire value `LASTQUARTER`.
    ClosingYieldMostRecentQuarter,
    /// Wire value `LASTYEAR`.
    ClosingYieldMostRecentYear,
    /// Wire value `LONGAVGLIFE`.
    YieldToLongestAverageLife,
    /// Wire value `LONGEST`.
    YieldToLongestAverage,
    /// Wire value `MARK`.
    MarkToMarketYield,
    /// Wire value `MATURITY`.
    YieldToMaturity,
    /// Wire value `NEXTREFUND`.
    YieldToNextRefund,
    /// Wire value `OPENAVG`.
    OpenAverageYield,
    /// Wire value `PREVCLOSE`.
    PreviousCloseYield,
    /// Wire value `PROCEEDS`.
    ProceedsYield,
    /// Wire value `PUT`.
    YieldToNextPut,
    /// Wire value `SEMIANNUAL`.
    SemiAnnualYield,
    /// Wire value `SHORTAVGLIFE`.
    YieldToShortestAverageLife,
    /// Wire value `SHORTEST`.
    YieldToShortestAverage,
    /// Wire value `SIMPLE`.
    SimpleYield,
    /// Wire value `TAXEQUIV`.
    TaxEquivalentYield,
    /// Wire value `TENDER`.
    YieldToTenderDate,
    /// Wire value `TRUE`.
    TrueYield,
    /// Wire value `WORST`.
    YieldToWorstConvention,
}

impl YieldType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::AfterTaxYield => "AFTERTAX",
            Self::AnnualYield => "ANNUAL",
            Self::YieldAtIssue => "ATISSUE",
            Self::YieldToAverageLife => "AVGLIFE",
            Self::YieldToAverageMaturity => "AVGMATURITY",
            Self::BookYield => "BOOK",
            Self::YieldToNextCall => "CALL",
            Self::YieldChangeSinceClose => "CHANGE",
            Self::ClosingYield => "CLOSE",
            Self::CompoundYield => "COMPOUND",
            Self::CurrentYield => "CURRENT",
            Self::GovernmentEquivalentYield => "GOVTEQUIV",
            Self::TrueGrossYield => "GROSS",
            Self::YieldWithInflationAssumption => "INFLATION",
            Self::InverseFloaterBondYield => "INVERSEFLOATER",
            Self::MostRecentClosingYield => "LASTCLOSE",
            Self::ClosingYieldMostRecentMonth => "LASTMONTH",
            Self::ClosingYieldMostRecentQuarter => "LASTQUARTER",
            Self::ClosingYieldMostRecentYear => "LASTYEAR",
            Self::YieldToLongestAverageLife => "LONGAVGLIFE",
            Self::YieldToLongestAverage => "LONGEST",
            Self::MarkToMarketYield => "MARK",
            Self::YieldToMaturity => "MATURITY",
            Self::YieldToNextRefund => "NEXTREFUND",
            Self::OpenAverageYield => "OPENAVG",
            Self::PreviousCloseYield => "PREVCLOSE",
            Self::ProceedsYield => "PROCEEDS",
            Self::YieldToNextPut => "PUT",
            Self::SemiAnnualYield => "SEMIANNUAL",
            Self::YieldToShortestAverageLife => "SHORTAVGLIFE",
            Self::YieldToShortestAverage => "SHORTEST",
            Self::SimpleYield => "SIMPLE",
            Self::TaxEquivalentYield => "TAXEQUIV",
            Self::YieldToTenderDate => "TENDER",
            Self::TrueYield => "TRUE",
            Self::YieldToWorstConvention => "WORST",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "AFTERTAX" => Some(Self::AfterTaxYield),
            "ANNUAL" => Some(Self::AnnualYield),
            "ATISSUE" => Some(Self::YieldAtIssue),
            "AVGLIFE" => Some(Self::YieldToAverageLife),
            "AVGMATURITY" => Some(Self::YieldToAverageMaturity),
            "BOOK" => Some(Self::BookYield),
            "CALL" => Some(Self::YieldToNextCall),
            "CHANGE" => Some(Self::YieldChangeSinceClose),
            "CLOSE" => Some(Self::ClosingYield),
            "COMPOUND" => Some(Self::CompoundYield),
            "CURRENT" => Some(Self::CurrentYield),
            "GOVTEQUIV" => Some(Self::GovernmentEquivalentYield),
            "GROSS" => Some(Self::TrueGrossYield),
            "INFLATION" => Some(Self::YieldWithInflationAssumption),
            "INVERSEFLOATER" => Some(Self::InverseFloaterBondYield),
            "LASTCLOSE" => Some(Self::MostRecentClosingYield),
            "LASTMONTH" => Some(Self::ClosingYieldMostRecentMonth),
            "LASTQUARTER" => Some(Self::ClosingYieldMostRecentQuarter),
            "LASTYEAR" => Some(Self::ClosingYieldMostRecentYear),
            "LONGAVGLIFE" => Some(Self::YieldToLongestAverageLife),
            "LONGEST" => Some(Self::YieldToLongestAverage),
            "MARK" => Some(Self::MarkToMarketYield),
            "MATURITY" => Some(Self::YieldToMaturity),
            "NEXTREFUND" => Some(Self::YieldToNextRefund),
            "OPENAVG" => Some(Self::OpenAverageYield),
            "PREVCLOSE" => Some(Self::PreviousCloseYield),
            "PROCEEDS" => Some(Self::ProceedsYield),
            "PUT" => Some(Self::YieldToNextPut),
            "SEMIANNUAL" => Some(Self::SemiAnnualYield),
            "SHORTAVGLIFE" => Some(Self::YieldToShortestAverageLife),
            "SHORTEST" => Some(Self::YieldToShortestAverage),
            "SIMPLE" => Some(Self::SimpleYield),
            "TAXEQUIV" => Some(Self::TaxEquivalentYield),
            "TENDER" => Some(Self::YieldToTenderDate),
            "TRUE" => Some(Self::TrueYield),
            "WORST" => Some(Self::YieldToWorstConvention),
            _ => None,
        }
    }
}

/// `SubscriptionRequestType` (tag 263, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionRequestType {
    /// Wire value `0`.
    Snapshot,
    /// Wire value `1`.
    SnapshotPlusUpdates,
    /// Wire value `2`.
    DisablePreviousSnapshotPlusUpdateRequest,
}

impl SubscriptionRequestType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Snapshot => '0',
            Self::SnapshotPlusUpdates => '1',
            Self::DisablePreviousSnapshotPlusUpdateRequest => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Snapshot),
            '1' => Some(Self::SnapshotPlusUpdates),
            '2' => Some(Self::DisablePreviousSnapshotPlusUpdateRequest),
            _ => None,
        }
    }
}

/// `MdUpdateType` (tag 265, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdUpdateType {
    /// Wire value `0`.
    FullRefresh,
    /// Wire value `1`.
    IncrementalRefresh,
}

impl MdUpdateType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::FullRefresh => 0,
            Self::IncrementalRefresh => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::FullRefresh),
            1 => Some(Self::IncrementalRefresh),
            _ => None,
        }
    }
}

/// `MdEntryType` (tag 269, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdEntryType {
    /// Wire value `0`.
    Bid,
    /// Wire value `1`.
    Offer,
    /// Wire value `2`.
    Trade,
    /// Wire value `3`.
    IndexValue,
    /// Wire value `4`.
    OpeningPrice,
    /// Wire value `5`.
    ClosingPrice,
    /// Wire value `6`.
    SettlementPrice,
    /// Wire value `7`.
    TradingSessionHighPrice,
    /// Wire value `8`.
    TradingSessionLowPrice,
    /// Wire value `9`.
    TradingSessionVwapPrice,
    /// Wire value `A`.
    Imbalance,
}

impl MdEntryType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Bid => '0',
            Self::Offer => '1',
            Self::Trade => '2',
            Self::IndexValue => '3',
            Self::OpeningPrice => '4',
            Self::ClosingPrice => '5',
            Self::SettlementPrice => '6',
            Self::TradingSessionHighPrice => '7',
            Self::TradingSessionLowPrice => '8',
            Self::TradingSessionVwapPrice => '9',
            Self::Imbalance => 'A',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Bid),
            '1' => Some(Self::Offer),
            '2' => Some(Self::Trade),
            '3' => Some(Self::IndexValue),
            '4' => Some(Self::OpeningPrice),
            '5' => Some(Self::ClosingPrice),
            '6' => Some(Self::SettlementPrice),
            '7' => Some(Self::TradingSessionHighPrice),
            '8' => Some(Self::TradingSessionLowPrice),
            '9' => Some(Self::TradingSessionVwapPrice),
            'A' => Some(Self::Imbalance),
            _ => None,
        }
    }
}

/// `TickDirection` (tag 274, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickDirection {
    /// Wire value `0`.
    PlusTick,
    /// Wire value `1`.
    ZeroPlusTick,
    /// Wire value `2`.
    MinusTick,
    /// Wire value `3`.
    ZeroMinusTick,
}

impl TickDirection {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::PlusTick => '0',
            Self::ZeroPlusTick => '1',
            Self::MinusTick => '2',
            Self::ZeroMinusTick => '3',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::PlusTick),
            '1' => Some(Self::ZeroPlusTick),
            '2' => Some(Self::MinusTick),
            '3' => Some(Self::ZeroMinusTick),
            _ => None,
        }
    }
}

/// `QuoteCondition` (tag 276, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteCondition {
    /// Wire value `A`.
    Open,
    /// Wire value `B`.
    Closed,
    /// Wire value `C`.
    ExchangeBest,
    /// Wire value `D`.
    ConsolidatedBest,
    /// Wire value `E`.
    Locked,
    /// Wire value `F`.
    Crossed,
    /// Wire value `G`.
    Depth,
    /// Wire value `H`.
    FastTrading,
    /// Wire value `I`.
    NonFirm,
}

impl QuoteCondition {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Open => "A",
            Self::Closed => "B",
            Self::ExchangeBest => "C",
            Self::ConsolidatedBest => "D",
            Self::Locked => "E",
            Self::Crossed => "F",
            Self::Depth => "G",
            Self::FastTrading => "H",
            Self::NonFirm => "I",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "A" => Some(Self::Open),
            "B" => Some(Self::Closed),
            "C" => Some(Self::ExchangeBest),
            "D" => Some(Self::ConsolidatedBest),
            "E" => Some(Self::Locked),
            "F" => Some(Self::Crossed),
            "G" => Some(Self::Depth),
            "H" => Some(Self::FastTrading),
            "I" => Some(Self::NonFirm),
            _ => None,
        }
    }
}

/// `TradeCondition` (tag 277, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeCondition {
    /// Wire value `A`.
    Cash,
    /// Wire value `B`.
    AveragePriceTrade,
    /// Wire value `C`.
    CashTrade,
    /// Wire value `D`.
    NextDay,
    /// Wire value `E`.
    Opening,
    /// Wire value `F`.
    IntradayTradeDetail,
    /// Wire value `G`.
    Rule127Trade,
    /// Wire value `H`.
    Rule155Trade,
    /// Wire value `I`.
    SoldLast,
    /// Wire value `J`.
    NextDayTrade,
    /// Wire value `K`.
    Opened,
    /// Wire value `L`.
    Seller,
    /// Wire value `M`.
    Sold,
    /// Wire value `N`.
    StoppedStock,
    /// Wire value `P`.
    ImbalanceMoreBuyers,
    /// Wire value `Q`.
    ImbalanceMoreSellers,
    /// Wire value `R`.
    OpeningPrice,
}

impl TradeCondition {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Cash => "A",
            Self::AveragePriceTrade => "B",
            Self::CashTrade => "C",
            Self::NextDay => "D",
            Self::Opening => "E",
            Self::IntradayTradeDetail => "F",
            Self::Rule127Trade => "G",
            Self::Rule155Trade => "H",
            Self::SoldLast => "I",
            Self::NextDayTrade => "J",
            Self::Opened => "K",
            Self::Seller => "L",
            Self::Sold => "M",
            Self::StoppedStock => "N",
            Self::ImbalanceMoreBuyers => "P",
            Self::ImbalanceMoreSellers => "Q",
            Self::OpeningPrice => "R",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "A" => Some(Self::Cash),
            "B" => Some(Self::AveragePriceTrade),
            "C" => Some(Self::CashTrade),
            "D" => Some(Self::NextDay),
            "E" => Some(Self::Opening),
            "F" => Some(Self::IntradayTradeDetail),
            "G" => Some(Self::Rule127Trade),
            "H" => Some(Self::Rule155Trade),
            "I" => Some(Self::SoldLast),
            "J" => Some(Self::NextDayTrade),
            "K" => Some(Self::Opened),
            "L" => Some(Self::Seller),
            "M" => Some(Self::Sold),
            "N" => Some(Self::StoppedStock),
            "P" => Some(Self::ImbalanceMoreBuyers),
            "Q" => Some(Self::ImbalanceMoreSellers),
            "R" => Some(Self::OpeningPrice),
            _ => None,
        }
    }
}

/// `MdUpdateAction` (tag 279, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdUpdateAction {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    Change,
    /// Wire value `2`.
    Delete,
}

impl MdUpdateAction {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::Change => '1',
            Self::Delete => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::Change),
            '2' => Some(Self::Delete),
            _ => None,
        }
    }
}

/// `MdReqRejReason` (tag 281, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdReqRejReason {
    /// Wire value `0`.
    UnknownSymbol,
    /// Wire value `1`.
    DuplicateMdreqid,
    /// Wire value `2`.
    InsufficientBandwidth,
    /// Wire value `3`.
    InsufficientPermissions,
    /// Wire value `4`.
    UnsupportedSubscriptionrequesttype,
    /// Wire value `5`.
    UnsupportedMarketdepth,
    /// Wire value `6`.
    UnsupportedMdupdatetype,
    /// Wire value `7`.
    UnsupportedAggregatedbook,
    /// Wire value `8`.
    UnsupportedMdentrytype,
    /// Wire value `9`.
    UnsupportedTradingsessionid,
    /// Wire value `A`.
    UnsupportedScope,
    /// Wire value `B`.
    UnsupportedOpenclosesettleflag,
    /// Wire value `C`.
    UnsupportedMdimplicitdelete,
}

impl MdReqRejReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::UnknownSymbol => '0',
            Self::DuplicateMdreqid => '1',
            Self::InsufficientBandwidth => '2',
            Self::InsufficientPermissions => '3',
            Self::UnsupportedSubscriptionrequesttype => '4',
            Self::UnsupportedMarketdepth => '5',
            Self::UnsupportedMdupdatetype => '6',
            Self::UnsupportedAggregatedbook => '7',
            Self::UnsupportedMdentrytype => '8',
            Self::UnsupportedTradingsessionid => '9',
            Self::UnsupportedScope => 'A',
            Self::UnsupportedOpenclosesettleflag => 'B',
            Self::UnsupportedMdimplicitdelete => 'C',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::UnknownSymbol),
            '1' => Some(Self::DuplicateMdreqid),
            '2' => Some(Self::InsufficientBandwidth),
            '3' => Some(Self::InsufficientPermissions),
            '4' => Some(Self::UnsupportedSubscriptionrequesttype),
            '5' => Some(Self::UnsupportedMarketdepth),
            '6' => Some(Self::UnsupportedMdupdatetype),
            '7' => Some(Self::UnsupportedAggregatedbook),
            '8' => Some(Self::UnsupportedMdentrytype),
            '9' => Some(Self::UnsupportedTradingsessionid),
            'A' => Some(Self::UnsupportedScope),
            'B' => Some(Self::UnsupportedOpenclosesettleflag),
            'C' => Some(Self::UnsupportedMdimplicitdelete),
            _ => None,
        }
    }
}

/// `DeleteReason` (tag 285, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteReason {
    /// Wire value `0`.
    Cancelation,
    /// Wire value `1`.
    Error,
}

impl DeleteReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Cancelation => '0',
            Self::Error => '1',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Cancelation),
            '1' => Some(Self::Error),
            _ => None,
        }
    }
}

/// `OpenCloseSettleFlag` (tag 286, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenCloseSettleFlag {
    /// Wire value `0`.
    DailyOpen,
    /// Wire value `1`.
    SessionOpen,
    /// Wire value `2`.
    DeliverySettlementPrice,
    /// Wire value `3`.
    ExpectedPrice,
    /// Wire value `4`.
    PriceFromPreviousBusinessDay,
}

impl OpenCloseSettleFlag {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::DailyOpen => "0",
            Self::SessionOpen => "1",
            Self::DeliverySettlementPrice => "2",
            Self::ExpectedPrice => "3",
            Self::PriceFromPreviousBusinessDay => "4",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "0" => Some(Self::DailyOpen),
            "1" => Some(Self::SessionOpen),
            "2" => Some(Self::DeliverySettlementPrice),
            "3" => Some(Self::ExpectedPrice),
            "4" => Some(Self::PriceFromPreviousBusinessDay),
            _ => None,
        }
    }
}

/// `FinancialStatus` (tag 291, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialStatus {
    /// Wire value `1`.
    Bankrupt,
    /// Wire value `2`.
    PendingDelisting,
}

impl FinancialStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Bankrupt => "1",
            Self::PendingDelisting => "2",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "1" => Some(Self::Bankrupt),
            "2" => Some(Self::PendingDelisting),
            _ => None,
        }
    }
}

/// `CorporateAction` (tag 292, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorporateAction {
    /// Wire value `A`.
    ExDividend,
    /// Wire value `B`.
    ExDistribution,
    /// Wire value `C`.
    ExRights,
    /// Wire value `D`.
    New,
    /// Wire value `E`.
    ExInterest,
}

impl CorporateAction {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::ExDividend => "A",
            Self::ExDistribution => "B",
            Self::ExRights => "C",
            Self::New => "D",
            Self::ExInterest => "E",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "A" => Some(Self::ExDividend),
            "B" => Some(Self::ExDistribution),
            "C" => Some(Self::ExRights),
            "D" => Some(Self::New),
            "E" => Some(Self::ExInterest),
            _ => None,
        }
    }
}

/// `QuoteStatus` (tag 297, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteStatus {
    /// Wire value `0`.
    Accepted,
    /// Wire value `1`.
    CanceledForSymbol,
    /// Wire value `2`.
    CanceledForSecurityType,
    /// Wire value `3`.
    CanceledForUnderlying,
    /// Wire value `4`.
    CanceledAll,
    /// Wire value `5`.
    Rejected,
    /// Wire value `6`.
    RemovedFromMarket,
    /// Wire value `7`.
    Expired,
    /// Wire value `8`.
    Query,
    /// Wire value `9`.
    QuoteNotFound,
    /// Wire value `10`.
    Pending,
}

impl QuoteStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Accepted => 0,
            Self::CanceledForSymbol => 1,
            Self::CanceledForSecurityType => 2,
            Self::CanceledForUnderlying => 3,
            Self::CanceledAll => 4,
            Self::Rejected => 5,
            Self::RemovedFromMarket => 6,
            Self::Expired => 7,
            Self::Query => 8,
            Self::QuoteNotFound => 9,
            Self::Pending => 10,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Accepted),
            1 => Some(Self::CanceledForSymbol),
            2 => Some(Self::CanceledForSecurityType),
            3 => Some(Self::CanceledForUnderlying),
            4 => Some(Self::CanceledAll),
            5 => Some(Self::Rejected),
            6 => Some(Self::RemovedFromMarket),
            7 => Some(Self::Expired),
            8 => Some(Self::Query),
            9 => Some(Self::QuoteNotFound),
            10 => Some(Self::Pending),
            _ => None,
        }
    }
}

/// `QuoteRequestType` (tag 303, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteRequestType {
    /// Wire value `1`.
    Manual,
    /// Wire value `2`.
    Automatic,
}

impl QuoteRequestType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Manual => 1,
            Self::Automatic => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Manual),
            2 => Some(Self::Automatic),
            _ => None,
        }
    }
}

/// `SecurityRequestType` (tag 321, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityRequestType {
    /// Wire value `0`.
    RequestSecurityIdentityAndSpecifications,
    /// Wire value `1`.
    RequestSecurityIdentityForTheSpecificationsProvided,
    /// Wire value `2`.
    RequestListSecurityTypes,
    /// Wire value `3`.
    RequestListSecurities,
}

impl SecurityRequestType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::RequestSecurityIdentityAndSpecifications => 0,
            Self::RequestSecurityIdentityForTheSpecificationsProvided => 1,
            Self::RequestListSecurityTypes => 2,
            Self::RequestListSecurities => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::RequestSecurityIdentityAndSpecifications),
            1 => Some(Self::RequestSecurityIdentityForTheSpecificationsProvided),
            2 => Some(Self::RequestListSecurityTypes),
            3 => Some(Self::RequestListSecurities),
            _ => None,
        }
    }
}

/// `SecurityResponseType` (tag 323, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityResponseType {
    /// Wire value `1`.
    AcceptSecurityProposalAsIs,
    /// Wire value `2`.
    AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage,
    /// Wire value `3`.
    ListOfSecurityTypesReturnedPerRequest,
    /// Wire value `4`.
    ListOfSecuritiesReturnedPerRequest,
    /// Wire value `5`.
    RejectSecurityProposal,
    /// Wire value `6`.
    CanNotMatchSelectionCriteria,
}

impl SecurityResponseType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::AcceptSecurityProposalAsIs => 1,
            Self::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage => 2,
            Self::ListOfSecurityTypesReturnedPerRequest => 3,
            Self::ListOfSecuritiesReturnedPerRequest => 4,
            Self::RejectSecurityProposal => 5,
            Self::CanNotMatchSelectionCriteria => 6,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::AcceptSecurityProposalAsIs),
            2 => Some(Self::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage),
            3 => Some(Self::ListOfSecurityTypesReturnedPerRequest),
            4 => Some(Self::ListOfSecuritiesReturnedPerRequest),
            5 => Some(Self::RejectSecurityProposal),
            6 => Some(Self::CanNotMatchSelectionCriteria),
            _ => None,
        }
    }
}

/// `SecurityTradingStatus` (tag 326, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityTradingStatus {
    /// Wire value `1`.
    OpeningDelay,
    /// Wire value `2`.
    TradingHalt,
    /// Wire value `3`.
    Resume,
    /// Wire value `4`.
    NoOpen,
    /// Wire value `5`.
    PriceIndication,
    /// Wire value `6`.
    TradingRangeIndication,
    /// Wire value `7`.
    MarketImbalanceBuy,
    /// Wire value `8`.
    MarketImbalanceSell,
    /// Wire value `9`.
    MarketOnCloseImbalanceBuy,
    /// Wire value `10`.
    MarketOnCloseImbalanceSell,
    /// Wire value `11`.
    NotAssigned,
    /// Wire value `12`.
    NoMarketImbalance,
    /// Wire value `13`.
    NoMarketOnCloseImbalance,
    /// Wire value `14`.
    ItsPreOpening,
    /// Wire value `15`.
    NewPriceIndication,
    /// Wire value `16`.
    TradeDisseminationTime,
    /// Wire value `17`.
    ReadyToTrade,
    /// Wire value `18`.
    NotAvailableForTrading,
    /// Wire value `19`.
    NotTradedOnThisMarket,
    /// Wire value `20`.
    UnknownOrInvalid,
    /// Wire value `21`.
    PreOpen,
    /// Wire value `22`.
    OpeningRotation,
    /// Wire value `23`.
    FastMarket,
}

impl SecurityTradingStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::OpeningDelay => 1,
            Self::TradingHalt => 2,
            Self::Resume => 3,
            Self::NoOpen => 4,
            Self::PriceIndication => 5,
            Self::TradingRangeIndication => 6,
            Self::MarketImbalanceBuy => 7,
            Self::MarketImbalanceSell => 8,
            Self::MarketOnCloseImbalanceBuy => 9,
            Self::MarketOnCloseImbalanceSell => 10,
            Self::NotAssigned => 11,
            Self::NoMarketImbalance => 12,
            Self::NoMarketOnCloseImbalance => 13,
            Self::ItsPreOpening => 14,
            Self::NewPriceIndication => 15,
            Self::TradeDisseminationTime => 16,
            Self::ReadyToTrade => 17,
            Self::NotAvailableForTrading => 18,
            Self::NotTradedOnThisMarket => 19,
            Self::UnknownOrInvalid => 20,
            Self::PreOpen => 21,
            Self::OpeningRotation => 22,
            Self::FastMarket => 23,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::OpeningDelay),
            2 => Some(Self::TradingHalt),
            3 => Some(Self::Resume),
            4 => Some(Self::NoOpen),
            5 => Some(Self::PriceIndication),
            6 => Some(Self::TradingRangeIndication),
            7 => Some(Self::MarketImbalanceBuy),
            8 => Some(Self::MarketImbalanceSell),
            9 => Some(Self::MarketOnCloseImbalanceBuy),
            10 => Some(Self::MarketOnCloseImbalanceSell),
            11 => Some(Self::NotAssigned),
            12 => Some(Self::NoMarketImbalance),
            13 => Some(Self::NoMarketOnCloseImbalance),
            14 => Some(Self::ItsPreOpening),
            15 => Some(Self::NewPriceIndication),
            16 => Some(Self::TradeDisseminationTime),
            17 => Some(Self::ReadyToTrade),
            18 => Some(Self::NotAvailableForTrading),
            19 => Some(Self::NotTradedOnThisMarket),
            20 => Some(Self::UnknownOrInvalid),
            21 => Some(Self::PreOpen),
            22 => Some(Self::OpeningRotation),
            23 => Some(Self::FastMarket),
            _ => None,
        }
    }
}

/// `HaltReason` (tag 327, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltReason {
    /// Wire value `D`.
    NewsDissemination,
    /// Wire value `E`.
    OrderInflux,
    /// Wire value `I`.
    OrderImbalance,
    /// Wire value `M`.
    AdditionalInformation,
    /// Wire value `P`.
    NewsPending,
    /// Wire value `X`.
    EquipmentChangeover,
}

impl HaltReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::NewsDissemination => 'D',
            Self::OrderInflux => 'E',
            Self::OrderImbalance => 'I',
            Self::AdditionalInformation => 'M',
            Self::NewsPending => 'P',
            Self::EquipmentChangeover => 'X',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'D' => Some(Self::NewsDissemination),
            'E' => Some(Self::OrderInflux),
            'I' => Some(Self::OrderImbalance),
            'M' => Some(Self::AdditionalInformation),
            'P' => Some(Self::NewsPending),
            'X' => Some(Self::EquipmentChangeover),
            _ => None,
        }
    }
}

/// `Adjustment` (tag 334, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjustment {
    /// Wire value `1`.
    Cancel,
    /// Wire value `2`.
    Error,
    /// Wire value `3`.
    Correction,
}

impl Adjustment {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Cancel => 1,
            Self::Error => 2,
            Self::Correction => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Cancel),
            2 => Some(Self::Error),
            3 => Some(Self::Correction),
            _ => None,
        }
    }
}

/// `TradSesMethod` (tag 338, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesMethod {
    /// Wire value `1`.
    Electronic,
    /// Wire value `2`.
    OpenOutcry,
    /// Wire value `3`.
    TwoParty,
}

impl TradSesMethod {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Electronic => 1,
            Self::OpenOutcry => 2,
            Self::TwoParty => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Electronic),
            2 => Some(Self::OpenOutcry),
            3 => Some(Self::TwoParty),
            _ => None,
        }
    }
}

/// `TradSesMode` (tag 339, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesMode {
    /// Wire value `1`.
    Testing,
    /// Wire value `2`.
    Simulated,
    /// Wire value `3`.
    Production,
}

impl TradSesMode {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Testing => 1,
            Self::Simulated => 2,
            Self::Production => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Testing),
            2 => Some(Self::Simulated),
            3 => Some(Self::Production),
            _ => None,
        }
    }
}

/// `TradSesStatus` (tag 340, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesStatus {
    /// Wire value `0`.
    Unknown,
    /// Wire value `1`.
    Halted,
    /// Wire value `2`.
    Open,
    /// Wire value `3`.
    Closed,
    /// Wire value `4`.
    PreOpen,
    /// Wire value `5`.
    PreClose,
    /// Wire value `6`.
    RequestRejected,
}

impl TradSesStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Halted => 1,
            Self::Open => 2,
            Self::Closed => 3,
            Self::PreOpen => 4,
            Self::PreClose => 5,
            Self::RequestRejected => 6,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Unknown),
            1 => Some(Self::Halted),
            2 => Some(Self::Open),
            3 => Some(Self::Closed),
            4 => Some(Self::PreOpen),
            5 => Some(Self::PreClose),
            6 => Some(Self::RequestRejected),
            _ => None,
        }
    }
}

/// `QuoteEntryRejectReason` (tag 368, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteEntryRejectReason {
    /// Wire value `1`.
    UnknownSymbol,
    /// Wire value `2`.
    Exchange,
    /// Wire value `3`.
    QuoteExceedsLimit,
    /// Wire value `4`.
    TooLateToEnter,
    /// Wire value `5`.
    UnknownQuote,
    /// Wire value `6`.
    DuplicateQuote,
    /// Wire value `7`.
    InvalidBid,
    /// Wire value `8`.
    InvalidPrice,
    /// Wire value `9`.
    NotAuthorizedToQuoteSecurity,
}

impl QuoteEntryRejectReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::UnknownSymbol => 1,
            Self::Exchange => 2,
            Self::QuoteExceedsLimit => 3,
            Self::TooLateToEnter => 4,
            Self::UnknownQuote => 5,
            Self::DuplicateQuote => 6,
            Self::InvalidBid => 7,
            Self::InvalidPrice => 8,
            Self::NotAuthorizedToQuoteSecurity => 9,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::UnknownSymbol),
            2 => Some(Self::Exchange),
            3 => Some(Self::QuoteExceedsLimit),
            4 => Some(Self::TooLateToEnter),
            5 => Some(Self::UnknownQuote),
            6 => Some(Self::DuplicateQuote),
            7 => Some(Self::InvalidBid),
            8 => Some(Self::InvalidPrice),
            9 => Some(Self::NotAuthorizedToQuoteSecurity),
            _ => None,
        }
    }
}

/// `SessionRejectReason` (tag 373, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRejectReason {
    /// Wire value `0`.
    InvalidTagNumber,
    /// Wire value `1`.
    RequiredTagMissing,
    /// Wire value `2`.
    TagNotDefinedForThisMessageType,
    /// Wire value `3`.
    UndefinedTag,
    /// Wire value `4`.
    TagSpecifiedWithoutAValue,
    /// Wire value `5`.
    ValueIsIncorrect,
    /// Wire value `6`.
    IncorrectDataFormatForValue,
    /// Wire value `7`.
    DecryptionProblem,
    /// Wire value `8`.
    SignatureProblem,
    /// Wire value `9`.
    CompidProblem,
    /// Wire value `10`.
    SendingtimeAccuracyProblem,
    /// Wire value `11`.
    InvalidMsgtype,
    /// Wire value `12`.
    XmlValidationError,
    /// Wire value `13`.
    TagAppearsMoreThanOnce,
    /// Wire value `14`.
    TagSpecifiedOutOfRequiredOrder,
    /// Wire value `15`.
    RepeatingGroupFieldsOutOfOrder,
    /// Wire value `16`.
    IncorrectNumingroupCountForRepeatingGroup,
    /// Wire value `17`.
    NonDataValueIncludesFieldDelimiter,
}

impl SessionRejectReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::InvalidTagNumber => 0,
            Self::RequiredTagMissing => 1,
            Self::TagNotDefinedForThisMessageType => 2,
            Self::UndefinedTag => 3,
            Self::TagSpecifiedWithoutAValue => 4,
            Self::ValueIsIncorrect => 5,
            Self::IncorrectDataFormatForValue => 6,
            Self::DecryptionProblem => 7,
            Self::SignatureProblem => 8,
            Self::CompidProblem => 9,
            Self::SendingtimeAccuracyProblem => 10,
            Self::InvalidMsgtype => 11,
            Self::XmlValidationError => 12,
            Self::TagAppearsMoreThanOnce => 13,
            Self::TagSpecifiedOutOfRequiredOrder => 14,
            Self::RepeatingGroupFieldsOutOfOrder => 15,
            Self::IncorrectNumingroupCountForRepeatingGroup => 16,
            Self::NonDataValueIncludesFieldDelimiter => 17,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::InvalidTagNumber),
            1 => Some(Self::RequiredTagMissing),
            2 => Some(Self::TagNotDefinedForThisMessageType),
            3 => Some(Self::UndefinedTag),
            4 => Some(Self::TagSpecifiedWithoutAValue),
            5 => Some(Self::ValueIsIncorrect),
            6 => Some(Self::IncorrectDataFormatForValue),
            7 => Some(Self::DecryptionProblem),
            8 => Some(Self::SignatureProblem),
            9 => Some(Self::CompidProblem),
            10 => Some(Self::SendingtimeAccuracyProblem),
            11 => Some(Self::InvalidMsgtype),
            12 => Some(Self::XmlValidationError),
            13 => Some(Self::TagAppearsMoreThanOnce),
            14 => Some(Self::TagSpecifiedOutOfRequiredOrder),
            15 => Some(Self::RepeatingGroupFieldsOutOfOrder),
            16 => Some(Self::IncorrectNumingroupCountForRepeatingGroup),
            17 => Some(Self::NonDataValueIncludesFieldDelimiter),
            _ => None,
        }
    }
}

/// `BidRequestTransType` (tag 374, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidRequestTransType {
    /// Wire value `C`.
    Cancel,
    /// Wire value `N`.
    New,
}

impl BidRequestTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Cancel => 'C',
            Self::New => 'N',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'C' => Some(Self::Cancel),
            'N' => Some(Self::New),
            _ => None,
        }
    }
}

/// `ExecRestatementReason` (tag 378, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecRestatementReason {
    /// Wire value `0`.
    GtCorporateAction,
    /// Wire value `1`.
    GtRenewal,
    /// Wire value `2`.
    VerbalChange,
    /// Wire value `3`.
    RepricingOfOrder,
    /// Wire value `4`.
    BrokerOption,
    /// Wire value `5`.
    PartialDeclineOfOrderqty,
    /// Wire value `6`.
    CancelOnTradingHalt,
    /// Wire value `7`.
    CancelOnSystemFailure,
    /// Wire value `8`.
    Market,
}

impl ExecRestatementReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::GtCorporateAction => 0,
            Self::GtRenewal => 1,
            Self::VerbalChange => 2,
            Self::RepricingOfOrder => 3,
            Self::BrokerOption => 4,
            Self::PartialDeclineOfOrderqty => 5,
            Self::CancelOnTradingHalt => 6,
            Self::CancelOnSystemFailure => 7,
            Self::Market => 8,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::GtCorporateAction),
            1 => Some(Self::GtRenewal),
            2 => Some(Self::VerbalChange),
            3 => Some(Self::RepricingOfOrder),
            4 => Some(Self::BrokerOption),
            5 => Some(Self::PartialDeclineOfOrderqty),
            6 => Some(Self::CancelOnTradingHalt),
            7 => Some(Self::CancelOnSystemFailure),
            8 => Some(Self::Market),
            _ => None,
        }
    }
}

/// `BusinessRejectReason` (tag 380, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessRejectReason {
    /// Wire value `0`.
    Other,
    /// Wire value `1`.
    UnkownId,
    /// Wire value `2`.
    UnknownSecurity,
    /// Wire value `3`.
    UnsupportedMessageType,
    /// Wire value `4`.
    ApplicationNotAvailable,
    /// Wire value `5`.
    ConditionallyRequiredFieldMissing,
    /// Wire value `6`.
    NotAuthorized,
    /// Wire value `7`.
    DelivertoFirmNotAvailableAtThisTime,
}

impl BusinessRejectReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Other => 0,
            Self::UnkownId => 1,
            Self::UnknownSecurity => 2,
            Self::UnsupportedMessageType => 3,
            Self::ApplicationNotAvailable => 4,
            Self::ConditionallyRequiredFieldMissing => 5,
            Self::NotAuthorized => 6,
            Self::DelivertoFirmNotAvailableAtThisTime => 7,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Other),
            1 => Some(Self::UnkownId),
            2 => Some(Self::UnknownSecurity),
            3 => Some(Self::UnsupportedMessageType),
            4 => Some(Self::ApplicationNotAvailable),
            5 => Some(Self::ConditionallyRequiredFieldMissing),
            6 => Some(Self::NotAuthorized),
            7 => Some(Self::DelivertoFirmNotAvailableAtThisTime),
            _ => None,
        }
    }
}

/// `MsgDirection` (tag 385, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgDirection {
    /// Wire value `R`.
    Receive,
    /// Wire value `S`.
    Send,
}

impl MsgDirection {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Receive => 'R',
            Self::Send => 'S',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'R' => Some(Self::Receive),
            'S' => Some(Self::Send),
            _ => None,
        }
    }
}

/// `DiscretionInst` (tag 388, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscretionInst {
    /// Wire value `0`.
    RelatedToDisplayedPrice,
    /// Wire value `1`.
    RelatedToMarketPrice,
    /// Wire value `2`.
    RelatedToPrimaryPrice,
    /// Wire value `3`.
    RelatedToLocalPrimaryPrice,
    /// Wire value `4`.
    RelatedToMidpointPrice,
    /// Wire value `5`.
    RelatedToLastTradePrice,
}

impl DiscretionInst {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::RelatedToDisplayedPrice => '0',
            Self::RelatedToMarketPrice => '1',
            Self::RelatedToPrimaryPrice => '2',
            Self::RelatedToLocalPrimaryPrice => '3',
            Self::RelatedToMidpointPrice => '4',
            Self::RelatedToLastTradePrice => '5',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::RelatedToDisplayedPrice),
            '1' => Some(Self::RelatedToMarketPrice),
            '2' => Some(Self::RelatedToPrimaryPrice),
            '3' => Some(Self::RelatedToLocalPrimaryPrice),
            '4' => Some(Self::RelatedToMidpointPrice),
            '5' => Some(Self::RelatedToLastTradePrice),
            _ => None,
        }
    }
}

/// `BidType` (tag 394, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidType {
    /// Wire value `1`.
    NonDisclosedStyle,
    /// Wire value `2`.
    DisclosedStyle,
    /// Wire value `3`.
    NoBiddingProcess,
}

impl BidType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::NonDisclosedStyle => 1,
            Self::DisclosedStyle => 2,
            Self::NoBiddingProcess => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::NonDisclosedStyle),
            2 => Some(Self::DisclosedStyle),
            3 => Some(Self::NoBiddingProcess),
            _ => None,
        }
    }
}

/// `BidDescriptorType` (tag 399, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidDescriptorType {
    /// Wire value `1`.
    Sector,
    /// Wire value `2`.
    Country,
    /// Wire value `3`.
    Index,
}

impl BidDescriptorType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Sector => 1,
            Self::Country => 2,
            Self::Index => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Sector),
            2 => Some(Self::Country),
            3 => Some(Self::Index),
            _ => None,
        }
    }
}

/// `SideValueInd` (tag 401, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideValueInd {
    /// Wire value `1`.
    Sidevalue1,
    /// Wire value `2`.
    Sidevalue2,
}

impl SideValueInd {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Sidevalue1 => 1,
            Self::Sidevalue2 => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Sidevalue1),
            2 => Some(Self::Sidevalue2),
            _ => None,
        }
    }
}

/// `LiquidityIndType` (tag 409, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidityIndType {
    /// Wire value `1`.
    FivedayMovingAverage,
    /// Wire value `2`.
    TwentydayMovingAverage,
    /// Wire value `3`.
    NormalMarketSize,
    /// Wire value `4`.
    Other,
}

impl LiquidityIndType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::FivedayMovingAverage => 1,
            Self::TwentydayMovingAverage => 2,
            Self::NormalMarketSize => 3,
            Self::Other => 4,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::FivedayMovingAverage),
            2 => Some(Self::TwentydayMovingAverage),
            3 => Some(Self::NormalMarketSize),
            4 => Some(Self::Other),
            _ => None,
        }
    }
}

/// `ProgRptReqs` (tag 414, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgRptReqs {
    /// Wire value `1`.
    BuysideExplicitlyRequestsStatusUsingStatusrequest,
    /// Wire value `2`.
    SellsidePeriodicallySendsStatusUsingListstatus,
    /// Wire value `3`.
    RealTimeExecutionReports,
}

impl ProgRptReqs {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::BuysideExplicitlyRequestsStatusUsingStatusrequest => 1,
            Self::SellsidePeriodicallySendsStatusUsingListstatus => 2,
            Self::RealTimeExecutionReports => 3,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::BuysideExplicitlyRequestsStatusUsingStatusrequest),
            2 => Some(Self::SellsidePeriodicallySendsStatusUsingListstatus),
            3 => Some(Self::RealTimeExecutionReports),
            _ => None,
        }
    }
}

/// `IncTaxInd` (tag 416, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncTaxInd {
    /// Wire value `1`.
    Net,
    /// Wire value `2`.
    Gross,
}

impl IncTaxInd {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Net => 1,
            Self::Gross => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Net),
            2 => Some(Self::Gross),
            _ => None,
        }
    }
}

/// `BasisPxType` (tag 419, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasisPxType {
    /// Wire value `2`.
    ClosingPriceAtMorningSession,
    /// Wire value `3`.
    ClosingPrice,
    /// Wire value `4`.
    CurrentPrice,
    /// Wire value `5`.
    Sq,
    /// Wire value `6`.
    VwapThroughADay,
    /// Wire value `7`.
    VwapThroughAMorningSession,
    /// Wire value `8`.
    VwapThroughAnAfternoonSession,
    /// Wire value `9`.
    VwapThroughADayExceptYori,
    /// Wire value `A`.
    VwapThroughAMorningSessionExceptYori,
    /// Wire value `B`.
    VwapThroughAnAfternoonSessionExceptYori,
    /// Wire value `C`.
    Strike,
    /// Wire value `D`.
    Open,
    /// Wire value `Z`.
    Others,
}

impl BasisPxType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::ClosingPriceAtMorningSession => '2',
            Self::ClosingPrice => '3',
            Self::CurrentPrice => '4',
            Self::Sq => '5',
            Self::VwapThroughADay => '6',
            Self::VwapThroughAMorningSession => '7',
            Self::VwapThroughAnAfternoonSession => '8',
            Self::VwapThroughADayExceptYori => '9',
            Self::VwapThroughAMorningSessionExceptYori => 'A',
            Self::VwapThroughAnAfternoonSessionExceptYori => 'B',
            Self::Strike => 'C',
            Self::Open => 'D',
            Self::Others => 'Z',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '2' => Some(Self::ClosingPriceAtMorningSession),
            '3' => Some(Self::ClosingPrice),
            '4' => Some(Self::CurrentPrice),
            '5' => Some(Self::Sq),
            '6' => Some(Self::VwapThroughADay),
            '7' => Some(Self::VwapThroughAMorningSession),
            '8' => Some(Self::VwapThroughAnAfternoonSession),
            '9' => Some(Self::VwapThroughADayExceptYori),
            'A' => Some(Self::VwapThroughAMorningSessionExceptYori),
            'B' => Some(Self::VwapThroughAnAfternoonSessionExceptYori),
            'C' => Some(Self::Strike),
            'D' => Some(Self::Open),
            'Z' => Some(Self::Others),
            _ => None,
        }
    }
}

/// `PriceType` (tag 423, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceType {
    /// Wire value `1`.
    Percentage,
    /// Wire value `2`.
    PerShare,
    /// Wire value `3`.
    FixedAmount,
    /// Wire value `4`.
    Discount,
    /// Wire value `5`.
    Premium,
    /// Wire value `6`.
    BasisPointsRelativeToBenchmark,
    /// Wire value `7`.
    TedPrice,
    /// Wire value `8`.
    TedYield,
}

impl PriceType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Percentage => 1,
            Self::PerShare => 2,
            Self::FixedAmount => 3,
            Self::Discount => 4,
            Self::Premium => 5,
            Self::BasisPointsRelativeToBenchmark => 6,
            Self::TedPrice => 7,
            Self::TedYield => 8,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Percentage),
            2 => Some(Self::PerShare),
            3 => Some(Self::FixedAmount),
            4 => Some(Self::Discount),
            5 => Some(Self::Premium),
            6 => Some(Self::BasisPointsRelativeToBenchmark),
            7 => Some(Self::TedPrice),
            8 => Some(Self::TedYield),
            _ => None,
        }
    }
}

/// `GtBookingInst` (tag 427, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtBookingInst {
    /// Wire value `0`.
    BookOutAllTradesOnDayOfExecution,
    /// Wire value `1`.
    AccumulateExecutionsUntilOrderIsFilledOrExpires,
    /// Wire value `2`.
    AccumulateUntilVerballyNotifiedOtherwise,
}

impl GtBookingInst {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::BookOutAllTradesOnDayOfExecution => 0,
            Self::AccumulateExecutionsUntilOrderIsFilledOrExpires => 1,
            Self::AccumulateUntilVerballyNotifiedOtherwise => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::BookOutAllTradesOnDayOfExecution),
            1 => Some(Self::AccumulateExecutionsUntilOrderIsFilledOrExpires),
            2 => Some(Self::AccumulateUntilVerballyNotifiedOtherwise),
            _ => None,
        }
    }
}

/// `ListStatusType` (tag 429, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListStatusType {
    /// Wire value `1`.
    Ack,
    /// Wire value `2`.
    Response,
    /// Wire value `3`.
    Timed,
    /// Wire value `4`.
    Execstarted,
    /// Wire value `5`.
    Alldone,
    /// Wire value `6`.
    Alert,
}

impl ListStatusType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Ack => 1,
            Self::Response => 2,
            Self::Timed => 3,
            Self::Execstarted => 4,
            Self::Alldone => 5,
            Self::Alert => 6,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Ack),
            2 => Some(Self::Response),
            3 => Some(Self::Timed),
            4 => Some(Self::Execstarted),
            5 => Some(Self::Alldone),
            6 => Some(Self::Alert),
            _ => None,
        }
    }
}

/// `NetGrossInd` (tag 430, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetGrossInd {
    /// Wire value `1`.
    Net,
    /// Wire value `2`.
    Gross,
}

impl NetGrossInd {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Net => 1,
            Self::Gross => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Net),
            2 => Some(Self::Gross),
            _ => None,
        }
    }
}

/// `ListOrderStatus` (tag 431, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListOrderStatus {
    /// Wire value `1`.
    Inbiddingprocess,
    /// Wire value `2`.
    Receivedforexecution,
    /// Wire value `3`.
    Executing,
    /// Wire value `4`.
    Canceling,
    /// Wire value `5`.
    Alert,
    /// Wire value `6`.
    AllDone,
    /// Wire value `7`.
    Reject,
}

impl ListOrderStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Inbiddingprocess => 1,
            Self::Receivedforexecution => 2,
            Self::Executing => 3,
            Self::Canceling => 4,
            Self::Alert => 5,
            Self::AllDone => 6,
            Self::Reject => 7,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Inbiddingprocess),
            2 => Some(Self::Receivedforexecution),
            3 => Some(Self::Executing),
            4 => Some(Self::Canceling),
            5 => Some(Self::Alert),
            6 => Some(Self::AllDone),
            7 => Some(Self::Reject),
            _ => None,
        }
    }
}

/// `ListExecInstType` (tag 433, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListExecInstType {
    /// Wire value `1`.
    Immediate,
    /// Wire value `2`.
    WaitForExecuteInstruction,
    /// Wire value `3`.
    ExchangeSwitchCivOrderSellDriven,
    /// Wire value `4`.
    ExchangeSwitchCivOrderBuyDrivenCashTopUp,
    /// Wire value `5`.
    ExchangeSwitchCivOrderBuyDrivenCashWithdraw,
}

impl ListExecInstType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Immediate => '1',
            Self::WaitForExecuteInstruction => '2',
            Self::ExchangeSwitchCivOrderSellDriven => '3',
            Self::ExchangeSwitchCivOrderBuyDrivenCashTopUp => '4',
            Self::ExchangeSwitchCivOrderBuyDrivenCashWithdraw => '5',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Immediate),
            '2' => Some(Self::WaitForExecuteInstruction),
            '3' => Some(Self::ExchangeSwitchCivOrderSellDriven),
            '4' => Some(Self::ExchangeSwitchCivOrderBuyDrivenCashTopUp),
            '5' => Some(Self::ExchangeSwitchCivOrderBuyDrivenCashWithdraw),
            _ => None,
        }
    }
}

/// `CxlRejResponseTo` (tag 434, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CxlRejResponseTo {
    /// Wire value `1`.
    OrderCancelRequest,
    /// Wire value `2`.
    OrderCancelReplaceRequest,
}

impl CxlRejResponseTo {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::OrderCancelRequest => '1',
            Self::OrderCancelReplaceRequest => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::OrderCancelRequest),
            '2' => Some(Self::OrderCancelReplaceRequest),
            _ => None,
        }
    }
}

/// `PartyIdSource` (tag 447, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyIdSource {
    /// Wire value `1`.
    KoreanInvestorId,
    /// Wire value `2`.
    TaiwaneseQualifiedForeignInvestorIdQfii,
    /// Wire value `3`.
    TaiwaneseTradingAccount,
    /// Wire value `4`.
    MalaysianCentralDepository,
    /// Wire value `5`.
    ChineseBShare,
    /// Wire value `6`.
    UkNationalInsuranceOrPensionNumber,
    /// Wire value `7`.
    UsSocialSecurityNumber,
    /// Wire value `8`.
    UsEmployerIdentificationNumber,
    /// Wire value `9`.
    AustralianBusinessNumber,
    /// Wire value `A`.
    AustralianTaxFileNumber,
    /// Wire value `B`.
    Bic,
    /// Wire value `C`.
    GenerallyAcceptedMarketParticipantIdentifier,
    /// Wire value `D`.
    Proprietary,
    /// Wire value `E`.
    IsoCountryCode,
    /// Wire value `F`.
    SettlementEntityLocation,
}

impl PartyIdSource {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::KoreanInvestorId => '1',
            Self::TaiwaneseQualifiedForeignInvestorIdQfii => '2',
            Self::TaiwaneseTradingAccount => '3',
            Self::MalaysianCentralDepository => '4',
            Self::ChineseBShare => '5',
            Self::UkNationalInsuranceOrPensionNumber => '6',
            Self::UsSocialSecurityNumber => '7',
            Self::UsEmployerIdentificationNumber => '8',
            Self::AustralianBusinessNumber => '9',
            Self::AustralianTaxFileNumber => 'A',
            Self::Bic => 'B',
            Self::GenerallyAcceptedMarketParticipantIdentifier => 'C',
            Self::Proprietary => 'D',
            Self::IsoCountryCode => 'E',
            Self::SettlementEntityLocation => 'F',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::KoreanInvestorId),
            '2' => Some(Self::TaiwaneseQualifiedForeignInvestorIdQfii),
            '3' => Some(Self::TaiwaneseTradingAccount),
            '4' => Some(Self::MalaysianCentralDepository),
            '5' => Some(Self::ChineseBShare),
            '6' => Some(Self::UkNationalInsuranceOrPensionNumber),
            '7' => Some(Self::UsSocialSecurityNumber),
            '8' => Some(Self::UsEmployerIdentificationNumber),
            '9' => Some(Self::AustralianBusinessNumber),
            'A' => Some(Self::AustralianTaxFileNumber),
            'B' => Some(Self::Bic),
            'C' => Some(Self::GenerallyAcceptedMarketParticipantIdentifier),
            'D' => Some(Self::Proprietary),
            'E' => Some(Self::IsoCountryCode),
            'F' => Some(Self::SettlementEntityLocation),
            _ => None,
        }
    }
}

/// `PartyRole` (tag 452, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyRole {
    /// Wire value `1`.
    ExecutingFirm,
    /// Wire value `2`.
    BrokerOfCredit,
    /// Wire value `3`.
    ClientId,
    /// Wire value `4`.
    ClearingFirm,
    /// Wire value `5`.
    InvestorId,
    /// Wire value `6`.
    IntroducingFirm,
    /// Wire value `7`.
    EnteringFirm,
    /// Wire value `8`.
    Locate,
    /// Wire value `9`.
    FundManagerClientId,
    /// Wire value `10`.
    SettlementLocation,
    /// Wire value `11`.
    OrderOriginationTrader,
    /// Wire value `12`.
    ExecutingTrader,
    /// Wire value `13`.
    OrderOriginationFirm,
    /// Wire value `14`.
    GiveupClearingFirm,
    /// Wire value `15`.
    CorrespondantClearingFirm,
    /// Wire value `16`.
    ExecutingSystem,
    /// Wire value `17`.
    ContraFirm,
    /// Wire value `18`.
    ContraClearingFirm,
    /// Wire value `19`.
    SponsoringFirm,
    /// Wire value `20`.
    UnderlyingContraFirm,
}

impl PartyRole {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::ExecutingFirm => 1,
            Self::BrokerOfCredit => 2,
            Self::ClientId => 3,
            Self::ClearingFirm => 4,
            Self::InvestorId => 5,
            Self::IntroducingFirm => 6,
            Self::EnteringFirm => 7,
            Self::Locate => 8,
            Self::FundManagerClientId => 9,
            Self::SettlementLocation => 10,
            Self::OrderOriginationTrader => 11,
            Self::ExecutingTrader => 12,
            Self::OrderOriginationFirm => 13,
            Self::GiveupClearingFirm => 14,
            Self::CorrespondantClearingFirm => 15,
            Self::ExecutingSystem => 16,
            Self::ContraFirm => 17,
            Self::ContraClearingFirm => 18,
            Self::SponsoringFirm => 19,
            Self::UnderlyingContraFirm => 20,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::ExecutingFirm),
            2 => Some(Self::BrokerOfCredit),
            3 => Some(Self::ClientId),
            4 => Some(Self::ClearingFirm),
            5 => Some(Self::InvestorId),
            6 => Some(Self::IntroducingFirm),
            7 => Some(Self::EnteringFirm),
            8 => Some(Self::Locate),
            9 => Some(Self::FundManagerClientId),
            10 => Some(Self::SettlementLocation),
            11 => Some(Self::OrderOriginationTrader),
            12 => Some(Self::ExecutingTrader),
            13 => Some(Self::OrderOriginationFirm),
            14 => Some(Self::GiveupClearingFirm),
            15 => Some(Self::CorrespondantClearingFirm),
            16 => Some(Self::ExecutingSystem),
            17 => Some(Self::ContraFirm),
            18 => Some(Self::ContraClearingFirm),
            19 => Some(Self::SponsoringFirm),
            20 => Some(Self::UnderlyingContraFirm),
            _ => None,
        }
    }
}

/// `Product` (tag 460, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Product {
    /// Wire value `1`.
    Agency,
    /// Wire value `2`.
    Commodity,
    /// Wire value `3`.
    Corporate,
    /// Wire value `4`.
    Currency,
    /// Wire value `5`.
    Equity,
    /// Wire value `6`.
    Government,
    /// Wire value `7`.
    Index,
    /// Wire value `8`.
    Loan,
    /// Wire value `9`.
    Moneymarket,
    /// Wire value `10`.
    Mortgage,
    /// Wire value `11`.
    Municipal,
    /// Wire value `12`.
    Other,
}

impl Product {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Agency => 1,
            Self::Commodity => 2,
            Self::Corporate => 3,
            Self::Currency => 4,
            Self::Equity => 5,
            Self::Government => 6,
            Self::Index => 7,
            Self::Loan => 8,
            Self::Moneymarket => 9,
            Self::Mortgage => 10,
            Self::Municipal => 11,
            Self::Other => 12,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Agency),
            2 => Some(Self::Commodity),
            3 => Some(Self::Corporate),
            4 => Some(Self::Currency),
            5 => Some(Self::Equity),
            6 => Some(Self::Government),
            7 => Some(Self::Index),
            8 => Some(Self::Loan),
            9 => Some(Self::Moneymarket),
            10 => Some(Self::Mortgage),
            11 => Some(Self::Municipal),
            12 => Some(Self::Other),
            _ => None,
        }
    }
}

/// `QuantityType` (tag 465, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityType {
    /// Wire value `1`.
    Shares,
    /// Wire value `2`.
    Bonds,
    /// Wire value `3`.
    Currentface,
    /// Wire value `4`.
    Originalface,
    /// Wire value `5`.
    Currency,
    /// Wire value `6`.
    Contracts,
    /// Wire value `7`.
    Other,
    /// Wire value `8`.
    Par,
}

impl QuantityType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Shares => 1,
            Self::Bonds => 2,
            Self::Currentface => 3,
            Self::Originalface => 4,
            Self::Currency => 5,
            Self::Contracts => 6,
            Self::Other => 7,
            Self::Par => 8,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Shares),
            2 => Some(Self::Bonds),
            3 => Some(Self::Currentface),
            4 => Some(Self::Originalface),
            5 => Some(Self::Currency),
            6 => Some(Self::Contracts),
            7 => Some(Self::Other),
            8 => Some(Self::Par),
            _ => None,
        }
    }
}

/// `MoneyLaunderingStatus` (tag 481, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneyLaunderingStatus {
    /// Wire value `1`.
    ExemptBelowTheLimit,
    /// Wire value `2`.
    ExemptClientMoneyTypeExemption,
    /// Wire value `3`.
    ExemptAuthorisedCreditOrFinancialInstitution,
    /// Wire value `N`.
    NotChecked,
    /// Wire value `Y`.
    Passed,
}

impl MoneyLaunderingStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::ExemptBelowTheLimit => '1',
            Self::ExemptClientMoneyTypeExemption => '2',
            Self::ExemptAuthorisedCreditOrFinancialInstitution => '3',
            Self::NotChecked => 'N',
            Self::Passed => 'Y',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::ExemptBelowTheLimit),
            '2' => Some(Self::ExemptClientMoneyTypeExemption),
            '3' => Some(Self::ExemptAuthorisedCreditOrFinancialInstitution),
            'N' => Some(Self::NotChecked),
            'Y' => Some(Self::Passed),
            _ => None,
        }
    }
}

/// `TradeReportTransType` (tag 487, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeReportTransType {
    /// Wire value `C`.
    Cancel,
    /// Wire value `N`.
    New,
    /// Wire value `R`.
    Replace,
}

impl TradeReportTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Cancel => 'C',
            Self::New => 'N',
            Self::Replace => 'R',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'C' => Some(Self::Cancel),
            'N' => Some(Self::New),
            'R' => Some(Self::Replace),
            _ => None,
        }
    }
}

/// `RegistTransType` (tag 514, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistTransType {
    /// Wire value `0`.
    New,
    /// Wire value `1`.
    Replace,
    /// Wire value `2`.
    Cancel,
}

impl RegistTransType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::New => '0',
            Self::Replace => '1',
            Self::Cancel => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::New),
            '1' => Some(Self::Replace),
            '2' => Some(Self::Cancel),
            _ => None,
        }
    }
}

/// `OwnerType` (tag 522, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerType {
    /// Wire value `1`.
    IndividualInvestor,
    /// Wire value `2`.
    PublicCompany,
    /// Wire value `3`.
    PrivateCompany,
    /// Wire value `4`.
    IndividualTrustee,
    /// Wire value `5`.
    CompanyTrustee,
    /// Wire value `6`.
    PensionPlan,
    /// Wire value `7`.
    CustodianUnderGiftsToMinorsAct,
    /// Wire value `8`.
    Trusts,
    /// Wire value `9`.
    Fiduciaries,
    /// Wire value `10`.
    NetworkingSubAccount,
    /// Wire value `11`.
    NonProfitOrganization,
    /// Wire value `12`.
    CorporateBody,
    /// Wire value `13`.
    Nominee,
}

impl OwnerType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::IndividualInvestor => 1,
            Self::PublicCompany => 2,
            Self::PrivateCompany => 3,
            Self::IndividualTrustee => 4,
            Self::CompanyTrustee => 5,
            Self::PensionPlan => 6,
            Self::CustodianUnderGiftsToMinorsAct => 7,
            Self::Trusts => 8,
            Self::Fiduciaries => 9,
            Self::NetworkingSubAccount => 10,
            Self::NonProfitOrganization => 11,
            Self::CorporateBody => 12,
            Self::Nominee => 13,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::IndividualInvestor),
            2 => Some(Self::PublicCompany),
            3 => Some(Self::PrivateCompany),
            4 => Some(Self::IndividualTrustee),
            5 => Some(Self::CompanyTrustee),
            6 => Some(Self::PensionPlan),
            7 => Some(Self::CustodianUnderGiftsToMinorsAct),
            8 => Some(Self::Trusts),
            9 => Some(Self::Fiduciaries),
            10 => Some(Self::NetworkingSubAccount),
            11 => Some(Self::NonProfitOrganization),
            12 => Some(Self::CorporateBody),
            13 => Some(Self::Nominee),
            _ => None,
        }
    }
}

/// `OrderCapacity` (tag 528, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderCapacity {
    /// Wire value `A`.
    Agency,
    /// Wire value `G`.
    Proprietary,
    /// Wire value `I`.
    Individual,
    /// Wire value `P`.
    Principal,
    /// Wire value `R`.
    RisklessPrincipal,
    /// Wire value `W`.
    AgentForOtherMember,
}

impl OrderCapacity {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Agency => 'A',
            Self::Proprietary => 'G',
            Self::Individual => 'I',
            Self::Principal => 'P',
            Self::RisklessPrincipal => 'R',
            Self::AgentForOtherMember => 'W',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            'A' => Some(Self::Agency),
            'G' => Some(Self::Proprietary),
            'I' => Some(Self::Individual),
            'P' => Some(Self::Principal),
            'R' => Some(Self::RisklessPrincipal),
            'W' => Some(Self::AgentForOtherMember),
            _ => None,
        }
    }
}

/// `OrderRestrictions` (tag 529, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderRestrictions {
    /// Wire value `1`.
    ProgramTrade,
    /// Wire value `2`.
    IndexArbitrage,
    /// Wire value `3`.
    NonIndexArbitrage,
    /// Wire value `4`.
    CompetingMarketMaker,
    /// Wire value `5`.
    ActingAsMarketMakerOrSpecialistInTheSecurity,
    /// Wire value `6`.
    ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
    /// Wire value `7`.
    ForeignEntity,
    /// Wire value `8`.
    ExternalMarketParticipant,
    /// Wire value `9`.
    ExternalInterConnectedMarketLinkage,
    /// Wire value `A`.
    RisklessArbitrage,
}

impl OrderRestrictions {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::ProgramTrade => "1",
            Self::IndexArbitrage => "2",
            Self::NonIndexArbitrage => "3",
            Self::CompetingMarketMaker => "4",
            Self::ActingAsMarketMakerOrSpecialistInTheSecurity => "5",
            Self::ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity => {
                "6"
            }
            Self::ForeignEntity => "7",
            Self::ExternalMarketParticipant => "8",
            Self::ExternalInterConnectedMarketLinkage => "9",
            Self::RisklessArbitrage => "A",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "1" => Some(Self::ProgramTrade),
            "2" => Some(Self::IndexArbitrage),
            "3" => Some(Self::NonIndexArbitrage),
            "4" => Some(Self::CompetingMarketMaker),
            "5" => Some(Self::ActingAsMarketMakerOrSpecialistInTheSecurity),
            "6" => Some(
                Self::ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
            ),
            "7" => Some(Self::ForeignEntity),
            "8" => Some(Self::ExternalMarketParticipant),
            "9" => Some(Self::ExternalInterConnectedMarketLinkage),
            "A" => Some(Self::RisklessArbitrage),
            _ => None,
        }
    }
}

/// `MassCancelRequestType` (tag 530, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassCancelRequestType {
    /// Wire value `1`.
    CancelOrdersForASecurity,
    /// Wire value `2`.
    CancelOrdersForAnUnderlyingSecurity,
    /// Wire value `3`.
    CancelOrdersForAProduct,
    /// Wire value `4`.
    CancelOrdersForACficode,
    /// Wire value `5`.
    CancelOrdersForASecuritytype,
    /// Wire value `6`.
    CancelOrdersForATradingSession,
    /// Wire value `7`.
    CancelAllOrders,
}

impl MassCancelRequestType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::CancelOrdersForASecurity => '1',
            Self::CancelOrdersForAnUnderlyingSecurity => '2',
            Self::CancelOrdersForAProduct => '3',
            Self::CancelOrdersForACficode => '4',
            Self::CancelOrdersForASecuritytype => '5',
            Self::CancelOrdersForATradingSession => '6',
            Self::CancelAllOrders => '7',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::CancelOrdersForASecurity),
            '2' => Some(Self::CancelOrdersForAnUnderlyingSecurity),
            '3' => Some(Self::CancelOrdersForAProduct),
            '4' => Some(Self::CancelOrdersForACficode),
            '5' => Some(Self::CancelOrdersForASecuritytype),
            '6' => Some(Self::CancelOrdersForATradingSession),
            '7' => Some(Self::CancelAllOrders),
            _ => None,
        }
    }
}

/// `MassCancelResponse` (tag 531, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassCancelResponse {
    /// Wire value `0`.
    CancelRequestRejected,
    /// Wire value `1`.
    CancelOrdersForASecurity,
    /// Wire value `2`.
    CancelOrdersForAnUnderlyingSecurity,
    /// Wire value `3`.
    CancelOrdersForAProduct,
    /// Wire value `4`.
    CancelOrdersForACficode,
    /// Wire value `5`.
    CancelOrdersForASecuritytype,
    /// Wire value `6`.
    CancelOrdersForATradingSession,
    /// Wire value `7`.
    CancelAllOrders,
}

impl MassCancelResponse {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::CancelRequestRejected => '0',
            Self::CancelOrdersForASecurity => '1',
            Self::CancelOrdersForAnUnderlyingSecurity => '2',
            Self::CancelOrdersForAProduct => '3',
            Self::CancelOrdersForACficode => '4',
            Self::CancelOrdersForASecuritytype => '5',
            Self::CancelOrdersForATradingSession => '6',
            Self::CancelAllOrders => '7',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::CancelRequestRejected),
            '1' => Some(Self::CancelOrdersForASecurity),
            '2' => Some(Self::CancelOrdersForAnUnderlyingSecurity),
            '3' => Some(Self::CancelOrdersForAProduct),
            '4' => Some(Self::CancelOrdersForACficode),
            '5' => Some(Self::CancelOrdersForASecuritytype),
            '6' => Some(Self::CancelOrdersForATradingSession),
            '7' => Some(Self::CancelAllOrders),
            _ => None,
        }
    }
}

/// `QuoteType` (tag 537, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    /// Wire value `0`.
    Indicative,
    /// Wire value `1`.
    Tradeable,
    /// Wire value `2`.
    RestrictedTradeable,
}

impl QuoteType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Indicative => 0,
            Self::Tradeable => 1,
            Self::RestrictedTradeable => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Indicative),
            1 => Some(Self::Tradeable),
            2 => Some(Self::RestrictedTradeable),
            _ => None,
        }
    }
}

/// `InstrRegistry` (tag 543, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrRegistry {
    /// Wire value `Code`.
    CountryInWhichRegistryIsKept,
    /// Wire value `ZZ`.
    PhysicalOrBearer,
}

impl InstrRegistry {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::CountryInWhichRegistryIsKept => "Code",
            Self::PhysicalOrBearer => "ZZ",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "Code" => Some(Self::CountryInWhichRegistryIsKept),
            "ZZ" => Some(Self::PhysicalOrBearer),
            _ => None,
        }
    }
}

/// `CashMargin` (tag 544, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CashMargin {
    /// Wire value `1`.
    Cash,
    /// Wire value `2`.
    MarginOpen,
    /// Wire value `3`.
    MarginClose,
}

impl CashMargin {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Cash => '1',
            Self::MarginOpen => '2',
            Self::MarginClose => '3',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '1' => Some(Self::Cash),
            '2' => Some(Self::MarginOpen),
            '3' => Some(Self::MarginClose),
            _ => None,
        }
    }
}

/// `Scope` (tag 546, FIX type MULTIPLEVALUESTRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Wire value `1`.
    Local,
    /// Wire value `2`.
    National,
    /// Wire value `3`.
    Global,
}

impl Scope {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::Local => "1",
            Self::National => "2",
            Self::Global => "3",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "1" => Some(Self::Local),
            "2" => Some(Self::National),
            "3" => Some(Self::Global),
            _ => None,
        }
    }
}

/// `CrossType` (tag 549, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossType {
    /// Wire value `1`.
    CrossTradeWhichIsExecutedCompletelyOrNot,
    /// Wire value `2`.
    CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelled,
    /// Wire value `3`.
    CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActive,
    /// Wire value `4`.
    CrossTradeIsExecutedWithExistingOrdersWithTheSamePrice,
}

impl CrossType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::CrossTradeWhichIsExecutedCompletelyOrNot => 1,
            Self::CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelled => 2,
            Self::CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActive => 3,
            Self::CrossTradeIsExecutedWithExistingOrdersWithTheSamePrice => 4,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::CrossTradeWhichIsExecutedCompletelyOrNot),
            2 => Some(Self::CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelled),
            3 => {
                Some(Self::CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActive)
            }
            4 => Some(Self::CrossTradeIsExecutedWithExistingOrdersWithTheSamePrice),
            _ => None,
        }
    }
}

/// `NoSides` (tag 552, FIX type NUMINGROUP).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoSides {
    /// Wire value `1`.
    OneSide,
    /// Wire value `2`.
    BothSides,
}

impl NoSides {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::OneSide => 1,
            Self::BothSides => 2,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::OneSide),
            2 => Some(Self::BothSides),
            _ => None,
        }
    }
}

/// `SecurityListRequestType` (tag 559, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityListRequestType {
    /// Wire value `0`.
    Symbol,
    /// Wire value `1`.
    SecuritytypeAnd,
    /// Wire value `2`.
    Product,
    /// Wire value `3`.
    Tradingsessionid,
    /// Wire value `4`.
    AllSecurities,
}

impl SecurityListRequestType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::Symbol => 0,
            Self::SecuritytypeAnd => 1,
            Self::Product => 2,
            Self::Tradingsessionid => 3,
            Self::AllSecurities => 4,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Symbol),
            1 => Some(Self::SecuritytypeAnd),
            2 => Some(Self::Product),
            3 => Some(Self::Tradingsessionid),
            4 => Some(Self::AllSecurities),
            _ => None,
        }
    }
}

/// `SecurityRequestResult` (tag 560, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityRequestResult {
    /// Wire value `0`.
    ValidRequest,
    /// Wire value `1`.
    InvalidOrUnsupportedRequest,
    /// Wire value `2`.
    NoInstrumentsFoundThatMatchSelectionCriteria,
    /// Wire value `3`.
    NotAuthorizedToRetrieveInstrumentData,
    /// Wire value `4`.
    InstrumentDataTemporarilyUnavailable,
    /// Wire value `5`.
    RequestForInstrumentDataNotSupported,
}

impl SecurityRequestResult {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::ValidRequest => 0,
            Self::InvalidOrUnsupportedRequest => 1,
            Self::NoInstrumentsFoundThatMatchSelectionCriteria => 2,
            Self::NotAuthorizedToRetrieveInstrumentData => 3,
            Self::InstrumentDataTemporarilyUnavailable => 4,
            Self::RequestForInstrumentDataNotSupported => 5,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::ValidRequest),
            1 => Some(Self::InvalidOrUnsupportedRequest),
            2 => Some(Self::NoInstrumentsFoundThatMatchSelectionCriteria),
            3 => Some(Self::NotAuthorizedToRetrieveInstrumentData),
            4 => Some(Self::InstrumentDataTemporarilyUnavailable),
            5 => Some(Self::RequestForInstrumentDataNotSupported),
            _ => None,
        }
    }
}

/// `TradSesStatusRejReason` (tag 567, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesStatusRejReason {
    /// Wire value `1`.
    UnknownOrInvalidTradingsessionid,
}

impl TradSesStatusRejReason {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::UnknownOrInvalidTradingsessionid => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::UnknownOrInvalidTradingsessionid),
            _ => None,
        }
    }
}

/// `TradeRequestType` (tag 569, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeRequestType {
    /// Wire value `0`.
    AllTrades,
    /// Wire value `1`.
    MatchedTradesMatchingCriteriaProvidedOnRequest,
    /// Wire value `2`.
    UnmatchedTradesThatMatchCriteria,
    /// Wire value `3`.
    UnreportedTradesThatMatchCriteria,
    /// Wire value `4`.
    AdvisoriesThatMatchCriteria,
}

impl TradeRequestType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::AllTrades => 0,
            Self::MatchedTradesMatchingCriteriaProvidedOnRequest => 1,
            Self::UnmatchedTradesThatMatchCriteria => 2,
            Self::UnreportedTradesThatMatchCriteria => 3,
            Self::AdvisoriesThatMatchCriteria => 4,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::AllTrades),
            1 => Some(Self::MatchedTradesMatchingCriteriaProvidedOnRequest),
            2 => Some(Self::UnmatchedTradesThatMatchCriteria),
            3 => Some(Self::UnreportedTradesThatMatchCriteria),
            4 => Some(Self::AdvisoriesThatMatchCriteria),
            _ => None,
        }
    }
}

/// `MatchStatus` (tag 573, FIX type CHAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchStatus {
    /// Wire value `0`.
    Compared,
    /// Wire value `1`.
    Uncompared,
    /// Wire value `2`.
    AdvisoryOrAlert,
}

impl MatchStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> char {
        match self {
            Self::Compared => '0',
            Self::Uncompared => '1',
            Self::AdvisoryOrAlert => '2',
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: char) -> Option<Self> {
        match value {
            '0' => Some(Self::Compared),
            '1' => Some(Self::Uncompared),
            '2' => Some(Self::AdvisoryOrAlert),
            _ => None,
        }
    }
}

/// `MatchType` (tag 574, FIX type STRING).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    /// Wire value `A1`.
    ExactMatchPlusFourBadgesAndExecutionTime,
    /// Wire value `A2`.
    ExactMatchPlusFourBadges,
    /// Wire value `A3`.
    ExactMatchPlusTwoBadgesAndExecutionTime,
    /// Wire value `A4`.
    ExactMatchPlusTwoBadges,
    /// Wire value `A5`.
    ExactMatchPlusExecutionTime,
    /// Wire value `AQ`.
    ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAccepts,
    /// Wire value `M1`.
    M1Match,
    /// Wire value `M2`.
    M2Match,
    /// Wire value `M3`.
    ActAcceptedTrade,
    /// Wire value `M4`.
    ActDefaultTrade,
    /// Wire value `M5`.
    ActDefaultAfterM2,
    /// Wire value `M6`.
    ActM6Match,
    /// Wire value `MT`.
    OcsLockedInOrNonAct,
    /// Wire value `S1`.
    SummarizedMatchUsingA1,
    /// Wire value `S2`.
    SummarizedMatchUsingA2,
    /// Wire value `S3`.
    SummarizedMatchUsingA3,
    /// Wire value `S4`.
    SummarizedMatchUsingA4,
    /// Wire value `S5`.
    SummarizedMatchUsingA5,
}

impl MatchType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> &'static str {
        match self {
            Self::ExactMatchPlusFourBadgesAndExecutionTime => "A1",
            Self::ExactMatchPlusFourBadges => "A2",
            Self::ExactMatchPlusTwoBadgesAndExecutionTime => "A3",
            Self::ExactMatchPlusTwoBadges => "A4",
            Self::ExactMatchPlusExecutionTime => "A5",
            Self::ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAccepts => "AQ",
            Self::M1Match => "M1",
            Self::M2Match => "M2",
            Self::ActAcceptedTrade => "M3",
            Self::ActDefaultTrade => "M4",
            Self::ActDefaultAfterM2 => "M5",
            Self::ActM6Match => "M6",
            Self::OcsLockedInOrNonAct => "MT",
            Self::SummarizedMatchUsingA1 => "S1",
            Self::SummarizedMatchUsingA2 => "S2",
            Self::SummarizedMatchUsingA3 => "S3",
            Self::SummarizedMatchUsingA4 => "S4",
            Self::SummarizedMatchUsingA5 => "S5",
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "A1" => Some(Self::ExactMatchPlusFourBadgesAndExecutionTime),
            "A2" => Some(Self::ExactMatchPlusFourBadges),
            "A3" => Some(Self::ExactMatchPlusTwoBadgesAndExecutionTime),
            "A4" => Some(Self::ExactMatchPlusTwoBadges),
            "A5" => Some(Self::ExactMatchPlusExecutionTime),
            "AQ" => Some(Self::ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAccepts),
            "M1" => Some(Self::M1Match),
            "M2" => Some(Self::M2Match),
            "M3" => Some(Self::ActAcceptedTrade),
            "M4" => Some(Self::ActDefaultTrade),
            "M5" => Some(Self::ActDefaultAfterM2),
            "M6" => Some(Self::ActM6Match),
            "MT" => Some(Self::OcsLockedInOrNonAct),
            "S1" => Some(Self::SummarizedMatchUsingA1),
            "S2" => Some(Self::SummarizedMatchUsingA2),
            "S3" => Some(Self::SummarizedMatchUsingA3),
            "S4" => Some(Self::SummarizedMatchUsingA4),
            "S5" => Some(Self::SummarizedMatchUsingA5),
            _ => None,
        }
    }
}

/// `ClearingInstruction` (tag 577, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearingInstruction {
    /// Wire value `0`.
    ProcessNormally,
    /// Wire value `1`.
    ExcludeFromAllNetting,
    /// Wire value `2`.
    BilateralNettingOnly,
    /// Wire value `3`.
    ExClearing,
    /// Wire value `4`.
    SpecialTrade,
    /// Wire value `5`.
    MultilateralNetting,
    /// Wire value `6`.
    ClearAgainstCentralCounterparty,
    /// Wire value `7`.
    ExcludeFromCentralCounterparty,
    /// Wire value `8`.
    ManualMode,
    /// Wire value `9`.
    AutomaticPostingMode,
    /// Wire value `10`.
    AutomaticGiveUpMode,
}

impl ClearingInstruction {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::ProcessNormally => 0,
            Self::ExcludeFromAllNetting => 1,
            Self::BilateralNettingOnly => 2,
            Self::ExClearing => 3,
            Self::SpecialTrade => 4,
            Self::MultilateralNetting => 5,
            Self::ClearAgainstCentralCounterparty => 6,
            Self::ExcludeFromCentralCounterparty => 7,
            Self::ManualMode => 8,
            Self::AutomaticPostingMode => 9,
            Self::AutomaticGiveUpMode => 10,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::ProcessNormally),
            1 => Some(Self::ExcludeFromAllNetting),
            2 => Some(Self::BilateralNettingOnly),
            3 => Some(Self::ExClearing),
            4 => Some(Self::SpecialTrade),
            5 => Some(Self::MultilateralNetting),
            6 => Some(Self::ClearAgainstCentralCounterparty),
            7 => Some(Self::ExcludeFromCentralCounterparty),
            8 => Some(Self::ManualMode),
            9 => Some(Self::AutomaticPostingMode),
            10 => Some(Self::AutomaticGiveUpMode),
            _ => None,
        }
    }
}

/// `AccountType` (tag 581, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    /// Wire value `1`.
    AccountIsCarriedOnCustomerSideOfBooks,
    /// Wire value `2`.
    AccountIsCarriedOnNonCustomerSideOfBooks,
    /// Wire value `3`.
    HouseTrader,
    /// Wire value `4`.
    FloorTrader,
    /// Wire value `6`.
    AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
    /// Wire value `7`.
    AccountIsHouseTraderAndIsCrossMargined,
    /// Wire value `8`.
    JointBackofficeAccount,
}

impl AccountType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::AccountIsCarriedOnCustomerSideOfBooks => 1,
            Self::AccountIsCarriedOnNonCustomerSideOfBooks => 2,
            Self::HouseTrader => 3,
            Self::FloorTrader => 4,
            Self::AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined => 6,
            Self::AccountIsHouseTraderAndIsCrossMargined => 7,
            Self::JointBackofficeAccount => 8,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::AccountIsCarriedOnCustomerSideOfBooks),
            2 => Some(Self::AccountIsCarriedOnNonCustomerSideOfBooks),
            3 => Some(Self::HouseTrader),
            4 => Some(Self::FloorTrader),
            6 => Some(Self::AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined),
            7 => Some(Self::AccountIsHouseTraderAndIsCrossMargined),
            8 => Some(Self::JointBackofficeAccount),
            _ => None,
        }
    }
}

/// `MassStatusReqType` (tag 585, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassStatusReqType {
    /// Wire value `1`.
    StatusForOrdersForASecurity,
    /// Wire value `2`.
    StatusForOrdersForAnUnderlyingSecurity,
    /// Wire value `3`.
    StatusForOrdersForAProduct,
    /// Wire value `4`.
    StatusForOrdersForACficode,
    /// Wire value `5`.
    StatusForOrdersForASecuritytype,
    /// Wire value `6`.
    StatusForOrdersForATradingSession,
    /// Wire value `7`.
    StatusForAllOrders,
    /// Wire value `8`.
    StatusForOrdersForAPartyid,
}

impl MassStatusReqType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::StatusForOrdersForASecurity => 1,
            Self::StatusForOrdersForAnUnderlyingSecurity => 2,
            Self::StatusForOrdersForAProduct => 3,
            Self::StatusForOrdersForACficode => 4,
            Self::StatusForOrdersForASecuritytype => 5,
            Self::StatusForOrdersForATradingSession => 6,
            Self::StatusForAllOrders => 7,
            Self::StatusForOrdersForAPartyid => 8,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::StatusForOrdersForASecurity),
            2 => Some(Self::StatusForOrdersForAnUnderlyingSecurity),
            3 => Some(Self::StatusForOrdersForAProduct),
            4 => Some(Self::StatusForOrdersForACficode),
            5 => Some(Self::StatusForOrdersForASecuritytype),
            6 => Some(Self::StatusForOrdersForATradingSession),
            7 => Some(Self::StatusForAllOrders),
            8 => Some(Self::StatusForOrdersForAPartyid),
            _ => None,
        }
    }
}

/// `AllocType` (tag 626, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocType {
    /// Wire value `1`.
    BuysideCalculated,
    /// Wire value `2`.
    BuysidePreliminary,
    /// Wire value `3`.
    SellsideCalculatedUsingPreliminary,
    /// Wire value `4`.
    SellsideCalculatedWithoutPreliminary,
    /// Wire value `5`.
    BuysideReadyToBookSingleOrder,
    /// Wire value `6`.
    BuysideReadyToBookCombinedSetOfOrders,
}

impl AllocType {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::BuysideCalculated => 1,
            Self::BuysidePreliminary => 2,
            Self::SellsideCalculatedUsingPreliminary => 3,
            Self::SellsideCalculatedWithoutPreliminary => 4,
            Self::BuysideReadyToBookSingleOrder => 5,
            Self::BuysideReadyToBookCombinedSetOfOrders => 6,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::BuysideCalculated),
            2 => Some(Self::BuysidePreliminary),
            3 => Some(Self::SellsideCalculatedUsingPreliminary),
            4 => Some(Self::SellsideCalculatedWithoutPreliminary),
            5 => Some(Self::BuysideReadyToBookSingleOrder),
            6 => Some(Self::BuysideReadyToBookCombinedSetOfOrders),
            _ => None,
        }
    }
}

/// `PriorityIndicator` (tag 638, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityIndicator {
    /// Wire value `0`.
    PriorityUnchanged,
    /// Wire value `1`.
    LostPriorityAsResultOfOrderChange,
}

impl PriorityIndicator {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::PriorityUnchanged => 0,
            Self::LostPriorityAsResultOfOrderChange => 1,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::PriorityUnchanged),
            1 => Some(Self::LostPriorityAsResultOfOrderChange),
            _ => None,
        }
    }
}

/// `SecDefStatus` (tag 653, FIX type INT).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecDefStatus {
    /// Wire value `0`.
    PendingApproval,
    /// Wire value `1`.
    Approved,
    /// Wire value `2`.
    Rejected,
    /// Wire value `3`.
    UnauthorizedRequest,
    /// Wire value `4`.
    InvalidDefinitionRequest,
}

impl SecDefStatus {
    /// On-wire value for this variant.
    pub fn to_fix(self) -> i32 {
        match self {
            Self::PendingApproval => 0,
            Self::Approved => 1,
            Self::Rejected => 2,
            Self::UnauthorizedRequest => 3,
            Self::InvalidDefinitionRequest => 4,
        }
    }

    /// Parse an on-wire value; `None` for a value outside the FIX enum.
    pub fn from_fix(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::PendingApproval),
            1 => Some(Self::Approved),
            2 => Some(Self::Rejected),
            3 => Some(Self::UnauthorizedRequest),
            4 => Some(Self::InvalidDefinitionRequest),
            _ => None,
        }
    }
}
