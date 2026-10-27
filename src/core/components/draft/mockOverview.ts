import type { DraftOverview } from './types';

/** Dữ liệu mẫu cho khối tổng quan dự án trong Draft Building. */
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
};
