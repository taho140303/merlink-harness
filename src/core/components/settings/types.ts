export type SettingsTabId = "general" | "ai-providers";

export interface ModelConfig {
  id: string;
  name: string;
  contextWindow: string;
  maxOutputTokens: string;
  inputTypes: {
    text: boolean;
    image: boolean;
  };
  reasoning: boolean;
}

export interface AIProviderItem {
  id: string;
  name: string;
  status: "connected" | "signed-in";
  isCustom?: boolean;
  providerType?: string;
  apiKey?: string;
  baseUrl?: string;
  apiProtocol?: string;
  models?: ModelConfig[];
}
