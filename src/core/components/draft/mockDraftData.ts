import type {
  DraftOverview, DraftArc, DraftChapter, DraftScene,
  DraftConfig, DraftWorld, DraftCharacter, DraftCharacterDetail, DraftMotif
} from './types';

// Mẫu Motif (Dùng chung)
const motifSample1: DraftMotif = {
  id: 'mo-1', code: 'mc-redemption-arc', name: 'Redemption Arc', category: 'character_arc', scope: 'project',
  version: '1.0.0', isSystem: true, description: 'Character wrestles with past wrongdoing, seeks atonement through costly action.',
  from: 'aethelgard', 
  eventBlocks: [
    { id: 'eb-1', code: 'eb-loyal-choice', component: 'Choice', state: 'Loyal' },
    { id: 'eb-2', code: 'eb-conflicted-revelation', component: 'Revelation', state: 'Conflicted' },
    { id: 'eb-3', code: 'eb-resolved-choice', component: 'Choice', state: 'Resolved' }
  ]
};

const motifSample2: DraftMotif = {
  id: 'mo-2', code: 'mc-crystal-light', name: 'Crystal Light', category: 'symbolic', scope: 'arc',
  version: '1.0.0', isSystem: false, description: 'Light refracted through crystal represents clarity won through pain.',
  from: 'aethelgard.sundering',
  eventBlocks: [
    { id: 'eb-4', code: 'eb-shatter', component: 'Confrontation', state: 'Betrayed' }
  ]
};

const motifSample3: DraftMotif = {
  id: 'mo-3', code: 'mc-rising-tension', name: 'Rising Tension', category: 'plot', scope: 'chapter',
  version: '1.0.0', isSystem: true, description: 'Escalating stakes and shrinking windows of resolution. Each success narrows the path.',
  minChapter: 1, maxChapter: 2, from: 'aethelgard.sundering.khe-uoc',
  eventBlocks: [
    { id: 'eb-5', code: 'eb-tension-1', component: 'Revelation', state: 'Conflicted' },
    { id: 'eb-6', code: 'eb-tension-2', component: 'Confrontation', state: 'Betrayed' }
  ]
};

export const mockOverview: DraftOverview = {
  title: 'Chronicles of Aethelgard',
  author: 'Merlink AI',
  language: 'VI',
  summarize: 'Một lục địa trôi dạt giữa các đứt gãy không gian ma thuật...',
  targetWords: '1,500 words',
  motifs: [motifSample1]
};

export const mockArcs: DraftArc[] = [
  {
    id: 'a1', name: 'Arc I: The Sundering',
    summarize: 'Kaelen phát hiện di tích ngầm bị kích hoạt trái phép...',
    motifs: [motifSample2],
    themes: [{
      id: 'th1', code: 'th-power-corruption', scope: 'ARC', question: 'Sức mạnh có tha hoá con người?',
      resolutionType: 'OPEN', resolution: 'Để ngỏ...', message: 'Sức mạnh chỉ là công cụ...',
      arguments: [{ id: 'arg1', characterName: 'Kaelen Vance', role: 'PRIMARY', answerMode: 'ACTION', note: 'Sẵn sàng dùng ma pháp cấm...' }]
    }]
  },
  {
    id: 'a2', name: 'Arc II: Echoes of Void',
    summarize: 'Hành trình lặn sâu vào vùng cấm...',
    motifs: [],
    themes: [{
      id: 'th2', code: 'th-identity-loss', scope: 'ARC', question: 'Ký ức định hình bản ngã như thế nào?',
      resolutionType: 'AMBIGUOUS', resolution: 'Kaelen lấy lại ký ức...', message: 'Chúng ta là những gì chúng ta chọn nhớ.',
      arguments: []
    }]
  }
];

export const mockChapters: DraftChapter[] = [
  { 
    id: 'c1', arcId: 'a1', name: 'Chapter 1: Khế ước quỷ dữ', 
    description: 'Kaelen ký kết một khế ước nguy hiểm...',
    motifs: [motifSample3] // Chapter-level motif
  },
  { id: 'c2', arcId: 'a1', name: 'Chapter 2: Cuộc sống bị đảo lộn', description: 'Những hệ quả của khế ước bắt đầu bộc lộ...', motifs: [] },
  { id: 'c3', arcId: 'a2', name: 'Chapter 3: Bóng tối dưới lòng đất', description: 'Bước chân vào di tích bị lãng quên...', motifs: [] }
];

export const mockScenes: DraftScene[] = [
  {
    id: 's1', chapterId: 'c1', name: 'Scene 1.1: The Contract',
    summarize: 'Kaelen thực hiện nghi thức triệu hồi trong bóng tối.',
    stylePreset: 'psychological_drama.inner_monologue', narratorConfig: 'first_person_unreliable',
    characters: [{ name: 'Kaelen Vance', isPov: true }, { name: 'The Voice', isPov: false }],
    forbidden: ['Ánh sáng mặt trời'], allowed: ['Ma pháp hệ Bóng tối']
  },
  {
    id: 's2', chapterId: 'c1', name: 'Scene 1.2: Awakening',
    summarize: 'Kaelen tỉnh dậy với vết ấn chú trên tay...',
    stylePreset: 'mystery.clue_hunting', narratorConfig: 'third_person_limited',
    characters: [{ name: 'Kaelen Vance', isPov: true }],
    forbidden: ['Sự xuất hiện của Lyra'], allowed: ['Hồi tưởng mờ nhạt']
  }
];

export const mockConfigs: DraftConfig[] = [
  { id: 'cfg1', targetScope: 'Arc I: The Sundering', theme: 'Sức mạnh có tha hoá con người?' },
  { id: 'cfg2', targetScope: 'Chapter 1: Khế ước quỷ dữ', theme: 'Sự đánh đổi của ký ức và bản ngã' }
];

export const mockWorlds: DraftWorld[] = [
  { id: 'w1', locationName: 'The Sunken Cathedral', locationType: 'FACILITY', sensory: 'Không khí ẩm mốc...', lore: ['Chợ đen Aethelgard'] }
];

export const mockCharacters: DraftCharacter[] = [
  { id: 'ch1', name: 'Kaelen Vance', role: 'Protagonist', appearances: ['Ch 1 - Scene 1.1', 'Ch 1 - Scene 1.2'] }
];

export const mockCharacterDetails: DraftCharacterDetail[] = [
  { id: 'cd1', name: 'Kaelen Vance', personality: 'Lạnh lùng, tính toán...', actionLogic: 'Sẵn sàng làm điều tàn nhẫn...', themeAlignment: 'PRIMARY', sceneRole: 'Trung tâm của mọi mâu thuẫn' }
];
