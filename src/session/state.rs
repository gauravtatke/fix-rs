use std::time::{Duration, Instant};

const LOGON_TIMEOUT_THRESHOLD: Duration = Duration::from_secs(10);
const LOGOUT_TIMEOUT_THRESHOLD: Duration = Duration::from_secs(2);

#[derive(Debug, Clone)]
pub(crate) struct SessionState {
    pub(crate) logon_sent: bool,
    pub(crate) logon_received: bool,
    pub(crate) logout_sent: bool,
    pub(crate) logout_received: bool,
    pub(crate) reset_sent: bool,
    pub(crate) reset_received: bool,
    pub(crate) is_initiator: bool,
    pub(crate) heartbeat_interval: u32,
    pub(crate) last_sent_time: Instant,
    pub(crate) last_received_time: Instant,
    pub(crate) test_request_counter: u32,
    // Sequence number we will stamp on the next outbound message.
    pub(crate) next_sender_msg_seq_num: u32,
    // Sequence number we expect on the next inbound message from the counterparty.
    pub(crate) next_target_msg_seq_num: u32,
}

impl SessionState {
    pub(crate) fn new(heartbeat_int: u32, is_initiator: bool, instant_now: Instant) -> Self {
        Self {
            logon_sent: false,
            logon_received: false,
            logout_sent: false,
            logout_received: false,
            reset_sent: false,
            reset_received: false,
            is_initiator,
            heartbeat_interval: heartbeat_int,
            last_sent_time: instant_now,
            last_received_time: instant_now,
            test_request_counter: 0,
            next_sender_msg_seq_num: 1,
            next_target_msg_seq_num: 1,
        }
    }

    pub(crate) fn is_heartbeat_needed(&self, instant_now: Instant) -> bool {
        instant_now.duration_since(self.last_sent_time)
            >= Duration::from_secs(self.heartbeat_interval as u64)
    }

    pub(crate) fn is_test_request_needed(&self, instant_now: Instant) -> bool {
        let threshold = (self.heartbeat_interval * 3) / 2;
        instant_now.duration_since(self.last_received_time) > Duration::from_secs(threshold as u64)
    }

    pub(crate) fn is_timed_out(&self, instant_now: Instant) -> bool {
        instant_now.duration_since(self.last_received_time)
            > Duration::from_secs(self.heartbeat_interval as u64 * 2)
            && self.test_request_counter > 1
    }

    pub(crate) fn is_logon_timed_out(&self, instant_now: Instant) -> bool {
        self.logon_sent
            && !self.logon_received
            && instant_now.duration_since(self.last_sent_time) > LOGON_TIMEOUT_THRESHOLD
    }

    pub(crate) fn is_logout_timed_out(&self, instant_now: Instant) -> bool {
        self.logout_sent
            && !self.logout_received
            && instant_now.duration_since(self.last_sent_time) > LOGOUT_TIMEOUT_THRESHOLD
    }

    pub(crate) fn incr_next_sender_msg_seq_num(&mut self) {
        self.next_sender_msg_seq_num += 1;
    }

    pub(crate) fn incr_next_target_msg_seq_num(&mut self) {
        self.next_target_msg_seq_num += 1;
    }

    pub(crate) fn reset(&mut self, instant_now: Instant) {
        self.logon_sent = false;
        self.logon_received = false;
        self.logout_sent = false;
        self.logout_received = false;
        self.reset_sent = false;
        self.reset_received = false;
        self.test_request_counter = 0;
        self.next_sender_msg_seq_num = 1;
        self.next_target_msg_seq_num = 1;
        self.last_sent_time = instant_now;
        self.last_received_time = instant_now;
    }
}

#[cfg(test)]
mod session_state_tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn make_state(heartbeat_secs: u32) -> (SessionState, Instant) {
        let now = Instant::now();
        let state = SessionState::new(heartbeat_secs, false, now);
        (state, now)
    }

    #[test]
    fn test_new_initial_values() {
        let (state, _) = make_state(30);
        assert!(!state.logon_sent);
        assert!(!state.logon_received);
        assert!(!state.logout_sent);
        assert!(!state.logout_received);
        assert!(!state.reset_sent);
        assert!(!state.reset_received);
        assert!(!state.is_initiator);
        assert_eq!(state.heartbeat_interval, 30);
        assert_eq!(state.test_request_counter, 0);
        assert_eq!(state.next_sender_msg_seq_num, 1);
        assert_eq!(state.next_target_msg_seq_num, 1);
    }

    #[test]
    fn test_heartbeat_needed_at_interval() {
        let (state, now) = make_state(30);
        assert!(!state.is_heartbeat_needed(now));
        assert!(!state.is_heartbeat_needed(now + Duration::from_secs(29)));
        assert!(state.is_heartbeat_needed(now + Duration::from_secs(30)));
        assert!(state.is_heartbeat_needed(now + Duration::from_secs(31)));
    }

    #[test]
    fn test_test_request_needed_at_1_5x() {
        let (state, now) = make_state(30);
        assert!(!state.is_test_request_needed(now + Duration::from_secs(44)));
        assert!(!state.is_test_request_needed(now + Duration::from_secs(45)));
        assert!(state.is_test_request_needed(now + Duration::from_secs(46)));
    }

    #[test]
    fn test_timed_out_requires_counter_above_1() {
        let (mut state, now) = make_state(30);
        let past_timeout = now + Duration::from_secs(61);

        state.test_request_counter = 0;
        assert!(!state.is_timed_out(past_timeout));

        state.test_request_counter = 1;
        assert!(!state.is_timed_out(past_timeout));

        state.test_request_counter = 2;
        assert!(state.is_timed_out(past_timeout));
    }

    #[test]
    fn test_timed_out_requires_2x_interval() {
        let (mut state, now) = make_state(30);
        state.test_request_counter = 2;

        assert!(!state.is_timed_out(now + Duration::from_secs(59)));
        assert!(!state.is_timed_out(now + Duration::from_secs(60)));
        assert!(state.is_timed_out(now + Duration::from_secs(61)));
    }

    #[test]
    fn test_logon_timed_out() {
        let (mut state, now) = make_state(30);

        // not timed out if logon not sent
        assert!(!state.is_logon_timed_out(now + Duration::from_secs(11)));

        state.logon_sent = true;
        assert!(!state.is_logon_timed_out(now + Duration::from_secs(9)));
        assert!(!state.is_logon_timed_out(now + Duration::from_secs(10)));
        assert!(state.is_logon_timed_out(now + Duration::from_secs(11)));

        // not timed out once logon received
        state.logon_received = true;
        assert!(!state.is_logon_timed_out(now + Duration::from_secs(11)));
    }

    #[test]
    fn test_logout_timed_out() {
        let (mut state, now) = make_state(30);

        assert!(!state.is_logout_timed_out(now + Duration::from_secs(3)));

        state.logout_sent = true;
        assert!(!state.is_logout_timed_out(now + Duration::from_secs(1)));
        assert!(!state.is_logout_timed_out(now + Duration::from_secs(2)));
        assert!(state.is_logout_timed_out(now + Duration::from_secs(3)));

        state.logout_received = true;
        assert!(!state.is_logout_timed_out(now + Duration::from_secs(3)));
    }

    #[test]
    fn test_incr_seq_nums() {
        let (mut state, _) = make_state(30);
        assert_eq!(state.next_sender_msg_seq_num, 1);
        assert_eq!(state.next_target_msg_seq_num, 1);

        state.incr_next_sender_msg_seq_num();
        state.incr_next_sender_msg_seq_num();
        assert_eq!(state.next_sender_msg_seq_num, 3);

        state.incr_next_target_msg_seq_num();
        assert_eq!(state.next_target_msg_seq_num, 2);
    }

    #[test]
    fn test_reset_clears_state_but_preserves_config() {
        let (mut state, now) = make_state(30);

        // dirty up the state
        state.logon_sent = true;
        state.logon_received = true;
        state.logout_sent = true;
        state.logout_received = true;
        state.reset_sent = true;
        state.reset_received = true;
        state.test_request_counter = 5;
        state.next_sender_msg_seq_num = 42;
        state.next_target_msg_seq_num = 37;

        let reset_time = now + Duration::from_secs(100);
        state.reset(reset_time);

        assert!(!state.logon_sent);
        assert!(!state.logon_received);
        assert!(!state.logout_sent);
        assert!(!state.logout_received);
        assert!(!state.reset_sent);
        assert!(!state.reset_received);
        assert_eq!(state.test_request_counter, 0);
        assert_eq!(state.next_sender_msg_seq_num, 1);
        assert_eq!(state.next_target_msg_seq_num, 1);
        assert_eq!(state.last_sent_time, reset_time);
        assert_eq!(state.last_received_time, reset_time);

        // config fields preserved
        assert_eq!(state.heartbeat_interval, 30);
        assert!(!state.is_initiator);
    }

    #[test]
    fn test_reset_preserves_initiator_flag() {
        let now = Instant::now();
        let mut state = SessionState::new(30, true, now);
        assert!(state.is_initiator);

        state.reset(now + Duration::from_secs(10));
        assert!(state.is_initiator);
    }
}
