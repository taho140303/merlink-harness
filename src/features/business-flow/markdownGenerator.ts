import type { BiologicalSex, BusinessFlowData } from './types';

export function formatVal(value: string | number | undefined | null): string {
  const safe = value !== undefined && value !== null ? String(value) : '';
  return `\`${safe}\``;
}

/**
 * The backend serializes every mutable draft field as a tracked object
 * (`{ value, source }`), while older fixtures use plain scalars. Reading must
 * therefore unwrap both shapes; calling `String.prototype.trim` directly on a
 * tracked object crashes the process detail view.
 */
type TrackedLike<T> = { value?: T | null; source?: string | null };

function unwrapScalar(field: unknown): unknown {
  if (field === null || field === undefined) return undefined;
  if (typeof field === 'object') {
    return (field as TrackedLike<unknown>).value ?? undefined;
  }
  return field;
}

/** Reads a tracked/plain value as trimmed text, never throwing on object shapes. */
export function readText(field: unknown): string {
  const value = unwrapScalar(field);
  if (value === null || value === undefined) return '';
  if (typeof value === 'string') return value.trim();
  if (typeof value === 'number' || typeof value === 'boolean') return String(value);
  return '';
}

/** Reads a tracked/plain value as a finite number, defaulting to `0`. */
export function readNumber(field: unknown): number {
  const raw = unwrapScalar(field);
  if (typeof raw === 'number') return Number.isFinite(raw) ? raw : 0;
  const text = readText(raw);
  if (!text) return 0;
  const parsed = Number(text);
  return Number.isFinite(parsed) ? parsed : 0;
}

/** Reads a tracked/plain value as a boolean flag. */
export function readBoolean(field: unknown): boolean {
  const raw = unwrapScalar(field);
  if (typeof raw === 'boolean') return raw;
  const text = readText(raw).toLowerCase();
  return text === 'true' || text === '1';
}

export const LANGUAGE_LABEL_MAP: Record<'vi' | 'en', Record<string, string>> = {
  vi: {
    VI: 'Tiếng Việt',
    EN: 'Tiếng Anh',
    JA: 'Tiếng Nhật',
    ZH_HANT: 'Tiếng Trung (Phồn thể)',
    ZH_HANS: 'Tiếng Trung (Giản thể)',
  },
  en: {
    VI: 'Vietnamese',
    EN: 'English',
    JA: 'Japanese',
    ZH_HANT: 'Traditional Chinese',
    ZH_HANS: 'Simplified Chinese',
  },
};

export const GENDER_LABEL_MAP: Record<'vi' | 'en', Record<BiologicalSex, string>> = {
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

  const displayName = readText(project?.display_name);
  const language = readText(project?.language);
  const arcCount = readNumber(project?.target?.arc_count);
  const chapterCount = readNumber(project?.target?.chapter_count);
  const wordPerChapter = readNumber(project?.target?.word_per_chapter_count);
  const summary = readText(project?.summary);

  const primaryGenres = genres.filter((g) => readBoolean(g.is_primary));
  const secondaryGenres = genres.filter((g) => !readBoolean(g.is_primary));

  const genreLabel = (genre: (typeof genres)[number]): string =>
    readText(genre.display_name) || readText(genre.code);

  if (lang === 'vi') {
    const languageLabel = LANGUAGE_LABEL_MAP.vi[language] || language;

    // Overview Section
    const overviewLines = [
      '## Overview',
      `Dự án tên ${formatVal(displayName)}, sử dụng ngôn ngữ sáng tác là ${formatVal(languageLabel)}. Mục tiêu quy mô tác phẩm được thiết lập gồm ${formatVal(arcCount)} phân đoạn arc lớn, dự kiến đạt ${formatVal(chapterCount)} chương với mật độ bình quân ${formatVal(wordPerChapter)} từ mỗi chương.`,
    ];

    // Content Section
    const contentLines: string[] = ['## Content', '**Tóm tắt kịch bản:**', formatVal(summary), ''];

    // Genres
    contentLines.push('**Thể loại:**');
    if (primaryGenres.length > 0) {
      const primaryFormatted = primaryGenres.map((g) => formatVal(genreLabel(g))).join(', ');
      contentLines.push(`- Thể loại chủ đạo: ${primaryFormatted}`);
    } else {
      contentLines.push(`- Thể loại chủ đạo: ${formatVal('Chưa xác định')}`);
    }

    if (secondaryGenres.length > 0) {
      const secondaryFormatted = secondaryGenres.map((g) => formatVal(genreLabel(g))).join(', ');
      contentLines.push(`- Thể loại phối hợp: ${secondaryFormatted}`);
    }

    contentLines.push('');

    // Characters
    contentLines.push('**Nhân vật khởi tạo:**');
    if (characters.length > 0) {
      for (const char of characters) {
        const sex = readText(char.biological_sex) as BiologicalSex;
        const sexLabel = GENDER_LABEL_MAP.vi[sex] || sex;
        const charName = readText(char.display_name) || readText(char.nickname) || 'Không xác định';
        const nicknameLabel = readText(char.nickname) || 'Không xác định';
        contentLines.push(
          `- Nhân vật ${formatVal(charName)} (Biệt danh: ${formatVal(nicknameLabel)}): Giới tính ${formatVal(sexLabel)}, Motif khởi điểm ${formatVal(readText(char.motif_code))}`,
        );
      }
    } else {
      contentLines.push(`- ${formatVal('Không có nhân vật khởi tạo nào')}`);
    }

    return `${overviewLines.join('\n')}\n\n${contentLines.join('\n')}`;
  }

  // English version
  const languageLabel = LANGUAGE_LABEL_MAP.en[language] || language;

  const overviewLines = [
    '## Overview',
    `The project is titled ${formatVal(displayName)}, written in ${formatVal(languageLabel)}. The planned scale comprises ${formatVal(arcCount)} major arcs, targeting ${formatVal(chapterCount)} chapters with an average of ${formatVal(wordPerChapter)} words per chapter.`,
  ];

  const contentLines: string[] = ['## Content', '**Story Summary:**', formatVal(summary), ''];

  contentLines.push('**Genres:**');
  if (primaryGenres.length > 0) {
    const label = primaryGenres.length > 1 ? 'Primary genres' : 'Primary genre';
    const primaryFormatted = primaryGenres.map((g) => formatVal(genreLabel(g))).join(', ');
    contentLines.push(`- ${label}: ${primaryFormatted}`);
  } else {
    contentLines.push(`- Primary genre: ${formatVal('None')}`);
  }

  if (secondaryGenres.length > 0) {
    const secondaryFormatted = secondaryGenres.map((g) => formatVal(genreLabel(g))).join(', ');
    contentLines.push(`- Secondary genres: ${secondaryFormatted}`);
  }

  contentLines.push('');

  contentLines.push('**Initial Characters:**');
  if (characters.length > 0) {
    for (const char of characters) {
      const sex = readText(char.biological_sex) as BiologicalSex;
      const sexLabel = GENDER_LABEL_MAP.en[sex] || sex;
      const charName = readText(char.display_name) || readText(char.nickname) || 'Unknown';
      const nicknameLabel = readText(char.nickname) || 'Unknown';
      contentLines.push(
        `- Character ${formatVal(charName)} (Nickname: ${formatVal(nicknameLabel)}): Biological sex ${formatVal(sexLabel)}, Initial motif ${formatVal(readText(char.motif_code))}`,
      );
    }
  } else {
    contentLines.push(`- ${formatVal('No initial characters defined')}`);
  }

  return `${overviewLines.join('\n')}\n\n${contentLines.join('\n')}`;
}
