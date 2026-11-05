import type { BusinessFlowData } from './types';

export interface BusinessFlowProgressResult {
  percent: number;
  filledCount: number;
  totalCount: number;
  isRequireComplete: boolean;
  missingRequireFields: string[];
}

export const MASTER_ATTRIBUTES_LIST: readonly string[] = [
  // Require Phase (10)
  'require.project.display_name',
  'require.project.summary',
  'require.project.language',
  'require.project.target.arc_count',
  'require.project.target.chapter_count',
  'require.project.target.word_per_chapter_count',
  'require.project.genres.code',
  'require.character.display_name',
  'require.character.biological_sex',
  'require.character.motif_code',

  // Optional: Overview Phase (12)
  'optional.overview_phase.project.target.arc_tolerance',
  'optional.overview_phase.project.target.chapter_tolerance',
  'optional.overview_phase.project.target.word_per_chapter_tolerance',
  'optional.overview_phase.arcs.display_name',
  'optional.overview_phase.arcs.summary',
  'optional.overview_phase.arcs.position',
  'optional.overview_phase.arcs.motif.code',
  'optional.overview_phase.arcs.motif.event_blocks',
  'optional.overview_phase.arcs.themes.question',
  'optional.overview_phase.arcs.themes.resolution',
  'optional.overview_phase.arcs.themes.message',
  'optional.overview_phase.arcs.themes.arguments',

  // Optional: Chapter Phase (4)
  'optional.chapter_phase.arcs.chapters.display_name',
  'optional.chapter_phase.arcs.chapters.summary',
  'optional.chapter_phase.arcs.chapters.scenes.display_name',
  'optional.chapter_phase.arcs.chapters.scenes.summary',

  // Optional: World Building Phase (6)
  'optional.world_building_phase.world_settings.display_name',
  'optional.world_building_phase.world_settings.type',
  'optional.world_building_phase.world_settings.description',
  'optional.world_building_phase.world_settings.access',
  'optional.world_building_phase.world_settings.lore_entries.display_name',
  'optional.world_building_phase.world_settings.lore_entries.description',

  // Optional: Scene Config Phase (4)
  'optional.scene_config_phase.arcs.chapters.scenes.style_preset_code',
  'optional.scene_config_phase.arcs.chapters.scenes.narrator_config_code',
  'optional.scene_config_phase.arcs.chapters.scenes.lore_entries_code',
  'optional.scene_config_phase.arcs.chapters.scenes.characters.is_pov',

  // Optional: Character Appearance Phase (6)
  'optional.character_appearance_phase.character.species',
  'optional.character_appearance_phase.character.gender.identity',
  'optional.character_appearance_phase.character.height_cm',
  'optional.character_appearance_phase.character.hair.color',
  'optional.character_appearance_phase.character.skin.tone',
  'optional.character_appearance_phase.character.outfit_style',

  // Optional: Character Psychological Phase (4)
  'optional.charactar_psychological_phase.character.core_trauma_and_belief.the_wound',
  'optional.charactar_psychological_phase.character.cognitive_moral_framework.moral_code',
  'optional.charactar_psychological_phase.character.narrative_engine.external_want',
  'optional.charactar_psychological_phase.character.behavioral_adaptation.social_mask',

  // Optional: Character Life Experience Phase (2)
  'optional.character_life_experience_phase.character.memories',
  'optional.character_life_experience_phase.character.competences',

  // Optional: Relationship Phase (2)
  'optional.relationship_phase.relationship.motif.event_blocks',
  'optional.relationship_phase.relationship.character.scenes',
] as const;

function isValidValue(value: unknown): boolean {
  if (value === null || value === undefined) return false;
  if (typeof value === 'string') return value.trim().length > 0;
  if (typeof value === 'number') return !Number.isNaN(value);
  if (typeof value === 'boolean') return true;
  if (Array.isArray(value)) return value.length > 0;
  if (typeof value === 'object') return Object.keys(value).length > 0;
  return false;
}

const ATTRIBUTE_EVALUATORS: Record<string, (data: BusinessFlowData) => boolean> = {
  // Require Phase
  'require.project.display_name': (d) => isValidValue(d?.require?.project?.display_name),
  'require.project.summary': (d) => isValidValue(d?.require?.project?.summary),
  'require.project.language': (d) => isValidValue(d?.require?.project?.language),
  'require.project.target.arc_count': (d) =>
    isValidValue(d?.require?.project?.target?.arc_count) &&
    (d?.require?.project?.target?.arc_count ?? 0) > 0,
  'require.project.target.chapter_count': (d) =>
    isValidValue(d?.require?.project?.target?.chapter_count) &&
    (d?.require?.project?.target?.chapter_count ?? 0) > 0,
  'require.project.target.word_per_chapter_count': (d) =>
    isValidValue(d?.require?.project?.target?.word_per_chapter_count) &&
    (d?.require?.project?.target?.word_per_chapter_count ?? 0) > 0,
  'require.project.genres.code': (d) =>
    Boolean(d?.require?.project?.genres?.some((g) => isValidValue(g?.code))),
  'require.character.display_name': (d) =>
    Boolean(
      d?.require?.character?.some(
        (c) => isValidValue(c?.display_name) || isValidValue(c?.nickname),
      ),
    ),
  'require.character.biological_sex': (d) =>
    Boolean(d?.require?.character?.some((c) => isValidValue(c?.biological_sex))),
  'require.character.motif_code': (d) =>
    Boolean(d?.require?.character?.some((c) => isValidValue(c?.motif_code))),

  // Optional: Overview Phase
  'optional.overview_phase.project.target.arc_tolerance': (d) =>
    isValidValue(d?.optional?.overview_phase?.project?.target?.arc_tolerance),
  'optional.overview_phase.project.target.chapter_tolerance': (d) =>
    isValidValue(d?.optional?.overview_phase?.project?.target?.chapter_tolerance),
  'optional.overview_phase.project.target.word_per_chapter_tolerance': (d) =>
    isValidValue(d?.optional?.overview_phase?.project?.target?.word_per_chapter_tolerance),
  'optional.overview_phase.arcs.display_name': (d) =>
    Boolean(d?.optional?.overview_phase?.arcs?.some((a) => isValidValue(a?.display_name))),
  'optional.overview_phase.arcs.summary': (d) =>
    Boolean(d?.optional?.overview_phase?.arcs?.some((a) => isValidValue(a?.summary))),
  'optional.overview_phase.arcs.position': (d) =>
    Boolean(d?.optional?.overview_phase?.arcs?.some((a) => isValidValue(a?.position))),
  'optional.overview_phase.arcs.motif.code': (d) =>
    Boolean(d?.optional?.overview_phase?.arcs?.some((a) => isValidValue(a?.motif?.code))),
  'optional.overview_phase.arcs.motif.event_blocks': (d) =>
    Boolean(
      d?.optional?.overview_phase?.arcs?.some(
        (a) => Array.isArray(a?.motif?.event_blocks) && a.motif.event_blocks.length > 0,
      ),
    ),
  'optional.overview_phase.arcs.themes.question': (d) =>
    Boolean(d?.optional?.overview_phase?.arcs?.some((a) => isValidValue(a?.themes?.question))),
  'optional.overview_phase.arcs.themes.resolution': (d) =>
    Boolean(
      d?.optional?.overview_phase?.arcs?.some(
        (a) =>
          isValidValue(a?.themes?.resolution?.content) ||
          isValidValue(a?.themes?.resolution?.type),
      ),
    ),
  'optional.overview_phase.arcs.themes.message': (d) =>
    Boolean(d?.optional?.overview_phase?.arcs?.some((a) => isValidValue(a?.themes?.message))),
  'optional.overview_phase.arcs.themes.arguments': (d) =>
    Boolean(
      d?.optional?.overview_phase?.arcs?.some(
        (a) => Array.isArray(a?.themes?.arguments) && a.themes.arguments.length > 0,
      ),
    ),

  // Optional: Chapter Phase
  'optional.chapter_phase.arcs.chapters.display_name': (d) =>
    Boolean(
      d?.optional?.chapter_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) => isValidValue(c?.display_name)),
      ),
    ),
  'optional.chapter_phase.arcs.chapters.summary': (d) =>
    Boolean(
      d?.optional?.chapter_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) => isValidValue(c?.summary)),
      ),
    ),
  'optional.chapter_phase.arcs.chapters.scenes.display_name': (d) =>
    Boolean(
      d?.optional?.chapter_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) =>
          c?.scenes?.some((s: any) => isValidValue(s?.display_name)),
        ),
      ),
    ),
  'optional.chapter_phase.arcs.chapters.scenes.summary': (d) =>
    Boolean(
      d?.optional?.chapter_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) =>
          c?.scenes?.some((s: any) => isValidValue(s?.summary)),
        ),
      ),
    ),

  // Optional: World Building Phase
  'optional.world_building_phase.world_settings.display_name': (d) =>
    isValidValue(d?.optional?.world_building_phase?.world_settings?.display_name),
  'optional.world_building_phase.world_settings.type': (d) =>
    isValidValue(d?.optional?.world_building_phase?.world_settings?.type),
  'optional.world_building_phase.world_settings.description': (d) =>
    isValidValue(d?.optional?.world_building_phase?.world_settings?.description),
  'optional.world_building_phase.world_settings.access': (d) =>
    isValidValue(d?.optional?.world_building_phase?.world_settings?.access),
  'optional.world_building_phase.world_settings.lore_entries.display_name': (d) =>
    Boolean(
      d?.optional?.world_building_phase?.world_settings?.lore_entries?.some((l: any) =>
        isValidValue(l?.display_name),
      ),
    ),
  'optional.world_building_phase.world_settings.lore_entries.description': (d) =>
    Boolean(
      d?.optional?.world_building_phase?.world_settings?.lore_entries?.some((l: any) =>
        isValidValue(l?.description),
      ),
    ),

  // Optional: Scene Config Phase
  'optional.scene_config_phase.arcs.chapters.scenes.style_preset_code': (d) =>
    Boolean(
      d?.optional?.scene_config_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) =>
          c?.scenes?.some((s: any) => isValidValue(s?.style_preset_code)),
        ),
      ),
    ),
  'optional.scene_config_phase.arcs.chapters.scenes.narrator_config_code': (d) =>
    Boolean(
      d?.optional?.scene_config_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) =>
          c?.scenes?.some((s: any) => isValidValue(s?.narrator_config_code)),
        ),
      ),
    ),
  'optional.scene_config_phase.arcs.chapters.scenes.lore_entries_code': (d) =>
    Boolean(
      d?.optional?.scene_config_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) =>
          c?.scenes?.some((s: any) => isValidValue(s?.lore_entries_code)),
        ),
      ),
    ),
  'optional.scene_config_phase.arcs.chapters.scenes.characters.is_pov': (d) =>
    Boolean(
      d?.optional?.scene_config_phase?.arcs?.some((a) =>
        a?.chapters?.some((c: any) =>
          c?.scenes?.some((s: any) =>
            s?.characters?.some((ch: any) => typeof ch?.is_pov === 'boolean'),
          ),
        ),
      ),
    ),

  // Optional: Character Appearance Phase
  'optional.character_appearance_phase.character.species': (d) =>
    Boolean(
      d?.optional?.character_appearance_phase?.character?.some((c) =>
        isValidValue(c?.species),
      ),
    ),
  'optional.character_appearance_phase.character.gender.identity': (d) =>
    Boolean(
      d?.optional?.character_appearance_phase?.character?.some((c) =>
        isValidValue(c?.gender?.identity),
      ),
    ),
  'optional.character_appearance_phase.character.height_cm': (d) =>
    Boolean(
      d?.optional?.character_appearance_phase?.character?.some((c) =>
        isValidValue(c?.height_cm),
      ),
    ),
  'optional.character_appearance_phase.character.hair.color': (d) =>
    Boolean(
      d?.optional?.character_appearance_phase?.character?.some((c) =>
        isValidValue(c?.hair?.color),
      ),
    ),
  'optional.character_appearance_phase.character.skin.tone': (d) =>
    Boolean(
      d?.optional?.character_appearance_phase?.character?.some((c) =>
        isValidValue(c?.skin?.tone),
      ),
    ),
  'optional.character_appearance_phase.character.outfit_style': (d) =>
    Boolean(
      d?.optional?.character_appearance_phase?.character?.some((c) =>
        isValidValue(c?.outfit_style),
      ),
    ),

  // Optional: Character Psychological Phase
  'optional.charactar_psychological_phase.character.core_trauma_and_belief.the_wound': (
    d,
  ) =>
    Boolean(
      d?.optional?.charactar_psychological_phase?.character?.some((c) =>
        isValidValue(c?.core_trauma_and_belief?.the_wound),
      ),
    ),
  'optional.charactar_psychological_phase.character.cognitive_moral_framework.moral_code': (
    d,
  ) =>
    Boolean(
      d?.optional?.charactar_psychological_phase?.character?.some((c) =>
        isValidValue(c?.cognitive_moral_framework?.moral_code),
      ),
    ),
  'optional.charactar_psychological_phase.character.narrative_engine.external_want': (
    d,
  ) =>
    Boolean(
      d?.optional?.charactar_psychological_phase?.character?.some((c) =>
        isValidValue(c?.narrative_engine?.external_want),
      ),
    ),
  'optional.charactar_psychological_phase.character.behavioral_adaptation.social_mask': (
    d,
  ) =>
    Boolean(
      d?.optional?.charactar_psychological_phase?.character?.some((c) =>
        isValidValue(c?.behavioral_adaptation?.social_mask?.persona_type),
      ),
    ),

  // Optional: Character Life Experience Phase
  'optional.character_life_experience_phase.character.memories': (d) =>
    Boolean(
      d?.optional?.character_life_experience_phase?.character?.some(
        (c) => Array.isArray(c?.memories) && c.memories.length > 0,
      ),
    ),
  'optional.character_life_experience_phase.character.competences': (d) =>
    Boolean(
      d?.optional?.character_life_experience_phase?.character?.some(
        (c) => Array.isArray(c?.competences) && c.competences.length > 0,
      ),
    ),

  // Optional: Relationship Phase
  'optional.relationship_phase.relationship.motif.event_blocks': (d) =>
    Boolean(
      d?.optional?.relationship_phase?.relationship?.some(
        (r) => Array.isArray(r?.motif?.event_blocks) && r.motif.event_blocks.length > 0,
      ),
    ),
  'optional.relationship_phase.relationship.character.scenes': (d) =>
    Boolean(
      d?.optional?.relationship_phase?.relationship?.some(
        (r) => Array.isArray(r?.character?.scenes) && r.character.scenes.length > 0,
      ),
    ),
};

export function getMissingRequireFields(data: BusinessFlowData): string[] {
  const missing: string[] = [];

  if (
    !data?.require?.project?.display_name ||
    data.require.project.display_name.trim().length === 0
  ) {
    missing.push('require.project.display_name');
  }

  if (
    !data?.require?.project?.summary ||
    data.require.project.summary.trim().length === 0
  ) {
    missing.push('require.project.summary');
  }

  if (
    !data?.require?.project?.language ||
    data.require.project.language.trim().length === 0
  ) {
    missing.push('require.project.language');
  }

  if (
    typeof data?.require?.project?.target?.arc_count !== 'number' ||
    data.require.project.target.arc_count <= 0
  ) {
    missing.push('require.project.target.arc_count');
  }

  if (
    typeof data?.require?.project?.target?.chapter_count !== 'number' ||
    data.require.project.target.chapter_count <= 0
  ) {
    missing.push('require.project.target.chapter_count');
  }

  if (
    typeof data?.require?.project?.target?.word_per_chapter_count !== 'number' ||
    data.require.project.target.word_per_chapter_count <= 0
  ) {
    missing.push('require.project.target.word_per_chapter_count');
  }

  if (
    !Array.isArray(data?.require?.project?.genres) ||
    data.require.project.genres.length === 0
  ) {
    missing.push('require.project.genres');
  }

  if (
    !Array.isArray(data?.require?.character) ||
    data.require.character.length === 0
  ) {
    missing.push('require.character');
  } else {
    data.require.character.forEach((char, idx) => {
      const hasDisplayName = Boolean(char?.display_name && char.display_name.trim().length > 0);
      const hasNickname = Boolean(char?.nickname && char.nickname.trim().length > 0);
      if (!hasDisplayName && !hasNickname) {
        missing.push(`require.character[${idx}].display_name`);
      }
      if (
        !char?.biological_sex ||
        !['NONE', 'MALE', 'FEMALE'].includes(char.biological_sex)
      ) {
        missing.push(`require.character[${idx}].biological_sex`);
      }
      if (!char?.motif_code || char.motif_code.trim().length === 0) {
        missing.push(`require.character[${idx}].motif_code`);
      }
    });
  }

  return missing;
}

export function isRequireComplete(data: BusinessFlowData): boolean {
  return getMissingRequireFields(data).length === 0;
}

export function calculateBusinessFlowProgress(
  data: BusinessFlowData,
): BusinessFlowProgressResult {
  const missingRequireFields = getMissingRequireFields(data);
  const requireComplete = missingRequireFields.length === 0;

  let filledCount = 0;
  for (const attr of MASTER_ATTRIBUTES_LIST) {
    const evaluator = ATTRIBUTE_EVALUATORS[attr];
    if (evaluator && evaluator(data)) {
      filledCount++;
    }
  }

  const totalCount = MASTER_ATTRIBUTES_LIST.length;
  const percent = totalCount > 0 ? Math.round((filledCount / totalCount) * 100) : 0;

  return {
    percent,
    filledCount,
    totalCount,
    isRequireComplete: requireComplete,
    missingRequireFields,
  };
}
