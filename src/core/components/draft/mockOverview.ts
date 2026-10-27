import type { DraftOverview } from './types';

export const DRAFT_OVERVIEW: DraftOverview = {
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
    },
  ],
  characters: [
    { id: 'c-1', name: 'Phù Thanh Cơ', gender: 'Nữ', genderBadge: 'female', motif: { description: 'strong girl' } },
    { id: 'c-2', name: 'Hạc Minh', gender: 'Nam', genderBadge: 'male', motif: { description: 'loyal knight' } },
    { id: 'c-3', name: 'Trưởng lão Hạc', gender: 'Nam', genderBadge: 'male', motif: { description: 'blind zealot' } },
  ],
};
