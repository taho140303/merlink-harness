//! Test-only backend that replays canned replies so the agent loop can be
//! exercised offline.

use super::{AdapterReply, BoxedFuture, ChatRequest, LlmBackend};

/* ------------------------------------------------------------------ */
/* Test backend                                                        */
/* ------------------------------------------------------------------ */

/// Replays canned replies so the agent loop can be exercised offline.
#[cfg(test)]
#[derive(Default)]
pub struct ScriptedBackend {
    replies: std::sync::Mutex<std::collections::VecDeque<AdapterReply>>,
    requests: std::sync::Mutex<Vec<ChatRequest>>,
}

#[cfg(test)]
impl ScriptedBackend {
    pub fn new(replies: Vec<AdapterReply>) -> Self {
        Self {
            replies: std::sync::Mutex::new(replies.into()),
            requests: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn request_count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    pub fn requests(&self) -> Vec<ChatRequest> {
        self.requests.lock().unwrap().clone()
    }
}

#[cfg(test)]
impl LlmBackend for ScriptedBackend {
    fn complete<'a>(&'a self, request: ChatRequest) -> BoxedFuture<'a, Result<AdapterReply, String>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            self.replies
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| "ScriptedBackend ran out of scripted replies".to_string())
        })
    }
}
