//! Segmented Workflow IPC Commands (#16, #17, #18)
//! (`REQ-CMD-016`..`018`, `be-core.md` §10.2.5, `core.md` §3.9).

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::error::IpcError;
use crate::ipc::config_commands::{workflow_config_from_subagent, RuntimeConfigState};
use crate::ipc::dto::{
    CancelWorkflowReq, CancelWorkflowResponseDto, GetWorkflowStatusReq, StartOrResumeWorkflowReq,
    WorkflowExecutionSummaryDto, WorkflowProgressEvent, WorkflowStatusDto,
};
use crate::service::workflow_service::WorkflowService;

// ============================================================================
// IPC #16: start_or_resume_workflow
// ============================================================================

/// Direct execution of IPC #16 `start_or_resume_workflow`.
pub async fn exec_start_or_resume_workflow(
    service: &WorkflowService,
    req: &StartOrResumeWorkflowReq,
    event_tx: Option<mpsc::Sender<WorkflowProgressEvent>>,
) -> Result<WorkflowExecutionSummaryDto, IpcError> {
    service.start_or_resume_workflow(req, event_tx).await
}

/// Append a structured line to `<home>/.merlink/logs/backend.log` so a failed
/// workflow can be diagnosed from outside the terminal output.
fn log_workflow_error(stage: &str, error: &IpcError) {
    use std::io::Write;
    let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) else {
        return;
    };
    let dir = home.join(".merlink").join("logs");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("backend.log"))
    else {
        return;
    };
    let line = serde_json::json!({
        "at": chrono::Utc::now().to_rfc3339(),
        "stage": stage,
        "code": format!("{:?}", error.code),
        "message": error.message,
    });
    let _ = writeln!(file, "{line}");
}

fn log_workflow_event(message: &str) {
    use std::io::Write;
    let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) else {
        return;
    };
    let dir = home.join(".merlink").join("logs");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("backend.log"))
    else {
        return;
    };
    let line = serde_json::json!({
        "at": chrono::Utc::now().to_rfc3339(),
        "stage": "workflow_start",
        "message": message,
    });
    let _ = writeln!(file, "{line}");
}

#[tauri::command]
pub async fn start_or_resume_workflow(
    service: tauri::State<'_, Arc<WorkflowService>>,
    config: tauri::State<'_, Arc<RuntimeConfigState>>,
    req: StartOrResumeWorkflowReq,
    on_event: tauri::ipc::Channel<WorkflowProgressEvent>,
) -> Result<WorkflowExecutionSummaryDto, IpcError> {
    let (tx, mut rx) = mpsc::channel(100);
    let channel_clone = on_event.clone();
    tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            let _ = channel_clone.send(evt);
        }
    });
    let saved = config.subagent_config.read().clone();
    let planner_config = match workflow_config_from_subagent(&saved) {
        Ok(value) => value,
        Err(error) => {
            log_workflow_error("planner_config", &error);
            return Err(error);
        }
    };
    log_workflow_event(&format!(
        "requested draft={} phase={} subphase={:?}",
        req.draft_session_id, req.phase, req.subphase
    ));
    let result = service
        .start_or_resume_workflow_with_config(&req, Some(tx), planner_config)
        .await;
    match &result {
        Err(error) => log_workflow_error("workflow_run", error),
        Ok(summary) if summary.status != "completed" => log_workflow_error(
            "workflow_status",
            &IpcError::new(
                crate::error::ErrorCode::ValidationFailed,
                format!(
                    "workflow finished with status '{}' (recovery_reason: {:?}, tasks {}/{})",
                    summary.status,
                    summary.recovery_reason,
                    summary.completed_tasks,
                    summary.total_tasks
                ),
            ),
        ),
        Ok(_) => {}
    }
    result
}

// ============================================================================
// IPC #17: cancel_workflow
// ============================================================================

/// Direct execution of IPC #17 `cancel_workflow`.
pub async fn exec_cancel_workflow(
    service: &WorkflowService,
    req: &CancelWorkflowReq,
) -> Result<CancelWorkflowResponseDto, IpcError> {
    service.cancel_workflow(req).await
}

#[tauri::command]
pub async fn cancel_workflow(
    service: tauri::State<'_, Arc<WorkflowService>>,
    req: CancelWorkflowReq,
) -> Result<CancelWorkflowResponseDto, IpcError> {
    exec_cancel_workflow(&service, &req).await
}

// ============================================================================
// IPC #18: get_workflow_status
// ============================================================================

/// Direct execution of IPC #18 `get_workflow_status`.
pub fn exec_get_workflow_status(
    service: &WorkflowService,
    req: &GetWorkflowStatusReq,
) -> Result<WorkflowStatusDto, IpcError> {
    service.get_workflow_status(req)
}

#[tauri::command]
pub async fn get_workflow_status(
    service: tauri::State<'_, Arc<WorkflowService>>,
    req: GetWorkflowStatusReq,
) -> Result<WorkflowStatusDto, IpcError> {
    exec_get_workflow_status(&service, &req)
}
