import type { DraftOverview } from './types';

export const DRAFT_OVERVIEW: DraftOverview = {
  currentPhase: 'chapter',
  version: 'v.03',
  updatedAt: '10:45 AM, Today',
  versionHistory: [
    { version: 'v.03', updatedAt: '10:45 AM, Today' },
    { version: 'v.02', updatedAt: 'Yesterday, 14:30' },
    { version: 'v.01', updatedAt: '12/10/2026, 09:00' },
  ],
  displayName: 'Tế nữ ngưu nhân',
  language: 'VI',
  summary:
    'Truyện kể về Phù Thanh Cơ — nữ tế duy nhất còn lại của bộ tộc Ngưu Nhân giữa vương quốc Hạ Nam. Nàng phải giằng xé giữa lòng trung thành mù quáng với thần quyền tế đàn và sứ mệnh sinh tồn đẫm máu của giống loài trước hiểm họa tuyệt chủng.',
  target: {
    arcs: 7,
    chapters: 210,
    wordPerChapter: 2100,
  },
  genres: [
    { name: 'Tiên hiệp', isPrimary: true },
    { name: 'Dị giới', isPrimary: false },
    { name: 'Huyền huyễn', isPrimary: false },
    { name: 'Nữ cường', isPrimary: false },
    { name: 'Tộc nhân', isPrimary: false },
    { name: 'Chiến tranh', isPrimary: false },
    { name: 'Mưu lược', isPrimary: false },
    { name: 'Huyết mạch phục hưng', isPrimary: false },
    { name: 'Bi kịch', isPrimary: false },
    { name: 'Thần thoại viễn cổ', isPrimary: false },
    { name: 'Tôn giáo bí sử', isPrimary: false },
    { name: 'Bàn cờ vương quyền', isPrimary: false },
  ],
  arcs: [
    {
      id: 'arc-1',
      number: 1,
      title: 'Hồi 1: Huyết tế tế đàn',
      subnote: 'Khởi đầu tại Hạ Nam, nơi Phù Thanh Cơ bị triệu hồi.',
      segmentsCount: 4,
      summary: 'Phù Thanh Cơ bị đưa lên tế đàn, chứng kiến cái chết của trưởng lão và bắt đầu hành trình chạy trốn.',
      motif: {
        code: 'sacrifice-and-rebirth',
        displayName: 'Hiến tế & Tái sinh',
        category: 'PLOT',
        note: 'Bắt đầu chuỗi sự kiện phản nghịch.'
      },
      theme: {
        question: 'Sự trung thành mù quáng với thần quyền có đáng để đánh đổi sinh mạng của cả tộc?',
        message: 'Thần linh không phù hộ kẻ yếu, chỉ kẻ mạnh mới tự cứu lấy mình.',
        resolution: 'Phù Thanh Cơ từ bỏ tư tế, chấp nhận trở thành kẻ phản nghịch để bảo vệ tộc nhân.',
        arguments: [
          {
            id: 'ta-1',
            characterName: 'Phù Thanh Cơ',
            role: 'Chính',
            reactionType: 'Hành động',
            note: 'Quyết định phá hủy tế đàn, đánh đổi tu vi để cứu đồng tộc.',
            requiredKnowledge: 'Bí mật về cấu trúc linh hồn Ngưu Nhân',
            requiredCompetence: 'Khả năng điều khiển huyết mạch',
          },
          {
            id: 'ta-2',
            characterName: 'Trưởng lão Hạc',
            role: 'Phản đề',
            reactionType: 'Lời nói',
            note: 'Tin rằng sự hy sinh là cần thiết để duy trì cân bằng.',
            requiredKnowledge: 'Kinh điển cổ của tế đàn',
          },
        ],
      },
      segments: [
        { id: 's-1', chapterRange: 'Chương 1 - 3', phaseNumber: 1, summary: 'Buổi hiến tế cát bụi, mùi máu tươi và sự hoài nghi trước lời phán của tế ti.' },
        { id: 's-2', chapterRange: 'Chương 4 - 6', phaseNumber: 2, summary: 'Đêm tế lễ, dị biến xảy ra, Phù Thanh Cơ thức tỉnh huyết mạch.' },
        { id: 's-3', chapterRange: 'Chương 7 - 9', phaseNumber: 3, summary: 'Trưởng lão Hạc xuất hiện, ngăn cản cuộc tẩu thoát của Thanh Cơ.' },
        { id: 's-4', chapterRange: 'Chương 10 - 12', phaseNumber: 4, summary: 'Phá hủy tế đàn, bắt đầu cuộc sống lưu vong đầy rẫy hiểm nguy.' },
      ],
      chapters: [
        {
          displayName: 'Khởi nguyên đứt gãy',
          position: 1,
          summary: 'Phù Thanh Cơ phát hiện ra sự thật về huyết mạch của mình.',
          scenes: [
            { code: 'sc-1', displayName: 'Thức tỉnh', position: 1, summary: 'Thanh Cơ tỉnh dậy trong tế đàn, xung quanh là máu.' },
            { code: 'sc-2', displayName: 'Tiếng gọi', position: 2, summary: 'Nghe thấy tiếng gọi từ sâu trong huyết mạch.' }
          ]
        },
        {
          displayName: 'Bóng tối bao trùm',
          position: 2,
          summary: 'Trưởng lão Hạc xuất hiện, buộc tội Thanh Cơ phản bội.',
          scenes: [
            { code: 'sc-3', displayName: 'Đối chất', position: 1, summary: 'Cuộc tranh cãi nảy lửa tại đại điện.' },
            { code: 'sc-4', displayName: 'Trốn chạy', position: 2, summary: 'Thanh Cơ chạy trốn khỏi sự truy sát.' }
          ]
        }
      ],
    },
    {
      id: 'arc-2',
      number: 2,
      title: 'Hồi 2: Bóng tối vương quyền',
      subnote: 'Hành trình đến kinh đô Hạ Nam.',
      segmentsCount: 3,
      summary: 'Phù Thanh Cơ gia nhập quân đội nổi loạn, tìm cách tiếp cận hoàng thất.',
      motif: {
        code: 'shadow-ascension',
        displayName: 'Thăng trầm bóng tối',
        category: 'PLOT',
        note: 'Phát triển kỹ năng lãnh đạo và mưu lược.'
      },
      theme: {
        question: 'Liệu quyền lực có thể được sử dụng để thay đổi số phận?',
        message: 'Quyền lực chỉ là công cụ, kẻ nắm giữ mới quyết định nó là ánh sáng hay bóng tối.',
        resolution: 'Thanh Cơ thâm nhập hoàng cung, khám phá ra sự thật về nguồn gốc của mình.',
        arguments: [
          {
            id: 'ta-3',
            characterName: 'Phù Thanh Cơ',
            role: 'Chính',
            reactionType: 'Hành động',
            note: 'Âm thầm xây dựng mạng lưới đồng minh trong quân đội.',
            requiredCompetence: 'Khả năng lãnh đạo và mưu lược',
          },
        ],
      },
      segments: [
        { id: 's-5', chapterRange: 'Chương 13 - 15', phaseNumber: 1, summary: 'Gia nhập quân đội nổi loạn dưới vỏ bọc một y sư.' },
        { id: 's-6', chapterRange: 'Chương 16 - 18', phaseNumber: 2, summary: 'Trận chiến đầu tiên, Thanh Cơ thể hiện sức mạnh kinh người.' },
        { id: 's-7', chapterRange: 'Chương 19 - 21', phaseNumber: 3, summary: 'Bí mật về huyết mạch hoàng thất và nguồn gốc Ngưu Nhân được hé lộ.' },
      ],
      chapters: [
        {
          displayName: 'Bước chân lưu vong',
          position: 1,
          summary: 'Thanh Cơ tìm cách gia nhập quân nổi loạn.',
          scenes: [
            { code: 'sc-5', displayName: 'Gặp gỡ', position: 1, summary: 'Thanh Cơ gặp Hạc Minh tại doanh trại.' }
          ]
        }
      ],
    },
  ],
  characters: [
    { 
      id: 'c-1', 
      name: 'Phù Thanh Cơ', 
      gender: 'Nữ', 
      genderBadge: 'female', 
      motif: { description: 'strong girl' },
      appearance: {
        species: 'Ngưu Nhân (Humanoid)',
        birth: '10/10/1998',
        gender: { identity: 'Nữ', expression: 'Nữ tính' },
        heightCm: 165,
        weightKg: 52,
        hair: { color: 'Đen', style: 'Buộc hai bên kiểu twintail thấp', texture: 'Thẳng mượt', length: 'Dài' },
        skin: { tone: 'Trắng sáng', texture: 'Mịn' },
        eyes: { pupil: { shape: 'Tròn', color: 'Đen' }, iris: { texture: null, color: 'Đen ánh xanh tím' }, sclera: 'Trắng' },
        outfitStyle: 'Trang phục kiếm sĩ',
        distinguishingMarks: ['Nốt ruồi đen khóe mắt trái'],
        summary: 'Nữ kiếm sĩ trẻ tuổi, mang vẻ ngoài lạnh lùng nhưng ẩn chứa sự kiên định.'
      },
      competences: [
        {
          id: 'comp-1', name: 'Ngự Huyết Kiếm Pháp', domain: 'Kiếm thuật', description: 'Điều khiển huyết mạch hòa quyện cùng kiếm khí.',
          sceneLearned: 'Chương 3', capabilities: ['Tăng tốc độ chém', 'Tạo lá chắn huyết mạch'], limitations: ['Tiêu hao sinh lực']
        }
      ],
      memories: [
        {
          id: 'mem-1', type: 'Episodic', status: 'Active', sourceScene: 'Chương 1',
          content: 'Đêm tế lễ, ánh mắt trưởng lão Hạc lạnh lẽo nhìn tôi bị đẩy lên tế đàn. Mùi máu tanh nồng và tiếng niệm chú chói tai.'
        }
      ],
      psychological: {
        coreTrauma: { wound: 'Bị phản bội bởi chính tộc nhân', lie: 'Chỉ có sức mạnh mới bảo vệ được bản thân', fear: 'Bị giam cầm và bất lực' },
        moralFramework: { alignment: 'Chaotic Good', code: 'Bảo vệ kẻ yếu, tiêu diệt kẻ áp bức', breakingPoint: 'Bất kỳ ai làm hại Hạc Minh' },
        narrativeEngine: { want: 'Phục hưng bộ tộc', need: 'Học cách tin tưởng người khác', conflict: 'Sự trả thù và lòng nhân từ' }
      }
    },
    { id: 'c-2', name: 'Hạc Minh', gender: 'Nam', genderBadge: 'male', motif: { description: 'loyal knight' }, appearance: null as any, competences: [], memories: [], psychological: null as any },
    { id: 'c-3', name: 'Trưởng lão Hạc', gender: 'Nam', genderBadge: 'male', motif: { description: 'blind zealot' }, appearance: null as any, competences: [], memories: [], psychological: null as any },
  ],
  sceneConfig: {
    arcs: [
      {
        displayName: 'Hồi 1: Huyết tế tế đàn',
        position: 1,
        chapters: [
          {
            displayName: 'Khởi nguyên đứt gãy',
            position: 1,
            scenes: [
              {
                code: 'sc-1',
                displayName: 'Thức tỉnh',
                position: 1,
                summary: 'Thanh Cơ tỉnh dậy trong tế đàn, xung quanh là máu và tiếng niệm chú vang vọng.',
                worldSettingCode: 'ws-1',
                stylePresetCode: 'action.combat',
                narratorConfigCode: 'first-person-protagonist',
                loreEntriesCode: 'Thánh tích Ngưu Nhân',
                characters: [
                  { characterId: 'c-1', characterCode: 'phu-thanh-co', displayName: 'Phù Thanh Cơ', isPov: true },
                  { characterId: 'c-3', characterCode: 'truong-lao-hac', displayName: 'Trưởng lão Hạc', isPov: false },
                ],
              },
              {
                code: 'sc-2',
                displayName: 'Tiếng gọi',
                position: 2,
                summary: 'Nghe thấy tiếng gọi từ sâu trong huyết mạch, Thanh Cơ quyết định phá vỡ phong ấn.',
                worldSettingCode: 'ws-1',
                stylePresetCode: 'mystery.deduction',
                narratorConfigCode: 'third-person-limited-single',
                loreEntriesCode: null,
                characters: [
                  { characterId: 'c-1', characterCode: 'phu-thanh-co', displayName: 'Phù Thanh Cơ', isPov: true },
                  { characterId: 'c-2', characterCode: 'hac-minh', displayName: 'Hạc Minh', isPov: false },
                ],
              },
            ],
          },
        ],
      },
    ],
  },
  worldSettings: {
    code: 'ws-1',
    displayName: 'Hạ Nam Vương Quốc',
    type: 'REALM',
    parentCode: '',
    description: 'Một vương quốc cổ đại nơi con người và các chủng tộc khác sinh sống. Nơi đây tồn tại những tàn tích ma pháp từ thời kỳ hỗn mang.',
    access: 'Tự do đi lại (ngoại trừ khu vực Tế Đàn bị phong tỏa)',
    loreEntries: [
      {
        code: 'lore-1',
        displayName: 'Thánh tích Ngưu Nhân',
        type: 'ITEM',
        description: 'Một mảnh vỡ pha lê chứa đựng linh hồn của tộc trưởng đời đầu.',
        data: 'Có thể phát sáng khi đến gần khu vực tế đàn.'
      },
      {
        code: 'lore-2',
        displayName: 'Sự kiện Đêm Đen',
        type: 'HISTORY',
        description: 'Sự kiện tàn sát toàn bộ tộc Ngưu Nhân trong quá khứ.',
        data: 'Ghi chép bị thất lạc, chỉ còn tồn tại trong truyền miệng.'
      }
    ]
  }
};
