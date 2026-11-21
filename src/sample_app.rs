use fix_rs::application::Application;
use fix_rs::core::message::Message;
use fix_rs::errors::{BusinessMsgReject, DonotSend, RejectLogon};
use fix_rs::messages::fix43::Fix43Message;
use fix_rs::messages::fix43::execution_report::ExecutionReport;
use fix_rs::messages::fix43::fields::{ExecType, MdEntryType, OrdStatus};
use fix_rs::messages::fix43::market_data_request::MarketDataRequest;
use fix_rs::messages::fix43::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh;
use fix_rs::messages::fix43::new_order_single::NewOrderSingle;
use fix_rs::session::SessionId;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct SampleApp {
    // Monotonic counter for generating unique OrderID (37) / ExecID (17) values
    // on outbound ExecutionReports. A real venue would use something durable;
    // for the sample a per-process counter is enough.
    next_id: u64,
}

impl SampleApp {
    pub fn new() -> SampleApp {
        SampleApp { next_id: 0 }
    }

    fn next_id(&mut self) -> String {
        self.next_id += 1;
        self.next_id.to_string()
    }

    // Builds an ExecutionReport (35=8) acknowledging a NewOrderSingle as "New"
    // (accepted, unfilled). Echoes ClOrdID/Side/Symbol from the order so the
    // client can correlate it, and fills the fields FIX 4.3 marks required:
    // OrderID(37), ExecID(17), ExecType(150), OrdStatus(39), Side(54),
    // LeavesQty(151), CumQty(14), AvgPx(6). No repeating group — a New report is
    // flat. Header CompIDs/MsgSeqNum/SendingTime are stamped later by the engine.
    // `order_id` is the caller's: the fill for the same order reuses it.
    //
    // The order's getters can't fail here: ClOrdID, Side, Symbol and OrderQty are
    // dictionary-required for D, so the engine rejected any order missing one (or
    // carrying a malformed value) before dispatch. The `expect`s name that contract.
    fn build_exec_report(&mut self, order_id: &str, nos: &NewOrderSingle) -> ExecutionReport {
        let mut er = ExecutionReport::new();
        er.set_order_id(order_id);
        er.set_exec_id(&self.next_id());
        er.set_exec_type(ExecType::New);
        er.set_ord_status(OrdStatus::New);
        er.set_side(nos.side().expect("Side(54) required for D"));
        er.set_leaves_qty(nos.order_qty().expect("OrderQty(38) required for D")); // nothing filled yet
        er.set_cum_qty(Decimal::ZERO);
        er.set_avg_px(Decimal::ZERO);
        er.set_cl_ord_id(&nos.cl_ord_id().expect("ClOrdID(11) required for D"));
        er.set_symbol(&nos.symbol().expect("Symbol(55) required for D"));
        er
    }

    // Builds an ExecutionReport (35=8) reporting a full fill of the order at its
    // limit price. This is what makes an execution appear in Banzai's Execution tab:
    // Banzai computes fillSize = OrderQty - LeavesQty, and only records an
    // execution when that is > 0. So a fill must carry LeavesQty(151)=0 with
    // CumQty(14)=qty, plus ExecType(150)=F (Trade) / OrdStatus(39)=2 (Filled),
    // and LastPx(31)/LastQty(32) for the execution row.
    fn build_fill_report(&mut self, order_id: &str, nos: &NewOrderSingle) -> ExecutionReport {
        let qty = nos.order_qty().expect("OrderQty(38) required for D");
        // Price(44) is optional: a market order has none, so it fills at 0.
        let px = nos.price().unwrap_or(Decimal::ZERO);
        let mut er = ExecutionReport::new();
        er.set_order_id(order_id);
        er.set_exec_id(&self.next_id());
        er.set_exec_type(ExecType::Trade);
        er.set_ord_status(OrdStatus::Filled);
        er.set_side(nos.side().expect("Side(54) required for D"));
        er.set_leaves_qty(Decimal::ZERO); // fully filled
        er.set_cum_qty(qty);
        er.set_avg_px(px);
        er.set_last_px(px); // this fill's price
        er.set_last_qty(qty); // this fill's size
        er.set_cl_ord_id(&nos.cl_ord_id().expect("ClOrdID(11) required for D"));
        er.set_symbol(&nos.symbol().expect("Symbol(55) required for D"));
        er
    }

    // Builds a MarketDataSnapshotFullRefresh (35=W) for one instrument: MDReqID(262)
    // echoed so the client can match it to its request, Symbol(55), and a fixed
    // two-sided book in NoMDEntries(268) — a bid at 1.4 and an offer at 1.8, 1000
    // each. Prices are canned: this is a simulator, not a market.
    fn build_md_snapshot(
        &self,
        ccy_pair: &str,
        mdr: &MarketDataRequest,
    ) -> MarketDataSnapshotFullRefresh {
        let mut mds = MarketDataSnapshotFullRefresh::new();
        mds.set_md_req_id(&mdr.md_req_id().expect("MDReqID(262) required for V"));
        mds.set_symbol(ccy_pair);
        mds.add_no_md_entries_entry(|e| {
            e.set_md_entry_type(MdEntryType::Bid);
            e.set_md_entry_px(dec!(1.4));
            e.set_md_entry_size(dec!(1000));
        });
        mds.add_no_md_entries_entry(|e| {
            e.set_md_entry_type(MdEntryType::Offer);
            e.set_md_entry_px(dec!(1.8));
            e.set_md_entry_size(dec!(1000));
        });
        mds
    }
}

impl Application for SampleApp {
    fn on_create(&mut self, session_id: &SessionId) {
        println!("on_create");
    }

    fn on_logon(&mut self, session_id: &SessionId) {
        println!("on_logon");
    }

    fn on_logout(&mut self, session_id: &SessionId) {
        println!("on_logout");
    }

    fn on_admin_msg_sending(&mut self, session_id: &SessionId, message: &mut Message) {
        println!("on_admin_msg_sending");
    }

    fn on_admin_msg_received(
        &mut self,
        session_id: &SessionId,
        message: &Message,
    ) -> Result<(), RejectLogon> {
        println!("on_admin_msg_received");
        Ok(())
    }

    fn on_app_msg_sending(
        &mut self,
        session_id: &SessionId,
        message: &mut Message,
    ) -> Result<(), DonotSend> {
        println!("on_app_msg_sending");
        Ok(())
    }

    fn on_app_msg_received(
        &mut self,
        session_id: &SessionId,
        message: Message,
    ) -> Result<Vec<Message>, BusinessMsgReject> {
        // MsgType(35) is guaranteed: from_str requires it, and next_message routed
        // on it to reach us. Read before the message moves into Fix43Message, for the
        // unsupported-type reject below.
        let msg_type = message.get_msg_type().expect("MsgType(35) present on a routed message");
        // Typed dispatch: Fix43Message::from reads 35 once and wraps the message in
        // its facade; each arm gets a typed message, by value.
        match Fix43Message::from(message) {
            Fix43Message::MarketDataRequest(msg) => {
                // MarketDataRequest → one MarketDataSnapshotFullRefresh. FIX43 carries
                // the instrument inside the NoRelatedSym(146) group, not at top level.
                // Our contract with the counterparty: one V asks for one currency pair,
                // so we answer the first entry only (a multi-pair V still parses; the
                // extra pairs are ignored).
                //
                // NoRelatedSym is dictionary-required, so the engine rejects a V without
                // it before dispatch; a count of 0 (no entries) isn't ruled out, though.
                // Either way there's no pair to quote → BusinessMessageReject (380=5).
                let Some(entry) = msg.no_related_sym().and_then(|sym| sym.get(0)) else {
                    return Err(BusinessMsgReject::MissingConditionallyRequiredField { tag: 146 });
                };
                // Symbol(55) is NoRelatedSym's delimiter: every parsed entry starts with it.
                let ccy_pair = entry.symbol().expect("Symbol(55) is NoRelatedSym's delimiter");
                let snapshot = self.build_md_snapshot(&ccy_pair, &msg);
                let outbound = Message::from(snapshot);
                Ok(vec![outbound])
            }
            Fix43Message::NewOrderSingle(nos) => {
                // NewOrderSingle → New ack, then a full fill. Two ExecutionReports:
                // the first (New) acknowledges the order; the second (Trade/Filled)
                // executes it so a row appears in the client's execution blotter.
                // ClOrdID(11), Side(54), Symbol(55, via the required Instrument
                // component) and OrderQty(38, via OrderQtyData) are all
                // dictionary-required for D — the engine validated them before
                // dispatch, so they're guaranteed present here.
                // OrderID(37) is the venue's id for the order: one per order, the same on
                // every report about it. ExecID(17) is per report (each builder draws one).
                let order_id = self.next_id();
                let accept = self.build_exec_report(&order_id, &nos);
                let fill = self.build_fill_report(&order_id, &nos);
                Ok(vec![Message::from(accept), Message::from(fill)])
            }
            // Any other app message (including one this dictionary doesn't define):
            // not something this sample handles → BusinessMessageReject 380=3
            // (Unsupported Message Type), as the FIX spec and QFJ's cracker do.
            _ => Err(BusinessMsgReject::UnknownMessageTye { msg_type }),
        }
    }
}

#[cfg(test)]
mod sample_app_tests {
    //! SampleApp against what actually crosses the wire. Inbound requests are raw FIX
    //! strings parsed by the engine (`Message::from_str` + the FIX43 dictionary), so they
    //! have the shape a real FIX 4.3 counterparty sends, not one hand-built to suit the
    //! handler. Outbound responses are stamped, serialized and parsed again, so a
    //! counterparty's parser would accept them. Values are read back through the typed
    //! facades.

    use super::*;
    use fix_rs::core::dictionary::DataDictionary;
    use fix_rs::errors::SessionRejectError;
    use fix_rs::messages::fix43::fields::{MdEntryType, Side, SubscriptionRequestType};
    use std::str::FromStr;
    use std::sync::LazyLock;

    static DD: LazyLock<DataDictionary> =
        LazyLock::new(|| DataDictionary::from_xml("resources/FIX43.xml"));

    fn sid() -> SessionId {
        SessionId::new("FIX.4.3", "EXEC", "BANZAI")
    }

    fn dec(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    /// Parse a `|`-separated wire fixture (BodyLength / CheckSum from
    /// `.claude/skills/fix-test-design/scripts/fix_fixture.py`) the way the engine does
    /// before dispatch.
    fn inbound(raw: &str) -> Message {
        Message::from_str(&raw.replace('|', "\x01"), &DD).expect("fixture parses")
    }

    /// Stamp the header the session adds on send, serialize, and parse with the FIX43
    /// dictionary: `Ok` means a counterparty engine would accept the response.
    fn as_counterparty_sees(msg: Message) -> Result<Message, SessionRejectError> {
        let mut msg = msg;
        msg.set_header_field(8, "FIX.4.3");
        msg.set_header_field(49, "EXEC");
        msg.set_header_field(56, "BANZAI");
        msg.set_header_field(34, "2");
        msg.set_header_field(52, "20261121-09:00:01.000");
        msg.set_body_len();
        msg.set_checksum();
        Message::from_str(&msg.to_string(), &DD)
    }

    // Snapshot request (263=0) for one pair, Bid + Offer, as a FIX 4.3
    // client sends it: the pair inside NoRelatedSym(146), not at top level.
    const V_ONE_PAIR: &str = "8=FIX.4.3|9=110|35=V|34=2|49=BANZAI|52=20261121-09:00:00.000|56=EXEC|262=REQ-1|263=0|264=1|267=2|269=0|269=1|146=1|55=EUR/USD|10=255|";
    // The same request for two pairs.
    const V_TWO_PAIRS: &str = "8=FIX.4.3|9=121|35=V|34=2|49=BANZAI|52=20261121-09:00:00.000|56=EXEC|262=REQ-2|263=0|264=1|267=2|269=0|269=1|146=2|55=EUR/USD|55=GBP/USD|10=159|";
    // Banzai-shaped limit buy: 100 AAPL at 101.25.
    const D_LIMIT: &str = "8=FIX.4.3|9=123|35=D|34=2|49=BANZAI|52=20261121-09:00:00.000|56=EXEC|11=ORD-1|21=1|55=AAPL|54=1|60=20261121-09:00:00|38=100|40=2|44=101.25|10=003|";
    // Market sell: 50 AAPL, no Price(44).
    const D_MARKET: &str = "8=FIX.4.3|9=112|35=D|34=2|49=BANZAI|52=20261121-09:00:00.000|56=EXEC|11=ORD-2|21=1|55=AAPL|54=2|60=20261121-09:00:00|38=50|40=1|10=009|";

    // ---- MarketDataRequest (V) → MarketDataSnapshotFullRefresh (W) ----

    // A spec-shaped V gets exactly one W: MDReqID echoed so the client can match it,
    // the pair from NoRelatedSym, and the canned two-sided book. The W passes the
    // counterparty's parser.
    #[test]
    fn v_request_gets_one_snapshot_for_its_pair() {
        let mut app = SampleApp::new();
        let responses = app.on_app_msg_received(&sid(), inbound(V_ONE_PAIR)).unwrap();
        assert_eq!(responses.len(), 1);

        let parsed = as_counterparty_sees(responses.into_iter().next().unwrap())
            .expect("counterparty accepts the snapshot");
        let w = MarketDataSnapshotFullRefresh::try_from(parsed).unwrap();
        assert_eq!(w.md_req_id().unwrap(), "REQ-1");
        assert_eq!(w.symbol().unwrap(), "EUR/USD");

        let book: Vec<_> = w
            .no_md_entries()
            .expect("NoMDEntries present")
            .iter()
            .map(|e| {
                (e.md_entry_type().unwrap(), e.md_entry_px().unwrap(), e.md_entry_size().unwrap())
            })
            .collect();
        assert_eq!(
            book,
            [
                (MdEntryType::Bid, dec("1.4"), dec("1000")),
                (MdEntryType::Offer, dec("1.8"), dec("1000")),
            ]
        );
    }

    // Contract: one V asks for one pair. A multi-pair V is still a valid FIX message
    // (it parses) and is answered for its first pair only.
    #[test]
    fn multi_pair_v_parses_and_is_answered_for_the_first_pair() {
        let mut app = SampleApp::new();
        let responses = app.on_app_msg_received(&sid(), inbound(V_TWO_PAIRS)).unwrap();
        assert_eq!(responses.len(), 1);
        let w =
            MarketDataSnapshotFullRefresh::try_from(responses.into_iter().next().unwrap()).unwrap();
        assert_eq!(w.md_req_id().unwrap(), "REQ-2");
        assert_eq!(w.symbol().unwrap(), "EUR/USD");
    }

    // No pair to quote → BusinessMessageReject 380=5 naming NoRelatedSym (146). Two ways
    // to get there; neither comes through the parser for an absent group (it's
    // dictionary-required), so both are built through the facade, as a defence in depth.
    #[test]
    fn v_without_a_pair_is_business_rejected() {
        let base = || {
            let mut req = MarketDataRequest::new();
            req.set_md_req_id("REQ-3");
            req.set_subscription_request_type(SubscriptionRequestType::Snapshot);
            req.set_market_depth(1);
            req
        };
        let no_group = base();
        let mut empty_group = base();
        empty_group.set_no_related_sym(0);

        for req in [no_group, empty_group] {
            let err = SampleApp::new().on_app_msg_received(&sid(), req.into()).unwrap_err();
            assert!(matches!(
                err,
                BusinessMsgReject::MissingConditionallyRequiredField { tag: 146 }
            ));
        }
    }

    // ---- NewOrderSingle (D) → New ack + fill ----

    // A limit order gets two ExecutionReports, both accepted by the counterparty's
    // parser: a New ack (nothing filled, LeavesQty = OrderQty), then a full fill at the
    // limit price (LeavesQty 0, CumQty / LastQty = OrderQty, ExecType Trade = 150=F, the
    // wire value Banzai was verified against). Decimals compare exactly.
    #[test]
    fn limit_order_gets_ack_then_fill_at_limit_price() {
        let mut app = SampleApp::new();
        let responses = app.on_app_msg_received(&sid(), inbound(D_LIMIT)).unwrap();
        assert_eq!(responses.len(), 2);
        let mut reports = responses.into_iter().map(|m| {
            let parsed = as_counterparty_sees(m).expect("counterparty accepts the report");
            ExecutionReport::try_from(parsed).unwrap()
        });
        let (ack, fill) = (reports.next().unwrap(), reports.next().unwrap());

        assert_eq!(ack.exec_type().unwrap(), ExecType::New);
        assert_eq!(ack.ord_status().unwrap(), OrdStatus::New);
        assert_eq!(ack.leaves_qty().unwrap(), dec("100"));
        assert_eq!(ack.cum_qty().unwrap(), Decimal::ZERO);
        assert_eq!(ack.avg_px().unwrap(), Decimal::ZERO);

        assert_eq!(fill.exec_type().unwrap(), ExecType::Trade);
        assert_eq!(fill.message().get_body_field::<String>(150).unwrap(), "F");
        assert_eq!(fill.ord_status().unwrap(), OrdStatus::Filled);
        assert_eq!(fill.leaves_qty().unwrap(), Decimal::ZERO);
        assert_eq!(fill.cum_qty().unwrap(), dec("100"));
        assert_eq!(fill.avg_px().unwrap(), dec("101.25"));
        assert_eq!(fill.last_px().unwrap(), dec("101.25"));
        assert_eq!(fill.last_qty().unwrap(), dec("100"));

        // Both echo the order so the client can correlate them.
        for er in [&ack, &fill] {
            assert_eq!(er.cl_ord_id().unwrap(), "ORD-1");
            assert_eq!(er.side().unwrap(), Side::Buy);
            assert_eq!(er.symbol().unwrap(), "AAPL");
        }
        // Same order, so the same OrderID on both; each report its own ExecID.
        assert_eq!(ack.order_id().unwrap(), fill.order_id().unwrap());
        assert_ne!(ack.exec_id().unwrap(), fill.exec_id().unwrap());
    }

    // A market order has no Price(44): the fill reports price 0 instead of failing
    // (the getter's Err is the one optional field the builder must handle).
    #[test]
    fn market_order_fills_at_zero_price() {
        let mut app = SampleApp::new();
        let responses = app.on_app_msg_received(&sid(), inbound(D_MARKET)).unwrap();
        let fill = ExecutionReport::try_from(
            as_counterparty_sees(responses.into_iter().nth(1).unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(fill.side().unwrap(), Side::Sell);
        assert_eq!(fill.cum_qty().unwrap(), dec("50"));
        assert_eq!(fill.avg_px().unwrap(), Decimal::ZERO);
        assert_eq!(fill.last_px().unwrap(), Decimal::ZERO);
    }

    // ---- Anything else ----

    // An app message the sample doesn't handle gets BusinessMessageReject 380=3 with
    // its MsgType: one the dictionary defines (an ExecutionReport arriving inbound) and
    // one it doesn't (Fix43Message::Unknown).
    #[test]
    fn unsupported_message_type_is_business_rejected() {
        for msg_type in ["8", "U1"] {
            let mut msg = Message::new();
            msg.set_header_field(35, msg_type);
            let err = SampleApp::new().on_app_msg_received(&sid(), msg).unwrap_err();
            assert!(
                matches!(&err, BusinessMsgReject::UnknownMessageTye { msg_type: got } if got == msg_type),
                "35={msg_type}: {err:?}"
            );
        }
    }
}
