//! Project IPC Commands (#22, #23, #24)
//! (`REQ-CMD-022`, `REQ-CMD-023`, `REQ-CMD-024`, `be-core.md` §10.2.7).

use std::sync::Arc;

use crate::error::IpcError;
use crate::ipc::dto::{
    DeleteProjectReq, DeleteProjectResponseDto, GetProjectDetailReq, ListProjectsReq,
    ProjectDetailDto, ProjectListDto,
};
use crate::service::project_service::ProjectService;

// ============================================================================
// IPC #22: list_projects
// ============================================================================

/// Direct execution of IPC #22 `list_projects`.
pub fn exec_list_projects(
    service: &ProjectService,
    req: &ListProjectsReq,
) -> Result<ProjectListDto, IpcError> {
    service.list_projects(req)
}

#[tauri::command]
pub async fn list_projects(
    service: tauri::State<'_, Arc<ProjectService>>,
    req: ListProjectsReq,
) -> Result<ProjectListDto, IpcError> {
    exec_list_projects(&service, &req)
}

// ============================================================================
// IPC #23: get_project_detail
// ============================================================================

/// Direct execution of IPC #23 `get_project_detail`.
pub fn exec_get_project_detail(
    service: &ProjectService,
    req: &GetProjectDetailReq,
) -> Result<ProjectDetailDto, IpcError> {
    service.get_project_detail(req)
}

#[tauri::command]
pub async fn get_project_detail(
    service: tauri::State<'_, Arc<ProjectService>>,
    req: GetProjectDetailReq,
) -> Result<ProjectDetailDto, IpcError> {
    exec_get_project_detail(&service, &req)
}

// ============================================================================
// IPC #24: delete_project
// ============================================================================

/// Direct execution of IPC #24 `delete_project`.
pub fn exec_delete_project(
    service: &ProjectService,
    req: &DeleteProjectReq,
) -> Result<DeleteProjectResponseDto, IpcError> {
    service.delete_project(req)
}

#[tauri::command]
pub async fn delete_project(
    service: tauri::State<'_, Arc<ProjectService>>,
    req: DeleteProjectReq,
) -> Result<DeleteProjectResponseDto, IpcError> {
    exec_delete_project(&service, &req)
}
