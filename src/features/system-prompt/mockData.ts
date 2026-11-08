import type {
  Component,
  Constraint,
  EmotionalRange,
  EventBlock,
  EventBlockEmotional,
  EventTransition,
  Genre,
  Motif,
  MotifApplicableGenre,
  MotifConstraint,
  NarrativeDirective,
  NarratorConfig,
  RhetoricalDevice,
  StateDownByRule,
  StateItem,
  StateRule,
  StylePreset,
  StylePresetFocusItem,
  StylePresetNarrative,
  StylePresetRhetoricalDevice,
  StylePresetVocabItem,
  Theme,
  ThemeArgument,
} from './types';

export const motifs: Motif[] = [];
export const genres: Genre[] = [];
export const motifApplicableGenres: MotifApplicableGenre[] = [];
export const components: Component[] = [];
export const states: StateItem[] = [];
export const stateRules: StateRule[] = [];
export const stateDownByRules: StateDownByRule[] = [];
export const emotionalRanges: EmotionalRange[] = [];
export const eventBlocks: EventBlock[] = [];
export const eventTransitions: EventTransition[] = [];
export const constraints: Constraint[] = [];
export const motifConstraints: MotifConstraint[] = [];
export const stylePresets: StylePreset[] = [];
export const rhetoricalDevices: RhetoricalDevice[] = [];
export const stylePresetRhetoricalDevices: StylePresetRhetoricalDevice[] = [];
export const stylePresetVocabItems: StylePresetVocabItem[] = [];
export const stylePresetFocusItems: StylePresetFocusItem[] = [];
export const narratorConfigs: NarratorConfig[] = [];
export const narrativeDirectives: NarrativeDirective[] = [];
export const stylePresetNarratives: StylePresetNarrative[] = [];
export const themes: Theme[] = [];
export const themeArguments: ThemeArgument[] = [];
export const eventBlockEmotionals: EventBlockEmotional[] = [];
