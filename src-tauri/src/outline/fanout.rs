//! Bounded parallel fan-out over the provider adapter.
//!
//! Every shard is one provider call with no tools; parse/validate failures are
//! retried twice with the error fed back. Results are returned to the caller so
//! the facade can store proposals; `sink` only carries progress events.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use serde_json::Value;
use tokio::{
    sync::{Semaphore, mpsc::UnboundedSender},
    task::JoinSet,
};

use super::contract::{self, Lane};
use crate::{
    adapters::LlmBackend,
    models::{ModelConfig, ProviderConfig},
    runtime::complete,
};

#[derive(Clone, Debug)]
pub struct Shard {
    pub lane: Lane,
    pub key: String,
    pub system: String,
    pub user: String,
}

#[derive(Clone, Debug, Default)]
pub struct RunStats {
    pub calls: usize,
    pub failures: usize,
    pub tokens_in: u64,
    pub tokens_out: u64,
}

#[derive(Clone, Debug)]
pub enum ShardEvent {
    Started {
        key: String,
        lane: Lane,
    },
    Finished {
        key: String,
        lane: Lane,
        result: Result<Value, String>,
    },
}

/// Result of one shard, keyed by its shard key.
pub type ShardResult = (String, Result<Value, String>);

/// What one spawned task hands back: the shard, its parsed answer, the number of
/// provider calls it took and the tokens it spent.
type ShardTask = (Shard, Result<Value, String>, usize, u64, u64);

const MAX_PARSE_RETRIES: usize = 2;

/// Runs every shard with at most `concurrency` provider calls in flight.
///
/// A cancelled run stops launching new shards; shards already in flight still
/// report their outcome so the run is resumable (`pending` items are simply
/// requested again).
pub async fn run_shards(
    backend: Arc<dyn LlmBackend>,
    provider: ProviderConfig,
    model: ModelConfig,
    shards: Vec<Shard>,
    concurrency: usize,
    cancel: Arc<AtomicBool>,
    sink: UnboundedSender<ShardEvent>,
) -> (RunStats, Vec<ShardResult>) {
    let limit = concurrency.clamp(1, 8);
    let semaphore = Arc::new(Semaphore::new(limit));
    let mut pending = shards.into_iter();
    let mut tasks: JoinSet<ShardTask> = JoinSet::new();
    let mut stats = RunStats::default();
    let mut results: Vec<ShardResult> = Vec::new();

    let launch = |shard: Shard, tasks: &mut JoinSet<ShardTask>| {
        let backend = Arc::clone(&backend);
        let provider = provider.clone();
        let model = model.clone();
        let semaphore = Arc::clone(&semaphore);
        let sink = sink.clone();
        let cancel = Arc::clone(&cancel);
        let _ = sink.send(ShardEvent::Started {
            key: shard.key.clone(),
            lane: shard.lane,
        });
        tasks.spawn(async move {
            let mut retries = 0usize;
            let mut user = shard.user.clone();
            let mut calls = 0usize;
            let mut tokens_in = 0u64;
            let mut tokens_out = 0u64;
            let result = loop {
                let Ok(_permit) = semaphore.acquire().await else {
                    break Err("Đã hủy lượt chạy".to_owned());
                };
                if cancel.load(Ordering::Relaxed) {
                    break Err("Đã hủy lượt chạy".to_owned());
                }
                calls += 1;
                match complete(
                    backend.as_ref(),
                    &provider,
                    &model,
                    &shard.system,
                    &user,
                )
                .await
                {
                    Ok(completion) => {
                        if let Some(usage) = completion.usage {
                            tokens_in += usage.input_tokens;
                            tokens_out += usage.output_tokens;
                        }
                        match contract::parse(shard.lane, &completion.text) {
                            Ok(value) => break Ok(value),
                            Err(error) if retries < MAX_PARSE_RETRIES => {
                                retries += 1;
                                user = format!(
                                    "{user}\nLần trước lỗi: {error}. Chỉ trả một object JSON hợp lệ \
                                     bằng tiếng Việt, không markdown fence."
                                );
                            }
                            Err(error) => break Err(error),
                        }
                    }
                    Err(error) => break Err(error),
                }
            };
            (shard, result, calls, tokens_in, tokens_out)
        });
    };

    while tasks.len() < limit && !cancel.load(Ordering::Relaxed) {
        match pending.next() {
            Some(shard) => launch(shard, &mut tasks),
            None => break,
        }
    }

    while let Some(joined) = tasks.join_next().await {
        match joined {
            Ok((shard, result, calls, tokens_in, tokens_out)) => {
                stats.calls += calls;
                stats.tokens_in += tokens_in;
                stats.tokens_out += tokens_out;
                if result.is_err() {
                    stats.failures += 1;
                }
                let _ = sink.send(ShardEvent::Finished {
                    key: shard.key.clone(),
                    lane: shard.lane,
                    result: result.clone(),
                });
                results.push((shard.key, result));
            }
            Err(_) => stats.failures += 1,
        }
        if !cancel.load(Ordering::Relaxed)
            && let Some(shard) = pending.next()
        {
            launch(shard, &mut tasks);
        }
    }

    (stats, results)
}
