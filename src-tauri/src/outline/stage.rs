use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    Seed,
    Arcs,
    Chapters,
    Cast,
    World,
    Scenes,
    Ready,
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::Arcs => "arcs",
            Self::Chapters => "chapters",
            Self::Cast => "cast",
            Self::World => "world",
            Self::Scenes => "scenes",
            Self::Ready => "ready",
        }
    }
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "seed" => Ok(Self::Seed),
            "arcs" => Ok(Self::Arcs),
            "chapters" => Ok(Self::Chapters),
            "cast" => Ok(Self::Cast),
            "world" => Ok(Self::World),
            "scenes" => Ok(Self::Scenes),
            "ready" => Ok(Self::Ready),
            _ => Err(format!("Giai đoạn không hợp lệ: {value}")),
        }
    }
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Seed => Some(Self::Arcs),
            Self::Arcs => Some(Self::Chapters),
            Self::Chapters => Some(Self::Cast),
            Self::Cast | Self::World => Some(Self::Scenes),
            Self::Scenes => Some(Self::Ready),
            Self::Ready => None,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Seed => "Chuẩn bị",
            Self::Arcs => "Cốt truyện",
            Self::Chapters => "Chương",
            Self::Cast => "Nhân vật",
            Self::World => "Thế giới",
            Self::Scenes => "Cảnh",
            Self::Ready => "Sẵn sàng",
        }
    }
}

pub fn gate(stage: Stage, committed: &BTreeSet<Stage>) -> Result<(), String> {
    let required: &[Stage] = match stage {
        Stage::Seed => &[],
        Stage::Arcs => &[Stage::Seed],
        Stage::Chapters => &[Stage::Arcs],
        Stage::Cast | Stage::World => &[Stage::Chapters],
        Stage::Scenes => &[Stage::Cast, Stage::World],
        Stage::Ready => &[Stage::Scenes],
    };
    for missing in required {
        if !committed.contains(missing) {
            return Err(format!("Cần chốt giai đoạn {} trước", missing.label()));
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Catalogs,
    ProjectLeaves,
    ProjectGenres,
    Arcs,
    Chapters,
    Characters,
    WorldSettings,
    LoreEntries,
    PowerSystems,
    Appearances,
    CharacterMotifs,
    Competences,
    Memories,
    MemoryLinks,
    Relationships,
    RelationshipStates,
    RelationshipRoles,
    RelationshipMotifs,
    Themes,
    ThemeArguments,
    ProjectMotifs,
    Scenes,
    SceneCharacters,
    SceneRelationships,
    MemorySourceLinks,
    SceneContent,
}

pub const STATE_ORDER: &[Entity] = &[
    Entity::Catalogs,
    Entity::ProjectLeaves,
    Entity::ProjectGenres,
    Entity::Arcs,
    Entity::Chapters,
    Entity::Characters,
    Entity::WorldSettings,
    Entity::LoreEntries,
    Entity::PowerSystems,
    Entity::Appearances,
    Entity::CharacterMotifs,
    Entity::Competences,
    Entity::Memories,
    Entity::MemoryLinks,
    Entity::Relationships,
    Entity::RelationshipStates,
    Entity::RelationshipRoles,
    Entity::RelationshipMotifs,
    Entity::Themes,
    Entity::ThemeArguments,
    Entity::ProjectMotifs,
    Entity::Scenes,
    Entity::SceneCharacters,
    Entity::SceneRelationships,
    Entity::MemorySourceLinks,
    Entity::SceneContent,
];

impl Entity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Catalogs => "catalogs",
            Self::ProjectLeaves => "project",
            Self::ProjectGenres => "project_genres",
            Self::Arcs => "arcs",
            Self::Chapters => "chapters",
            Self::Characters => "characters",
            Self::WorldSettings => "world_settings",
            Self::LoreEntries => "lore_entries",
            Self::PowerSystems => "power_systems",
            Self::Appearances => "character_appearances",
            Self::CharacterMotifs => "character_motifs",
            Self::Competences => "competence_domains",
            Self::Memories => "memories",
            Self::MemoryLinks => "memory_emotions/memory_references/memory_connections",
            Self::Relationships => "relationships",
            Self::RelationshipStates => "relationship_states",
            Self::RelationshipRoles => "relationship_roles",
            Self::RelationshipMotifs => "relationship_motifs",
            Self::Themes => "themes",
            Self::ThemeArguments => "theme_arguments",
            Self::ProjectMotifs => "project_motifs",
            Self::Scenes => "scenes",
            Self::SceneCharacters => "scene_characters",
            Self::SceneRelationships => "scenes.relationships",
            Self::MemorySourceLinks => "memories.source_scene_id",
            Self::SceneContent => "scenes.content",
        }
    }
}

pub fn assert_order(writes: &[Entity]) -> Result<(), String> {
    let mut previous = 0usize;
    for (index, entity) in writes.iter().enumerate() {
        let rank = STATE_ORDER
            .iter()
            .position(|entry| entry == entity)
            .unwrap();
        if index > 0 && rank < previous {
            return Err(format!("Ghi state sai thứ tự tại {}", entity.as_str()));
        }
        previous = rank;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_order_is_sorted_and_complete() {
        assert_eq!(STATE_ORDER.len(), 26);
        assert!(assert_order(&[Entity::Arcs, Entity::Chapters]).is_ok());
        assert!(assert_order(&[Entity::Chapters, Entity::Arcs]).is_err());
    }
    #[test]
    fn test_stage_22_gate_requires_committed_chapters() {
        let committed = BTreeSet::from([Stage::Arcs]);
        assert!(
            gate(Stage::Cast, &committed)
                .unwrap_err()
                .contains("Chương")
        );
        assert!(
            gate(Stage::World, &committed)
                .unwrap_err()
                .contains("Chương")
        );
        let committed = BTreeSet::from([Stage::Chapters]);
        assert!(gate(Stage::Cast, &committed).is_ok());
        assert!(gate(Stage::World, &committed).is_ok());
    }

    #[test]
    fn test_scenes_requires_cast_and_world() {
        let committed = BTreeSet::from([Stage::Cast]);
        assert!(
            gate(Stage::Scenes, &committed)
                .unwrap_err()
                .contains("Thế giới")
        );
    }
}
