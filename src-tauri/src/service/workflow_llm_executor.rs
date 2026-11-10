//! Live LLM workflow subagent executor (`core.md` §3.9, `be-core.md` §6.3–§6.4).
//!
//! This executor replaces the placeholder implementation for Phase 3 Stage 1 `world_probe`
//! tasks: it asks the configured provider for a `ProbeCandidatesV1` report and validates the
//! shape before the artifact can be sealed. It never fabricates entity codes, never mutates
//! draft state directly, and never claims a write for tasks without an implemented consumer.

use std::sync::Arc;


use crate::domain::business_flow::{BusinessFlowData, ProvenanceSource};
use crate::domain::phase3_artifact::{
    validate_probe_candidates, ArtifactError, WorldProbeReportV1, DEFAULT_MAX_ARTIFACT_BUDGET,
};
use crate::domain::sf06::codec::{canonical_diff, PatchOp};
use crate::domain::tools::registry::CallerCategory;
use crate::domain::tools::write_tools::{ExecutionGuard, ToolCall, ToolExecutor};
use crate::error::{ErrorCode, IpcError};
use crate::providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderMessage, ProviderToolCall, ProviderToolDefinition,
};
use crate::service::workflow_service::{
    TaskExecutionContext, TaskExecutionOutput, WorkflowTaskExecutor,
};

/// Executes workflow tasks through the active LLM provider.
pub struct LlmWorkflowTaskExecutor {
    provider: Arc<dyn LlmProvider>,
    artifact_budget: usize,
    /// Maps probe `tempKey`s to the draft `code`s allocated while writing Phase 3 candidates.
    world_codes: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

impl LlmWorkflowTaskExecutor {
    pub fn new(provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            provider,
            artifact_budget: DEFAULT_MAX_ARTIFACT_BUDGET,
            world_codes: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    pub fn with_budget(provider: Arc<dyn LlmProvider>, artifact_budget: usize) -> Self {
        Self {
            provider,
            artifact_budget,
            world_codes: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Writes the merged Phase 3 candidates through the backend tool executor, parents first.
    /// Candidate content was authored by the probe LLM; this step only allocates codes and
    /// produces canonical patches, so it never lets the model invent draft identity.
    fn write_world_candidates(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        let empty = Vec::new();
        let scope = &ctx.task_spec.assigned_scope;
        let worlds = scope
            .pointer("/candidates/worldSettings")
            .and_then(|value| value.as_array())
            .unwrap_or(&empty);
        let lores = scope
            .pointer("/candidates/loreEntries")
            .and_then(|value| value.as_array())
            .unwrap_or(&empty);

        let executor = ToolExecutor::default();
        let mut state = ctx.snapshot.business_flow_data.clone();
        let mut mutations: Vec<PatchOp> = Vec::new();
        let mut applied = 0u32;

        for candidate in worlds {
            if ctx.cancel_token.is_cancelled() {
                return Err(IpcError::new(ErrorCode::InternalError, "Workflow task cancelled"));
            }
            let temp_key = candidate.get("tempKey").and_then(|v| v.as_str()).unwrap_or_default();
            if temp_key.is_empty() || self.world_codes.lock().unwrap().contains_key(temp_key) {
                continue;
            }
            let display_name = candidate.get("displayName").and_then(|v| v.as_str()).unwrap_or("Setting");
            let parent_code = candidate
                .get("existingParentCode")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| {
                    candidate
                        .get("parentTempKey")
                        .and_then(|v| v.as_str())
                        .and_then(|parent| self.world_codes.lock().unwrap().get(parent).cloned())
                });
            let mut parameters = serde_json::json!({
                "code": unique_code(&state, display_name),
                "display_name": display_name,
                "type": candidate.get("type").and_then(|v| v.as_str()).unwrap_or("CITY"),
                "declared_sources": {"display_name": "ai_generated"},
            });
            if let Some(parent) = parent_code {
                parameters["parent_code"] = serde_json::Value::String(parent);
            }
            for key in ["description", "access"] {
                if let Some(value) = candidate.get(key).and_then(|v| v.as_str()) {
                    parameters[key] = serde_json::Value::String(value.to_string());
                }
            }
            let code = parameters["code"].as_str().unwrap_or_default().to_string();
            let call = ToolCall {
                name: "add_world_setting".to_string(),
                caller: CallerCategory::AiCaller,
                parameters,
                declared_sources: None,
                declared_source: None,
            };
            let before = state.clone();
            let outcome = executor.execute_call(&call, &ExecutionGuard::new(&state));
            if outcome.is_applied() {
                if let Some(next) = outcome.mutated_state() {
                    mutations.extend(
                        canonical_diff(&before, next)
                            .map_err(|code| IpcError::new(code, "Canonical diff failed"))?
                            .ops,
                    );
                    state = next.clone();
                    applied += 1;
                    self.world_codes.lock().unwrap().insert(temp_key.to_string(), code);
                }
            }
        }

        for lore in lores {
            let temp_key = lore.get("tempKey").and_then(|v| v.as_str()).unwrap_or_default();
            if temp_key.is_empty() || self.world_codes.lock().unwrap().contains_key(temp_key) {
                continue;
            }
            let world_code = lore
                .get("existingWorldCode")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| {
                    lore.get("worldTempKey")
                        .and_then(|v| v.as_str())
                        .and_then(|world| self.world_codes.lock().unwrap().get(world).cloned())
                });
            let Some(world_code) = world_code else { continue };
            let display_name = lore.get("displayName").and_then(|v| v.as_str()).unwrap_or("Lore");
            let mut parameters = serde_json::json!({
                "code": unique_lore_code(&state, display_name),
                "world_setting_code": world_code,
                "display_name": display_name,
                "type": lore.get("type").and_then(|v| v.as_str()).unwrap_or("HISTORY"),
                "data": lore.get("data").cloned().unwrap_or_else(|| serde_json::json!({})),
                "declared_sources": {"display_name": "ai_generated"},
            });
            if let Some(description) = lore.get("description").and_then(|v| v.as_str()) {
                parameters["description"] = serde_json::Value::String(description.to_string());
            }
            let code = parameters["code"].as_str().unwrap_or_default().to_string();
            let call = ToolCall {
                name: "add_lore_entry".to_string(),
                caller: CallerCategory::AiCaller,
                parameters,
                declared_sources: None,
                declared_source: None,
            };
            let before = state.clone();
            let outcome = executor.execute_call(&call, &ExecutionGuard::new(&state));
            if outcome.is_applied() {
                if let Some(next) = outcome.mutated_state() {
                    mutations.extend(
                        canonical_diff(&before, next)
                            .map_err(|code| IpcError::new(code, "Canonical diff failed"))?
                            .ops,
                    );
                    state = next.clone();
                    applied += 1;
                    self.world_codes.lock().unwrap().insert(temp_key.to_string(), code);
                }
            }
        }

        Ok(TaskExecutionOutput {
            task_index: ctx.task_index,
            task_spec: ctx.task_spec.clone(),
            task_identity: ctx.task_identity.clone(),
            mutations,
            applied_write_count: applied,
            artifact: None,
        })
    }

    /// Runs a bounded ReAct tool loop for planning tasks. Every mutation is produced by the
    /// backend tool executor and converted to a canonical patch, never authored by the model.
    async fn write_through_tools(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        let tools: Vec<ProviderToolDefinition> = ctx
            .tool_schemas
            .iter()
            .filter(|schema| {
                // Read tools are executed by the read-tools layer, not this executor; the
                // workflow injects the state slice instead so the model never gets a call
                // that can only fail.
                let name = schema.get("name").and_then(|v| v.as_str()).unwrap_or_default();
                !(name.starts_with("get_") || name.starts_with("search_") || name.starts_with("lookup_"))
            })
            .filter_map(|schema| {
                Some(ProviderToolDefinition {
                    name: schema.get("name")?.as_str()?.to_string(),
                    description: schema
                        .get("description")
                        .and_then(|value| value.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    parameters: schema.get("parameters")?.clone(),
                })
            })
            .collect();
        if tools.is_empty() {
            return Ok(TaskExecutionOutput {
                task_index: ctx.task_index,
                task_spec: ctx.task_spec.clone(),
                task_identity: ctx.task_identity.clone(),
                mutations: Vec::new(),
                applied_write_count: 0,
                artifact: None,
            });
        }

        let mut state: BusinessFlowData = ctx.snapshot.business_flow_data.clone();
        let mut messages = vec![
            ProviderMessage::system(format!(
                "{prompt}\n\nTask scope: {scope}\nCall the provided tools to write the required data. \
When the scope is complete, reply with a short confirmation and no tool calls. \
Use only entity codes shown in the context or returned by a previous tool call. \
Headless workflow: the author already authorized automatic writes for this scope; record \
generated content with declared_sources = ai_generated instead of asking for confirmation.",
                prompt = ctx
                    .phase_prompt
                    .replace("{{story_context}}", &render_story_context(&ctx.snapshot.business_flow_data)),
                scope = ctx.task_spec.assigned_scope
            )),
            ProviderMessage::user(task_instruction(ctx)),
        ];
        let task_start = state.clone();
        let mut mutations: Vec<PatchOp> = Vec::new();
        let mut applied = 0u32;
        let executor = ToolExecutor::default();
        let scope = &ctx.task_spec.assigned_scope;
        let pair_count = scope
            .get("pairs")
            .and_then(|value| value.as_array())
            .map(|pairs| pairs.len())
            .unwrap_or(0);
        let arcs_target = scope.get("arcsTarget").and_then(|value| value.as_u64()).unwrap_or(0) as usize;
        let range_size = scope
            .get("range")
            .and_then(|value| value.as_array())
            .filter(|range| range.len() == 2)
            .and_then(|range| {
                let lo = range[0].as_u64()?;
                let hi = range[1].as_u64()?;
                Some((hi.saturating_sub(lo) + 1) as usize)
            })
            .unwrap_or(0);
        let max_turns = (6 * arcs_target + 2 * pair_count + 2 * range_size + 6).clamp(6, 48);

        for _ in 0..max_turns {
            if ctx.cancel_token.is_cancelled() {
                return Err(IpcError::new(ErrorCode::InternalError, "Workflow task cancelled"));
            }
            let request = ProviderChatRequest {
                model: "chat".to_string(),
                messages: messages.clone(),
                tools: Some(tools.clone()),
                temperature: Some(0.2),
                max_tokens: Some(4096),
            };
            let response = tokio::time::timeout(
                std::time::Duration::from_secs(180),
                self.provider.chat_with_tools(&request, ctx.cancel_token.clone()),
            )
            .await
            .map_err(|_| {
                IpcError::new(
                    ErrorCode::InternalError,
                    "Workflow task provider call timed out after 180s",
                )
            })?
            .map_err(|error| error.to_ipc_error())?;

            if response.tool_calls.is_empty() {
                break;
            }

            messages.push(ProviderMessage {
                role: "assistant".to_string(),
                content: response.content.clone().unwrap_or_default(),
                name: None,
                tool_calls: Some(response.tool_calls.clone()),
                tool_call_id: None,
            });

            for call in &response.tool_calls {
                let tool_call = to_tool_call(call);
                let outcome = executor.execute_call(&tool_call, &ExecutionGuard::new(&state));
                let mut result_note = format!("status={}", outcome.status);
                if outcome.is_applied() {
                    if let Some(next) = outcome.mutated_state() {
                        applied += 1;
                        state = next.clone();
                    }
                } else if outcome.is_pending() {
                    result_note.push_str(" pending_confirmation");
                }
                if let Some(results) = serde_json::to_value(&outcome.results).ok() {
                    result_note.push_str(&format!(" results={results}"));
                }
                messages.push(ProviderMessage::tool_result(
                    call.id.clone(),
                    call.name.clone(),
                    result_note,
                ));
            }
        }

        if applied == 0 && !ctx.task_spec.task_kind.starts_with("world_") {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Workflow task '{}' produced no writes for scope {}",
                    ctx.task_spec.task_kind, ctx.task_spec.assigned_scope
                ),
            ));
        }

        // One canonical diff for the whole task: per-call diffs emit duplicate paths for
        // array-level entities and the round-trip verifier rejects those.
        if applied > 0 {
            let patch = canonical_diff(&task_start, &state)
                .map_err(|code| IpcError::new(code, "Canonical diff failed for task writes"))?;
            mutations = patch.ops;
        }

        Ok(TaskExecutionOutput {
            task_index: ctx.task_index,
            task_spec: ctx.task_spec.clone(),
            task_identity: ctx.task_identity.clone(),
            mutations,
            applied_write_count: applied,
            artifact: None,
        })
    }

    async fn probe(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        let scope = ctx.task_spec.assigned_scope.clone();
        let (lo, hi) = probe_range(&scope)?;
        let chapters = chapter_context(&ctx.snapshot.business_flow_data, lo, hi);
        let existing_codes = existing_world_codes(&ctx.snapshot.business_flow_data);

        let system = format!(
            "{}\n\n{}\n\n{}\n\nĐây là workflow tự động không có người dùng trong lượt: không hỏi xác nhận, chỉ trả đúng một JSON object.",
            ctx.phase_prompt
                .replace("{{story_context}}", &render_story_context(&ctx.snapshot.business_flow_data)),
            probe_system_prompt(&ctx.system_language),
            probe_output_contract()
        );
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
            "world_write" => self.write_world_candidates(ctx),
            _ => self.write_through_tools(ctx).await,
        }
    }
}

fn artifact_error(error: ArtifactError, context: &str) -> IpcError {
    IpcError::new(
        ErrorCode::ValidationFailed,
        format!("Invalid Phase 3 {context}: {error}"),
    )
}

/// Explicit per-kind instruction so the model knows exactly what the assigned scope needs.
fn task_instruction(ctx: &TaskExecutionContext) -> String {
    let scope = &ctx.task_spec.assigned_scope;
    let number = |key: &str| scope.get(key).and_then(|value| value.as_u64());
    let range = |key: &str| {
        scope
            .get(key)
            .and_then(|value| value.as_array())
            .filter(|array| array.len() == 2)
            .and_then(|array| Some((array[0].as_u64()?, array[1].as_u64()?)))
    };
    match ctx.task_spec.task_kind.as_str() {
        "outline_window" => {
            let arcs = number("arcsTarget").unwrap_or(1).max(1);
            format!(
                "Hãy viết dàn ý Arc cho phạm vi được giao. Mục tiêu toàn dự án là {arcs} Arc. Mỗi Arc bắt buộc có: display_name, summary, position, motif_code, ít nhất một event block (start_chapter/end_chapter/summary), theme gồm question, message, resolution (kèm resolution_type) và 2-4 luận điểm (add_theme_argument) cho các nhân vật đã có. Gọi add_arc trước để lấy code, rồi add_theme/add_theme_argument/add_arc_event_block với code đó."
            )
        }
        "chapter_outline" => {
            let (lo, hi) = range("writeRange").unwrap_or((1, 1));
            let count = hi.saturating_sub(lo) + 1;
            let arc = scope.get("arcCode").and_then(|value| value.as_str()).unwrap_or("arc");
            format!(
                "Hãy tạo đúng {count} chương cho Arc '{arc}', position {lo}..={hi}. Dùng add_chapter với arc_code='{arc}', position và summary cho từng chương. Sau mỗi chương, tạo ít nhất một cảnh bằng add_scene (chapter_code là code trả về, position bắt đầu từ 1, có summary). Không bỏ trống cảnh và không tạo chương ngoài khoảng position được giao."
            )
        }
        "world_probe" => "Thăm dò bối cảnh theo phạm vi chương được giao và trả về JSON đúng hợp đồng.".to_string(),
        "scene_config" => {
            let (lo, hi) = range("range").unwrap_or((1, 1));
            format!(
                "Cấu hình cảnh cho chương {lo}..={hi}: với từng cảnh gọi set_scene_config (world_setting_code, style_preset_code, narrator_config_code), add_scene_character cho mọi nhân vật có mặt (kèm is_pov đúng ngôi kể) và add_scene_lore_entry khi cảnh dùng lore."
            )
        }
        "char_profile" => {
            let sub = ctx.subphase.unwrap_or(1);
            format!(
                "Hoàn thiện hồ sơ nhân vật sub-phase {sub}. 5.1: add_character_appearance/set_character_appearance. 5.2: add_character_psychological. 5.3: add_character_memory và add_character_competence (domain_code, scene_code bắt buộc, kèm capabilities/limitations). Chỉ ghi cho các nhân vật trong assigned scope."
            )
        }
        "rel_matrix" => {
            let pairs = scope.get("pairs").and_then(|value| value.as_array()).cloned().unwrap_or_default();
            let listed = pairs.iter().filter_map(|pair| pair.as_array()).filter(|pair| pair.len()==2)
                .map(|pair| format!("{}->{}", pair[0].as_str().unwrap_or(""), pair[1].as_str().unwrap_or("")))
                .collect::<Vec<_>>().join(", ");
            format!(
                "Tạo quan hệ có hướng cho các cặp: {listed}. Mỗi cặp gọi add_relationship(from_code, to_code, motif_code), add_relationship_role nếu cần, và set_relationship_state cho các cảnh có thay đổi (ghi đủ 7 chiều với enum NONE/LOW/MODERATE/HIGH/ABSOLUTE)."
            )
        }
        _ => format!("Hãy thực hiện task '{}' trong phạm vi được giao.", ctx.task_spec.task_kind),
    }
}

/// Compact, deterministic State slice injected where `{{story_context}}` is used by chat.
fn render_story_context(flow: &BusinessFlowData) -> String {
    let project = &flow.require.project;
    let name = project.display_name.value.clone().unwrap_or_default();
    let language = project.language.value.clone().unwrap_or_default();
    let arc_target = project.target.arc_count.value.unwrap_or(0);
    let chapter_target = project.target.chapter_count.value.unwrap_or(0);

    let characters: Vec<String> = flow
        .require
        .character
        .iter()
        .map(|character| {
            format!(
                "{} | {} | sex={} | motif={}",
                character.code,
                character.display_name.value.clone().unwrap_or_default(),
                character.biological_sex.value.clone().unwrap_or_default(),
                character.motif_code.value.clone().unwrap_or_default(),
            )
        })
        .collect();

    let mut arcs: Vec<String> = Vec::new();
    for arc in &flow.optional.overview_phase.arcs {
        if let Some(code) = flow
            .optional
            .chapter_phase
            .arcs
            .iter()
            .find(|chapter_arc| chapter_arc.code == arc.code)
            .map(|chapter_arc| chapter_arc.code.clone())
        {
            let positions: Vec<String> = flow
                .optional
                .chapter_phase
                .arcs
                .iter()
                .find(|chapter_arc| chapter_arc.code == code)
                .map(|chapter_arc| {
                    chapter_arc
                        .chapters
                        .iter()
                        .filter_map(|chapter| {
                            chapter.position.value.map(|position| {
                                format!(
                                    "{}:{}:{}",
                                    position,
                                    chapter.code,
                                    chapter.display_name.value.clone().unwrap_or_default()
                                )
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            arcs.push(format!("{code} (chapters: {})", positions.join(", ")));
        }
    }

    let scenes: Vec<String> = flow
        .optional
        .chapter_phase
        .arcs
        .iter()
        .flat_map(|arc| arc.chapters.iter())
        .flat_map(|chapter| {
            chapter.scenes.iter().map(move |scene| {
                format!(
                    "{}:{}:{}",
                    scene.code,
                    scene.display_name.value.clone().unwrap_or_default(),
                    chapter.code
                )
            })
        })
        .collect();

    let world_codes: Vec<String> = flow
        .optional
        .world_building_phase
        .world_settings
        .iter()
        .map(|node| {
            format!(
                "{}:{}:{}",
                node.code,
                node.display_name.value.clone().unwrap_or_default(),
                node.setting_type.value.clone().unwrap_or_default()
            )
        })
        .collect();

    let approved: Vec<String> = flow
        .decision_log
        .iter()
        .map(|entry| {
            let summary: String = entry.summary.chars().take(4000).collect();
            format!("{}: {}", entry.id, summary)
        })
        .collect();

    format!(
        "Project: {name} | language={language} | arc_count={arc_target} | chapter_count={chapter_target}\nCharacters:\n{characters}\nArcs and chapters:\n{arcs}\nWorld settings:\n{world_codes}\nScenes:\n{scenes}\nApproved outlines (decision_log):\n{approved}",
        characters = if characters.is_empty() { "(none)".to_string() } else { characters.join("\n") },
        arcs = if arcs.is_empty() { "(none)".to_string() } else { arcs.join("\n") },
        world_codes = if world_codes.is_empty() { "(none)".to_string() } else { world_codes.join(", ") },
        scenes = if scenes.is_empty() { "(none)".to_string() } else { scenes.join(", ") },
        approved = if approved.is_empty() { "(none)".to_string() } else { approved.join("\n") },
    )
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    for ch in value.chars() {
        if ch.is_alphanumeric() {
            slug.push(ch.to_lowercase().next().unwrap_or(ch));
        } else if !slug.ends_with('_') {
            slug.push('_');
        }
    }
    let trimmed = slug.trim_matches('_').to_string();
    if trimmed.is_empty() { "world".to_string() } else { trimmed }
}

fn unique_code(state: &BusinessFlowData, display_name: &str) -> String {
    let base = format!("ws_{}", slugify(display_name));
    unique_among(state.optional.world_building_phase.world_settings.iter().map(|w| w.code.as_str()), &base)
}

fn unique_lore_code(state: &BusinessFlowData, display_name: &str) -> String {
    let base = format!("lore_{}", slugify(display_name));
    let existing = state
        .optional
        .world_building_phase
        .world_settings
        .iter()
        .flat_map(|world| world.lore_entries.iter().map(|lore| lore.code.as_str()));
    unique_among(existing, &base)
}

fn unique_among<'a>(existing: impl Iterator<Item = &'a str>, base: &str) -> String {
    let taken: std::collections::BTreeSet<&str> = existing.collect();
    if !taken.contains(base) {
        return base.to_string();
    }
    for suffix in 2.. {
        let candidate = format!("{base}_{suffix}");
        if !taken.contains(candidate.as_str()) {
            return candidate;
        }
    }
    base.to_string()
}

/// Converts a provider tool call into the typed tool-call contract, lifting the declared
/// provenance parameters the model supplied inside `parameters`.
fn to_tool_call(call: &ProviderToolCall) -> ToolCall {
    let mut parameters = call.arguments.clone();
    let declared_sources = parameters
        .get("declared_sources")
        .and_then(|value| value.as_object())
        .map(|map| {
            map.iter()
                .filter_map(|(field, source)| {
                    source
                        .as_str()
                        .and_then(parse_source)
                        .map(|parsed| (field.clone(), parsed))
                })
                .collect::<std::collections::BTreeMap<String, ProvenanceSource>>()
        });
    let declared_source = parameters
        .get("declared_source")
        .and_then(|value| value.as_str())
        .and_then(parse_source);
    if let Some(object) = parameters.as_object_mut() {
        object.remove("declared_sources");
        object.remove("declared_source");
    }
    ToolCall {
        name: call.name.clone(),
        caller: CallerCategory::AiCaller,
        parameters,
        declared_sources,
        declared_source,
    }
}

fn parse_source(raw: &str) -> Option<ProvenanceSource> {
    match raw {
        "user" => Some(ProvenanceSource::User),
        "ai_generated" => Some(ProvenanceSource::AiGenerated),
        "user_edited" => Some(ProvenanceSource::UserEdited),
        _ => None,
    }
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

fn probe_output_contract() -> String {
    "Chỉ trả về một JSON object đúng shape ProbeCandidatesV1, không markdown.".to_string()
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

    #[test]
    fn story_context_carries_approved_outline_decisions() {
        let mut flow = BusinessFlowData::new();
        flow.decision_log.push(crate::domain::business_flow::DecisionLogEntry {
            id: "decision-1".into(),
            summary: "5 Arc: Gặp gỡ tình cờ / Thử thách / Khẳng định / Đối đầu / Kết thúc".into(),
            source: crate::domain::business_flow::ProvenanceSource::User,
        });
        let context = render_story_context(&flow);
        assert!(context.contains("Approved outlines (decision_log)"));
        assert!(context.contains("5 Arc: Gặp gỡ tình cờ"));
    }

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
            phase_prompt: "Phase prompt".into(),
            tool_schemas: Vec::new(),
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

    struct ScriptedProvider {
        responses: std::sync::Mutex<std::collections::VecDeque<ProviderChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmProvider for ScriptedProvider {
        fn provider_id(&self) -> &'static str {
            "scripted"
        }

        async fn chat_with_tools(
            &self,
            _req: &ProviderChatRequest,
            _cancel: CancellationToken,
        ) -> Result<ProviderChatResponse, crate::providers::traits::ProviderError> {
            Ok(self
                .responses
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_default())
        }

        async fn stream_completion(
            &self,
            _req: &ProviderStreamRequest,
            _tx: mpsc::Sender<ProviderStreamChunk>,
            _cancel: CancellationToken,
        ) -> Result<ProviderStreamSummary, crate::providers::traits::ProviderError> {
            Err(crate::providers::traits::ProviderError::Internal("not used".into()))
        }
    }

    #[tokio::test]
    async fn write_task_applies_tool_calls_through_backend_and_returns_patches() {
        let calls = std::sync::Mutex::new(std::collections::VecDeque::from(vec![
            ProviderChatResponse {
                content: Some("add arc".into()),
                tool_calls: vec![ProviderToolCall {
                    id: "call-1".into(),
                    name: "add_arc".into(),
                    arguments: json!({
                        "code": "arc_test",
                        "display_name": "Arc Test",
                        "declared_sources": {"display_name": "ai_generated"}
                    }),
                }],
                ..Default::default()
            },
            ProviderChatResponse::default(),
        ]));
        let provider = Arc::new(ScriptedProvider { responses: calls });
        let (_, mut ctx) = probe_context("{}");
        ctx.task_spec.task_kind = "outline_window".into();
        ctx.tool_schemas = vec![json!({"name": "add_arc", "description": "Add arc", "parameters": {"type": "object"}})];

        let output = LlmWorkflowTaskExecutor::new(provider as Arc<dyn LlmProvider>)
            .execute_task(&ctx)
            .await
            .expect("tool loop must succeed");
        assert!(output.applied_write_count >= 1, "applied tool call must be counted");
        assert!(!output.mutations.is_empty(), "applied mutation must produce patch ops");
    }

    #[tokio::test]
    async fn world_write_creates_parent_child_and_lore_through_tool_executor() {
        let (provider, mut ctx) = probe_context("{}");
        ctx.task_spec.task_kind = "world_write".into();
        ctx.task_spec.assigned_scope = json!({
            "groupKey": "depth_0",
            "range": [1, 2],
            "candidates": {
                "worldSettings": [
                    {"tempKey": "w1", "displayName": "Realm", "type": "REALM"},
                    {"tempKey": "w2", "parentTempKey": "w1", "displayName": "City", "type": "CITY", "description": "Port"}
                ],
                "loreEntries": [
                    {"tempKey": "l1", "worldTempKey": "w2", "displayName": "Fire", "type": "HISTORY", "data": {}}
                ]
            }
        });
        let output = LlmWorkflowTaskExecutor::new(provider as Arc<dyn LlmProvider>)
            .execute_task(&ctx)
            .await
            .expect("world write must succeed");
        assert_eq!(output.applied_write_count, 3, "two worlds and one lore must be written");
        assert!(!output.mutations.is_empty(), "writes must produce canonical patch ops");
    }

    #[tokio::test]
    async fn non_probe_tasks_never_claim_fabricated_mutations() {
        let (provider, mut ctx) = probe_context("{}");
        ctx.task_spec.task_kind = "world_write".into();
        ctx.tool_schemas = Vec::new();
        let output = executor(provider).execute_task(&ctx).await.expect("outcome");
        assert!(output.mutations.is_empty());
        assert_eq!(output.applied_write_count, 0);
        assert!(output.artifact.is_none());
    }
}
