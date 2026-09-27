use super::registry::SessionMap;
use std::thread;
use std::time::Duration;

/// Spawns a background thread that ticks every session once per second.
/// Each tick drives session-level timers (heartbeat, logon timeout, etc.) and
/// then drains any unsolicited outbound messages the app has queued
/// (`poll_outbound`) — e.g. a streaming market-data feed.
pub fn start_timer(session_map: SessionMap) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            for session_arc in session_map.values() {
                let mut session = session_arc.lock().unwrap();
                session.next_tick();
                let _ = session.poll_outbound();
            }
        }
    })
}
