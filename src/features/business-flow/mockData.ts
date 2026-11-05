import type { BusinessFlowData } from './types';

export const SAMPLE_BUSINESS_FLOW_DATA: BusinessFlowData = {
  meta: {
    version: 'v.03',
    phase: 1,
    updated_at: '2026-10-29T10:45:00Z',
  },
  require: {
    project: {
      display_name: 'Tế nữ ngưu nhân',
      summary:
        'Truyện kể về Phù Thanh Cơ — nữ tế duy nhất còn lại của bộ tộc Ngưu Nhân giữa vương quốc Hạ Nam. Nàng phải giằng xé giữa lòng trung thành mù quáng với thần quyền tế đàn và sứ mệnh sinh tồn đẫm máu của giống loài trước hiểm họa tuyệt chủng.',
      language: 'VI',
      target: {
        arc_count: 4,
        chapter_count: 120,
        word_per_chapter_count: 2100,
      },
      genres: [
        { code: 'xianxia', is_primary: true, display_name: 'Tiên hiệp' },
        { code: 'fantasy', is_primary: false, display_name: 'Huyền huyễn' },
        { code: 'isekai', is_primary: false, display_name: 'Dị giới' },
        { code: 'strong_female', is_primary: false, display_name: 'Nữ cường' },
        { code: 'clan', is_primary: false, display_name: 'Tộc nhân' },
        { code: 'war', is_primary: false, display_name: 'Chiến tranh' },
      ],
    },
    character: [
      {
        code: 'phu-thanh-co',
        display_name: 'Phù Thanh Cơ',
        nickname: 'A Cơ',
        biological_sex: 'FEMALE',
        motif_code: 'strong_girl',
      },
      {
        code: 'ha-nam-vuong',
        display_name: 'Hạ Nam Vương',
        nickname: null,
        biological_sex: 'MALE',
        motif_code: 'ruler_king',
      },
      {
        code: 'hac-minh',
        display_name: 'Hạc Minh',
        nickname: 'A Minh',
        biological_sex: 'MALE',
        motif_code: 'loyal_knight',
      },
      {
        code: 'truong-lao-hac',
        display_name: 'Trưởng lão Hạc',
        nickname: null,
        biological_sex: 'MALE',
        motif_code: 'blind_zealot',
      },
    ],
  },
  optional: {
    overview_phase: {
      project: {
        target: {
          arc_tolerance: 0,
          chapter_tolerance: 5,
          word_per_chapter_tolerance: 300,
        },
      },
      arcs: [
        {
          display_name: 'Hồi 1: Huyết tế tế đàn',
          summary:
            'Phù Thanh Cơ bị đưa lên tế đàn, chứng kiến cái chết của trưởng lão và bắt đầu hành trình chạy trốn.',
          position: 1,
          motif: {
            code: 'sacrifice-and-rebirth',
            note: 'Bắt đầu chuỗi sự kiện phản nghịch.',
            event_blocks: [
              {
                code: 'eb-1',
                start_chapter: 1,
                end_chapter: 3,
                summary:
                  'Buổi hiến tế cát bụi, mùi máu tươi và sự hoài nghi trước lời phán của tế ti.',
              },
              {
                code: 'eb-2',
                start_chapter: 4,
                end_chapter: 6,
                summary:
                  'Đêm tế lễ, dị biến xảy ra, Phù Thanh Cơ thức tỉnh huyết mạch.',
              },
              {
                code: 'eb-3',
                start_chapter: 7,
                end_chapter: 9,
                summary:
                  'Trưởng lão Hạc xuất hiện, ngăn cản cuộc tẩu thoát của Thanh Cơ.',
              },
              {
                code: 'eb-4',
                start_chapter: 10,
                end_chapter: 12,
                summary:
                  'Phá hủy tế đàn, bắt đầu cuộc sống lưu vong đầy rẫy hiểm nguy.',
              },
            ],
          },
          themes: {
            code: 'theme-loyalty-survival',
            parent_theme_code: '',
            question:
              'Sự trung thành mù quáng với thần quyền có đáng để đánh đổi sinh mạng của cả tộc?',
            resolution: {
              type: 'action',
              content:
                'Phù Thanh Cơ từ bỏ tư tế, chấp nhận trở thành kẻ phản nghịch để bảo vệ tộc nhân.',
            },
            message:
              'Thần linh không phù hộ kẻ yếu, chỉ kẻ mạnh mới tự cứu lấy mình.',
            arguments: [
              {
                code: 'ta-1',
                character_display_name: 'Phù Thanh Cơ',
                role: 'primary',
                answer_mode: 'action',
                note: 'Quyết định phá hủy tế đàn, đánh đổi tu vi để cứu đồng tộc.',
                require: {
                  knowledge: 'Bí mật về cấu trúc linh hồn Ngưu Nhân',
                  competence: 'Khả năng điều khiển huyết mạch',
                },
              },
              {
                code: 'ta-2',
                character_display_name: 'Trưởng lão Hạc',
                role: 'antithesis',
                answer_mode: 'verbal',
                note: 'Tin rằng sự hy sinh là cần thiết để duy trì cân bằng.',
                require: {
                  knowledge: 'Kinh điển cổ của tế đàn',
                  competence: 'Thần quyền uy áp',
                },
              },
            ],
          },
        },
        {
          display_name: 'Hồi 2: Bóng tối vương quyền',
          summary:
            'Phù Thanh Cơ gia nhập quân đội nổi loạn, tìm cách tiếp cận hoàng thất.',
          position: 2,
          motif: {
            code: 'shadow-ascension',
            note: 'Phát triển kỹ năng lãnh đạo và mưu lược.',
            event_blocks: [
              {
                code: 'eb-5',
                start_chapter: 13,
                end_chapter: 15,
                summary:
                  'Gia nhập quân đội nổi loạn dưới vỏ bọc một y sư.',
              },
              {
                code: 'eb-6',
                start_chapter: 16,
                end_chapter: 18,
                summary:
                  'Trận chiến đầu tiên, Thanh Cơ thể hiện sức mạnh kinh người.',
              },
              {
                code: 'eb-7',
                start_chapter: 19,
                end_chapter: 21,
                summary:
                  'Bí mật về huyết mạch hoàng thất và nguồn gốc Ngưu Nhân được hé lộ.',
              },
            ],
          },
          themes: {
            code: 'theme-power-destiny',
            parent_theme_code: '',
            question: 'Liệu quyền lực có thể được sử dụng để thay đổi số phận?',
            resolution: {
              type: 'insight',
              content:
                'Thanh Cơ thâm nhập hoàng cung, khám phá ra sự thật về nguồn gốc của mình.',
            },
            message:
              'Quyền lực chỉ là công cụ, kẻ nắm giữ mới quyết định nó là ánh sáng hay bóng tối.',
            arguments: [
              {
                code: 'ta-3',
                character_display_name: 'Phù Thanh Cơ',
                role: 'primary',
                answer_mode: 'action',
                note: 'Âm thầm xây dựng mạng lưới đồng minh trong quân đội.',
                require: {
                  knowledge: 'Mạng lưới quý tộc Hạ Nam',
                  competence: 'Khả năng lãnh đạo và mưu lược',
                },
              },
            ],
          },
        },
      ],
    },
    chapter_phase: {
      arcs: [
        {
          chapters: [
            {
              display_name: 'Khởi nguyên đứt gãy',
              position: 1,
              summary: 'Phù Thanh Cơ phát hiện ra sự thật về huyết mạch của mình.',
              scenes: [
                {
                  code: 'sc-1',
                  display_name: 'Thức tỉnh',
                  position: 1,
                  summary:
                    'Thanh Cơ tỉnh dậy trong tế đàn, xung quanh là máu và tiếng niệm chú.',
                },
                {
                  code: 'sc-2',
                  display_name: 'Tiếng gọi',
                  position: 2,
                  summary:
                    'Nghe thấy tiếng gọi từ sâu trong huyết mạch, quyết định phá vỡ phong ấn.',
                },
              ],
            },
            {
              display_name: 'Bóng tối bao trùm',
              position: 2,
              summary: 'Trưởng lão Hạc xuất hiện, buộc tội Thanh Cơ phản bội.',
              scenes: [
                {
                  code: 'sc-3',
                  display_name: 'Đối chất',
                  position: 1,
                  summary: 'Cuộc tranh cãi nảy lửa tại đại điện tế đàn.',
                },
                {
                  code: 'sc-4',
                  display_name: 'Trốn chạy',
                  position: 2,
                  summary: 'Thanh Cơ vượt qua vòng vây chạy trốn khỏi sự truy sát.',
                },
              ],
            },
          ],
        },
      ],
    },
    world_building_phase: {
      world_settings: {
        code: 'ws-1',
        display_name: 'Hạ Nam Vương Quốc',
        type: 'REALM',
        parent_code: '',
        description:
          'Một vương quốc cổ đại nơi con người và các chủng tộc khác sinh sống. Nơi đây tồn tại những tàn tích ma pháp từ thời kỳ hỗn mang.',
        access: 'Tự do đi lại (ngoại trừ khu vực Tế Đàn bị phong tỏa)',
        lore_entries: [
          {
            code: 'lore-1',
            display_name: 'Thánh tích Ngưu Nhân',
            type: 'ITEM',
            description:
              'Một mảnh vỡ pha lê chứa đựng linh hồn của tộc trưởng đời đầu.',
            data: 'Có thể phát sáng khi đến gần khu vực tế đàn.',
          },
          {
            code: 'lore-2',
            display_name: 'Sự kiện Đêm Đen',
            type: 'HISTORY',
            description: 'Sự kiện tàn sát toàn bộ tộc Ngưu Nhân trong quá khứ.',
            data: 'Ghi chép bị thất lạc, chỉ còn tồn tại trong truyền miệng.',
          },
        ],
      },
    },
    scene_config_phase: {
      arcs: [
        {
          chapters: [
            {
              scenes: [
                {
                  style_preset_code: 'action.combat',
                  narrator_config_code: 'first-person-protagonist',
                  lore_entries_code: 'lore-1',
                  characters: [
                    { display_name: 'Phù Thanh Cơ', is_pov: true },
                    { display_name: 'Trưởng lão Hạc', is_pov: false },
                  ],
                },
                {
                  style_preset_code: 'mystery.deduction',
                  narrator_config_code: 'third-person-limited-single',
                  lore_entries_code: 'lore-2',
                  characters: [
                    { display_name: 'Phù Thanh Cơ', is_pov: true },
                    { display_name: 'Hạc Minh', is_pov: false },
                  ],
                },
              ],
            },
          ],
        },
      ],
    },
    character_appearance_phase: {
      character: [
        {
          code: 'phu-thanh-co',
          species: 'Ngưu Nhân (Humanoid)',
          birth: '10/10/1998',
          gender: {
            identity: 'Nữ',
            expresion: 'Nữ tính',
          },
          height_cm: 165,
          weight_kg: 52,
          hair: {
            color: 'đen',
            style: 'buộc hai bên kiểu twintail thấp',
            texture: 'thẳng mượt',
            length: 'dài',
          },
          skin: {
            tone: 'trắng sáng',
            texture: 'mịn',
          },
          eyes: {
            pupil: {
              shape: 'tròn',
              color: 'đen',
            },
            iris: {
              texture: null,
              color: 'đen ánh xanh tím',
            },
            sclera: 'trắng',
          },
          outfit_style: 'trang phục kiếm sĩ',
          distinguishing_marks: ['nốt ruồi đen khóe mắt trái'],
          summary:
            'Nữ kiếm sĩ trẻ tuổi, mang vẻ ngoài lạnh lùng nhưng ẩn chứa sự kiên định.',
        },
      ],
    },
    charactar_psychological_phase: {
      character: [
        {
          code: 'phu-thanh-co',
          core_trauma_and_belief: {
            the_wound:
              'Bị phản bội bởi chính tộc nhân, chứng kiến sự hiến tế đẫm máu của đồng bào.',
            the_lie:
              'Chỉ cần tận tụy với thần quyền thì bộ tộc sẽ được che chở và hòa bình.',
            fundamental_fear:
              'Mất đi những người thân yêu và rơi vào trạng thái bất lực không bảo vệ được gia tộc.',
          },
          cognitive_moral_framework: {
            moral_alignment:
              'Neutral Good chuyển dần sang Chaotic Good / Pragmatic Ruler',
            moral_code:
              'Tuyệt đối không xâm lược vô cớ; bảo vệ tối đa người vô tội; nhưng kẻ nào hại đồng tộc thì phải trả giá triệt để.',
            cognitive_bias:
              'Thiên kiến hoài nghi thần quyền và lòng trung thành tôn giáo mù quáng.',
            breaking_point:
              'Khi người thân cận hoặc đồng tộc vô tội bị đưa lên tế đàn làm vật hy sinh.',
          },
          narrative_engine: {
            external_want:
              'Thoát khỏi sự truy sát của tế đàn và tìm ra nơi an toàn cho những người sống sót.',
            internal_need:
              'Chấp nhận gánh vác sứ mệnh dẫn dắt và chiến đấu thay vì tiếp tục chạy trốn hay phục tùng.',
            internal_conflict:
              'Nỗi đau giằng xé giữa đức tin thuở nhỏ vào thần linh và thực tế tàn khốc của sự lừa dối.',
          },
          behavioral_adaptation: {
            social_mask: {
              persona_type:
                'Kiếm sĩ quả cảm, ít nói, sắc sảo và cảnh giác cao độ',
              verbal_style:
                'Dứt khoát, điềm tĩnh, chỉ lên tiếng khi cần thiết hoặc khi ra hiệu lệnh chiến thuật',
            },
            defense_mechanisms: {
              primary_reaction:
                'Tập trung cao độ vào chiến đấu và kiểm soát huyết mạch để kìm nén cảm xúc run sợ',
              stress_trigger:
                'Nghe thấy những bài tế văn cổ hoặc nhìn thấy biểu tượng hiến tế của tế đàn',
              coping_strategy:
                'Rút gươm tự vệ, phân tích điểm yếu của đối phương và chủ động phản công',
            },
          },
        },
      ],
    },
    character_life_experience_phase: {
      character: [
        {
          code: 'phu-thanh-co',
          memories: [
            {
              scene_code: 'sc-1',
              content:
                'Đêm tế lễ, ánh mắt trưởng lão Hạc lạnh lẽo nhìn tôi bị đẩy lên tế đàn. Mùi máu tanh nồng và tiếng niệm chú chói tai.',
            },
          ],
          competences: [
            {
              display_name: 'Ngự Huyết Kiếm Pháp',
              description: 'Điều khiển huyết mạch hòa quyện cùng kiếm khí.',
              domain: {
                code: 'swordplay',
                display_name: 'Kiếm thuật',
                description:
                  'Võ học cận chiến thượng thừa kết hợp dị năng huyết tộc',
                level: {
                  code: 'master',
                  display_name: 'Đại sư',
                  description: 'Đã lĩnh ngộ kiếm ý và huyết mạch tương thông',
                },
              },
              capabilities: [
                'Tăng tốc độ chém',
                'Tạo lá chắn huyết mạch',
                'Phát hiện sát khí từ xa',
              ],
              limitations: [
                'Tiêu hao sinh lực khi thi triển kéo dài',
                'Bị khắc chế bởi ma pháp thanh tẩy cấp cao',
              ],
            },
          ],
        },
      ],
    },
    relationship_phase: {
      relationship: [
        {
          motif: {
            event_blocks: [
              {
                code: 'reb-1',
                component: {
                  code: 'ally_encounter',
                  display_name: 'Gặp gỡ đồng minh',
                },
                state: {
                  code: 'tentative_trust',
                  display_name: 'Thăm dò và tin tưởng ban đầu',
                },
                transition: {
                  display_name: 'Kề vai chiến đấu chống lại truy binh',
                },
              },
            ],
          },
          character: {
            from_code: 'phu-thanh-co',
            to_code: 'hac-minh',
            scenes: [
              {
                code: 'sc-2',
                trust: 'HIGH',
                affection: 'HIGH',
                hostility: 'NONE',
                fear: 'NONE',
                respect: 'HIGH',
                familarity: 'HIGH',
                dependency: 'MODERATE',
              },
            ],
            roles: [
              {
                hiarachy: 'EQUAL',
                category: 'SOCIAL',
                sub_type: 'Đồng minh sinh tử / Hộ vệ',
                is_primmary: true,
              },
            ],
          },
        },
        {
          motif: {
            event_blocks: [
              {
                code: 'reb-2',
                component: {
                  code: 'betrayal_confrontation',
                  display_name: 'Đối chất tế đàn',
                },
                state: {
                  code: 'hostile_rupture',
                  display_name: 'Đoạn tuyệt thần quyền',
                },
                transition: {
                  display_name: 'Trưởng lão hạ lệnh trừ khử tế nữ phản nghịch',
                },
              },
            ],
          },
          character: {
            from_code: 'phu-thanh-co',
            to_code: 'truong-lao-hac',
            scenes: [
              {
                code: 'sc-3',
                trust: 'NONE',
                affection: 'NONE',
                hostility: 'HIGH',
                fear: 'LOW',
                respect: 'MODERATE',
                familarity: 'HIGH',
                dependency: 'NONE',
              },
            ],
            roles: [
              {
                hiarachy: 'INFERIOR',
                category: 'ANTAGONISTIC',
                sub_type: 'Phản nghịch / Thù địch',
                is_primmary: true,
              },
            ],
          },
        },
      ],
    },
  },
};
