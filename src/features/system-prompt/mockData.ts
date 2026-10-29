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
  // Motif 1: Redemption Arc — loyal → conflicted → resolved
  { id: 'eb-1', code: 'eb-loyal-choice', componentId: 'co-1', stateId: 'st-1', motifId: 'mo-1' },
  { id: 'eb-4', code: 'eb-conflicted-revelation-arc', componentId: 'co-2', stateId: 'st-2', motifId: 'mo-1' },
  { id: 'eb-5', code: 'eb-resolved-choice-arc', componentId: 'co-1', stateId: 'st-4', motifId: 'mo-1' },

  // Motif 2: Bond & Betrayal — loyal → conflicted → betrayed
  { id: 'eb-6', code: 'eb-loyal-choice-bond', componentId: 'co-1', stateId: 'st-1', motifId: 'mo-2' },
  { id: 'eb-2', code: 'eb-conflicted-revelation', componentId: 'co-2', stateId: 'st-2', motifId: 'mo-2' },
  { id: 'eb-7', code: 'eb-betrayed-confrontation-bond', componentId: 'co-3', stateId: 'st-3', motifId: 'mo-2' },

  // Motif 3: Crystal Light — betrayed → resolved (shatter then clarity)
  { id: 'eb-9', code: 'eb-betrayed-confrontation-light', componentId: 'co-3', stateId: 'st-3', motifId: 'mo-3' },
  { id: 'eb-8', code: 'eb-resolved-revelation', componentId: 'co-2', stateId: 'st-4', motifId: 'mo-3' },

  // Motif 4: Rising Tension — conflicted → betrayed → resolved
  { id: 'eb-10', code: 'eb-conflicted-revelation-tension', componentId: 'co-2', stateId: 'st-2', motifId: 'mo-4' },
  { id: 'eb-3', code: 'eb-betrayed-confrontation', componentId: 'co-3', stateId: 'st-3', motifId: 'mo-4' },
  { id: 'eb-11', code: 'eb-resolved-choice-tension', componentId: 'co-1', stateId: 'st-4', motifId: 'mo-4' },
];

export const eventTransitions: EventTransition[] = [
  // Motif 1: Redemption Arc
  { id: 'et-1', name: 'Revelation Shift', fromEventBlockId: 'eb-1', toEventBlockId: 'eb-4', priority: 1 },
  { id: 'et-3', name: 'Resolution Path', fromEventBlockId: 'eb-4', toEventBlockId: 'eb-5', priority: 2 },

  // Motif 2: Bond & Betrayal
  { id: 'et-4', name: 'Trust Collapse', fromEventBlockId: 'eb-6', toEventBlockId: 'eb-2', priority: 1 },
  { id: 'et-2', name: 'Confrontation Break', fromEventBlockId: 'eb-2', toEventBlockId: 'eb-7', priority: 2 },

  // Motif 3: Crystal Light
  { id: 'et-5', name: 'Light Shatters', fromEventBlockId: 'eb-9', toEventBlockId: 'eb-8', priority: 1 },

  // Motif 4: Rising Tension
  { id: 'et-6', name: 'Tension Escalates', fromEventBlockId: 'eb-10', toEventBlockId: 'eb-3', priority: 1 },
  { id: 'et-7', name: 'Pressure Breaks', fromEventBlockId: 'eb-3', toEventBlockId: 'eb-11', priority: 2 },
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
  // Action
  { id: 'sp-1', code: 'action.combat', category: 'action', tonePace: 'VERY_FAST', toneEmotion: 'căng thẳng, dồn dập', isSystem: true, visual: 'HIGH', auditory: 'HIGH', tactile: 'HIGH', olfactory: 'LOW', gustatory: 'NONE', languageAbstract: 'LOW', syntaxSentence: 'SHORT', syntaxVariation: 'HIGH', syntaxStructure: 'SIMPLE' },
  { id: 'sp-2', code: 'action.assassination', category: 'action', tonePace: 'VERY_LOW', toneEmotion: 'căng thẳng nghẹt thở', isSystem: true, visual: 'HIGH', auditory: 'LOW', tactile: 'HIGH', olfactory: 'MEDIUM', gustatory: 'NONE', languageAbstract: 'MEDIUM', syntaxSentence: 'SHORT', syntaxVariation: 'HIGH', syntaxStructure: 'MIXED' },
  // Mystery
  { id: 'sp-3', code: 'mystery.deduction', category: 'mystery', tonePace: 'LOW', toneEmotion: 'tập trung, lạnh lùng', isSystem: true, visual: 'LOW', auditory: 'LOW', tactile: 'NONE', olfactory: 'NONE', gustatory: 'NONE', languageAbstract: 'HIGH', syntaxSentence: 'LONG', syntaxVariation: 'LOW', syntaxStructure: 'MIXED' },
  { id: 'sp-4', code: 'mystery.revelation', category: 'mystery', tonePace: 'VERY_FAST', toneEmotion: 'sốc, phẫn nộ, kinh hoàng', isSystem: true, visual: 'HIGH', auditory: 'HIGH', tactile: 'MEDIUM', olfactory: 'LOW', gustatory: 'NONE', languageAbstract: 'MEDIUM', syntaxSentence: 'SHORT', syntaxVariation: 'HIGH', syntaxStructure: 'MIXED' },
  // Romance
  { id: 'sp-5', code: 'romance_relationship.confession', category: 'romance_relationship', tonePace: 'LOW', toneEmotion: 'mong manh, dũng cảm, hồi hộp', isSystem: true, visual: 'HIGH', auditory: 'HIGH', tactile: 'MEDIUM', olfactory: 'LOW', gustatory: 'NONE', languageAbstract: 'MEDIUM', syntaxSentence: 'SHORT', syntaxVariation: 'HIGH', syntaxStructure: 'MIXED' },
  // Psychological
  { id: 'sp-6', code: 'psychological_drama.betrayal', category: 'psychological_drama', tonePace: 'FAST', toneEmotion: 'sốc, đau đớn tột cùng, phẫn uất', isSystem: true, visual: 'HIGH', auditory: 'HIGH', tactile: 'MEDIUM', olfactory: 'LOW', gustatory: 'LOW', languageAbstract: 'MEDIUM', syntaxSentence: 'SHORT', syntaxVariation: 'HIGH', syntaxStructure: 'MIXED' },
  // Slice of Life
  { id: 'sp-7', code: 'slice_of_life_comedy.daily_routine', category: 'slice_of_life_comedy', tonePace: 'LOW', toneEmotion: 'ấm cúng, thoải mái', isSystem: true, visual: 'HIGH', auditory: 'MEDIUM', tactile: 'MEDIUM', olfactory: 'MEDIUM', gustatory: 'LOW', languageAbstract: 'LOW', syntaxSentence: 'MEDIUM', syntaxVariation: 'LOW', syntaxStructure: 'MIXED' },
  // Political
  { id: 'sp-8', code: 'political_social.negotiation_treaty', category: 'political_social', tonePace: 'LOW', toneEmotion: 'căng thẳng, khôn khéo, thận trọng', isSystem: true, visual: 'MEDIUM', auditory: 'HIGH', tactile: 'LOW', olfactory: 'NONE', gustatory: 'NONE', languageAbstract: 'HIGH', syntaxSentence: 'LONG', syntaxVariation: 'LOW', syntaxStructure: 'COMPLEX' },
  // Exploration
  { id: 'sp-9', code: 'exploration_discovery.first_contact', category: 'exploration_discovery', tonePace: 'LOW', toneEmotion: 'kinh ngạc, thận trọng, hồi hộp', isSystem: true, visual: 'HIGH', auditory: 'MEDIUM', tactile: 'MEDIUM', olfactory: 'MEDIUM', gustatory: 'NONE', languageAbstract: 'MEDIUM', syntaxSentence: 'MEDIUM', syntaxVariation: 'MEDIUM', syntaxStructure: 'MIXED' },
  // Transition
  { id: 'sp-10', code: 'transition.time_skip', category: 'transition', tonePace: 'FAST', toneEmotion: 'hoài niệm, mới mẻ, trăn trở', isSystem: true, visual: 'MEDIUM', auditory: 'LOW', tactile: 'LOW', olfactory: 'LOW', gustatory: 'NONE', languageAbstract: 'MEDIUM', syntaxSentence: 'SHORT', syntaxVariation: 'HIGH', syntaxStructure: 'MIXED' },
];

export const rhetoricalDevices: RhetoricalDevice[] = [
  { id: 'rd-1', code: 'metaphor', description: 'An implied comparison without "like" or "as".' },
  { id: 'rd-2', code: 'irony', description: 'A contrast between expectation and reality.' },
  { id: 'rd-3', code: 'hyperbole', description: 'Deliberate exaggeration for emphasis.' },
];

export const stylePresetRhetoricalDevices: StylePresetRhetoricalDevice[] = [
  // action.combat
  { id: 'sprd-1', presetId: 'sp-1', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Điệp động từ mạnh & tượng thanh va chạm' },
  { id: 'sprd-2', presetId: 'sp-1', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Câu cụt liên tiếp' },
  { id: 'sprd-3', presetId: 'sp-1', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Ẩn dụ trữ tình' },
  // action.assassination
  { id: 'sprd-4', presetId: 'sp-2', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Ẩn dụ' },
  { id: 'sprd-5', presetId: 'sp-2', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Điệp cấu trúc câu cụt mơ hồ' },
  { id: 'sprd-6', presetId: 'sp-2', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Tượng thanh ồn' },
  // mystery.deduction
  { id: 'sprd-7', presetId: 'sp-3', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Phép tam đoạn luận & điệp cấu trúc suy luận' },
  { id: 'sprd-8', presetId: 'sp-3', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Phép loại suy' },
  { id: 'sprd-9', presetId: 'sp-3', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Câu hỏi tu từ bộc phát' },
  // mystery.revelation
  { id: 'sprd-10', presetId: 'sp-4', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Phép đảo ngữ & nhấn hoá ra/thì ra' },
  { id: 'sprd-11', presetId: 'sp-4', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Câu cụt dứt khoát' },
  { id: 'sprd-12', presetId: 'sp-4', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Ngôn ngữ mơ hồ' },
  // romance.confession
  { id: 'sprd-13', presetId: 'sp-5', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Ẩn dụ tình cảm & câu chấm lửng do dự' },
  { id: 'sprd-14', presetId: 'sp-5', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Điệp động từ bộc lộ cảm xúc' },
  { id: 'sprd-15', presetId: 'sp-5', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Ngôn ngữ khô khan, lạnh lùng' },
  // psych.betrayal
  { id: 'sprd-16', presetId: 'sp-6', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Phép đối tôi đã tin – hoá ra & điệp khoảng lặng' },
  { id: 'sprd-17', presetId: 'sp-6', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Câu chấm lửng nghẹn lại' },
  { id: 'sprd-18', presetId: 'sp-6', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Ngôn ngữ bình thản' },
  // slice of life
  { id: 'sprd-19', presetId: 'sp-7', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Liệt kê thói quen lặp lại & điệp nhịp êm đềm' },
  { id: 'sprd-20', presetId: 'sp-7', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Phép đối tương phản hài hước' },
  { id: 'sprd-21', presetId: 'sp-7', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Câu giật gân' },
  // political
  { id: 'sprd-22', presetId: 'sp-8', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Phép đối hai phe & lặp điều kiện để nhấn' },
  { id: 'sprd-23', presetId: 'sp-8', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Câu hỏi tu từ thăm dò' },
  { id: 'sprd-24', presetId: 'sp-8', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Ngôn ngữ cảm xúc' },
  // exploration
  { id: 'sprd-25', presetId: 'sp-9', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Phép so sánh lạ – quen & ẩn dụ khám phá' },
  { id: 'sprd-26', presetId: 'sp-9', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Liệt kê chi tiết kỳ lạ' },
  { id: 'sprd-27', presetId: 'sp-9', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Ngôn ngữ quá đỗi thường' },
  // transition
  { id: 'sprd-28', presetId: 'sp-10', rhetoricalDeviceId: 'rd-1', usageType: 'primary', customInstruction: 'Phép liệt kê mùa / tháng & phép đối trước – sau' },
  { id: 'sprd-29', presetId: 'sp-10', rhetoricalDeviceId: 'rd-2', usageType: 'secondary', customInstruction: 'Ẩn dụ dòng sông thời gian' },
  { id: 'sprd-30', presetId: 'sp-10', rhetoricalDeviceId: 'rd-3', usageType: 'avoid', customInstruction: 'Miêu tả chi tiết từng ngày' },
];

export const stylePresetVocabItems: StylePresetVocabItem[] = [
  { id: 'spv-1', presetId: 'sp-1', content: 'động từ mạnh' },
  { id: 'spv-2', presetId: 'sp-1', content: 'từ tượng thanh' },
  { id: 'spv-3', presetId: 'sp-2', content: 'động từ tĩnh' },
  { id: 'spv-4', presetId: 'sp-2', content: 'từ ám chỉ' },
  { id: 'spv-5', presetId: 'sp-3', content: 'từ lập luận (do đó, như vậy)' },
  { id: 'spv-6', presetId: 'sp-3', content: 'thuật ngữ logic' },
  { id: 'spv-7', presetId: 'sp-4', content: 'từ ngữ mạnh, dứt khoát' },
  { id: 'spv-8', presetId: 'sp-4', content: 'sự tương phản (hoá ra, thì ra)' },
  { id: 'spv-9', presetId: 'sp-5', content: 'từ ngữ ẩn dụ tình cảm' },
  { id: 'spv-10', presetId: 'sp-5', content: 'động từ bộc lộ cảm xúc' },
  { id: 'spv-11', presetId: 'sp-6', content: 'từ ngữ đối lập (tôi đã tin – hóa ra)' },
  { id: 'spv-12', presetId: 'sp-6', content: 'động từ mạnh chỉ sự đâm thủng' },
  { id: 'spv-13', presetId: 'sp-7', content: 'động từ chỉ hành động quen thuộc' },
  { id: 'spv-14', presetId: 'sp-7', content: 'tính từ đời thường' },
  { id: 'spv-15', presetId: 'sp-8', content: 'thuật ngữ đàm phán' },
  { id: 'spv-16', presetId: 'sp-8', content: 'từ ngữ lập luận, nhượng bộ' },
  { id: 'spv-17', presetId: 'sp-9', content: 'từ miêu tả sự kỳ lạ' },
  { id: 'spv-18', presetId: 'sp-9', content: 'ẩn dụ so sánh với điều quen thuộc' },
  { id: 'spv-19', presetId: 'sp-10', content: 'từ chỉ sự trôi chảy thời gian' },
  { id: 'spv-20', presetId: 'sp-10', content: 'ẩn dụ quá khứ – hiện tại' },
];

export const stylePresetFocusItems: StylePresetFocusItem[] = [
  { id: 'spf-1', presetId: 'sp-1', content: 'chuyển động' },
  { id: 'spf-2', presetId: 'sp-1', content: 'va chạm' },
  { id: 'spf-3', presetId: 'sp-2', content: 'thời điểm' },
  { id: 'spf-4', presetId: 'sp-2', content: 'góc nhìn' },
  { id: 'spf-5', presetId: 'sp-3', content: 'mối quan hệ nhân quả' },
  { id: 'spf-6', presetId: 'sp-3', content: 'chứng cứ đối lập' },
  { id: 'spf-7', presetId: 'sp-4', content: 'sự thật bị che giấu' },
  { id: 'spf-8', presetId: 'sp-4', content: 'lời thú nhận' },
  { id: 'spf-9', presetId: 'sp-5', content: 'lời nói trực tiếp' },
  { id: 'spf-10', presetId: 'sp-5', content: 'phản ứng không lời' },
  { id: 'spf-11', presetId: 'sp-6', content: 'sự thay đổi nhận thức' },
  { id: 'spf-12', presetId: 'sp-6', content: 'lời nói kẻ phản bội' },
  { id: 'spf-13', presetId: 'sp-7', content: 'sự lặp lại êm đềm' },
  { id: 'spf-14', presetId: 'sp-7', content: 'những điều nhỏ nhặt' },
  { id: 'spf-15', presetId: 'sp-8', content: 'điểm mấu chốt' },
  { id: 'spf-16', presetId: 'sp-8', content: 'sự nhượng bộ' },
  { id: 'spf-17', presetId: 'sp-9', content: 'sự khác biệt' },
  { id: 'spf-18', presetId: 'sp-9', content: 'phản ứng nhân vật' },
  { id: 'spf-19', presetId: 'sp-10', content: 'sự thay đổi' },
  { id: 'spf-20', presetId: 'sp-10', content: 'điều giữ nguyên' },
];

// NARRATOR CONFIGS
export const narratorConfigs: NarratorConfig[] = [
  // ── FIRST PERSON · PROTAGONIST ────────────────────────────
  {
    id: 'nc-1',
    code: 'first-person-protagonist',
    name: 'First · Protagonist',
    person: 'FIRST_PERSON',
    personSubtype: 'PROTAGONIST',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Classic intimate narrator. Protagonist recounts events from memory in first person; all knowledge filtered through their perspective.',
  },
  {
    id: 'nc-2',
    code: 'first-person-protagonist-multiple',
    name: 'First · Protagonist (Multi)',
    person: 'FIRST_PERSON',
    personSubtype: 'PROTAGONIST',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Rotating first-person — several protagonists each narrate their own chapters. Same intimacy, wider cast.',
  },
  {
    id: 'nc-3',
    code: 'first-person-protagonist-meta',
    name: 'First · Protagonist (Meta)',
    person: 'FIRST_PERSON',
    personSubtype: 'PROTAGONIST',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'First-person protagonist who acknowledges the reader. Confessional and self-aware; uses asides and direct address within the narration.',
  },

  // ── FIRST PERSON · WITNESS ────────────────────────────────
  {
    id: 'nc-4',
    code: 'first-person-witness',
    name: 'First · Witness',
    person: 'FIRST_PERSON',
    personSubtype: 'WITNESS_PERIPHERAL',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Peripheral character recounts the story of another. Protagonist seen from outside; narrator observes more than they understand.',
  },
  {
    id: 'nc-5',
    code: 'first-person-witness-multiple',
    name: 'First · Witness (Multi)',
    person: 'FIRST_PERSON',
    personSubtype: 'WITNESS_PERIPHERAL',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Multiple peripheral witnesses narrate different chapters. The hero is constructed from the outside, from clashing testimonies.',
  },
  {
    id: 'nc-6',
    code: 'first-person-witness-meta',
    name: 'First · Witness (Meta)',
    person: 'FIRST_PERSON',
    personSubtype: 'WITNESS_PERIPHERAL',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'Witness narrator who speaks to the reader — framing the tale as a testimony being handed over.',
  },

  // ── FIRST PERSON · UNRELIABLE ─────────────────────────────
  {
    id: 'nc-7',
    code: 'first-person-unreliable',
    name: 'First · Unreliable',
    person: 'FIRST_PERSON',
    personSubtype: 'UNRELIABLE_FIRST',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: false,
    description:
      'Narrator withholds, misremembers, or reframes. Reader must infer truth from inconsistencies and gaps. Used for psychological tension.',
  },
  {
    id: 'nc-8',
    code: 'first-person-unreliable-meta',
    name: 'First · Unreliable (Meta)',
    person: 'FIRST_PERSON',
    personSubtype: 'UNRELIABLE_FIRST',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: true,
    description:
      'Unreliable narrator aware of the reader — acknowledges the act of narration, deepens doubt about what is being omitted.',
  },
  {
    id: 'nc-9',
    code: 'first-person-unreliable-multiple',
    name: 'First · Unreliable (Multi)',
    person: 'FIRST_PERSON',
    personSubtype: 'UNRELIABLE_FIRST',
    povScope: 'MULTIPLE_POV',
    isReliable: false,
    breaksFourthWall: false,
    description:
      'Multiple unreliable first-person narrators — accounts clash and contradict, and no single voice can be trusted.',
  },

  // ── SECOND PERSON · DIRECT ADDRESS ────────────────────────
  {
    id: 'nc-10',
    code: 'second-person-direct',
    name: 'Second · Direct',
    person: 'SECOND_PERSON',
    personSubtype: 'DIRECT_ADDRESS',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Narrator speaks to the reader as "you". Immersive, disorienting — actions phrased as commands or invitations.',
  },
  {
    id: 'nc-11',
    code: 'second-person-direct-meta',
    name: 'Second · Direct (Meta)',
    person: 'SECOND_PERSON',
    personSubtype: 'DIRECT_ADDRESS',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'Direct address that foregrounds the reader’s role — the narration remarks on being read, inviting complicity.',
  },
  {
    id: 'nc-12',
    code: 'second-person-direct-unreliable',
    name: 'Second · Direct (Unreliable)',
    person: 'SECOND_PERSON',
    personSubtype: 'DIRECT_ADDRESS',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: false,
    description:
      'The "you" being addressed is unreliable — memory, motive, or identity cannot be trusted. Reader must reconstruct the truth.',
  },

  // ── SECOND PERSON · SELF-REFLECTION ───────────────────────
  {
    id: 'nc-13',
    code: 'second-person-self-reflection',
    name: 'Second · Self-Reflection',
    person: 'SECOND_PERSON',
    personSubtype: 'SELF_REFLECTION',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'Narrator addresses their own past or shadow-self as "you". Introspective; blurs the line between narrator and subject.',
  },
  {
    id: 'nc-14',
    code: 'second-person-self-reflection-no-meta',
    name: 'Second · Self-Reflection (No Meta)',
    person: 'SECOND_PERSON',
    personSubtype: 'SELF_REFLECTION',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Internal self-address kept fully diegetic — the "you" is the narrator’s younger self, not the reader.',
  },
  {
    id: 'nc-15',
    code: 'second-person-self-reflection-multiple',
    name: 'Second · Self-Reflection (Multi)',
    person: 'SECOND_PERSON',
    personSubtype: 'SELF_REFLECTION',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'Multiple voices each address their own past — interlocking self-reflections across chapters.',
  },

  // ── SECOND PERSON · INTERACTIVE ───────────────────────────
  {
    id: 'nc-16',
    code: 'second-person-interactive',
    name: 'Second · Interactive',
    person: 'SECOND_PERSON',
    personSubtype: 'INTERACTIVE',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'Reader inhabits the protagonist; agency is explicitly granted or revoked. Used in branching or experimental narratives.',
  },
  {
    id: 'nc-17',
    code: 'second-person-interactive-unreliable',
    name: 'Second · Interactive (Unreliable)',
    person: 'SECOND_PERSON',
    personSubtype: 'INTERACTIVE',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: true,
    description:
      'Interactive narration that misreports the reader’s choices — the game lies, or the character’s perception does.',
  },
  {
    id: 'nc-18',
    code: 'second-person-interactive-multiple',
    name: 'Second · Interactive (Multi)',
    person: 'SECOND_PERSON',
    personSubtype: 'INTERACTIVE',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: true,
    description:
      'Branching interactive narration across multiple viewpoint chapters — the reader holds several "yous".',
  },

  // ── THIRD PERSON · OMNISCIENT ─────────────────────────────
  {
    id: 'nc-19',
    code: 'third-person-omniscient-shifting',
    name: 'Third · Omniscient (Shifting)',
    person: 'THIRD_PERSON',
    personSubtype: 'OMNISCIENT',
    povScope: 'SHIFTING_OMNISCIENT',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'All-knowing narrator moves freely between minds, times, and places. God’s-eye view; knowledge is not filtered through any single character.',
  },
  {
    id: 'nc-20',
    code: 'third-person-omniscient-multiple',
    name: 'Third · Omniscient (Multi)',
    person: 'THIRD_PERSON',
    personSubtype: 'OMNISCIENT',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Omniscient narrator who nevertheless alternates primary anchors chapter by chapter — omniscient knowledge, but with privileged focuses.',
  },
  {
    id: 'nc-21',
    code: 'third-person-omniscient-single',
    name: 'Third · Omniscient (Single)',
    person: 'THIRD_PERSON',
    personSubtype: 'OMNISCIENT',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Omniscient knowledge, single focus — the narrator could see all but chooses one character as the through-line.',
  },

  // ── THIRD PERSON · LIMITED ────────────────────────────────
  {
    id: 'nc-22',
    code: 'third-person-limited-single',
    name: 'Third · Limited (Single)',
    person: 'THIRD_PERSON',
    personSubtype: 'LIMITED',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Deeply anchored to one viewpoint throughout. Interiority is close, and only what the viewpoint perceives can be narrated.',
  },
  {
    id: 'nc-23',
    code: 'third-person-limited-multiple',
    name: 'Third · Limited (Multi)',
    person: 'THIRD_PERSON',
    personSubtype: 'LIMITED',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Viewpoints rotate between characters across chapters. Close interiority per chapter; whole picture assembled by the reader.',
  },
  {
    id: 'nc-24',
    code: 'third-person-limited-unreliable',
    name: 'Third · Limited (Unreliable)',
    person: 'THIRD_PERSON',
    personSubtype: 'LIMITED',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: false,
    description:
      'Limited perspective on a narrator whose perception is compromised — deep interiority, but the truth leaks around it.',
  },

  // ── THIRD PERSON · OBJECTIVE ──────────────────────────────
  {
    id: 'nc-25',
    code: 'third-person-objective-single',
    name: 'Third · Objective (Single)',
    person: 'THIRD_PERSON',
    personSubtype: 'OBJECTIVE',
    povScope: 'SINGLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Camera-eye narration. Records only what can be seen or heard — actions, dialogue, events. No interiority is disclosed.',
  },
  {
    id: 'nc-26',
    code: 'third-person-objective-multiple',
    name: 'Third · Objective (Multi)',
    person: 'THIRD_PERSON',
    personSubtype: 'OBJECTIVE',
    povScope: 'MULTIPLE_POV',
    isReliable: true,
    breaksFourthWall: false,
    description:
      'Camera-eye narration that observes several characters without entering any interior. Events only; interpretation left to the reader.',
  },
  {
    id: 'nc-27',
    code: 'third-person-objective-unreliable',
    name: 'Third · Objective (Unreliable)',
    person: 'THIRD_PERSON',
    personSubtype: 'OBJECTIVE',
    povScope: 'SINGLE_POV',
    isReliable: false,
    breaksFourthWall: false,
    description:
      'Camera-eye narration with a compromised lens — observations may be incomplete, framing may mislead, but no interior is ever stated.',
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
  { id: 'ebe-1',  eventBlockId: 'eb-1',  emotionalRangeId: 'er-1' },
  { id: 'ebe-2',  eventBlockId: 'eb-1',  emotionalRangeId: 'er-2' },
  { id: 'ebe-3',  eventBlockId: 'eb-4',  emotionalRangeId: 'er-2' },
  { id: 'ebe-4',  eventBlockId: 'eb-4',  emotionalRangeId: 'er-4' },
  { id: 'ebe-5',  eventBlockId: 'eb-5',  emotionalRangeId: 'er-1' },
  { id: 'ebe-6',  eventBlockId: 'eb-5',  emotionalRangeId: 'er-3' },

  { id: 'ebe-7',  eventBlockId: 'eb-6',  emotionalRangeId: 'er-1' },
  { id: 'ebe-8',  eventBlockId: 'eb-6',  emotionalRangeId: 'er-3' },
  { id: 'ebe-9',  eventBlockId: 'eb-2',  emotionalRangeId: 'er-2' },
  { id: 'ebe-10', eventBlockId: 'eb-2',  emotionalRangeId: 'er-4' },
  { id: 'ebe-11', eventBlockId: 'eb-7',  emotionalRangeId: 'er-4' },
  { id: 'ebe-12', eventBlockId: 'eb-7',  emotionalRangeId: 'er-2' },

  { id: 'ebe-13', eventBlockId: 'eb-9',  emotionalRangeId: 'er-4' },
  { id: 'ebe-14', eventBlockId: 'eb-8',  emotionalRangeId: 'er-1' },
  { id: 'ebe-15', eventBlockId: 'eb-8',  emotionalRangeId: 'er-3' },

  { id: 'ebe-16', eventBlockId: 'eb-10', emotionalRangeId: 'er-2' },
  { id: 'ebe-17', eventBlockId: 'eb-3',  emotionalRangeId: 'er-4' },
  { id: 'ebe-18', eventBlockId: 'eb-3',  emotionalRangeId: 'er-2' },
  { id: 'ebe-19', eventBlockId: 'eb-11', emotionalRangeId: 'er-3' },
  { id: 'ebe-20', eventBlockId: 'eb-11', emotionalRangeId: 'er-1' },
];
