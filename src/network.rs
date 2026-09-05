use crate::session::{Session, SessionId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub(crate) const SOCKET_ACCEPT_HOST_IP: &str = "127.0.0.1";

/// Immutable, thread-safe registry of all configured sessions.
///
/// Built once at startup via `.collect()` (`FromIterator`) and never modified afterward —
/// the outer `Arc<HashMap>` is read-only. Individual sessions are independently lockable
/// (`Arc<Mutex<Session>>`), so one thread dispatching messages to session A doesn't block
/// another thread running `next_tick` on session B.
///
/// `Clone` is cheap — it bumps the `Arc` refcount, not the map contents.
#[derive(Clone, Default)]
pub struct SessionMap {
    sessions: Arc<HashMap<SessionId, Arc<Mutex<Session>>>>,
}

impl SessionMap {
    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn get(&self, session_id: &SessionId) -> Option<Arc<Mutex<Session>>> {
        self.sessions.get(session_id).cloned()
    }

    pub fn values(&self) -> impl Iterator<Item=Arc<Mutex<Session>>> {
        self.sessions.values().cloned()
    }

    // pub fn insert(&mut self, session_id: &SessionId, session: Arc<Mutex<Session>>) {
    //     self.sessions.insert(session_id.clone(), session);
    // }
}

impl FromIterator<(SessionId, Session)> for SessionMap {
    fn from_iter<T: IntoIterator<Item=(SessionId, Session)>>(iter: T) -> Self {
        let mut session_map = HashMap::new();
        for (session_id, session) in iter {
            session_map.insert(session_id, Arc::new(Mutex::new(session)));
        }
        Self {
            sessions: Arc::new(session_map),
        }
    }
}

/// Spawns a background thread that ticks every session once per second.
/// Each tick drives session-level timers (heartbeat, logon timeout, etc.).
pub fn start_timer(session_map: SessionMap) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            for session_arc in session_map.values() {
                let mut session = session_arc.lock().unwrap();
                let _ = session.next_tick();
            }
        }
    })
}

#[cfg(test)]
mod session_map_tests {
    use super::*;
    use crate::application::DefaultApplication;
    use crate::data_dictionary::DataDictionary;
    use crate::session::schedule::SessionSchedule;
    use crate::session::state::SessionState;
    use std::time::Instant;

    struct StubResponder;
    impl crate::session::Responder for StubResponder {
        fn send(&self, _msg: &str) -> bool {
            true
        }
        fn disconnect(&self) {}
    }

    fn make_test_session(sender: &str, target: &str) -> (SessionId, Session) {
        let id = SessionId::new("FIX.4.3", sender, target);
        let state = SessionState::new(30, false, Instant::now());
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        let dd = DataDictionary::default();
        let app = DefaultApplication::new();
        let session = Session::new(
            id.clone(),
            state,
            schedule,
            Some(Box::new(StubResponder)),
            dd,
            Box::new(app),
        );
        (id, session)
    }

    #[test]
    fn test_collect_single_session() {
        let (id, session) = make_test_session("SENDER", "TARGET");
        let map: SessionMap = vec![(id.clone(), session)].into_iter().collect();

        assert!(map.get(&id).is_some());
    }

    #[test]
    fn test_collect_multiple_sessions() {
        let (id1, s1) = make_test_session("SENDER1", "TARGET1");
        let (id2, s2) = make_test_session("SENDER2", "TARGET2");
        let map: SessionMap = vec![(id1.clone(), s1), (id2.clone(), s2)].into_iter().collect();

        assert!(map.get(&id1).is_some());
        assert!(map.get(&id2).is_some());
    }

    #[test]
    fn test_get_unknown_session_returns_none() {
        let (id, session) = make_test_session("SENDER", "TARGET");
        let map: SessionMap = vec![(id, session)].into_iter().collect();

        let unknown = SessionId::new("FIX.4.3", "NOBODY", "NOWHERE");
        assert!(map.get(&unknown).is_none());
    }

    #[test]
    fn test_values_returns_all_sessions() {
        let (id1, s1) = make_test_session("SENDER1", "TARGET1");
        let (id2, s2) = make_test_session("SENDER2", "TARGET2");
        let (id3, s3) = make_test_session("SENDER3", "TARGET3");
        let map: SessionMap = vec![(id1, s1), (id2, s2), (id3, s3)].into_iter().collect();

        let count = map.values().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_clone_shares_same_sessions() {
        let (id, session) = make_test_session("SENDER", "TARGET");
        let map: SessionMap = vec![(id.clone(), session)].into_iter().collect();
        let cloned_map = map.clone();

        let arc1 = map.get(&id).unwrap();
        let arc2 = cloned_map.get(&id).unwrap();
        assert!(Arc::ptr_eq(&arc1, &arc2));
    }

    #[test]
    fn test_empty_map() {
        let map: SessionMap = std::iter::empty().collect();

        assert!(map.get(&SessionId::new("FIX.4.3", "A", "B")).is_none());
        assert_eq!(map.values().count(), 0);
    }
}
