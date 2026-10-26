import type {
  DraftOverview, DraftArc, DraftChapter, DraftScene,
  DraftConfig, DraftWorld, DraftCharacter, DraftCharacterDetail, DraftMotif
} from './types';

// 1. PROJECT-LEVEL MOTIF: Redemption Arc
const motifRedemption: DraftMotif = {
  id: 'mo-1', code: 'mc-redemption-arc', name: 'Redemption Arc', category: 'character_arc', scope: 'project',
  from: 'aethelgard', version: '1.0.0', isSystem: true,
  description: 'Character wrestles with past wrongdoing, seeks atonement through costly action. Arc resolves only when the character accepts responsibility without external validation.',
  genres: ['Fantasy', 'Dark Fantasy'],
  symbolics: ['Crystal Light', 'Shattered Mirror'],
  constraints: [
    { code: 'req-motive', require: 'Character must have a stated motive for change', reason: 'Without a motive the arc feels externally imposed.' }
  ],
  eventBlocks: [
    { 
      id: 'eb-1', code: 'eb-loyal-choice', component: 'Choice', state: 'Loyal', emotionalRanges: ['Calm', 'Tense'],
      transitions: [{ id: 'tr-1', name: 'Revelation Shift', targetEventCode: 'eb-conflicted-revelation', priority: 1 }],
      downByRules: [{ ruleName: 'Breaking a Trust', isAllowed: true }]
    },
    { 
      id: 'eb-2', code: 'eb-conflicted-revelation', component: 'Revelation', state: 'Conflicted', emotionalRanges: ['Tense', 'Grief'],
      transitions: [{ id: 'tr-2', name: 'Resolution Path', targetEventCode: 'eb-resolved-choice', priority: 2 }],
      downByRules: []
    },
    { 
      id: 'eb-3', code: 'eb-resolved-choice', component: 'Choice', state: 'Resolved', emotionalRanges: ['Euphoric'],
      transitions: [],
      downByRules: []
    }
  ]
};

// 2. ARC-LEVEL MOTIF: Rising Tension
const motifRisingTension: DraftMotif = {
  id: 'mo-2', code: 'mc-rising-tension', name: 'Rising Tension', category: 'plot', scope: 'arc',
  from: 'aethelgard.sundering', version: '1.0.0', isSystem: true,
  description: 'Escalating stakes and shrinking windows of resolution. Each success narrows the path to the next confrontation.',
  minChapter: 1, maxChapter: 3,
  genres: ['Thriller', 'Action'],
  symbolics: ['Ticking Clock', 'Shrinking Room'],
  constraints: [],
  eventBlocks: [
    {
      id: 'eb-4', code: 'eb-tension-escalates', component: 'Confrontation', state: 'Conflicted', emotionalRanges: ['Anxious'],
      transitions: [{ id: 'tr-3', name: 'Pressure Breaks', targetEventCode: 'eb-pressure-breaks', priority: 1 }]
    },
    {
      id: 'eb-5', code: 'eb-pressure-breaks', component: 'Revelation', state: 'Betrayed', emotionalRanges: ['Despair'],
      transitions: []
    }
  ]
};

// 3. CHAPTER-LEVEL MOTIF: Mystery Clue
const motifMysteryClue: DraftMotif = {
  id: 'mo-3', code: 'mc-mystery-clue', name: 'Mystery Clue', category: 'plot', scope: 'chapter',
  from: 'aethelgard.sundering.khe-uoc', version: '1.0.0', isSystem: true,
  description: 'A hidden clue surfaces, requiring careful deduction.',
  minChapter: 1, maxChapter: 1,
  genres: ['Mystery'],
  symbolics: ['Magnifying Glass'],
  constraints: [
    { code: 'no-magic', require: 'Cannot use magic to solve the clue', reason: 'The mystery must be solved by human intellect.' }
  ],
  eventBlocks: [
    {
      id: 'eb-6', code: 'eb-find-clue', component: 'Discovery', state: 'Curious', emotionalRanges: ['Surprised'],
      transitions: []
    }
  ]
};

export const mockOverview: DraftOverview = {
  title: 'Chronicles of Aethelgard',
  author: 'Merlink AI',
  language: 'VI',
  summarize: 'Một lục địa trôi dạt giữa các đứt gãy không gian ma thuật...',
  targetWords: '1,500 words',
  motifs: [motifRedemption]
};

export const mockArcs: DraftArc[] = [
  {
    id: 'a1', name: 'Arc I: The Sundering',
    summarize: 'Kaelen phát hiện di tích ngầm bị kích hoạt trái phép...',
    motifs: [motifRisingTension],
    themes: [{
      id: 'th1', code: 'th-power-corruption', scope: 'ARC', question: 'Sức mạnh có tha hoá con người?',
      resolutionType: 'OPEN', resolution: 'Để ngỏ...', message: 'Sức mạnh chỉ là công cụ...',
      arguments: [{ id: 'arg1', characterName: 'Kaelen Vance', role: 'PRIMARY', answerMode: 'ACTION', note: 'Sẵn sàng dùng ma pháp cấm...' }]
    }]
  }
];

export const mockChapters: DraftChapter[] = [
  { 
    id: 'c1', arcId: 'a1', name: 'Chapter 1: Khế ước quỷ dữ', 
    description: 'Kaelen ký kết một khế ước nguy hiểm...',
    motifs: [motifMysteryClue]
  }
];

export const mockScenes: DraftScene[] = [
  {
    id: 's1', chapterId: 'c1', name: 'Scene 1.1: The Contract',
    summarize: 'Kaelen thực hiện nghi thức triệu hồi trong bóng tối.',
    stylePreset: 'psychological_drama.inner_monologue', narratorConfig: 'first_person_unreliable',
    characters: [{ name: 'Kaelen Vance', isPov: true }],
    forbidden: ['Ánh sáng mặt trời'], allowed: ['Ma pháp hệ Bóng tối']
  }
];

export const mockConfigs: DraftConfig[] = [
  { id: 'cfg1', targetScope: 'Arc I: The Sundering', theme: 'Sức mạnh có tha hoá con người?' }
];

export const mockWorlds: DraftWorld[] = [
  { id: 'w1', locationName: 'The Sunken Cathedral', locationType: 'FACILITY', sensory: 'Không khí ẩm mốc...', lore: ['Chợ đen Aethelgard'] }
];

export const mockCharacters: DraftCharacter[] = [
  { 
    id: 'ch1', 
    name: 'Kaelen Vance', 
    role: 'Protagonist', 
    appearances: ['Ch 1 - Scene 1.1'],
    motifs: [motifRedemption] // Gắn motif Redemption Arc (Project level) vào nhân vật
  }
];

export const mockCharacterDetails: DraftCharacterDetail[] = [
  { id: 'cd1', name: 'Kaelen Vance', personality: 'Lạnh lùng, tính toán...', actionLogic: 'Sẵn sàng làm điều tàn nhẫn...', themeAlignment: 'PRIMARY', sceneRole: 'Trung tâm của mọi mâu thuẫn' }
];
