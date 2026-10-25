import { useState } from "react";
import { Dialog, DialogContent } from "../../../shared/components/ui/dialog";
import { SettingsSidebar } from "../settings/SettingsSidebar";
import { GeneralTab } from "../settings/GeneralTab";
import { AIProvidersTab } from "../settings/AIProvidersTab";
import type { SettingsTabId } from "../settings/types";

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const [activeTab, setActiveTab] = useState<SettingsTabId>("general");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl p-0 gap-0 overflow-hidden h-[620px] flex flex-col md:flex-row">
        <SettingsSidebar activeTab={activeTab} onSelectTab={setActiveTab} />
        <main className="flex-1 overflow-y-auto p-6">
          {activeTab === "general" && <GeneralTab />}
          {activeTab === "ai-providers" && <AIProvidersTab />}
        </main>
      </DialogContent>
    </Dialog>
  );
}
