//! Live progress events for one agent turn.

use std::time::Instant;

use crate::models::AgentEvent;

/// Sink for live turn progress (D1/D4).
///
/// Hand-rolled like [`LlmBackend`] instead of `async-trait`: the production
/// implementation wraps a Tauri `Channel`, tests wrap [`RecordingSink`].
pub trait ProgressSink: Send + Sync {
    fn emit(&self, event: AgentEvent);
}

/// Drops every event, for callers that do not render progress.
#[derive(Default)]
#[allow(dead_code)] // kept alongside `ChannelSink`; tests are its only user today
pub struct NoopSink;

impl ProgressSink for NoopSink {
    fn emit(&self, _event: AgentEvent) {}
}

/// Forwards each event to the UI over the `Channel` passed to `send_chat` (D1).
pub(super) struct ChannelSink(pub(super) tauri::ipc::Channel<AgentEvent>);

impl ProgressSink for ChannelSink {
    fn emit(&self, event: AgentEvent) {
        // A channel closed mid-turn (window gone) must not abort the agent loop.
        let _ = self.0.send(event);
    }
}

/// Milliseconds as `u64`, saturating instead of wrapping.
pub(super) fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

/// Test-only sink that keeps every event together with the instant it arrived,
/// which is how the streaming tests prove events are emitted during the turn
/// rather than batched at the end.
#[cfg(test)]
#[derive(Default)]
pub struct RecordingSink {
    events: std::sync::Mutex<Vec<(Instant, AgentEvent)>>,
}

#[cfg(test)]
impl RecordingSink {
    pub fn events(&self) -> Vec<AgentEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .map(|(_, event)| event.clone())
            .collect()
    }

    /// The instant the first event matching `predicate` arrived.
    pub fn first_instant(&self, predicate: impl Fn(&AgentEvent) -> bool) -> Option<Instant> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .find(|(_, event)| predicate(event))
            .map(|(instant, _)| *instant)
    }
}

#[cfg(test)]
impl ProgressSink for RecordingSink {
    fn emit(&self, event: AgentEvent) {
        self.events.lock().unwrap().push((Instant::now(), event));
    }
}
