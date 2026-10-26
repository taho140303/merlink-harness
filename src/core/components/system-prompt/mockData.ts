import type {
  Component,
  Constraint,
  EmotionalRange,
  EventBlock,
  EventBlockEmotional,
  EventTransition,
  Genre,
  Motif,
  MotifApplicableGenre,
  MotifConstraint,
  NarrativeDirective,
  NarratorConfig,
  RhetoricalDevice,
  StateDownByRule,
  StateItem,
  StateRule,
  StylePreset,
  StylePresetFocusItem,
  StylePresetNarrative,
  StylePresetRhetoricalDevice,
  StylePresetVocabItem,
  Theme,
  ThemeArgument,
} from './types';

export const motifs: Motif[] = [
  {
    id: 'mo-1',
    code: 'mc-redemption-arc',
    name: 'Redemption Arc',
    category: 'character_arc',
    version: '1.0.0',
    isSystem: true,
    description:
      'Character wrestles with past wrongdoing, seeks atonement through costly action. Arc resolves only when the character accepts responsibility without external validation.',
    minChapter: 1,
    maxChapter: 12,
  },
  {
    id: 'mo-2',
    code: 'mc-bond-betrayal',
    name: 'Bond & Betrayal',
    category: 'relationship',
    version: '1.0.0',
    isSystem: true,
    description:
      'Two characters grow close, then one commits a breach of trust. The scar reshapes future encounters — restoration is possible but never full.',
    minChapter: 3,
    maxChapter: 20,
  },
  {
    id: 'mo-3',
    code: 'mc-crystal-light',
    name: 'Crystal Light',
    category: 'symbolic',
    version: '1.0.0',
    isSystem: false,
    description:
      'Light refracted through crystal — represents clarity won through pain. Recurs at moments of moral revelation.',
  },
  {
    id: 'mo-4',
    code: 'mc-rising-tension',
    name: 'Rising Tension',
    category: 'plot',
    version: '1.0.0',
    isSystem: true,
    description:
      'Escalating stakes and shrinking windows of resolution. Each success narrows the path to the next confrontation.',
  },
];

export const genres: Genre[] = [
  { id: 'ge-1', code: 'fantasy', name: 'Fantasy', description: 'Magic, myth, and otherworldly settings.' },
  { id: 'ge-2', code: 'romance', name: 'Romance', description: 'Focus on romantic relationships and their arcs.' },
  { id: 'ge-3', code: 'mystery', name: 'Mystery', description: 'Puzzles, investigations, and concealed truths.' },
];

export const motifApplicableGenres: MotifApplicableGenre[] = [
  { id: 'mag-1', motifId: 'mo-1', genreId: 'ge-1' },
  { id: 'mag-2', motifId: 'mo-1', genreId: 'ge-3' },
  { id: 'mag-3', motifId: 'mo-2', genreId: 'ge-1' },
  { id: 'mag-4', motifId: 'mo-2', genreId: 'ge-2' },
  { id: 'mag-5', motifId: 'mo-3', genreId: 'ge-1' },
  { id: 'mag-6', motifId: 'mo-4', genreId: 'ge-1' },
  { id: 'mag-7', motifId: 'mo-4', genreId: 'ge-3' },
];

export const components: Component[] = [
  { id: 'co-1', code: 'choice', name: 'Choice', description: 'A decisive action with irreversible consequence.' },
  { id: 'co-2', code: 'revelation', name: 'Revelation', description: 'A truth surfaced that changes the character’s frame.' },
  { id: 'co-3', code: 'confrontation', name: 'Confrontation', description: 'Direct, open opposition between entities.' },
];

export const states: StateItem[] = [
  { id: 'st-1', code: 'loyal', name: 'Loyal', description: 'Aligned with the object of loyalty without doubt.' },
  { id: 'st-2', code: 'conflicted', name: 'Conflicted', description: 'Held between two incompatible commitments.' },
  { id: 'st-3', code: 'betrayed', name: 'Betrayed', description: 'Trust shattered; perception inverted.' },
  { id: 'st-4', code: 'resolved', name: 'Resolved', description: 'Integrated; contradiction no longer binds.' },
];

export const stateRules: StateRule[] = [
  { id: 'sr-1', code: 'breaking_trust', name: 'Breaking a Trust' },
  { id: 'sr-2', code: 'saving_life', name: 'Saving a Life at Cost' },
  { id: 'sr-3', code: 'shared_trauma', name: 'Shared Trauma' },
];

export const stateDownByRules: StateDownByRule[] = [
  { id: 'sdr-1', stateId: 'st-1', stateRuleId: 'sr-1', isAllowed: true },
  { id: 'sdr-2', stateId: 'st-2', stateRuleId: 'sr-2', isAllowed: true },
  { id: 'sdr-3', stateId: 'st-3', stateRuleId: 'sr-3', isAllowed: false },
];

export const emotionalRanges: EmotionalRange[] = [
  { id: 'er-1', code: 'calm', name: 'Calm' },
  { id: 'er-2', code: 'tense', name: 'Tense' },
  { id: 'er-3', code: 'euphoric', name: 'Euphoric' },
  { id: 'er-4', code: 'grief', name: 'Grief' },
];

export const eventBlocks: EventBlock[] = [
  { id: 'eb-1', code: 'eb-loyal-choice', componentId: 'co-1', stateId: 'st-1', motifId: 'mo-1' },
  { id: 'eb-2', code: 'eb-conflicted-revelation', componentId: 'co-2', stateId: 'st-2', motifId: 'mo-2' },
  { id: 'eb-3', code: 'eb-betrayed-confrontation', componentId: 'co-3', stateId: 'st-3', motifId: 'mo-4' },
];

export const eventTransitions: EventTransition[] = [
  { id: 'et-1', name: 'Revelation Shift', fromEventBlockId: 'eb-1', toEventBlockId: 'eb-2', priority: 1 },
  { id: 'et-2', name: 'Confrontation Break', fromEventBlockId: 'eb-2', toEventBlockId: 'eb-3', priority: 1 },
];

export const constraints: Constraint[] = [
  {
    id: 'cs-1',
    code: 'req-motive',
    require: 'Character must have a stated motive for change',
    reason: 'Without a motive the arc feels externally imposed and rings false.',
  },
  {
    id: 'cs-2',
    code: 'req-vulnerability',
    require: 'Both parties must show vulnerability before trust is restored',
    reason: 'Trust feels earned only when risk is shared.',
  },
];

export const motifConstraints: MotifConstraint[] = [
  { id: 'mcs-1', code: 'mc-redemption-arc-req-motive', constraintId: 'cs-1', motifId: 'mo-1' },
  { id: 'mcs-2', code: 'mc-bond-betrayal-req-vulnerability', constraintId: 'cs-2', motifId: 'mo-2' },
];

// STYLE PRESETS
export const stylePresets: StylePreset[] = [
  {
    id: 'sp-1',
    code: 'noir-melancholy',
    category: 'dark.noir',
    tonePace: 'VERY_LOW',
    toneEmotion: 'Melancholic',
    isSystem: true,
    visual: 'HIGH',
    auditory: 'MEDIUM',
    tactile: 'LOW',
    olfactory: 'MEDIUM',
    gustatory: 'NONE',
    languageAbstract: 'HIGH',
    syntaxSentence: 'SHORT',
    syntaxVariation: 'MEDIUM',
    syntaxStructure: 'FRAGMENT',
  },
  {
    id: 'sp-2',
    code: 'epic-heroic',
    category: 'epic.heroic',
    tonePace: 'FAST',
    toneEmotion: 'Awe',
    isSystem: true,
    visual: 'HIGH',
    auditory: 'HIGH',
    tactile: 'MEDIUM',
    olfactory: 'LOW',
    gustatory: 'NONE',
    languageAbstract: 'LOW',
    syntaxSentence: 'MEDIUM',
    syntaxVariation: 'HIGH',
    syntaxStructure: 'COMPOUND',
  },
];

export const rhetoricalDevices: RhetoricalDevice[] = [
  { id: 'rd-1', code: 'metaphor', description: 'An implied comparison without "like" or "as".' },
  { id: 'rd-2', code: 'irony', description: 'A contrast between expectation and reality.' },
  { id: 'rd-3', code: 'hyperbole', description: 'Deliberate exaggeration for emphasis.' },
];

export const stylePresetRhetoricalDevices: StylePresetRhetoricalDevice[] = [
  { id: 'sprd-1', presetId: 'sp-1', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Sparse metaphor — one per scene, image-driven, no explanation.' },
  { id: 'sprd-2', presetId: 'sp-1', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Quiet, structural irony — never stated outright.' },
  { id: 'sprd-3', presetId: 'sp-2', rhetoricalDeviceId: 'rd-3', usageType: 'primary', customInstruction: 'Stakes feel world-ending; exaggeration mirrors emotional reality.' },
];

export const stylePresetVocabItems: StylePresetVocabItem[] = [
  { id: 'spv-1', presetId: 'sp-1', content: 'sepia' },
  { id: 'spv-2', presetId: 'sp-1', content: 'ashen' },
  { id: 'spv-3', presetId: 'sp-1', content: 'drift' },
  { id: 'spv-4', presetId: 'sp-2', content: 'roar' },
  { id: 'spv-5', presetId: 'sp-2', content: 'triumphant' },
];

export const stylePresetFocusItems: StylePresetFocusItem[] = [
  { id: 'spf-1', presetId: 'sp-1', content: 'Stillness, negative space, restraint of gesture' },
  { id: 'spf-2', presetId: 'sp-2', content: 'Motion, impact, velocity of action' },
];

// NARRATOR CONFIGS
export const narratorConfigs: NarratorConfig[] = [
  {
    id: 'nc-1',
    code: 'first-person-past',
    name: 'First Person, Past Tense',
    person: 'FIRST_PERSON',
    personSubtype: 'PROTAGONIST',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Classic intimate narrator recalling events from memory. Confessional tone, all knowledge filtered through the narrator.',
  },
  {
    id: 'nc-2',
    code: 'third-limited-present',
    name: 'Third Person Limited, Present Tense',
    person: 'THIRD_PERSON',
    personSubtype: 'LIMITED',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Deeply anchored to one viewpoint per chapter, rotating between chapters. Immediate, present-tense — events unfold as they happen.',
  },
  {
    id: 'nc-3',
    code: 'unreliable-first',
    name: 'Unreliable First Person',
    person: 'FIRST_PERSON',
    personSubtype: 'UNRELIABLE_FIRST',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: false,
    description:
      'Narrator withholds and reframes. Reader must infer truth from inconsistencies. Used for psychological tension.',
  },
];

export const narrativeDirectives: NarrativeDirective[] = [
  {
    id: 'nd-1',
    code: 'clarity',
    description: 'Prioritise comprehensibility of action and causality over poetic flourish.',
    category: 'PROSE_PRINCIPLE',
    isSystem: true,
  },
  {
    id: 'nd-2',
    code: 'show_dont_tell',
    description: 'Render emotion through action and detail rather than stating it.',
    category: 'PROSE_PRINCIPLE',
    isSystem: true,
  },
  {
    id: 'nd-3',
    code: 'avoid_adverbs',
    description: 'Prefer stronger verbs to verb + adverb constructions.',
    category: 'RESTRAINT',
    isSystem: true,
  },
  {
    id: 'nd-4',
    code: 'withhold_truth',
    description: 'Delay key information to sustain tension; reveal only when a scene earns it.',
    category: 'INFORMATION_CONTROL',
    isSystem: true,
  },
];

export const stylePresetNarratives: StylePresetNarrative[] = [
  { id: 'spn-1', stylePresetId: 'sp-1', narrativeDirectiveId: 'nd-4', weight: 'HIGH' },
  { id: 'spn-2', stylePresetId: 'sp-1', narrativeDirectiveId: 'nd-3', weight: 'MEDIUM' },
  { id: 'spn-3', stylePresetId: 'sp-2', narrativeDirectiveId: 'nd-2', weight: 'MEDIUM' },
  { id: 'spn-4', stylePresetId: 'sp-2', narrativeDirectiveId: 'nd-1', weight: 'HIGH' },
];

// THEMES
export const themes: Theme[] = [
  {
    id: 'th-1',
    code: 'th-power-corruption',
    scope: 'PROJECT',
    projectId: 'proj-1',
    question: 'Does power inevitably corrupt, or can it be wielded ethically?',
    resolutionType: 'AMBIGUOUS',
    resolution:
      'Both corrupt and ethical wielding are possible — but the moral cost is never zero. The story refuses to absolve either path.',
    message: 'Power is a mirror: it reveals what was already there.',
    createdAt: '2026-01-04T00:00:00Z',
    updatedAt: '2026-03-11T00:00:00Z',
  },
  {
    id: 'th-2',
    code: 'th-trust-worth',
    scope: 'ARC',
    projectId: 'proj-1',
    arcId: 'arc-1',
    parentThemeId: 'th-1',
    question: 'Is trust worth the risk of betrayal?',
    resolutionType: 'OPEN',
    resolution:
      'Trust is shown to be costly but necessary — yet no character is allowed to answer with certainty.',
    createdAt: '2026-02-10T00:00:00Z',
    updatedAt: '2026-04-01T00:00:00Z',
  },
];

export const themeArguments: ThemeArgument[] = [
  {
    id: 'ta-1',
    themeId: 'th-1',
    characterId: 'char-1',
    characterName: 'Kaelen Vance',
    role: 'PRIMARY',
    answerMode: 'OUTCOME',
    note: 'His arc resolves by choosing to relinquish power at the moment he could seize it.',
    requiredKnowledge: 'Understanding of the Sundering’s true cause',
    requiredCompetence: 'Ability to channel without external focus',
  },
  {
    id: 'ta-2',
    themeId: 'th-1',
    characterId: 'char-2',
    characterName: 'Lyra Dawnseeker',
    role: 'ANTITHESIS',
    answerMode: 'ACTION',
    note: 'Believes ethical wielding is possible; her fate tests the claim.',
    requiredKnowledge: 'The Radiant Order’s internal schism',
  },
  {
    id: 'ta-3',
    themeId: 'th-2',
    characterId: 'char-1',
    characterName: 'Kaelen Vance',
    role: 'PRIMARY',
    answerMode: 'VERBAL',
    note: 'Explicit statement at the arc climax.',
  },
];

export const eventBlockEmotionals: EventBlockEmotional[] = [
  { id: 'ebe-1', eventBlockId: 'eb-1', emotionalRangeId: 'er-1' },
  { id: 'ebe-2', eventBlockId: 'eb-1', emotionalRangeId: 'er-2' },
  { id: 'ebe-3', eventBlockId: 'eb-2', emotionalRangeId: 'er-2' },
  { id: 'ebe-4', eventBlockId: 'eb-2', emotionalRangeId: 'er-4' },
  { id: 'ebe-5', eventBlockId: 'eb-3', emotionalRangeId: 'er-4' },
  { id: 'ebe-6', eventBlockId: 'eb-3', emotionalRangeId: 'er-3' },
];
