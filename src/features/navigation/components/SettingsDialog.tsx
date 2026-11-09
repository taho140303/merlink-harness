import { useEffect, useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/shared/components/ui/dialog";
import { SettingsSidebar } from "@/features/settings/SettingsSidebar";
import { GeneralTab } from "@/features/settings/GeneralTab";
import { AIProvidersTab } from "@/features/settings/AIProvidersTab";
import { RecycleBinTab } from "@/features/settings/RecycleBinTab";
import { SubagentOrchestrationTab } from "@/features/settings/SubagentOrchestrationTab";
import { DEFAULT_SUBAGENT_CONFIG, loadSubagentConfig, saveSubagentConfig, toDraft, type SubagentConfig } from "@/features/settings/subagentConfig";
import type { SettingsTabId } from "@/features/settings/types";
import { useT } from "@/shared/i18n/useT";

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const t = useT();
  const [activeTab, setActiveTab] = useState<SettingsTabId>("general");
  const [savedSubagentConfig, setSavedSubagentConfig] = useState<SubagentConfig>(DEFAULT_SUBAGENT_CONFIG);
  const [subagentDraft, setSubagentDraft] = useState(() => toDraft(DEFAULT_SUBAGENT_CONFIG));
  const [subagentLoading, setSubagentLoading] = useState(false);
  const [subagentLoaded, setSubagentLoaded] = useState(false);
  const [subagentSaving, setSubagentSaving] = useState(false);
  const [subagentError, setSubagentError] = useState<string | null>(null);

  useEffect(() => {
    if (!open || activeTab !== "subagent-orchestration") return;
    let active = true;
    setSubagentLoading(true);
    setSubagentLoaded(false);
    setSubagentError(null);
    loadSubagentConfig().then((config) => {
      if (!active) return;
      setSavedSubagentConfig(config);
      setSubagentDraft(toDraft(config));
      setSubagentLoaded(true);
    }).catch((error: unknown) => {
      if (active) setSubagentError(error instanceof Error ? error.message : String(error));
    }).finally(() => {
      if (active) setSubagentLoading(false);
    });
    return () => { active = false; };
  }, [open, activeTab]);

  const handleSaveSubagent = async (config: SubagentConfig) => {
    if (subagentSaving || !subagentLoaded) return;
    setSubagentSaving(true);
    setSubagentError(null);
    try {
      const saved = await saveSubagentConfig(config);
      setSavedSubagentConfig(saved);
      setSubagentDraft(toDraft(saved));
    } catch (error) {
      setSubagentError(error instanceof Error ? error.message : String(error));
    } finally {
      setSubagentSaving(false);
    }
  };

  useEffect(() => {
    const openRecycleBin = () => {
      setActiveTab("recycle-bin");
      onOpenChange(true);
    };
    window.addEventListener("merlink:open-recycle-bin", openRecycleBin);
    const closeSettings = () => onOpenChange(false);
    window.addEventListener("merlink:close-settings", closeSettings);
    return () => {
      window.removeEventListener("merlink:open-recycle-bin", openRecycleBin);
      window.removeEventListener("merlink:close-settings", closeSettings);
    };
  }, [onOpenChange]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex h-[620px] max-w-4xl flex-col gap-0 overflow-hidden p-0 md:flex-row">
        <DialogTitle className="sr-only">{t("settings.title")}</DialogTitle>
        <DialogDescription className="sr-only">{t("settings.subtitle")}</DialogDescription>
        <SettingsSidebar activeTab={activeTab} onSelectTab={setActiveTab} />
        <main className="min-h-0 min-w-0 flex-1 overflow-y-auto p-4 sm:p-6">
          {activeTab === "general" && <GeneralTab />}
          {activeTab === "ai-providers" && <AIProvidersTab />}
          {activeTab === "subagent-orchestration" && (
            <SubagentOrchestrationTab draft={subagentDraft} saved={savedSubagentConfig}
              loading={subagentLoading} loaded={subagentLoaded} saving={subagentSaving} error={subagentError}
              onDraftChange={setSubagentDraft} onSave={handleSaveSubagent} />
          )}
          {activeTab === "recycle-bin" && <RecycleBinTab />}
        </main>
      </DialogContent>
    </Dialog>
  );
}
