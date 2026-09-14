use crate::application::Application;
use crate::fix_errors::{BusinessMsgReject, DonotSend, RejectLogon};
use crate::message::Message;
use crate::session::SessionId;

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
    fn build_exec_report(
        &mut self,
        cl_ord_id: &str,
        side: &str,
        symbol: &str,
        qty: &str,
    ) -> Message {
        let mut msg = Message::new();
        msg.set_header_field(35, "8");
        msg.set_body_field(37, self.next_id()); // OrderID
        msg.set_body_field(17, self.next_id()); // ExecID
        msg.set_body_field(150, "0"); // ExecType = New
        msg.set_body_field(39, "0"); // OrdStatus = New
        msg.set_body_field(54, side); // Side (echoed)
        msg.set_body_field(151, qty); // LeavesQty = OrderQty
        msg.set_body_field(14, "0"); // CumQty
        msg.set_body_field(6, "0"); // AvgPx
        msg.set_body_field(11, cl_ord_id); // ClOrdID (echoed)
        msg.set_body_field(55, symbol); // Symbol (echoed)
        msg
    }

    // Builds an ExecutionReport (35=8) reporting a full fill of the order at
    // `price`. This is what makes an execution appear in Banzai's Execution tab:
    // Banzai computes fillSize = OrderQty - LeavesQty, and only records an
    // execution when that is > 0. So a fill must carry LeavesQty(151)=0 with
    // CumQty(14)=qty, plus ExecType(150)=F (Trade) / OrdStatus(39)=2 (Filled),
    // and LastPx(31)/LastQty(32) for the execution row.
    fn build_fill_report(
        &mut self,
        cl_ord_id: &str,
        side: &str,
        symbol: &str,
        qty: &str,
        price: &str,
    ) -> Message {
        let mut msg = Message::new();
        msg.set_header_field(35, "8");
        msg.set_body_field(37, self.next_id()); // OrderID
        msg.set_body_field(17, self.next_id()); // ExecID
        msg.set_body_field(150, "F"); // ExecType = Trade
        msg.set_body_field(39, "2"); // OrdStatus = Filled
        msg.set_body_field(54, side); // Side (echoed)
        msg.set_body_field(151, "0"); // LeavesQty = 0 (fully filled)
        msg.set_body_field(14, qty); // CumQty = OrderQty
        msg.set_body_field(6, price); // AvgPx
        msg.set_body_field(31, price); // LastPx (fill price)
        msg.set_body_field(32, qty); // LastQty (fill size)
        msg.set_body_field(11, cl_ord_id); // ClOrdID (echoed)
        msg.set_body_field(55, symbol); // Symbol (echoed)
        msg
    }

    fn build_md_snapshot(&self, mdreq_id: &str, ccy_pair: &str) -> Message {
        let mut msg = Message::new();
        msg.set_header_field(35, "W");
        msg.set_body_field(262, mdreq_id);
        msg.set_body_field(55, ccy_pair);
        let group = msg.body_mut().set_group(268, 2, 269);
        for i in 0..2 {
            if i % 2 == 0 {
                // bid side
                group[i].set_field(269, "0");
                group[i].set_field(270, "1.4");
                group[i].set_field(271, "1000");
            } else {
                // offer side
                group[i].set_field(269, "1");
                group[i].set_field(270, "1.8");
                group[i].set_field(271, "1000");
            }
        }
        msg
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
        message: &Message,
    ) -> Result<Vec<Message>, BusinessMsgReject> {
        // MsgType(35) is guaranteed: from_str requires it, and next_message routed
        // on it to reach us.
        let msg_type = message.get_msg_type().expect("MsgType(35) present on a routed message");
        match msg_type.as_str() {
            "V" => {
                // MarketDataRequest → MarketDataSnapshotFullRefresh
                // Symbol(55): FIX43 carries it inside the NoRelatedSym group, not at
                // top level, so the engine does NOT guarantee a top-level 55. Our v1
                // simulator contract puts it there; a peer that omits it is a genuine
                // business-level miss → BusinessMessageReject (380=5).
                let ccy_pair = message.get_body_field::<String>(55).map_err(|_| {
                    BusinessMsgReject::MissingConditionallyRequiredField { tag: 55 }
                })?;
                // MDReqID(262) is dictionary-required for V — the engine already
                // rejected any V missing it before dispatch, so it's present here.
                let mdreq_id = message
                    .get_body_field::<String>(262)
                    .expect("MDReqID(262) required for V; engine-validated");
                Ok(vec![self.build_md_snapshot(&mdreq_id, &ccy_pair)])
            }
            "D" => {
                // NewOrderSingle → New ack, then a full fill. Two ExecutionReports:
                // the first (New) acknowledges the order; the second (Trade/Filled)
                // executes it so a row appears in the client's execution blotter.
                // ClOrdID(11), Side(54), Symbol(55, via the required Instrument
                // component) and OrderQty(38, via OrderQtyData) are all
                // dictionary-required for D — the engine validated them before
                // dispatch, so they're guaranteed present here.
                let cl_ord_id =
                    message.get_body_field::<String>(11).expect("ClOrdID(11) required for D");
                let side = message.get_body_field::<String>(54).expect("Side(54) required for D");
                let symbol =
                    message.get_body_field::<String>(55).expect("Symbol(55) required for D");
                let qty =
                    message.get_body_field::<String>(38).expect("OrderQty(38) required for D");
                // Limit price if present (Price, tag 44); market orders have none.
                let price =
                    message.get_body_field::<String>(44).unwrap_or_else(|_| "0".to_string());
                let accept = self.build_exec_report(&cl_ord_id, &side, &symbol, &qty);
                let fill = self.build_fill_report(&cl_ord_id, &side, &symbol, &qty, &price);
                Ok(vec![accept, fill])
            }
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod sample_app_tests {
    use super::*;

    // Builds an inbound MarketDataRequest (35=V) shaped the way on_app_msg_received
    // reads it: MsgType in the header, MDReqID (262) and Symbol (55) in the body
    // (read via message.get_body_field). This mirrors the simulator contract we
    // chose for v1 — a top-level 55 on the request.
    fn market_data_request(mdreq_id: &str, symbol: &str) -> Message {
        let mut msg = Message::new();
        msg.set_header_field(35, "V");
        msg.set_body_field(262, mdreq_id);
        msg.set_body_field(55, symbol);
        msg
    }

    fn sid() -> SessionId {
        SessionId::new("FIX.4.3", "SENDER", "TARGET")
    }

    // A MarketDataRequest produces exactly one MarketDataSnapshotFullRefresh (35=W)
    // that echoes the request's MDReqID (262) and carries the requested symbol.
    #[test]
    fn test_v_request_returns_single_w_snapshot() {
        let mut app = SampleApp::new();
        let req = market_data_request("REQ-1", "EUR/USD");

        let responses = app.on_app_msg_received(&sid(), &req).unwrap();

        assert_eq!(responses.len(), 1);
        let resp = &responses[0];
        assert_eq!(resp.get_msg_type().unwrap(), "W");
        // MDReqID echoed so the client can correlate response to request.
        assert_eq!(resp.get_body_field::<String>(262).unwrap(), "REQ-1");
        assert_eq!(resp.get_body_field::<String>(55).unwrap(), "EUR/USD");
    }

    // The snapshot carries a NoMDEntries (268) group with two entries: a bid
    // (269=0) and an offer (269=1), each with price (270) and size (271).
    #[test]
    fn test_w_snapshot_has_bid_and_offer_entries() {
        let mut app = SampleApp::new();
        let req = market_data_request("REQ-2", "GBP/USD");

        let responses = app.on_app_msg_received(&sid(), &req).unwrap();
        let resp = &responses[0];

        let entries = resp.body().get_group(268).expect("NoMDEntries group present");
        assert_eq!(entries.size(), 2);
        // entry 0 = bid side
        assert_eq!(entries[0].get_field::<String>(269).unwrap(), "0");
        assert_eq!(entries[0].get_field::<String>(270).unwrap(), "1.4");
        assert_eq!(entries[0].get_field::<String>(271).unwrap(), "1000");
        // entry 1 = offer side
        assert_eq!(entries[1].get_field::<String>(269).unwrap(), "1");
        assert_eq!(entries[1].get_field::<String>(270).unwrap(), "1.8");
    }

    // An application message we don't handle is ignored: no response, no error.
    // (35=8 ExecutionReport arriving inbound — nothing for this venue to do.)
    #[test]
    fn test_unhandled_message_returns_empty() {
        let mut app = SampleApp::new();
        let mut other = Message::new();
        other.set_header_field(35, "8");

        let responses = app.on_app_msg_received(&sid(), &other).unwrap();
        assert!(responses.is_empty());
    }

    // A V missing the top-level Symbol (55) is business-rejected. This IS a real,
    // reachable case: the dictionary carries 55 inside the NoRelatedSym group, so
    // the engine does not require a top-level 55 — a spec-compliant V can reach the
    // app without one, and our simulator contract needs it → 380=5.
    // (MDReqID(262), by contrast, is dictionary-required, so the engine rejects a
    // V without it before dispatch — the app never sees that, hence no test here.)
    #[test]
    fn test_v_request_missing_symbol_business_rejects() {
        let mut app = SampleApp::new();
        let mut req = Message::new();
        req.set_header_field(35, "V");
        req.set_body_field(262, "REQ-3"); // has 262, no top-level 55

        let err = app.on_app_msg_received(&sid(), &req).unwrap_err();
        assert!(matches!(err, BusinessMsgReject::MissingConditionallyRequiredField { tag: 55 }));
    }

    // Builds an inbound NewOrderSingle (35=D) the way on_app_msg_received reads
    // it: MsgType in the header; ClOrdID(11), Side(54), Symbol(55), OrderQty(38)
    // in the body. (Banzai sends more — HandlInst, TransactTime, OrdType — but
    // our sample only needs these four.)
    fn new_order_single(cl_ord_id: &str, side: &str, symbol: &str, qty: &str) -> Message {
        let mut msg = Message::new();
        msg.set_header_field(35, "D");
        msg.set_body_field(11, cl_ord_id);
        msg.set_body_field(54, side);
        msg.set_body_field(55, symbol);
        msg.set_body_field(38, qty);
        msg
    }

    // A NewOrderSingle produces two ExecutionReports (35=8): a New ack, then a
    // full fill. The ack (index 0) echoes ClOrdID/Side/Symbol with New status
    // and LeavesQty = OrderQty; the fill (index 1) reports the trade so the
    // client's blotter shows an execution (LeavesQty=0, CumQty=qty).
    #[test]
    fn test_d_order_returns_ack_then_fill() {
        let mut app = SampleApp::new();
        let order = new_order_single("ORD-1", "1", "AAPL", "100");

        let responses = app.on_app_msg_received(&sid(), &order).unwrap();
        assert_eq!(responses.len(), 2);

        // --- index 0: New ack ---
        let ack = &responses[0];
        assert_eq!(ack.get_msg_type().unwrap(), "8");
        assert_eq!(ack.get_body_field::<String>(11).unwrap(), "ORD-1"); // ClOrdID echoed
        assert_eq!(ack.get_body_field::<String>(54).unwrap(), "1"); // Side
        assert_eq!(ack.get_body_field::<String>(55).unwrap(), "AAPL"); // Symbol
        assert_eq!(ack.get_body_field::<String>(150).unwrap(), "0"); // ExecType = New
        assert_eq!(ack.get_body_field::<String>(39).unwrap(), "0"); // OrdStatus = New
        assert_eq!(ack.get_body_field::<String>(151).unwrap(), "100"); // LeavesQty = OrderQty
        assert_eq!(ack.get_body_field::<String>(14).unwrap(), "0"); // CumQty
        assert!(ack.get_body_field::<String>(37).is_ok()); // OrderID present
        assert!(ack.get_body_field::<String>(17).is_ok()); // ExecID present

        // --- index 1: full fill ---
        let fill = &responses[1];
        assert_eq!(fill.get_msg_type().unwrap(), "8");
        assert_eq!(fill.get_body_field::<String>(11).unwrap(), "ORD-1"); // same order
        assert_eq!(fill.get_body_field::<String>(150).unwrap(), "F"); // ExecType = Trade
        assert_eq!(fill.get_body_field::<String>(39).unwrap(), "2"); // OrdStatus = Filled
        assert_eq!(fill.get_body_field::<String>(151).unwrap(), "0"); // LeavesQty = 0
        assert_eq!(fill.get_body_field::<String>(14).unwrap(), "100"); // CumQty = OrderQty
        assert_eq!(fill.get_body_field::<String>(32).unwrap(), "100"); // LastQty = fill size
    }

    // (No "D missing ClOrdID" test: ClOrdID(11), Side(54), Symbol(55), OrderQty(38)
    // are all dictionary-required for D, so the engine rejects a D missing any of
    // them before dispatch — the handler `expect`s their presence. That
    // required-field rejection is the engine's job, covered by message-layer tests.)
}
