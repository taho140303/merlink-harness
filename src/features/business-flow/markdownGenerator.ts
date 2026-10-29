import type { BiologicalSex, BusinessFlowData } from './types';

function formatVal(value: string | number | undefined | null): string {
  const safe = value !== undefined && value !== null ? String(value) : '';
  return `\`<${safe}>\``;
}

const GENDER_LABEL_MAP: Record<'vi' | 'en', Record<BiologicalSex, string>> = {
  vi: {
    FEMALE: 'Nữ',
    MALE: 'Nam',
    NONE: 'Không xác định',
  },
  en: {
    FEMALE: 'Female',
    MALE: 'Male',
    NONE: 'None',
  },
};

export function generateRequireMarkdown(
  data: BusinessFlowData,
  lang: 'vi' | 'en' = 'vi',
): string {
  const project = data?.require?.project;
  const characters = data?.require?.character ?? [];
  const genres = project?.genres ?? [];

  const displayName = project?.display_name ?? '';
  const language = project?.language ?? '';
  const arcCount = project?.target?.arc_count ?? 0;
  const chapterCount = project?.target?.chapter_count ?? 0;
  const wordPerChapter = project?.target?.word_per_chapter_count ?? 0;
  const summary = project?.summary ?? '';

  const primaryGenres = genres.filter((g) => g.is_primary);
  const secondaryGenres = genres.filter((g) => !g.is_primary);

  if (lang === 'vi') {
    // Overview Section
    const overviewLines = [
      '## Overview',
      `Dự án tên ${formatVal(displayName)}, sử dụng ngôn ngữ sáng tác là ${formatVal(language)}. Mục tiêu quy mô tác phẩm được thiết lập gồm ${formatVal(arcCount)} phân đoạn arc lớn, dự kiến đạt ${formatVal(chapterCount)} chương với mật độ bình quân ${formatVal(wordPerChapter)} từ mỗi chương.`,
    ];

    // Content Section
    const contentLines: string[] = ['## Content', '**Tóm tắt kịch bản:**', formatVal(summary), ''];

    // Genres
    contentLines.push('**Thể loại:**');
    if (primaryGenres.length > 0) {
      const primaryFormatted = primaryGenres
        .map(
          (g) =>
            `${formatVal(g.display_name || g.code)} (mã: ${formatVal(g.code)})`,
        )
        .join(', ');
      contentLines.push(`- Thể loại chủ đạo: ${primaryFormatted}`);
    } else {
      contentLines.push(`- Thể loại chủ đạo: ${formatVal('Chưa xác định')}`);
    }

    if (secondaryGenres.length > 0) {
      const secondaryFormatted = secondaryGenres
        .map(
          (g) =>
            `${formatVal(g.display_name || g.code)} (mã: ${formatVal(g.code)})`,
        )
        .join(', ');
      contentLines.push(`- Thể loại phối hợp: ${secondaryFormatted}`);
    }

    contentLines.push('');

    // Characters
    contentLines.push('**Nhân vật khởi tạo:**');
    if (characters.length > 0) {
      for (const char of characters) {
        const sex = char.biological_sex;
        const sexLabel = GENDER_LABEL_MAP.vi[sex] || sex;
        contentLines.push(
          `- Nhân vật ${formatVal(char.display_name)}: Giới tính ${formatVal(sexLabel)} (mã: ${formatVal(sex)}), Motif khởi điểm ${formatVal(char.motif_code)}`,
        );
      }
    } else {
      contentLines.push(`- ${formatVal('Không có nhân vật khởi tạo nào')}`);
    }

    return `${overviewLines.join('\n')}\n\n${contentLines.join('\n')}`;
  }

  // English version
  const overviewLines = [
    '## Overview',
    `The project is titled ${formatVal(displayName)}, written in ${formatVal(language)}. The planned scale comprises ${formatVal(arcCount)} major arcs, targeting ${formatVal(chapterCount)} chapters with an average of ${formatVal(wordPerChapter)} words per chapter.`,
  ];

  const contentLines: string[] = ['## Content', '**Story Summary:**', formatVal(summary), ''];

  contentLines.push('**Genres:**');
  if (primaryGenres.length > 0) {
    const label = primaryGenres.length > 1 ? 'Primary genres' : 'Primary genre';
    const primaryFormatted = primaryGenres
      .map(
        (g) =>
          `${formatVal(g.display_name || g.code)} (code: ${formatVal(g.code)})`,
      )
      .join(', ');
    contentLines.push(`- ${label}: ${primaryFormatted}`);
  } else {
    contentLines.push(`- Primary genre: ${formatVal('None')}`);
  }

  if (secondaryGenres.length > 0) {
    const secondaryFormatted = secondaryGenres
      .map(
        (g) =>
          `${formatVal(g.display_name || g.code)} (code: ${formatVal(g.code)})`,
      )
      .join(', ');
    contentLines.push(`- Secondary genres: ${secondaryFormatted}`);
  }

  contentLines.push('');

  contentLines.push('**Initial Characters:**');
  if (characters.length > 0) {
    for (const char of characters) {
      const sex = char.biological_sex;
      const sexLabel = GENDER_LABEL_MAP.en[sex] || sex;
      contentLines.push(
        `- Character ${formatVal(char.display_name)}: Biological sex ${formatVal(sexLabel)} (code: ${formatVal(sex)}), Initial motif ${formatVal(char.motif_code)}`,
      );
    }
  } else {
    contentLines.push(`- ${formatVal('No initial characters defined')}`);
  }

  return `${overviewLines.join('\n')}\n\n${contentLines.join('\n')}`;
}
