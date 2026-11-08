import type { BusinessFlowData } from './types';

export const SAMPLE_BUSINESS_FLOW_DATA: BusinessFlowData = {
  meta: {
    version: 'v.01',
    phase: 1,
    updated_at: new Date().toISOString(),
  },
  require: {
    project: {
      display_name: '',
      summary: '',
      language: 'VI',
      target: {
        arc_count: 0,
        chapter_count: 0,
        word_per_chapter_count: 0,
      },
      genres: [],
    },
    character: [],
  },
  optional: {},
};
