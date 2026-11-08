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

function isNonEmptyString(value: unknown): boolean {
  return typeof value === 'string' && value.trim().length > 0;
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

  if (!isNonEmptyString(data?.require?.project?.display_name)) {
    missing.push('require.project.display_name');
  }

  if (!isNonEmptyString(data?.require?.project?.summary)) {
    missing.push('require.project.summary');
  }

  if (!isNonEmptyString(data?.require?.project?.language)) {
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
      const hasDisplayName = isNonEmptyString(char?.display_name);
      const hasNickname = isNonEmptyString(char?.nickname);
      if (!hasDisplayName && !hasNickname) {
        missing.push(`require.character[${idx}].display_name`);
      }
      if (
        !char?.biological_sex ||
        !['NONE', 'MALE', 'FEMALE'].includes(char.biological_sex)
      ) {
        missing.push(`require.character[${idx}].biological_sex`);
      }
      if (!isNonEmptyString(char?.motif_code)) {
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

// ============================================================================
// Phase 5 Sprint 2: Dual Progress & Backend Progress Integration (46 Master Attributes)
// ============================================================================

/**
 * Total number of Master Attributes tracked by the progress engine (core.md §3.1, be-core.md §4.6).
 * 50 attributes in specification minus 4 explicitly excluded attributes = 46 fixed denominator.
 */
export const TOTAL_TRACKED_ATTRIBUTES = 46 as const;

/**
 * 4 explicitly excluded attributes that do not contribute to progress denominator or numerator:
 * - 3 tolerance fields in Overview Phase
 * - 1 motif event_blocks field in Relationship Phase
 */
export const EXCLUDED_PROGRESS_ATTRIBUTES: readonly string[] = [
  'optional.overview_phase.project.target.arc_tolerance',
  'optional.overview_phase.project.target.chapter_tolerance',
  'optional.overview_phase.project.target.word_per_chapter_tolerance',
  'optional.relationship_phase.relationship.motif.event_blocks',
] as const;

/**
 * Canonical list of 46 tracked Master Attributes (50 - 4 = 46).
 */
export const MASTER_TRACKED_ATTRIBUTES: readonly string[] = MASTER_ATTRIBUTES_LIST.filter(
  (attr) => !EXCLUDED_PROGRESS_ATTRIBUTES.includes(attr),
);

/**
 * Evaluated progress state for an individual Master Attribute.
 */
export type AttributeProgressState = 'confirmed' | 'ai_suggested' | 'incomplete';

/**
 * Raw progress DTO received from Tauri IPC backend (Group 1 / Group 2).
 * Supports both camelCase and snake_case representations from Rust serde serialization.
 */
export interface DraftProgressDto {
  confirmed_count?: number;
  confirmedCount?: number;
  ai_inferred_count?: number;
  aiInferredCount?: number;
  ai_suggested_count?: number;
  aiSuggestedCount?: number;
  completed_attributes?: number;
  completedAttributes?: number;
  total_attributes?: number;
  totalAttributes?: number;
  total_count?: number;
  totalCount?: number;
  progress_percentage?: number;
  progressPercentage?: number;
  confirmed_percent?: number;
  confirmedPercent?: number;
  ai_suggested_percent?: number;
  aiInferredPercent?: number;
  is_require_complete?: boolean;
  isRequireComplete?: boolean;
  readiness_gate_passed?: boolean;
  readinessGatePassed?: boolean;
  can_advance_phase?: boolean;
  canAdvancePhase?: boolean;
  current_phase?: number;
  currentPhase?: number;
  current_subphase?: number | null;
  currentSubphase?: number | null;
}

/**
 * Strongly-typed live frontend progress model adhering to P5S2-B:
 * - Dual progress tracking (confirmed vs AI inferred).
 * - Fixed 46-attribute denominator (TOTAL_TRACKED_ATTRIBUTES).
 * - Exact readiness gate passed status with zero synthetic fabrication.
 */
export interface DraftProgressState {
  confirmedCount: number;
  aiInferredCount: number;
  totalAttributes: 46;
  confirmedPercent: number;
  aiInferredPercent: number;
  totalPercent: number;
  isRequireComplete: boolean;
  readinessGatePassed: boolean;
  canAdvancePhase: boolean;
  currentPhase: number;
  currentSubphase?: number | null;
}

/**
 * Maps incoming backend DraftProgressDto / ProgressSummaryDto to strongly-typed DraftProgressState.
 * Guarantees fixed denominator 46, dual percentages, and no synthetic readiness fabrication (P5S2-B).
 */
export function fromDraftProgressDto(
  dto: DraftProgressDto | null | undefined,
): DraftProgressState {
  if (!dto) {
    return {
      confirmedCount: 0,
      aiInferredCount: 0,
      totalAttributes: TOTAL_TRACKED_ATTRIBUTES,
      confirmedPercent: 0,
      aiInferredPercent: 0,
      totalPercent: 0,
      isRequireComplete: false,
      readinessGatePassed: false,
      canAdvancePhase: false,
      currentPhase: 0,
      currentSubphase: null,
    };
  }

  const confirmedCount =
    dto.confirmedCount ??
    dto.confirmed_count ??
    dto.completedAttributes ??
    dto.completed_attributes ??
    0;

  const aiInferredCount =
    dto.aiInferredCount ??
    dto.ai_inferred_count ??
    dto.aiSuggestedCount ??
    dto.ai_suggested_count ??
    0;

  const totalAttributes = TOTAL_TRACKED_ATTRIBUTES; // Fixed denominator 46

  const confirmedPercent =
    dto.confirmedPercent ??
    dto.confirmed_percent ??
    Math.round((confirmedCount / totalAttributes) * 100);

  const aiInferredPercent =
    dto.aiInferredPercent ??
    dto.ai_suggested_percent ??
    Math.round((aiInferredCount / totalAttributes) * 100);

  const totalPercent = Math.min(
    100,
    dto.progressPercentage ??
      dto.progress_percentage ??
      confirmedPercent + aiInferredPercent,
  );

  // Strict readiness evaluation: zero synthetic readiness!
  const readinessGatePassed = Boolean(
    dto.readinessGatePassed ??
      dto.readiness_gate_passed ??
      dto.isRequireComplete ??
      dto.is_require_complete,
  );

  const canAdvancePhase = Boolean(
    dto.canAdvancePhase ??
      dto.can_advance_phase ??
      readinessGatePassed,
  );

  const currentPhase = dto.currentPhase ?? dto.current_phase ?? 0;
  const currentSubphase = dto.currentSubphase ?? dto.current_subphase ?? null;

  return {
    confirmedCount,
    aiInferredCount,
    totalAttributes,
    confirmedPercent,
    aiInferredPercent,
    totalPercent,
    isRequireComplete: readinessGatePassed,
    readinessGatePassed,
    canAdvancePhase,
    currentPhase,
    currentSubphase,
  };
}

/**
 * Reconciles current progress state with newly received backend progress DTO.
 */
export function reconcileDraftProgress(
  current: DraftProgressState | null | undefined,
  dto: DraftProgressDto | null | undefined,
): DraftProgressState {
  if (!dto) {
    return current ?? fromDraftProgressDto(null);
  }
  return fromDraftProgressDto(dto);
}

/**
 * Computes progress state for local BusinessFlowData across 46 Master Attributes.
 */
export function computeDraftProgress(
  data: BusinessFlowData | null | undefined,
): DraftProgressState {
  if (!data) {
    return fromDraftProgressDto(null);
  }

  let confirmedCount = 0;
  const aiInferredCount = 0;

  for (const attr of MASTER_TRACKED_ATTRIBUTES) {
    const evaluator = ATTRIBUTE_EVALUATORS[attr];
    if (evaluator && evaluator(data)) {
      confirmedCount++;
    }
  }

  const isReady = isRequireComplete(data);
  const totalAttributes = TOTAL_TRACKED_ATTRIBUTES;
  const confirmedPercent = Math.round((confirmedCount / totalAttributes) * 100);
  const aiInferredPercent = Math.round((aiInferredCount / totalAttributes) * 100);

  return {
    confirmedCount,
    aiInferredCount,
    totalAttributes,
    confirmedPercent,
    aiInferredPercent,
    totalPercent: Math.min(100, confirmedPercent + aiInferredPercent),
    isRequireComplete: isReady,
    readinessGatePassed: isReady,
    canAdvancePhase: isReady,
    currentPhase: 0,
    currentSubphase: null,
  };
}

/**
 * Resolves system language enforcing CI-09 authority precedence:
 * Trusted UI/settings language takes strict precedence; untrusted caller override cannot silently clobber it.
 */
export function resolveSystemLanguage(
  trustedSetting?: string | null,
  untrustedOverride?: string | null,
): 'vi' | 'en' {
  // If a valid trusted UI setting is explicitly provided, it strictly wins
  if (trustedSetting === 'vi' || trustedSetting === 'en') {
    return trustedSetting;
  }

  // If in browser environment, check trusted localStorage setting
  if (typeof window !== 'undefined') {
    try {
      const stored = window.localStorage?.getItem('merlink.locale');
      if (stored === 'vi' || stored === 'en') {
        return stored;
      }
    } catch {
      // Storage access failed, proceed to fallback
    }
  }

  // Untrusted override is only considered if trusted setting is absent and it's valid
  if (untrustedOverride === 'vi' || untrustedOverride === 'en') {
    return untrustedOverride;
  }

  // Authoritative default locale per Merlink specification
  return 'vi';
}

