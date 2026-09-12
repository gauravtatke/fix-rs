use crate::application::Application;
use crate::message::{Message, StringField};
use crate::quickfix_errors::{AppError, DonotSend, RejectLogon};
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
        msg.header_mut().set_field(StringField::new(35, "8"));
        msg.body_mut().set_field(StringField::new(37, &self.next_id())); // OrderID
        msg.body_mut().set_field(StringField::new(17, &self.next_id())); // ExecID
        msg.body_mut().set_field(StringField::new(150, "0")); // ExecType = New
        msg.body_mut().set_field(StringField::new(39, "0")); // OrdStatus = New
        msg.body_mut().set_field(StringField::new(54, side)); // Side (echoed)
        msg.body_mut().set_field(StringField::new(151, qty)); // LeavesQty = OrderQty
        msg.body_mut().set_field(StringField::new(14, "0")); // CumQty
        msg.body_mut().set_field(StringField::new(6, "0")); // AvgPx
        msg.body_mut().set_field(StringField::new(11, cl_ord_id)); // ClOrdID (echoed)
        msg.body_mut().set_field(StringField::new(55, symbol)); // Symbol (echoed)
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
        msg.header_mut().set_field(StringField::new(35, "8"));
        msg.body_mut().set_field(StringField::new(37, &self.next_id())); // OrderID
        msg.body_mut().set_field(StringField::new(17, &self.next_id())); // ExecID
        msg.body_mut().set_field(StringField::new(150, "F")); // ExecType = Trade
        msg.body_mut().set_field(StringField::new(39, "2")); // OrdStatus = Filled
        msg.body_mut().set_field(StringField::new(54, side)); // Side (echoed)
        msg.body_mut().set_field(StringField::new(151, "0")); // LeavesQty = 0 (fully filled)
        msg.body_mut().set_field(StringField::new(14, qty)); // CumQty = OrderQty
        msg.body_mut().set_field(StringField::new(6, price)); // AvgPx
        msg.body_mut().set_field(StringField::new(31, price)); // LastPx (fill price)
        msg.body_mut().set_field(StringField::new(32, qty)); // LastQty (fill size)
        msg.body_mut().set_field(StringField::new(11, cl_ord_id)); // ClOrdID (echoed)
        msg.body_mut().set_field(StringField::new(55, symbol)); // Symbol (echoed)
        msg
    }

    fn build_md_snapshot(&self, mdreq_id: &str, ccy_pair: &str) -> Message {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "W"));
        msg.body_mut().set_field(StringField::new(262, mdreq_id));
        msg.body_mut().set_field(StringField::new(55, ccy_pair));
        let group = msg.body_mut().set_group(268, 2, 269);
        for i in 0..2 {
            if i % 2 == 0 {
                // bid side
                group[i].set_field(StringField::new(269, "0"));
                group[i].set_field(StringField::new(270, "1.4"));
                group[i].set_field(StringField::new(271, "1000"));
            } else {
                // offer side
                group[i].set_field(StringField::new(269, "1"));
                group[i].set_field(StringField::new(270, "1.8"));
                group[i].set_field(StringField::new(271, "1000"));
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
    ) -> Result<Vec<Message>, AppError> {
        let msg_type = message.get_msg_type().map_err(|_| AppError::FieldNotFound { tag: 35 })?;
        match msg_type.as_str() {
            "V" => {
                // MarketDataRequest → MarketDataSnapshotFullRefresh
                let ccy_pair = message
                    .get_field::<String>(55)
                    .map_err(|_| AppError::FieldNotFound { tag: 55 })?;
                let mdreq_id = message
                    .get_field::<String>(262)
                    .map_err(|_| AppError::FieldNotFound { tag: 262 })?;
                Ok(vec![self.build_md_snapshot(&mdreq_id, &ccy_pair)])
            }
            "D" => {
                // NewOrderSingle → New ack, then a full fill. Two ExecutionReports:
                // the first (New) acknowledges the order; the second (Trade/Filled)
                // executes it so a row appears in the client's execution blotter.
                let cl_ord_id = message
                    .get_field::<String>(11)
                    .map_err(|_| AppError::FieldNotFound { tag: 11 })?;
                let side = message
                    .get_field::<String>(54)
                    .map_err(|_| AppError::FieldNotFound { tag: 54 })?;
                let symbol = message
                    .get_field::<String>(55)
                    .map_err(|_| AppError::FieldNotFound { tag: 55 })?;
                let qty = message
                    .get_field::<String>(38)
                    .map_err(|_| AppError::FieldNotFound { tag: 38 })?;
                // Limit price if present (Price, tag 44); market orders have none.
                let price = message.get_field::<String>(44).unwrap_or_else(|_| "0".to_string());
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
    // (Message::get_field reads the body). This mirrors the simulator contract we
    // chose for v1 — a top-level 55 on the request.
    fn market_data_request(mdreq_id: &str, symbol: &str) -> Message {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "V"));
        msg.set_field(StringField::new(262, mdreq_id));
        msg.set_field(StringField::new(55, symbol));
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
        assert_eq!(resp.get_field::<String>(262).unwrap(), "REQ-1");
        assert_eq!(resp.get_field::<String>(55).unwrap(), "EUR/USD");
    }

    // The snapshot carries a NoMDEntries (268) group with two entries: a bid
    // (269=0) and an offer (269=1), each with price (270) and size (271).
    #[test]
    fn test_w_snapshot_has_bid_and_offer_entries() {
        let mut app = SampleApp::new();
        let req = market_data_request("REQ-2", "GBP/USD");

        let responses = app.on_app_msg_received(&sid(), &req).unwrap();
        let resp = &responses[0];

        let entries = resp.get_group(268).expect("NoMDEntries group present");
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
        other.header_mut().set_field(StringField::new(35, "8"));

        let responses = app.on_app_msg_received(&sid(), &other).unwrap();
        assert!(responses.is_empty());
    }

    // A V request missing the required MDReqID (262) is rejected with a typed
    // error rather than panicking — a malformed peer message must not crash the
    // session thread.
    #[test]
    fn test_v_request_missing_mdreqid_errors() {
        let mut app = SampleApp::new();
        let mut req = Message::new();
        req.header_mut().set_field(StringField::new(35, "V"));
        req.set_field(StringField::new(55, "EUR/USD")); // has symbol, no 262

        let err = app.on_app_msg_received(&sid(), &req).unwrap_err();
        assert!(matches!(err, AppError::FieldNotFound { tag: 262 }));
    }

    // Likewise for the symbol (55), per our chosen simulator contract.
    #[test]
    fn test_v_request_missing_symbol_errors() {
        let mut app = SampleApp::new();
        let mut req = Message::new();
        req.header_mut().set_field(StringField::new(35, "V"));
        req.set_field(StringField::new(262, "REQ-3")); // has 262, no symbol

        let err = app.on_app_msg_received(&sid(), &req).unwrap_err();
        assert!(matches!(err, AppError::FieldNotFound { tag: 55 }));
    }

    // Builds an inbound NewOrderSingle (35=D) the way on_app_msg_received reads
    // it: MsgType in the header; ClOrdID(11), Side(54), Symbol(55), OrderQty(38)
    // in the body. (Banzai sends more — HandlInst, TransactTime, OrdType — but
    // our sample only needs these four.)
    fn new_order_single(cl_ord_id: &str, side: &str, symbol: &str, qty: &str) -> Message {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "D"));
        msg.set_field(StringField::new(11, cl_ord_id));
        msg.set_field(StringField::new(54, side));
        msg.set_field(StringField::new(55, symbol));
        msg.set_field(StringField::new(38, qty));
        msg
    }

    // A NewOrderSingle produces exactly one ExecutionReport (35=8) that echoes
    // ClOrdID (so Banzai can match the order in its table), Side, and Symbol,
    // and carries the required New-ack fields.
    #[test]
    fn test_d_order_returns_exec_report() {
        let mut app = SampleApp::new();
        let order = new_order_single("ORD-1", "1", "AAPL", "100");

        let responses = app.on_app_msg_received(&sid(), &order).unwrap();

        assert_eq!(responses.len(), 1);
        let er = &responses[0];
        assert_eq!(er.get_msg_type().unwrap(), "8");
        // echoes for correlation / display
        assert_eq!(er.get_field::<String>(11).unwrap(), "ORD-1"); // ClOrdID
        assert_eq!(er.get_field::<String>(54).unwrap(), "1"); // Side
        assert_eq!(er.get_field::<String>(55).unwrap(), "AAPL"); // Symbol
        // required ExecutionReport fields (New ack)
        assert_eq!(er.get_field::<String>(150).unwrap(), "0"); // ExecType = New
        assert_eq!(er.get_field::<String>(39).unwrap(), "0"); // OrdStatus = New
        assert_eq!(er.get_field::<String>(151).unwrap(), "100"); // LeavesQty = OrderQty
        assert_eq!(er.get_field::<String>(14).unwrap(), "0"); // CumQty
        assert_eq!(er.get_field::<String>(6).unwrap(), "0"); // AvgPx
        // OrderID / ExecID present (value is generated, not pinned here)
        assert!(er.get_field::<String>(37).is_ok()); // OrderID
        assert!(er.get_field::<String>(17).is_ok()); // ExecID
    }

    // A NewOrderSingle missing a field the handler needs is rejected with a
    // typed error rather than panicking. ClOrdID (11) shown here.
    #[test]
    fn test_d_order_missing_clordid_errors() {
        let mut app = SampleApp::new();
        let mut order = Message::new();
        order.header_mut().set_field(StringField::new(35, "D"));
        order.set_field(StringField::new(54, "1"));
        order.set_field(StringField::new(55, "AAPL"));
        order.set_field(StringField::new(38, "100")); // no ClOrdID (11)

        let err = app.on_app_msg_received(&sid(), &order).unwrap_err();
        assert!(matches!(err, AppError::FieldNotFound { tag: 11 }));
    }
}
