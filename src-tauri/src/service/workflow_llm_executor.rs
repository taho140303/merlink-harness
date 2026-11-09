//! Live LLM workflow subagent executor (`core.md` §3.9, `be-core.md` §6.3–§6.4).
//!
//! This executor replaces the placeholder implementation for Phase 3 Stage 1 `world_probe`
//! tasks: it asks the configured provider for a `ProbeCandidatesV1` report and validates the
//! shape before the artifact can be sealed. It never fabricates entity codes, never mutates
//! draft state directly, and never claims a write for tasks without an implemented consumer.

use std::sync::Arc;


use crate::domain::phase3_artifact::{
    validate_probe_candidates, ArtifactError, WorldProbeReportV1, DEFAULT_MAX_ARTIFACT_BUDGET,
};
use crate::error::{ErrorCode, IpcError};
use crate::providers::traits::{LlmProvider, ProviderChatRequest, ProviderMessage};
use crate::service::workflow_service::{
    TaskExecutionContext, TaskExecutionOutput, WorkflowTaskExecutor,
};

/// Executes workflow tasks through the active LLM provider.
pub struct LlmWorkflowTaskExecutor {
    provider: Arc<dyn LlmProvider>,
    artifact_budget: usize,
}

impl LlmWorkflowTaskExecutor {
    pub fn new(provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            provider,
            artifact_budget: DEFAULT_MAX_ARTIFACT_BUDGET,
        }
    }

    pub fn with_budget(provider: Arc<dyn LlmProvider>, artifact_budget: usize) -> Self {
        Self {
            provider,
            artifact_budget,
        }
    }

    async fn probe(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        let scope = ctx.task_spec.assigned_scope.clone();
        let (lo, hi) = probe_range(&scope)?;
        let chapters = chapter_context(&ctx.snapshot.business_flow_data, lo, hi);
        let existing_codes = existing_world_codes(&ctx.snapshot.business_flow_data);

        let system = probe_system_prompt(&ctx.system_language);
        let user = probe_user_prompt(&scope, &chapters, &existing_codes);

        let request = ProviderChatRequest {
            model: "chat".to_string(),
            messages: vec![ProviderMessage::system(system), ProviderMessage::user(user)],
            tools: None,
            temperature: Some(0.2),
            max_tokens: Some(2048),
        };

        let response = self
            .provider
            .chat_with_tools(&request, ctx.cancel_token.clone())
            .await
            .map_err(|error| error.to_ipc_error())?;

        if ctx.cancel_token.is_cancelled() {
            return Err(IpcError::new(ErrorCode::InternalError, "Workflow task cancelled"));
        }

        let raw = response
            .content
            .ok_or_else(|| IpcError::new(ErrorCode::ValidationFailed, "Probe subagent returned no content"))?;
        let report_text = strip_code_fences(&raw);

        let report = WorldProbeReportV1::new(scope_string(&scope), report_text.clone());
        report
            .encode_and_hash(Some(self.artifact_budget))
            .map_err(|error| artifact_error(error, "probe report"))?;
        validate_probe_candidates(&report_text, &existing_codes)
            .map_err(|error| artifact_error(error, "probe candidates"))?;

        let artifact = serde_json::to_value(&report)
            .map_err(|error| IpcError::new(ErrorCode::InternalError, error.to_string()))?;

        Ok(TaskExecutionOutput {
            task_index: ctx.task_index,
            task_spec: ctx.task_spec.clone(),
            task_identity: ctx.task_identity.clone(),
            mutations: Vec::new(),
            applied_write_count: 0,
            artifact: Some(artifact),
        })
    }
}

#[async_trait::async_trait]
impl WorkflowTaskExecutor for LlmWorkflowTaskExecutor {
    async fn execute_task(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        match ctx.task_spec.task_kind.as_str() {
            "world_probe" => self.probe(ctx).await,
            // Write stages consume a ratified merger artifact that this executor does not
            // fabricate. Returning a zero-mutation outcome keeps the round truthful: no
            // candidate may be claimed as written without an executed tool call.
            _ => Ok(TaskExecutionOutput {
                task_index: ctx.task_index,
                task_spec: ctx.task_spec.clone(),
                task_identity: ctx.task_identity.clone(),
                mutations: Vec::new(),
                applied_write_count: 0,
                artifact: None,
            }),
        }
    }
}

fn artifact_error(error: ArtifactError, context: &str) -> IpcError {
    IpcError::new(
        ErrorCode::ValidationFailed,
        format!("Invalid Phase 3 {context}: {error}"),
    )
}

/// Accepts either `{range:[lo,hi]}` or `{contextRange:[lo,hi]}` task scopes.
fn probe_range(scope: &serde_json::Value) -> Result<(i32, i32), IpcError> {
    let array = scope
        .get("contextRange")
        .filter(|value| !value.is_null())
        .or_else(|| scope.get("range"))
        .and_then(|value| value.as_array())
        .filter(|array| array.len() == 2)
        .ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Probe task scope has no chapter range")
        })?;
    let lo = array[0].as_i64().unwrap_or(0) as i32;
    let hi = array[1].as_i64().unwrap_or(0) as i32;
    if lo < 1 || hi < lo {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!("Invalid probe chapter range {lo}..={hi}"),
        ));
    }
    Ok((lo, hi))
}

/// Stable textual identity of a probe scope (stored in the artifact, never a draft `code`).
fn scope_string(scope: &serde_json::Value) -> String {
    serde_json::to_string(scope).unwrap_or_else(|_| "{}".to_string())
}

fn chapter_context(
    flow: &crate::domain::business_flow::BusinessFlowData,
    lo: i32,
    hi: i32,
) -> Vec<String> {
    let mut lines = Vec::new();
    for arc in &flow.optional.chapter_phase.arcs {
        for chapter in &arc.chapters {
            let Some(position) = chapter.position.value else { continue };
            let position = position as i32;
            if position < lo || position > hi {
                continue;
            }
            let name = chapter
                .display_name
                .value
                .clone()
                .unwrap_or_else(|| chapter.code.clone());
            let summary = chapter.summary.value.clone().unwrap_or_default();
            lines.push(format!("{position}. {name} — {summary}"));
        }
    }
    lines.sort();
    lines.dedup();
    lines
}

fn existing_world_codes(flow: &crate::domain::business_flow::BusinessFlowData) -> Vec<String> {
    flow.optional
        .world_building_phase
        .world_settings
        .iter()
        .map(|node| node.code.clone())
        .collect()
}

fn probe_system_prompt(system_language: &str) -> String {
    let language = if system_language == "vi" { "Tiếng Việt" } else { "English" };
    format!(
        "Bạn là subagent thăm dò thế giới (world_probe) cho một dàn ý truyện. \
Chỉ trả về DUY NHẤT một JSON object hợp lệ, không markdown, không giải thích.\n\
Shape bắt buộc:\n\
{{\"schemaVersion\":1,\"worldSettings\":[{{\"tempKey\":\"w1\",\"parentTempKey\":\"w0?\",\"existingParentCode\":\"code?\",\"displayName\":\"...\",\"type\":\"REALM|WORLD|REGION|COUNTRY|CITY|TOWN|FACILITY|STREET|ROOM\",\"description\":\"...\",\"access\":\"PUBLIC|PRIVATE|SECRET|LOCKED|GUARDED|RESTRICTED\"}}],\
\"loreEntries\":[{{\"tempKey\":\"l1\",\"worldTempKey\":\"w1\",\"existingWorldCode\":\"code?\",\"displayName\":\"...\",\"type\":\"CULTURE|HISTORY|FACTION|ITEM\",\"description\":\"...\",\"data\":{{}}}}]}}\n\
Quy tắc: `tempKey` duy nhất trong report; mỗi world chỉ có tối đa một trong `parentTempKey`/`existingParentCode`; \
mỗi lore phải có đúng một trong `worldTempKey`/`existingWorldCode`; chỉ dùng `existing*Code` khi mã đó nằm trong danh sách hiện có; \
`data` luôn là JSON object. Không bịa số Arc/chương hay mã thực thể. Viết nội dung bằng {language}."
    )
}

fn probe_user_prompt(
    scope: &serde_json::Value,
    chapters: &[String],
    existing_codes: &[String],
) -> String {
    format!(
        "Phạm vi task: {scope}\n\nCác chương trong phạm vi:\n{}\n\nMã world setting đã có (chỉ dùng cho existingParentCode): {}",
        if chapters.is_empty() { "(không có chương)".to_string() } else { chapters.join("\n") },
        if existing_codes.is_empty() { "(không có)".to_string() } else { existing_codes.join(", ") },
        scope = serde_json::to_string(scope).unwrap_or_else(|_| "{}".to_string())
    )
}

/// Providers often wrap JSON in a fenced block; the artifact must be raw JSON.
fn strip_code_fences(raw: &str) -> String {
    let trimmed = raw.trim();
    let without_open = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```JSON"))
        .or_else(|| trimmed.strip_prefix("```"))
        .unwrap_or(trimmed);
    without_open
        .strip_suffix("```")
        .unwrap_or(without_open)
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::business_flow::BusinessFlowData;
    use crate::providers::traits::{
        ProviderChatResponse, ProviderStreamChunk, ProviderStreamRequest, ProviderStreamSummary,
    };
    use crate::service::draft_runtime::WorkingState;
    use crate::service::round_seal_service::PlannedTaskSpec;
    use serde_json::json;
    use tokio::sync::mpsc;
    use tokio_util::sync::CancellationToken;
    use uuid::Uuid;

    struct FakeProvider {
        content: String,
        calls: std::sync::Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmProvider for FakeProvider {
        fn provider_id(&self) -> &'static str {
            "fake"
        }

        async fn chat_with_tools(
            &self,
            _req: &ProviderChatRequest,
            _cancel: CancellationToken,
        ) -> Result<ProviderChatResponse, crate::providers::traits::ProviderError> {
            *self.calls.lock().unwrap() += 1;
            Ok(ProviderChatResponse {
                content: Some(self.content.clone()),
                ..ProviderChatResponse::default()
            })
        }

        async fn stream_completion(
            &self,
            _req: &ProviderStreamRequest,
            _tx: mpsc::Sender<ProviderStreamChunk>,
            _cancel: CancellationToken,
        ) -> Result<ProviderStreamSummary, crate::providers::traits::ProviderError> {
            Err(crate::providers::traits::ProviderError::Internal(
                "not used".into(),
            ))
        }
    }

    fn probe_context(content: &str) -> (Arc<FakeProvider>, TaskExecutionContext) {
        let provider = Arc::new(FakeProvider {
            content: content.to_string(),
            calls: std::sync::Mutex::new(0),
        });
        let mut flow = BusinessFlowData::new();
        flow.optional.chapter_phase.arcs = vec![crate::domain::business_flow::ChapterPhaseArcItem {
            code: "arc_one".into(),
            chapters: vec![crate::domain::business_flow::ChapterPhaseChapterItem {
                code: "arc_one-c1".into(),
                position: crate::domain::business_flow::TrackedField::user(1),
                display_name: crate::domain::business_flow::TrackedField::user("Chương 1".to_string()),
                ..Default::default()
            }],
        }];
        let mut working = WorkingState::new(Uuid::new_v4());
        working.business_flow_data = flow;
        let ctx = TaskExecutionContext {
            draft_session_id: Uuid::new_v4(),
            workflow_run_id: Uuid::new_v4(),
            plan_generation: 1,
            phase: 3,
            subphase: None,
            round_index: 0,
            task_index: 0,
            task_spec: PlannedTaskSpec {
                phase: 3,
                subphase: None,
                task_kind: "world_probe".into(),
                assigned_scope: json!({"groupKey": "arc_one", "range": [1, 6], "contextRange": [1, 6]}),
                dependencies: vec![],
                deterministic_identity: Some("p3:s0:world_probe:arc=arc_one:ch=1..6".into()),
            },
            task_identity: "p3:s0:world_probe:arc=arc_one:ch=1..6".into(),
            snapshot: Arc::new(working),
            system_language: "vi".into(),
            cancel_token: CancellationToken::new(),
        };
        (provider, ctx)
    }

    fn executor(provider: Arc<FakeProvider>) -> LlmWorkflowTaskExecutor {
        LlmWorkflowTaskExecutor::new(provider as Arc<dyn LlmProvider>)
    }

    #[tokio::test]
    async fn probe_calls_provider_and_seals_valid_report_artifact() {
        let report = r#"{"schemaVersion":1,"worldSettings":[{"tempKey":"w1","displayName":"City","type":"CITY"}],"loreEntries":[]}"#;
        let (provider, ctx) = probe_context(&format!("```json\n{report}\n```"));
        let output = executor(provider.clone())
            .execute_task(&ctx)
            .await
            .expect("valid probe report must be accepted");

        assert_eq!(*provider.calls.lock().unwrap(), 1, "provider must be called once");
        assert!(output.mutations.is_empty(), "probe tasks never mutate draft state");
        let artifact = output.artifact.expect("probe must seal an artifact");
        assert_eq!(artifact["schemaVersion"], 1);
        assert_eq!(artifact["reportText"].as_str().unwrap(), report);
    }

    #[tokio::test]
    async fn probe_rejects_malformed_or_invalid_candidate_reports() {
        let (provider, ctx) = probe_context("not json at all");
        assert!(executor(provider).execute_task(&ctx).await.is_err());

        let invalid = r#"{"schemaVersion":1,"worldSettings":[{"tempKey":"w1","displayName":"City","type":"NOT_A_TYPE"}],"loreEntries":[]}"#;
        let (provider, ctx) = probe_context(invalid);
        assert!(executor(provider).execute_task(&ctx).await.is_err());
    }

    #[tokio::test]
    async fn non_probe_tasks_never_claim_fabricated_mutations() {
        let (provider, mut ctx) = probe_context("{}");
        ctx.task_spec.task_kind = "world_write".into();
        let output = executor(provider).execute_task(&ctx).await.expect("outcome");
        assert!(output.mutations.is_empty());
        assert_eq!(output.applied_write_count, 0);
        assert!(output.artifact.is_none());
    }
}
