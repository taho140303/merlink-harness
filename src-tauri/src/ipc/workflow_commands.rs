//! Segmented Workflow IPC Commands (#16, #17, #18)
//! (`REQ-CMD-016`..`018`, `be-core.md` §10.2.5, `core.md` §3.9).

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::error::IpcError;
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

#[tauri::command]
pub async fn start_or_resume_workflow(
    service: tauri::State<'_, Arc<WorkflowService>>,
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
    exec_start_or_resume_workflow(&service, &req, Some(tx)).await
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
