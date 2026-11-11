//! Project Service orchestrating project catalog and creation management
//! (`REQ-PROJECT-001`, IPC #22, #23, #24, `be-core.md` §10.2.7).

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    DeleteProjectReq, DeleteProjectResponseDto, GetProjectDetailReq, ListProjectsReq,
    ProjectDetailDto, ProjectListDto,
};
use crate::storage::db::DbPool;
use crate::storage::project_repo;

/// Service coordinating project queries, detail projections, and scoped soft-deletions.
pub struct ProjectService {
    pool: DbPool,
}

impl ProjectService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// IPC #22: `list_projects`
    pub fn list_projects(&self, req: &ListProjectsReq) -> Result<ProjectListDto, IpcError> {
        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire DB connection: {}", e),
            )
        })?;

        let items = project_repo::list_projects(&conn, req.query.as_deref(), req.sort.as_deref())?;

        Ok(ProjectListDto { items })
    }

    /// IPC #23: `get_project_detail`
    pub fn get_project_detail(
        &self,
        req: &GetProjectDetailReq,
    ) -> Result<ProjectDetailDto, IpcError> {
        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire DB connection: {}", e),
            )
        })?;

        project_repo::get_project_detail(&conn, &req.project_id)
    }

    /// IPC #24: `delete_project`
    pub fn delete_project(
        &self,
        req: &DeleteProjectReq,
    ) -> Result<DeleteProjectResponseDto, IpcError> {
        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire DB connection: {}", e),
            )
        })?;

        let deleted = project_repo::delete_project(&mut conn, &req.project_id)?;
        Ok(DeleteProjectResponseDto { deleted })
    }
}
