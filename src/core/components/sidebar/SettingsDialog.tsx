import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "../../../shared/components/ui/dialog";
import { SettingsSidebar } from "../settings/SettingsSidebar";
import { GeneralTab } from "../settings/GeneralTab";
import { AIProvidersTab } from "../settings/AIProvidersTab";
import { RecycleBinTab } from "../settings/RecycleBinTab";
import type { SettingsTabId } from "../settings/types";
import { useT } from "../../../shared/i18n/useT";

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const t = useT();
  const [activeTab, setActiveTab] = useState<SettingsTabId>("general");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex h-[620px] max-w-4xl flex-col gap-0 overflow-hidden p-0 md:flex-row">
        <DialogTitle className="sr-only">{t("settings.title")}</DialogTitle>
        <DialogDescription className="sr-only">{t("settings.subtitle")}</DialogDescription>
        <SettingsSidebar activeTab={activeTab} onSelectTab={setActiveTab} />
        <main className="flex-1 overflow-y-auto p-6">
          {activeTab === "general" && <GeneralTab />}
          {activeTab === "ai-providers" && <AIProvidersTab />}
          {activeTab === "recycle-bin" && <RecycleBinTab />}
        </main>
      </DialogContent>
    </Dialog>
  );
}
