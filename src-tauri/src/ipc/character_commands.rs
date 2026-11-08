//! Character Library IPC Commands (#25, #26, #27, #28)
//! (`REQ-CMD-025`, `REQ-CMD-026`, `REQ-CMD-027`, `REQ-CMD-028`, `be-core.md` §10.2.7).

use std::sync::Arc;

use crate::error::IpcError;
use crate::ipc::dto::{
    CharacterDetailDto, CharacterListDto, CreateOrUpdateLibraryCharacterReq, GetCharacterDetailReq,
    LinkLibraryCharacterResponseDto, LinkLibraryCharacterToDraftReq, ListCharactersReq,
};
use crate::service::character_service::CharacterService;

// ============================================================================
// IPC #25: list_characters
// ============================================================================

/// Direct execution of IPC #25 `list_characters`.
pub fn exec_list_characters(
    service: &CharacterService,
    req: &ListCharactersReq,
) -> Result<CharacterListDto, IpcError> {
    service.list_characters(req)
}

#[tauri::command]
pub async fn list_characters(
    service: tauri::State<'_, Arc<CharacterService>>,
    req: ListCharactersReq,
) -> Result<CharacterListDto, IpcError> {
    exec_list_characters(&service, &req)
}

// ============================================================================
// IPC #26: get_character_detail
// ============================================================================

/// Direct execution of IPC #26 `get_character_detail`.
pub fn exec_get_character_detail(
    service: &CharacterService,
    req: &GetCharacterDetailReq,
) -> Result<CharacterDetailDto, IpcError> {
    service.get_character_detail(req)
}

#[tauri::command]
pub async fn get_character_detail(
    service: tauri::State<'_, Arc<CharacterService>>,
    req: GetCharacterDetailReq,
) -> Result<CharacterDetailDto, IpcError> {
    exec_get_character_detail(&service, &req)
}

// ============================================================================
// IPC #27: create_or_update_library_character
// ============================================================================

/// Direct execution of IPC #27 `create_or_update_library_character`.
pub fn exec_create_or_update_library_character(
    service: &CharacterService,
    req: &CreateOrUpdateLibraryCharacterReq,
) -> Result<CharacterDetailDto, IpcError> {
    service.create_or_update_library_character(req)
}

#[tauri::command]
pub async fn create_or_update_library_character(
    service: tauri::State<'_, Arc<CharacterService>>,
    req: CreateOrUpdateLibraryCharacterReq,
) -> Result<CharacterDetailDto, IpcError> {
    exec_create_or_update_library_character(&service, &req)
}

// ============================================================================
// IPC #28: link_library_character_to_draft
// ============================================================================

/// Direct execution of IPC #28 `link_library_character_to_draft`.
pub async fn exec_link_library_character_to_draft(
    service: &CharacterService,
    req: &LinkLibraryCharacterToDraftReq,
) -> Result<LinkLibraryCharacterResponseDto, IpcError> {
    service.link_library_character_to_draft(req).await
}

#[tauri::command]
pub async fn link_library_character_to_draft(
    service: tauri::State<'_, Arc<CharacterService>>,
    req: LinkLibraryCharacterToDraftReq,
) -> Result<LinkLibraryCharacterResponseDto, IpcError> {
    exec_link_library_character_to_draft(&service, &req).await
}
