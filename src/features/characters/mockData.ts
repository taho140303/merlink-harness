import type { CharacterProfile } from './types';

export const rimuruProfile: CharacterProfile = {
  id: 'rimuru-tempest',
  code: 'rimuru-tempest',
  projectId: 'proj-3',
  projectName: 'Echoes of the Void',
  basicInfo: {
    name: 'Rimuru Tempest',
    species: 'Ultimate Slime (True Dragon-tier)',
    biologicalSex: 'NONE',
    genderIdentity: 'NONE',
    genderExpression: 'NONE',
  },
  avatarBg: 'bg-indigo-500',
  appearance: {
    height: 170,
    weight: 60,
    hair: 'Silver-blue',
    eyes: 'Golden',
    skin: 'Pale',
    hairNote: 'Long silver-blue hair, typically loose or in a ponytail. Shifts subtly with mood.',
    eyesNote: 'Golden eyes that shimmer. Can shift hue when expressing strong emotion.',
    skinNote: 'Pale, smooth, androgynous. Reshapable via slime physiology.',
    outfitStyle: 'Casual-elegant. Blue kimono-style top with black pants, rarely seen in formal attire.',
    defaultSummary: 'A humanoid slime with silver-blue hair and golden eyes. Androgynous beauty, calm and gentle presence.',
  },
  distinguishingMarks: [
    { id: 'm1', content: 'Predator skill mark subtly visible on upper chest when active' },
    { id: 'm2', content: 'Blue-tinted slime core glow under certain light' },
    { id: 'm3', content: 'Slight magicule aura (visible to magical sight)' },
  ],
  relationships: [
    {
      id: 'r1',
      targetName: 'Veldora Tempest',
      role: 'EQUAL',
      category: 'SOCIAL',
      states: {
        trust: 'ABSOLUTE',
        affection: 'HIGH',
        hostility: 'NONE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'ABSOLUTE',
        dependency: 'MODERATE',
      },
    },
    {
      id: 'r2',
      targetName: 'Great Sage',
      role: 'EQUAL',
      category: 'SOCIAL',
      states: {
        trust: 'ABSOLUTE',
        affection: 'MODERATE',
        hostility: 'NONE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'ABSOLUTE',
        dependency: 'HIGH',
      },
    },
    {
      id: 'r3',
      targetName: 'Shion',
      role: 'INFERIOR',
      category: 'SOCIAL',
      states: {
        trust: 'HIGH',
        affection: 'HIGH',
        hostility: 'NONE',
        fear: 'LOW',
        respect: 'HIGH',
        familiarity: 'HIGH',
        dependency: 'LOW',
      },
    },
    {
      id: 'r4',
      targetName: 'Hinata Sakaguchi',
      role: 'EQUAL',
      category: 'ANTAGONISTIC',
      states: {
        trust: 'MODERATE',
        affection: 'MODERATE',
        hostility: 'LOW',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'MODERATE',
        dependency: 'NONE',
      },
    },
  ],
  competences: [
    {
      id: 'c1',
      name: 'Predator',
      domain: 'Unique Skill',
      level: 'Ultimate',
      description: 'Absorbs and analyzes targets, gaining their abilities.',
      experience: 'Used extensively since reincarnation.',
      capabilities: [
        'Analyze and copy skills',
        'Store objects in stomach dimension',
        'Mimicry of absorbed beings',
      ],
      limitations: [
        'Requires analysis time for complex targets',
        'Cannot replicate non-physical traits',
        'Weakness to holy attributes',
      ],
    },
    {
      id: 'c2',
      name: 'Great Sage',
      domain: 'Ultimate Skill',
      level: 'Ultimate',
      description: 'An intelligence capable of independent thought and parallel processing.',
      experience: 'Constant companion since reincarnation.',
      capabilities: [
        'Parallel calculation',
        'Instant skill analysis',
        'Multi-tasking across battles',
      ],
      limitations: [
        "Limited by Rimuru's own comprehension",
        'Cannot act on decisions Rimuru rejects',
      ],
    },
    {
      id: 'c3',
      name: 'Jura Forest Records',
      domain: 'Lore Knowledge',
      level: 'Basic',
      description: 'Historical lore and ecological knowledge of the Great Jura Forest.',
      capabilities: [],
      limitations: ['Incomplete on ancient ruins'],
    },
  ],
  memories: [
    {
      id: 'mem1',
      content: 'Chết trong bệnh viện một mình sau khi bị đâm để cứu đồng nghiệp. Nghe thấy giọng nói mờ dần, sau đó tỉnh lại trong một bóng tối kỳ lạ dưới hình dạng slime.',
      sourceScene: 'Chương 1',
      createdAt: 'Khởi đầu',
    },
    {
      id: 'mem2',
      content: 'Gặp gỡ Veldora - Bạo Phong Long bị phong ấn trong hang động sâu. Cả hai trò chuyện về thế giới, bão tố và manga, sau đó trao tên cho nhau.',
      sourceScene: 'Chương 2',
      createdAt: 'Ngày 2',
    },
    {
      id: 'mem3',
      content: 'Trận chiến với Chúa Tể Orc và sự sụp đổ của các làng Ogre. Lần đầu tiên cảm nhận gánh nặng sinh mệnh khi đưa đồng minh vào chiến trường sinh tử.',
      sourceScene: 'Cuộc chiến rừng Jura',
      createdAt: 'Tháng thứ 3',
    },
  ],
  psychological: {
    coreTrauma: {
      theWound: 'Died alone in a hospital after a life of isolation. Waking as a slime - alone again in an unfamiliar world.',
      theLie: 'If I give enough to others, I will never be alone again.',
      fundamentalFear: 'Ending up alone again - loved by no one, remembered by no one.',
    },
    cognitiveMoral: {
      moralAlignment: 'Neutral Good',
      moralCode: 'Protect lives above all. Never sacrifice those who trust me.',
      cognitiveBias: 'Over-attribution of responsibility - believes every death in his sphere is his failure.',
      breakingPoint: 'Deliberate harm to those he has named.',
    },
    narrativeEngine: {
      externalWant: 'To build a nation where all races can live in peace.',
      internalNeed: 'To believe he is worthy of love without having to earn it by continuous sacrifice.',
      internalConflict: 'The pacifist who became a Demon Lord.',
    },
    behavioralAdaptation: {
      socialMask: {
        personaType: 'The easygoing, cheerful leader.',
        verbalStyle: 'Casual and warm. Uses humor to defuse tension.',
      },
      defenseMechanisms: {
        primaryReaction: 'Diplomatic negotiation first, overwhelming force second.',
        stressTrigger: 'Harm to subordinates or betrayal by allies.',
        copingStrategy: 'Retreating inward, consulting Great Sage, then decisive action.',
      },
    },
  },
};

export const kaelenProfile: CharacterProfile = {
  id: 'char-1',
  code: 'kaelen-vance',
  projectId: 'proj-1',
  projectName: 'Chronicles of Aethelgard',
  basicInfo: {
    name: 'Kaelen Vance',
    species: 'High Human (Aether-touched)',
    biologicalSex: 'MALE',
    genderIdentity: 'MALE',
    genderExpression: 'MALE',
  },
  avatarBg: 'bg-indigo-500',
  appearance: {
    height: 182,
    weight: 73,
    hair: 'Silver Ash',
    eyes: 'Arcane Violet',
    skin: 'Pale Alabaster',
    hairNote: 'Mái tóc bạc cắt ngắn chạm gáy, luôn óng ánh ánh sáng tinh thể ma pháp.',
    eyesNote: 'Đồng tử phát quang mỗi khi giải phóng mạch chú thuật.',
    skinNote: 'Làn da nhợt nhạt do dành nhiều năm nghiên cứu dưới hầm mộ ngầm.',
    outfitStyle: 'Áo choàng nhung đen thêu chỉ bạc cấm thuật kết hợp găng tay gia cố rune.',
    defaultSummary: 'Dáng người cao ráo, khí chất lạnh lùng và luôn giữ vẻ ngoài xa cách.',
  },
  distinguishingMarks: [
    { id: 'm-1', content: 'Vết sẹo cháy ở cẳng tay phải do quá tải mạch dẫn pha lê thời niên thiếu.' },
    { id: 'm-2', content: 'Ký tự ấn chú cổ ngữ thuộc về gia tộc Vance sau gáy.' },
  ],
  relationships: [
    {
      id: 'rel-1',
      targetName: 'Lyra Dawnseeker',
      category: 'ANTAGONISTIC',
      role: 'EQUAL',
      states: {
        trust: 'LOW',
        affection: 'MODERATE',
        hostility: 'MODERATE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'HIGH',
        dependency: 'LOW',
      },
    },
    {
      id: 'rel-2',
      targetName: 'Archon Malakor',
      category: 'PROFESSIONAL',
      role: 'INFERIOR',
      states: {
        trust: 'NONE',
        affection: 'NONE',
        hostility: 'HIGH',
        fear: 'MODERATE',
        respect: 'MODERATE',
        familiarity: 'LOW',
        dependency: 'ABSOLUTE',
      },
    },
  ],
  competences: [
    {
      id: 'comp-1',
      name: 'Crystal Magic Weaving',
      domain: 'Arcane Sorcery',
      level: 'Master',
      description: 'Điều khiển tinh thể pha lê và dệt các mạch ma pháp phòng thủ liên tầng.',
      capabilities: ['Dệt lá chắn pha lê', 'Phóng tinh thể bẫy năng lượng'],
      limitations: ['Gây quá tải nhiệt lên mạch máu khi duy trì quá 5 phút'],
    },
  ],
  memories: [
    {
      id: 'mem-k1',
      content: 'Đêm kinh thành thất thủ, nhìn người thầy phong ấn cổng ma pháp rồi tan biến trong dải sáng hư không.',
      sourceScene: 'Hồi 1 - Cảnh 3',
      createdAt: 'Quá khứ 10 năm trước',
    },
  ],
};

export const lyraProfile: CharacterProfile = {
  id: 'char-2',
  code: 'lyra-dawnseeker',
  projectId: 'proj-1',
  projectName: 'Chronicles of Aethelgard',
  basicInfo: {
    name: 'Lyra Dawnseeker',
    species: 'Human (Solar Blessed)',
    biologicalSex: 'FEMALE',
    genderIdentity: 'FEMALE',
    genderExpression: 'FEMALE',
  },
  avatarBg: 'bg-amber-500',
  appearance: {
    height: 174,
    weight: 64,
    hair: 'Golden Dawn',
    eyes: 'Amber Gold',
    skin: 'Warm Sun-kissed',
    hairNote: 'Tóc vàng dài buộc đuôi ngựa gọn gàng để thuận tiện chiến đấu.',
    eyesNote: 'Ánh nhìn kiên nghị, sắc sảo và uy nghiêm.',
    skinNote: 'Khỏe khoắn do liên tục luyện tập thao trường ngoài trời.',
    outfitStyle: 'Giáp phiến thánh ngân nhẹ kết hợp áo choàng hiệp sĩ hội Thánh Quang.',
    defaultSummary: 'Hiệp sĩ thanh lịch, phong thái đĩnh đạc và toát lên tinh thần kỷ luật thép.',
  },
  distinguishingMarks: [
    { id: 'm-3', content: 'Dấu ấn thái dương của Đội Cận Vệ Hoàng Gia ở bả vai trái.' },
  ],
  relationships: [
    {
      id: 'rel-3',
      targetName: 'Kaelen Vance',
      category: 'SOCIAL',
      role: 'EQUAL',
      states: {
        trust: 'LOW',
        affection: 'MODERATE',
        hostility: 'MODERATE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'HIGH',
        dependency: 'NONE',
      },
    },
  ],
  competences: [
    {
      id: 'comp-2',
      name: 'Thánh Quang Kiếm Thuật',
      domain: 'Chiến kỹ',
      level: 'Thánh Hiệp Sĩ',
      description: 'Truyền quang năng thần thánh vào lưỡi gươm, thanh tẩy mọi tà thuật.',
      capabilities: ['Phát quang chấn kích', 'Thanh tẩy độc chú'],
      limitations: ['Mất hiệu lực khi vào khu vực chân không ma pháp'],
    },
  ],
  memories: [
    {
      id: 'mem-l1',
      content: 'Lời thề hiệp sĩ dưới thánh điện mặt trời, nhận lấy thanh kiếm bạc từ tay Đại Giáo Trưởng.',
      sourceScene: 'Khởi đầu',
      createdAt: '5 năm trước',
    },
  ],
};

export const vaelinProfile: CharacterProfile = {
  id: 'char-3',
  code: 'vaelin-grey',
  projectId: 'proj-2',
  projectName: 'The Sunken Leyline',
  basicInfo: {
    name: 'Vaelin Grey',
    species: 'Shadeborn',
    biologicalSex: 'MALE',
    genderIdentity: 'NONE',
    genderExpression: 'NONE',
  },
  avatarBg: 'bg-emerald-600',
  appearance: {
    height: 168,
    weight: 58,
    hair: 'Midnight Black',
    eyes: 'Shadow Grey',
    skin: 'Ashen Grey',
    hairNote: 'Mái tóc rối bù che một bên mắt.',
    eyesNote: 'Đồng tử thích nghi hoàn hảo trong môi trường bóng tối.',
    skinNote: 'Nước da nhợt nhạt đặc trưng của cư dân tầng ngầm.',
    outfitStyle: 'Bộ đồ bó da mềm cách âm kết hợp thắt lưng đa năng chứa đầy phi dao.',
    defaultSummary: 'Dáng người mảnh khảnh, di chuyển không phát ra bất kỳ tiếng động nào.',
  },
  distinguishingMarks: [
    { id: 'm-4', content: 'Vết chém song song từ dao găm độc ở cổ tay.' },
  ],
  relationships: [],
  competences: [
    {
      id: 'comp-3',
      name: 'Bước Đi Trong Bóng',
      domain: 'Ám thuật',
      level: 'Thượng đẳng',
      description: 'Hòa tan cơ thể vào các vùng bóng tối để di chuyển tức thời.',
      capabilities: ['Ẩn thân tuyệt đối trong tối', 'Di chuyển không tiếng động'],
      limitations: ['Rất nhạy cảm với nguồn sáng mạnh bất ngờ'],
    },
  ],
  memories: [
    {
      id: 'mem-v1',
      content: 'Những năm tháng sinh tồn dưới cống ngầm của thành phố cổ, học cách giết chóc trước khi học chữ.',
      sourceScene: 'Ký ức đầu đời',
      createdAt: 'Thời thơ ấu',
    },
  ],
};

export const phuThanhCoProfile: CharacterProfile = {
  id: 'phu-thanh-co',
  code: 'phu-thanh-co',
  projectId: 'proj-sample-500',
  projectName: 'Tế nữ ngưu nhân',
  basicInfo: {
    name: 'Phù Thanh Cơ',
    species: 'Ngưu Nhân (Humanoid)',
    biologicalSex: 'FEMALE',
    genderIdentity: 'FEMALE',
    genderExpression: 'FEMALE',
  },
  avatarBg: 'bg-rose-500',
  appearance: {
    height: 165,
    weight: 52,
    hair: 'Đen mượt',
    eyes: 'Đen ánh tím',
    skin: 'Trắng sáng',
    hairNote: 'Mái tóc đen dài buộc hai bên kiểu twintail thấp.',
    eyesNote: 'Đồng tử đen ánh xanh tím, sáng rực khi kích hoạt huyết mạch.',
    skinNote: 'Nước da trắng sáng, mịn màng.',
    outfitStyle: 'Trang phục kiếm sĩ tế đàn kết hợp găng tay hộ vệ.',
    defaultSummary: 'Nữ kiếm sĩ trẻ tuổi, vẻ ngoài điềm tĩnh kiên cường, mang trên vai vận mệnh tộc nhân.',
  },
  distinguishingMarks: [
    { id: 'm-ptc-1', content: 'Nốt ruồi đen nhỏ ở khóe mắt trái.' },
    { id: 'm-ptc-2', content: 'Ấn ký huyết mạch Ngưu Nhân sau bờ vai.' },
  ],
  relationships: [
    {
      id: 'rel-ptc-1',
      targetName: 'Hạc Minh',
      category: 'SOCIAL',
      role: 'EQUAL',
      states: {
        trust: 'HIGH',
        affection: 'HIGH',
        hostility: 'NONE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'HIGH',
        dependency: 'MODERATE',
      },
    },
  ],
  competences: [
    {
      id: 'comp-ptc-1',
      name: 'Ngự Huyết Kiếm Pháp',
      domain: 'Kiếm thuật',
      level: 'Đại sư',
      description: 'Điều khiển huyết mạch hòa quyện cùng kiếm khí, gia tăng tốc độ và sức sát thương.',
      capabilities: ['Tăng tốc độ chém', 'Tạo lá chắn huyết mạch', 'Phát hiện sát khí từ xa'],
      limitations: ['Tiêu hao sinh lực khi thi triển kéo dài', 'Bị khắc chế bởi ma pháp thanh tẩy cấp cao'],
    },
  ],
  memories: [
    {
      id: 'mem-ptc-1',
      content: 'Đêm tế lễ, ánh mắt lạnh lẽo của trưởng lão nhìn tôi bị đẩy lên tế đàn. Mùi máu tanh nồng và tiếng niệm chú chói tai.',
      sourceScene: 'Hồi 1 - Cảnh 1',
      createdAt: 'Thời điểm thức tỉnh',
    },
  ],
  psychological: {
    coreTrauma: {
      theWound: 'Bị phản bội bởi chính tộc nhân, chứng kiến sự hiến tế đẫm máu của đồng bào.',
      theLie: 'Chỉ cần tận tụy với thần quyền thì bộ tộc sẽ được che chở và bình yên.',
      fundamentalFear: 'Mất đi những người thân yêu và rơi vào trạng thái bất lực không bảo vệ được gia tộc.',
    },
    cognitiveMoral: {
      moralAlignment: 'Chaotic Good / Pragmatic Protector',
      moralCode: 'Bảo vệ người vô tội; nhưng kẻ nào hại đồng tộc thì phải trả giá triệt để.',
      cognitiveBias: 'Hoài nghi thần quyền và lòng trung thành tôn giáo mù quáng.',
      breakingPoint: 'Khi người thân cận hoặc đồng tộc vô tội bị đưa lên tế đàn làm vật hy sinh.',
    },
    narrativeEngine: {
      externalWant: 'Thoát khỏi sự truy sát của tế đàn và tìm nơi an toàn cho những người sống sót.',
      internalNeed: 'Chấp nhận gánh vác sứ mệnh dẫn dắt và chiến đấu thay vì tiếp tục chạy trốn.',
      internalConflict: 'Nỗi đau giằng xé giữa đức tin thuở nhỏ và thực tế tàn khốc của sự lừa dối.',
    },
    behavioralAdaptation: {
      socialMask: {
        personaType: 'Kiếm sĩ quả cảm, ít nói, cảnh giác cao độ',
        verbalStyle: 'Dứt khoát, điềm tĩnh, chỉ lên tiếng khi cần thiết',
      },
      defenseMechanisms: {
        primaryReaction: 'Kiểm soát huyết mạch kìm nén run sợ, tập trung chiến đấu',
        stressTrigger: 'Nghe thấy những bài tế văn cổ hoặc nhìn thấy tế đàn',
        copingStrategy: 'Rút gươm tự vệ, phân tích điểm yếu đối phương và chủ động phản công',
      },
    },
  },
};

export const hacMinhProfile: CharacterProfile = {
  id: 'hac-minh',
  code: 'hac-minh',
  projectId: 'proj-sample-500',
  projectName: 'Tế nữ ngưu nhân',
  basicInfo: {
    name: 'Hạc Minh',
    species: 'Ngưu Nhân (Humanoid)',
    biologicalSex: 'MALE',
    genderIdentity: 'MALE',
    genderExpression: 'MALE',
  },
  avatarBg: 'bg-blue-600',
  appearance: {
    height: 185,
    weight: 80,
    hair: 'Xanh thẫm',
    eyes: 'Hổ phách',
    skin: 'Rắn rỏi',
    hairNote: 'Tóc cắt ngắn gọn gàng kiểu chiến binh.',
    eyesNote: 'Đôi mắt sắc bén quan sát mọi biến động.',
    skinNote: 'Nước da ngăm khỏe khoắn phong sương.',
    outfitStyle: 'Giáp nhẹ kỵ sĩ hộ vệ vương tộc.',
    defaultSummary: 'Kỵ sĩ trung thành, luôn đứng phía trước bảo vệ đồng đội.',
  },
  distinguishingMarks: [
    { id: 'm-hm-1', content: 'Vết sẹo kiếm dọc theo bả vai trái.' },
  ],
  relationships: [],
  competences: [],
  memories: [],
};

export const haNamVuongProfile: CharacterProfile = {
  id: 'ha-nam-vuong',
  code: 'ha-nam-vuong',
  projectId: 'proj-sample-500',
  projectName: 'Tế nữ ngưu nhân',
  basicInfo: {
    name: 'Hạ Nam Vương',
    species: 'Nhân Tộc (Vương Quyền)',
    biologicalSex: 'MALE',
    genderIdentity: 'MALE',
    genderExpression: 'MALE',
  },
  avatarBg: 'bg-amber-600',
  appearance: {
    height: 178,
    weight: 75,
    hair: 'Đen pha bạc',
    eyes: 'Nâu trầm',
    skin: 'Quý tộc',
    hairNote: 'Búi tóc đội vương miện ngọc bích.',
    eyesNote: 'Ánh nhìn uy quyền thâm sâu khó dò.',
    skinNote: 'Làn da ít phong trần nhưng đầy uy nghiêm.',
    outfitStyle: 'Cẩm bào hoàng gia thêu rồng vàng.',
    defaultSummary: 'Quân vương trị vì vương quốc Hạ Nam, luôn cân nhắc giữa lợi ích đế chế và nhân từ.',
  },
  distinguishingMarks: [],
  relationships: [],
  competences: [],
  memories: [],
};

export const MOCK_CHARACTERS: CharacterProfile[] = [
  kaelenProfile,
  lyraProfile,
  vaelinProfile,
  rimuruProfile,
  phuThanhCoProfile,
  hacMinhProfile,
  haNamVuongProfile,
];

export const MOCK_CHARACTERS_MAP: Record<string, CharacterProfile> = {
  // Kaelen Vance
  'char-1': kaelenProfile,
  'kaelen-vance': kaelenProfile,
  'char-kaelen-vance': kaelenProfile,

  // Lyra Dawnseeker
  'char-2': lyraProfile,
  'lyra-dawnseeker': lyraProfile,

  // Vaelin Grey
  'char-3': vaelinProfile,
  'vaelin-grey': vaelinProfile,

  // Rimuru Tempest
  'rimuru-tempest': rimuruProfile,
  'rimuru': rimuruProfile,

  // Phù Thanh Cơ (Tế nữ ngưu nhân)
  'phu-thanh-co': phuThanhCoProfile,
  'c-1': phuThanhCoProfile,
  'char-phu-thanh-co': phuThanhCoProfile,

  // Hạc Minh
  'hac-minh': hacMinhProfile,
  'c-2': hacMinhProfile,
  'char-hac-minh': hacMinhProfile,

  // Hạ Nam Vương
  'ha-nam-vuong': haNamVuongProfile,
  'char-ha-nam-vuong': haNamVuongProfile,
};
