//! Merlink Harness Tauri v2 Modular Monolith Library Root (REQ-ARCH-001, REQ-ARCH-002, REQ-OPS-001, P6S2).
//!
//! Exposes top-level module roots (`domain`, `storage`, `service`, `ipc`, `providers`)
//! and the authoritative Tauri Builder registering all 40 typed IPC commands, 3 streaming channels,
//! database migrations, and deterministic startup crash recovery.

pub mod domain;
pub mod error;
pub mod ipc;
pub mod providers;
pub mod service;
pub mod storage;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::domain::workflow_planner::WorkflowConfig;
use crate::ipc::config_commands::RuntimeConfigState;
use crate::providers::ollama::OllamaProvider;
use crate::providers::traits::LlmProvider;
use crate::service::character_service::CharacterService;
use crate::service::chat_orchestrator::ChatOrchestrator;
use crate::service::commit_service::CommitService;
use crate::service::draft_runtime::DraftRuntimeRegistry;
use crate::service::draft_service::DraftService;
use crate::service::patch_service::PatchService;
use crate::service::project_service::ProjectService;
use crate::service::recovery_service::{run_startup_recovery, StartupRecoveryReport};
use crate::service::workflow_service::WorkflowService;
use crate::service::writing_generation::{WritingGenerationService, WritingGenerationState};
use crate::service::writing_persistence::WritingPersistenceService;
use crate::storage::db::DbManager;

/// Authoritative AppState structure (§1.4.2).
pub struct AppState {
    pub db: DbManager,
    pub drafts: Arc<DraftRuntimeRegistry>,
    pub writing: Arc<std::sync::RwLock<WritingGenerationState>>,
    pub config: Arc<RuntimeConfigState>,
    pub app_shutdown_token: CancellationToken,
}

/// Complete application bootstrap context.
pub struct AppBootstrap {
    pub state: AppState,
    pub draft_service: Arc<DraftService>,
    pub patch_service: Arc<PatchService>,
    pub commit_service: Arc<CommitService>,
    pub character_service: Arc<CharacterService>,
    pub project_service: Arc<ProjectService>,
    pub chat_orchestrator: Arc<ChatOrchestrator>,
    pub workflow_service: Arc<WorkflowService>,
    pub writing_generation_service: Arc<WritingGenerationService>,
    pub writing_persistence_service: Arc<WritingPersistenceService>,
    pub provider: Arc<dyn LlmProvider>,
    pub recovery_report: StartupRecoveryReport,
}

impl AppBootstrap {
    /// Bootstraps database, migrations, services, and startup crash recovery.
    pub fn new(app_data_dir: &Path, temp_root: &Path) -> Result<Self, crate::error::AppError> {
        // 1. Initialize SQLite & execute migrations
        let db_manager = DbManager::initialize_and_migrate(app_data_dir)
            .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
        let pool = db_manager.pool().clone();

        // 2. Runtime registries and configurations
        let drafts = Arc::new(DraftRuntimeRegistry::new());
        let writing_state = Arc::new(std::sync::RwLock::new(WritingGenerationState::default()));
        let config = Arc::new(RuntimeConfigState::default());
        let app_shutdown_token = CancellationToken::new();

        // 3. Application Services
        let draft_service = Arc::new(DraftService::new(pool.clone(), drafts.clone()));
        let patch_service = Arc::new(PatchService::new(pool.clone(), drafts.clone(), draft_service.clone()));
        let commit_service = Arc::new(CommitService::new(pool.clone(), drafts.clone()));
        let character_service = Arc::new(CharacterService::new(pool.clone(), draft_service.clone()));
        let project_service = Arc::new(ProjectService::new(pool.clone()));
        let chat_orchestrator = Arc::new(ChatOrchestrator::new(
            pool.clone(),
            drafts.clone(),
            draft_service.clone(),
            patch_service.clone(),
        ));
        let workflow_service = Arc::new(WorkflowService::new(
            pool.clone(),
            drafts.clone(),
            draft_service.clone(),
            WorkflowConfig::default(),
        ));
        let writing_generation_service = Arc::new(WritingGenerationService::new(pool.clone()));
        let writing_persistence_service = Arc::new(WritingPersistenceService::new(pool.clone()));
        let provider: Arc<dyn LlmProvider> = Arc::new(OllamaProvider::new("http://localhost:11434", false));

        // 4. Deterministic Startup Crash Recovery
        let recovery_report = run_startup_recovery(&pool, temp_root, Some(&drafts))
            .map_err(|e| crate::error::AppError::Internal(e.message))?;

        let state = AppState {
            db: db_manager,
            drafts,
            writing: writing_state,
            config,
            app_shutdown_token,
        };

        Ok(Self {
            state,
            draft_service,
            patch_service,
            commit_service,
            character_service,
            project_service,
            chat_orchestrator,
            workflow_service,
            writing_generation_service,
            writing_persistence_service,
            provider,
            recovery_report,
        })
    }
}

/// Initializes tracing subscriber with secret redaction filter (§1.4.1).
pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

/// Creates Tauri v2 Builder with all 40 IPC commands registered.
pub fn create_builder() -> tauri::Builder<tauri::Wry> {
    init_tracing();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Group 1: Chat & Session (4)
            crate::ipc::chat_commands::create_or_load_chat_session,
            crate::ipc::chat_commands::list_chat_sessions,
            crate::ipc::send_chat_message,
            crate::ipc::chat_commands::cancel_chat_turn,
            // Group 2: Draft, Working State & Progress (5)
            crate::ipc::draft_commands::get_draft_workspace_state,
            crate::ipc::draft_commands::get_progress_detail,
            crate::ipc::draft_commands::apply_manual_draft_edit,
            crate::ipc::draft_commands::confirm_phase,
            crate::ipc::chat_commands::regenerate_draft_phase,
            // Group 3: Versioning & Restore (2)
            crate::ipc::draft_commands::list_draft_versions,
            crate::ipc::draft_commands::restore_draft_version,
            // Group 4: Pending Patches (4)
            crate::ipc::patch_commands::list_pending_patches,
            crate::ipc::patch_commands::confirm_pending_patch,
            crate::ipc::patch_commands::reject_pending_patch,
            crate::ipc::patch_commands::confirm_all_pending_patches,
            // Group 5: Segmented Workflow (3)
            crate::ipc::workflow_commands::start_or_resume_workflow,
            crate::ipc::workflow_commands::cancel_workflow,
            crate::ipc::workflow_commands::get_workflow_status,
            // Group 6: Start / Commit Draft (3)
            crate::ipc::commit_commands::get_commit_confirmation_preview,
            crate::ipc::commit_commands::commit_draft,
            crate::ipc::commit_commands::reopen_committed_draft,
            // Group 7: Projects & Creator Mode (3)
            crate::ipc::project_commands::list_projects,
            crate::ipc::project_commands::get_project_detail,
            crate::ipc::project_commands::delete_project,
            // Group 8: Character Library (4)
            crate::ipc::character_commands::list_characters,
            crate::ipc::character_commands::get_character_detail,
            crate::ipc::character_commands::create_or_update_library_character,
            crate::ipc::character_commands::link_library_character_to_draft,
            // Group 9: Writing Mode (9)
            crate::ipc::writing_persistence_commands::get_chapter_writing_state,
            crate::ipc::writing_persistence_commands::set_scene_lock,
            crate::ipc::writing_persistence_commands::set_chapter_scenes_lock_bulk,
            crate::ipc::writing_generation_commands::start_scene_generation,
            crate::ipc::writing_generation_commands::cancel_scene_generation,
            crate::ipc::writing_generation_commands::respond_edit_scope_expansion,
            crate::ipc::writing_persistence_commands::list_scene_versions,
            crate::ipc::writing_persistence_commands::save_chapter,
            crate::ipc::writing_persistence_commands::get_chapter_save_receipt,
            // Group 10: Model Config & Credentials (3)
            crate::ipc::config_commands::get_runtime_and_provider_config,
            crate::ipc::config_commands::update_runtime_and_provider_config,
            crate::ipc::config_commands::set_provider_api_key,
            // Auxiliary diagnostics (outside the 40-command master catalog)
            crate::ipc::provider_probe_commands::test_provider_connection,
            crate::ipc::provider_probe_commands::fetch_provider_models,
        ])
}

/// Configures builder with an explicit bootstrap instance.
pub fn configure_builder_with_bootstrap(
    builder: tauri::Builder<tauri::Wry>,
    bootstrap: AppBootstrap,
) -> tauri::Builder<tauri::Wry> {
    builder.setup(move |app| {
        use tauri::Manager;
        app.manage(bootstrap.state.config.clone());
        app.manage(bootstrap.draft_service.clone());
        app.manage(bootstrap.patch_service.clone());
        app.manage(bootstrap.commit_service.clone());
        app.manage(bootstrap.character_service.clone());
        app.manage(bootstrap.project_service.clone());
        app.manage(bootstrap.chat_orchestrator.clone());
        app.manage(bootstrap.workflow_service.clone());
        app.manage(bootstrap.writing_generation_service.clone());
        app.manage(bootstrap.writing_persistence_service.clone());
        app.manage(bootstrap.provider.clone());
        app.manage(bootstrap.state);
        Ok(())
    })
}

/// Creates and configures the production application builder.
pub fn create_app() -> tauri::Builder<tauri::Wry> {
    create_builder().setup(|app| {
        use tauri::Manager;

        // Set the bundled Merlink icon explicitly. In Linux dev runs the window
        // manager may otherwise display the generic Tauri icon.
        if let Some(window) = app.get_webview_window("main") {
            let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon.png"))?;
            window.set_icon(icon)?;
        }

        let app_data_dir = app
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("./app-data"));
        let temp_root = app_data_dir.join("merlink-harness").join("tmp");

        let bootstrap = AppBootstrap::new(&app_data_dir, &temp_root)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;

        app.manage(bootstrap.state.config.clone());
        app.manage(bootstrap.draft_service.clone());
        app.manage(bootstrap.patch_service.clone());
        app.manage(bootstrap.commit_service.clone());
        app.manage(bootstrap.character_service.clone());
        app.manage(bootstrap.project_service.clone());
        app.manage(bootstrap.chat_orchestrator.clone());
        app.manage(bootstrap.workflow_service.clone());
        app.manage(bootstrap.writing_generation_service.clone());
        app.manage(bootstrap.writing_persistence_service.clone());
        app.manage(bootstrap.provider.clone());
        app.manage(bootstrap.state);

        Ok(())
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    create_app()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
