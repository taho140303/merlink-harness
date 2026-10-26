import type {
  DraftOverview, DraftArc, DraftChapter, DraftScene,
  DraftConfig, DraftWorld, DraftCharacter, DraftCharacterDetail
} from './types';

export const mockOverview: DraftOverview = {
  title: 'Chronicles of Aethelgard',
  author: 'Merlink AI',
  language: 'VI',
  summarize: 'Một lục địa trôi dạt giữa các đứt gãy không gian ma thuật, nơi các hiệp sĩ pha lê bảo vệ tàn tích cổ xưa khỏi làn sương mờ Aetheric. Câu chuyện theo chân Kaelen Vance trên hành trình tìm lại ký ức đã mất và đối mặt với sự thật về nguồn gốc sức mạnh của mình.',
  targetWords: '1,500 words'
};

export const mockArcs: DraftArc[] = [
  {
    id: 'a1',
    name: 'Arc I: The Sundering',
    summarize: 'Kaelen phát hiện di tích ngầm bị kích hoạt trái phép, dẫn đến cuộc chạm trán đầu tiên với Lyra và sự sụp đổ của một phần thành phố.',
    themes: [
      {
        id: 'th1',
        code: 'th-power-corruption',
        scope: 'ARC',
        question: 'Sức mạnh có tha hoá con người?',
        resolutionType: 'OPEN',
        resolution: 'Để ngỏ cho người đọc tự đánh giá qua hành động của Kaelen.',
        message: 'Sức mạnh chỉ là công cụ, bản chất con người mới là thứ quyết định.',
        arguments: [
          {
            id: 'arg1',
            characterName: 'Kaelen Vance',
            role: 'PRIMARY',
            answerMode: 'ACTION',
            note: 'Sẵn sàng dùng ma pháp cấm để cứu người, bất chấp cái giá phải trả.'
          },
          {
            id: 'arg2',
            characterName: 'Lyra Dawnseeker',
            role: 'ANTITHESIS',
            answerMode: 'VERBAL',
            note: 'Tin rằng mọi sức mạnh phi tự nhiên đều dẫn đến diệt vong.'
          }
        ]
      }
    ]
  },
  {
    id: 'a2',
    name: 'Arc II: Echoes of Void',
    summarize: 'Hành trình lặn sâu vào vùng cấm để tìm kiếm mảnh ghép ký ức, đối mặt với các thế lực đen tối đang trỗi dậy.',
    themes: [
      {
        id: 'th2',
        code: 'th-identity-loss',
        scope: 'ARC',
        question: 'Ký ức định hình bản ngã như thế nào?',
        resolutionType: 'AMBIGUOUS',
        resolution: 'Kaelen lấy lại ký ức nhưng nhận ra mình đã trở thành một người khác.',
        message: 'Chúng ta là những gì chúng ta chọn nhớ.',
        arguments: [] // Mock trường hợp chưa có nhân vật nào trả lời theme
      }
    ]
  }
];

export const mockChapters: DraftChapter[] = [
  { id: 'c1', arcId: 'a1', name: 'Chapter 1: Khế ước quỷ dữ', description: 'Kaelen ký kết một khế ước nguy hiểm trong căn gác mái hoang tàn để cứu lấy mạng sống của mình, đánh đổi bằng một phần ký ức.' },
  { id: 'c2', arcId: 'a1', name: 'Chapter 2: Cuộc sống bị đảo lộn', description: 'Những hệ quả của khế ước bắt đầu bộc lộ. Kaelen phải đối mặt với sự truy đuổi của Hội Thánh Quang và những giấc mơ kỳ lạ.' },
  { id: 'c3', arcId: 'a2', name: 'Chapter 3: Bóng tối dưới lòng đất', description: 'Bước chân vào di tích bị lãng quên, nơi bóng tối che giấu những cỗ máy cổ đại và sự thật về The Sundering.' }
];

export const mockScenes: DraftScene[] = [
  {
    id: 's1', chapterId: 'c1', name: 'Scene 1.1: The Contract',
    summarize: 'Kaelen thực hiện nghi thức triệu hồi trong bóng tối.',
    stylePreset: 'psychological_drama.inner_monologue',
    narratorConfig: 'first_person_unreliable',
    characters: [
      { name: 'Kaelen Vance', isPov: true },
      { name: 'The Voice (Entity)', isPov: false }
    ],
    forbidden: ['Ánh sáng mặt trời trực tiếp', 'Phép thuật hệ Lửa'],
    allowed: ['Ma pháp hệ Bóng tối', 'Âm thanh môi trường ẩm ướt']
  },
  {
    id: 's2', chapterId: 'c1', name: 'Scene 1.2: Awakening',
    summarize: 'Kaelen tỉnh dậy với vết ấn chú trên tay, nhận ra mình đã mất đi ký ức về tuổi thơ.',
    stylePreset: 'mystery.clue_hunting',
    narratorConfig: 'third_person_limited',
    characters: [
      { name: 'Kaelen Vance', isPov: true }
    ],
    forbidden: ['Sự xuất hiện của Lyra'],
    allowed: ['Hồi tưởng mờ nhạt', 'Đồ vật cá nhân']
  },
  {
    id: 's3', chapterId: 'c2', name: 'Scene 2.1: Ambush at the Market',
    summarize: 'Cuộc phục kích bất ngờ của Hội Thánh Quang tại khu chợ đen.',
    stylePreset: 'action.combat',
    narratorConfig: 'third_person_objective',
    characters: [
      { name: 'Kaelen Vance', isPov: true },
      { name: 'Lyra Dawnseeker', isPov: false },
      { name: 'Zealots', isPov: false }
    ],
    forbidden: ['Thương lượng hòa bình'],
    allowed: ['Đổ máu', 'Phá hủy tài sản', 'Sử dụng ma pháp công khai']
  }
];

export const mockConfigs: DraftConfig[] = [
  {
    id: 'cfg1', targetScope: 'Arc I: The Sundering',
    theme: 'Sức mạnh có tha hoá con người?',
    motifs: ['mc-redemption-arc', 'mc-rising-tension']
  },
  {
    id: 'cfg2', targetScope: 'Chapter 1: Khế ước quỷ dữ',
    theme: 'Sự đánh đổi của ký ức và bản ngã',
    motifs: ['mc-crystal-light', 'mc-bond-betrayal']
  }
];

export const mockWorlds: DraftWorld[] = [
  {
    id: 'w1', locationName: 'The Sunken Cathedral (Chợ đen)',
    locationType: 'FACILITY (Ngầm)',
    sensory: 'Không khí ẩm mốc, mùi hương liệu nồng nặc, âm thanh mặc cả ồn ào từ các góc tối.',
    lore: ['Chợ đen Aethelgard', 'Hội Thánh Quang (Rival Faction)']
  },
  {
    id: 'w2', locationName: 'The Ruined Attic',
    locationType: 'ROOM (Bỏ hoang)',
    sensory: 'Bụi bặm, ánh trăng lọt qua cửa sổ vỡ, mùi giấy cũ và sự tĩnh lặng đáng sợ.',
    lore: ['Khế ước cổ đại', 'Nguồn gốc của Aetheric Veil']
  }
];

export const mockCharacters: DraftCharacter[] = [
  {
    id: 'ch1', name: 'Kaelen Vance',
    role: 'Protagonist (Anti-hero)',
    appearances: ['Ch 1 - Scene 1.1', 'Ch 1 - Scene 1.2', 'Ch 2 - Scene 2.1']
  },
  {
    id: 'ch2', name: 'Lyra Dawnseeker',
    role: 'Deuteragonist / Antagonist (Temporary)',
    appearances: ['Ch 2 - Scene 2.1']
  }
];

export const mockCharacterDetails: DraftCharacterDetail[] = [
  {
    id: 'cd1', name: 'Kaelen Vance',
    personality: 'Lạnh lùng, tính toán, nhưng bên trong chất chứa nỗi sợ mất đi bản ngã.',
    actionLogic: 'Sẵn sàng làm những điều tàn nhẫn để sinh tồn, nhưng sẽ do dự nếu điều đó chạm đến ký ức về người thân.',
    themeAlignment: 'PRIMARY (Chứng minh rằng con người có thể giữ được lý trí dù sở hữu sức mạnh cấm)',
    sceneRole: 'Trung tâm của mọi mâu thuẫn, là người đưa ra quyết định cuối cùng trong Scene 1.1.'
  },
  {
    id: 'cd2', name: 'Lyra Dawnseeker',
    personality: 'Kiên định, mù quáng vì lý tưởng của Hội Thánh, nhưng có một điểm yếu chí mạng là sự ngây thơ về bản chất của cái ác.',
    actionLogic: 'Tấn công không do dự, nhưng sẽ khựng lại nếu Kaelen đề cập đến một sự thật trong quá khứ.',
    themeAlignment: 'ANTITHESIS (Đại diện cho sự sụp đổ của lý tưởng khi đối mặt với thực tại khắc nghiệt)',
    sceneRole: 'Là chất xúc tác cho sự tha hoá của Kaelen trong Chapter 2.'
  }
];
