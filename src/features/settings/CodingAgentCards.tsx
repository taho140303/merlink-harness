import { useState } from "react";
import { Button } from "@/shared/components/ui/button";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
import { ChooseModelsDialog } from "./ChooseModelsDialog";
import claudeLight from "@/shared/assets/model-provider/claudecode-light.svg";
import claudeDark from "@/shared/assets/model-provider/claudecode-dark.svg";
import openLight from "@/shared/assets/model-provider/opencode-light.svg";
import openDark from "@/shared/assets/model-provider/opencode-dark.svg";
import ollamaLight from "@/shared/assets/model-provider/ollama-light.svg";
import ollamaDark from "@/shared/assets/model-provider/ollama-dark.svg";
import codexLogo from "@/shared/assets/model-provider/codex.svg";

interface AgentItem {
  id: string;
  name: string;
  light: string;
  dark: string;
}

const agents: readonly AgentItem[] = [
  { id: "claude", name: "Claude Code", light: claudeLight, dark: claudeDark },
  { id: "open", name: "OpenCode", light: openLight, dark: openDark },
  { id: "ollama", name: "Ollama", light: ollamaLight, dark: ollamaDark },
  { id: "codex", name: "Codex", light: codexLogo, dark: codexLogo },
];

function CodingAgentCard({
  agent,
  connected,
  onToggle,
  onConfig,
}: {
  agent: AgentItem;
  connected: boolean;
  onToggle: () => void;
  onConfig: () => void;
}) {
  const t = useT();
  const isSingleLogo = agent.light === agent.dark;

  return (
    <li className="grid grid-cols-[2.75rem_minmax(0,1fr)_auto] items-center gap-3 rounded-xl border border-line bg-surface-card p-3 sm:gap-4 sm:px-4">
      <div className="flex h-11 w-11 items-center justify-center rounded-lg border border-line bg-surface-muted p-2">
        {isSingleLogo ? (
          <img src={agent.light} alt="" className="h-full w-full object-contain" />
        ) : (
          <>
            <img src={agent.light} alt="" className="h-full w-full object-contain dark:hidden" />
            <img src={agent.dark} alt="" className="hidden h-full w-full object-contain dark:block" />
          </>
        )}
      </div>
      <div className="min-w-0 space-y-1">
        <p className="text-[11px] font-medium text-fg-muted">{t("settings.provider.codingAgent")}</p>
        <h4 className="truncate text-sm font-semibold text-fg">{agent.name}</h4>
        <p className="flex items-center gap-1.5 text-xs text-fg-muted" role="status">
          <span aria-hidden="true" className={`h-2 w-2 shrink-0 rounded-full ${connected ? "bg-emerald-500" : "bg-fg-subtle"}`} />
          {connected ? t("settings.provider.agentConnected") : t("settings.provider.agentDisconnected")}
        </p>
      </div>
      <div className="flex flex-col items-stretch gap-1.5">
        <Button type="button" size="sm" variant={connected ? "secondary" : "primary"} onClick={onToggle} className="min-w-20 rounded-lg px-2">
          {connected ? t("settings.provider.agentDisconnect") : t("settings.provider.agentConnect")}
          <span className="sr-only"> {agent.name}</span>
        </Button>
        <Button
          type="button"
          size="sm"
          variant="secondary"
          disabled={!connected}
          onClick={onConfig}
          className="min-w-20 rounded-lg px-2"
        >
          {t("settings.provider.agentConfig")}
          <span className="sr-only"> {agent.name}</span>
        </Button>
      </div>
    </li>
  );
}

export function CodingAgentCards() {
  const t = useT();
  const [connectedIds, setConnectedIds] = useState<ReadonlySet<string>>(() => new Set());
  const [configAgentId, setConfigAgentId] = useState<string | null>(null);
  const [agentModelsMap, setAgentModelsMap] = useState<Record<string, string[]>>({});

  const toggle = (id: string) => {
    setConnectedIds((current) => {
      const next = new Set(current);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const handleAddModelsForAgent = (modelIds: string[]) => {
    if (!configAgentId) return;
    setAgentModelsMap((prev) => {
      const existing = new Set(prev[configAgentId] ?? []);
      for (const id of modelIds) {
        existing.add(id);
      }
      return {
        ...prev,
        [configAgentId]: Array.from(existing),
      };
    });
    toast({ title: t("common.saved"), variant: "success" });
  };

  return (
    <section aria-label={t("settings.provider.codingAgent")} className="space-y-3">
      <p className="text-xs leading-relaxed text-fg-muted">{t("settings.provider.agentDemoHint")}</p>
      <ul className="space-y-2.5">
        {agents.map((agent) => (
          <CodingAgentCard
            key={agent.id}
            agent={agent}
            connected={connectedIds.has(agent.id)}
            onToggle={() => toggle(agent.id)}
            onConfig={() => setConfigAgentId(agent.id)}
          />
        ))}
      </ul>

      <ChooseModelsDialog
        open={Boolean(configAgentId)}
        onOpenChange={(open) => {
          if (!open) setConfigAgentId(null);
        }}
        initialSelectedIds={configAgentId ? agentModelsMap[configAgentId] ?? [] : []}
        onAddModels={handleAddModelsForAgent}
      />
    </section>
  );
}
