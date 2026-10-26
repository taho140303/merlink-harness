import type { Route } from "./+types/config";
import { SystemPromptLayout } from "../../core/components/system-prompt/SystemPromptLayout";

export function meta({}: Route.MetaArgs) {
  return [{ title: "Merlink Harness - System Prompt" }];
}

export default function ConfigPage() {
  return <SystemPromptLayout />;
}
