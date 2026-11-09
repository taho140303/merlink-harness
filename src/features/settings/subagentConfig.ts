import { backendClient, type BackendClient } from '@/shared/backend/client';

// Values are persisted by the Tauri backend; the browser preview fails closed without IPC.
export const DEFAULT_SUBAGENT_CONFIG = {
  maxSubagents: 6,
  rejectedThreshold: 3,
  outlineCapacity: 50,
  outlineLookback: 10,
  contextMargin: 5,
  chaptersPerSubagent: 30,
  loreRecordsPerSubagent: 10,
  charactersPerSubagent: 5,
  relationshipPairsPerSubagent: 10,
  model: 'chat',
} as const;

export type NumericSetting = Exclude<keyof typeof DEFAULT_SUBAGENT_CONFIG, 'model'>;
export type SubagentConfig = { [K in keyof typeof DEFAULT_SUBAGENT_CONFIG]: K extends 'model' ? string : number };
export type SubagentDraft = { [K in keyof typeof DEFAULT_SUBAGENT_CONFIG]: string };
export type ValidationError = 'required' | 'integer' | 'minimum' | 'maximum' | 'lookback';

export const NUMERIC_LIMITS: Record<NumericSetting, { min: number; max?: number }> = {
  maxSubagents: { min: 1, max: 6 },
  rejectedThreshold: { min: 1 },
  outlineCapacity: { min: 1 },
  outlineLookback: { min: 0 },
  contextMargin: { min: 0 },
  chaptersPerSubagent: { min: 1 },
  loreRecordsPerSubagent: { min: 1 },
  charactersPerSubagent: { min: 1 },
  relationshipPairsPerSubagent: { min: 1 },
};

export function toDraft(config: SubagentConfig): SubagentDraft {
  return Object.fromEntries(Object.entries(config).map(([key, value]) => [key, String(value)])) as SubagentDraft;
}

export function validateSubagentDraft(draft: SubagentDraft): Partial<Record<NumericSetting, ValidationError>> {
  const errors: Partial<Record<NumericSetting, ValidationError>> = {};
  for (const key of Object.keys(NUMERIC_LIMITS) as NumericSetting[]) {
    const raw = draft[key].trim();
    const { min, max } = NUMERIC_LIMITS[key];
    if (raw === '') errors[key] = 'required';
    else if (!/^-?\d+$/.test(raw) || !Number.isSafeInteger(Number(raw)) || Number(raw) > 2_147_483_647) errors[key] = 'integer';
    else if (Number(raw) < min) errors[key] = 'minimum';
    else if (max !== undefined && Number(raw) > max) errors[key] = 'maximum';
  }
  if (!errors.outlineLookback && !errors.outlineCapacity &&
      Number(draft.outlineLookback) >= Number(draft.outlineCapacity)) {
    errors.outlineLookback = 'lookback';
  }
  return errors;
}

export async function loadSubagentConfig(client: BackendClient = backendClient): Promise<SubagentConfig> {
  const response = await client.getRuntimeAndProviderConfig();
  return response.subagentConfig;
}

export async function saveSubagentConfig(config: SubagentConfig, client: BackendClient = backendClient): Promise<SubagentConfig> {
  const response = await client.updateRuntimeAndProviderConfig({ subagentConfig: config });
  return response.subagentConfig;
}

export function toConfig(draft: SubagentDraft): SubagentConfig {
  return {
    maxSubagents: Number(draft.maxSubagents),
    rejectedThreshold: Number(draft.rejectedThreshold),
    outlineCapacity: Number(draft.outlineCapacity),
    outlineLookback: Number(draft.outlineLookback),
    contextMargin: Number(draft.contextMargin),
    chaptersPerSubagent: Number(draft.chaptersPerSubagent),
    loreRecordsPerSubagent: Number(draft.loreRecordsPerSubagent),
    charactersPerSubagent: Number(draft.charactersPerSubagent),
    relationshipPairsPerSubagent: Number(draft.relationshipPairsPerSubagent),
    model: draft.model,
  };
}
