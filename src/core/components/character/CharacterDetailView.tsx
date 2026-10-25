import { useState } from "react";
import { Link } from "react-router";
import {
  Pencil,
  MessageSquare,
  Lock,
  Unlock,
  Plus,
  Trash2,
  GripVertical,
  X,
  Check,
} from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { truncate } from "../../../shared/utils/string";
import { useResizablePanel } from "../../../shared/hooks/useResizablePanel";
import { CharacterChatPane } from "./CharacterChatPane";
import { ConfirmDialog } from "../../../shared/components/ui/confirm-dialog";
import { toast } from "../../../shared/components/ui/toast";
import { useT } from "../../../shared/i18n/useT";
import type {
  CharacterItem,
  RelationshipStateLevel,
  DistinguishingMark,
  MeasurementUnit,
} from "./types";

interface CharacterDetailViewProps {
  initialCharacter: CharacterItem;
}

type TabType = "basic" | "appearance" | "marks" | "relationships";

const STATE_COLORS: Record<RelationshipStateLevel, string> = {
  NONE: "bg-slate-200 text-slate-600 dark:bg-slate-800 dark:text-slate-400",
  LOW: "bg-blue-100 text-blue-700 dark:bg-blue-950/70 dark:text-blue-300",
  MODERATE: "bg-amber-100 text-amber-800 dark:bg-amber-950/70 dark:text-amber-300",
  HIGH: "bg-indigo-100 text-indigo-800 dark:bg-indigo-950/70 dark:text-indigo-300",
  ABSOLUTE: "bg-rose-100 text-rose-800 dark:bg-rose-950/70 dark:text-rose-300",
};

export function CharacterDetailView({ initialCharacter }: CharacterDetailViewProps) {
  const t = useT();
  const [character, setCharacter] = useState<CharacterItem>(initialCharacter);
  const [editSnapshot, setEditSnapshot] = useState<CharacterItem | null>(null);
  const [activeTab, setActiveTab] = useState<TabType>("basic");
  const [isEditing, setIsEditing] = useState(false);
  const [discardOpen, setDiscardOpen] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [lockedFields, setLockedFields] = useState<Record<string, boolean>>({});

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  const handleStartEdit = () => {
    setEditSnapshot(structuredClone(character));
    setIsEditing(true);
  };
  const handleSave = () => {
    setIsEditing(false);
    setEditSnapshot(null);
    toast({ title: t("common.saved"), variant: "success" });
  };
  const handleCancelConfirm = () => {
    if (editSnapshot) setCharacter(editSnapshot);
    setIsEditing(false);
    setEditSnapshot(null);
    setDiscardOpen(false);
  };

  const toggleLock = (fieldKey: string) => {
    setLockedFields((prev) => ({ ...prev, [fieldKey]: !prev[fieldKey] }));
  };

  const handleUpdateField = (path: string[], value: any) => {
    setCharacter((prev) => {
      const clone = structuredClone(prev);
      let current: any = clone;
      for (let i = 0; i < path.length - 1; i++) {
        current = current[path[i]];
      }
      current[path[path.length - 1]] = value;
      return clone;
    });
  };

  const handleAddMark = () => {
    const newMark: DistinguishingMark = {
      id: `mark-${Date.now()}`,
      location: "",
      description: "",
    };
    setCharacter((prev) => ({
      ...prev,
      distinguishingMarks: [...prev.distinguishingMarks, newMark],
    }));
  };

  const handleDeleteMark = (id: string) => {
    setCharacter((prev) => ({
      ...prev,
      distinguishingMarks: prev.distinguishingMarks.filter((m) => m.id !== id),
    }));
  };

  const renderField = (
    label: string,
    fieldKey: string,
    value: string | number,
    path: string[],
    type: "text" | "number" | "textarea" = "text",
    unitSelector?: {
      currentUnit: MeasurementUnit;
      unitPath: string[];
      units: MeasurementUnit[];
    },
  ) => {
    const isLocked = lockedFields[fieldKey] ?? false;

    return (
      <div className="group relative rounded-xl border border-slate-200 bg-white p-3.5 shadow-2xs transition-all dark:border-slate-800 dark:bg-slate-900">
        {isEditing && (
          <div
            className={cn(
              "absolute right-2.5 top-2.5 z-10 transition-opacity",
              isLocked ? "opacity-100" : "opacity-0 group-hover:opacity-100",
            )}
          >
            <button
              type="button"
              onClick={() => toggleLock(fieldKey)}
              className={cn(
                "flex h-6 w-6 items-center justify-center rounded-lg border shadow-2xs transition-colors",
                isLocked
                  ? "border-rose-300 bg-rose-50 text-rose-600 dark:border-rose-800 dark:bg-rose-950 dark:text-rose-300"
                  : "border-slate-200 bg-white text-slate-400 hover:text-slate-800 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300",
              )}
            >
              {isLocked ? <Lock className="h-3 w-3" /> : <Unlock className="h-3 w-3" />}
            </button>
          </div>
        )}

        <label className="block text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
          {label}
        </label>

        <div className="mt-1.5 flex items-center gap-2">
          {isEditing ? (
            type === "textarea" ? (
              <textarea
                rows={3}
                disabled={isLocked}
                value={value}
                onChange={(e) => handleUpdateField(path, e.target.value)}
                className={cn(
                  "w-full resize-none rounded-lg border border-slate-200 bg-slate-50/60 p-2 text-xs leading-relaxed focus:outline-hidden dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200",
                  isLocked && "cursor-not-allowed text-slate-400 dark:text-slate-500",
                )}
              />
            ) : (
              <input
                type={type}
                disabled={isLocked}
                value={value}
                onChange={(e) =>
                  handleUpdateField(
                    path,
                    type === "number" ? Number(e.target.value) : e.target.value,
                  )
                }
                className={cn(
                  "w-full rounded-lg border border-slate-200 bg-slate-50/60 px-2.5 py-1.5 text-xs text-slate-800 focus:outline-hidden dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200",
                  isLocked && "cursor-not-allowed text-slate-400 dark:text-slate-500",
                )}
              />
            )
          ) : (
            <p className="text-xs font-medium leading-relaxed text-slate-800 dark:text-slate-200">
              {value || "—"}
            </p>
          )}

          {unitSelector && (
            <select
              disabled={isEditing ? isLocked : true}
              value={unitSelector.currentUnit}
              onChange={(e) =>
                handleUpdateField(unitSelector.unitPath, e.target.value as MeasurementUnit)
              }
              className="rounded-lg border border-slate-200 bg-slate-50 px-2 py-1.5 text-xs font-semibold text-slate-700 focus:outline-hidden dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300"
            >
              {unitSelector.units.map((u) => (
                <option key={u} value={u}>
                  {u}
                </option>
              ))}
            </select>
          )}
        </div>

        {isLocked && isEditing && (
          <span className="mt-1 inline-flex items-center gap-1 text-[10px] font-medium text-rose-500">
            <Lock className="h-2.5 w-2.5" />
            <span>{t("character.detail.lockedHint")}</span>
          </span>
        )}
      </div>
    );
  };

  const renderColorField = (
    label: string,
    colorKey: string,
    colorName: string,
    colorHex: string,
    colorPath: string[],
    hexPath: string[],
  ) => {
    const isLocked = lockedFields[colorKey] ?? false;

    return (
      <div className="group relative rounded-xl border border-slate-200 bg-white p-3.5 shadow-2xs dark:border-slate-800 dark:bg-slate-900">
        {isEditing && (
          <div
            className={cn(
              "absolute right-2.5 top-2.5 z-10 transition-opacity",
              isLocked ? "opacity-100" : "opacity-0 group-hover:opacity-100",
            )}
          >
            <button
              type="button"
              onClick={() => toggleLock(colorKey)}
              className={cn(
                "flex h-6 w-6 items-center justify-center rounded-lg border shadow-2xs transition-colors",
                isLocked
                  ? "border-rose-300 bg-rose-50 text-rose-600 dark:border-rose-800 dark:bg-rose-950 dark:text-rose-300"
                  : "border-slate-200 bg-white text-slate-400 hover:text-slate-800 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300",
              )}
            >
              {isLocked ? <Lock className="h-3 w-3" /> : <Unlock className="h-3 w-3" />}
            </button>
          </div>
        )}

        <label className="block text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
          {label}
        </label>

        <div className="mt-2 flex items-center gap-3">
          <div
            className="h-6 w-6 shrink-0 rounded-md border border-slate-300 shadow-2xs dark:border-slate-700"
            style={{ backgroundColor: colorHex }}
          />

          {isEditing ? (
            <div className="flex flex-1 gap-2">
              <input
                type="text"
                disabled={isLocked}
                value={colorName}
                onChange={(e) => handleUpdateField(colorPath, e.target.value)}
                className="w-full rounded-lg border border-slate-200 bg-slate-50/60 px-2 py-1 text-xs text-slate-800 focus:outline-hidden dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200"
                placeholder={t("character.detail.colorNamePlaceholder")}
              />
              <input
                type="color"
                disabled={isLocked}
                value={colorHex}
                onChange={(e) => handleUpdateField(hexPath, e.target.value)}
                className="h-7 w-9 cursor-pointer rounded-md border-0 bg-transparent p-0"
              />
            </div>
          ) : (
            <div className="text-xs">
              <span className="font-semibold text-slate-800 dark:text-slate-200">{colorName}</span>
              <span className="ml-2 font-mono text-[11px] text-slate-400">{colorHex}</span>
            </div>
          )}
        </div>
      </div>
    );
  };

  const tabs: Array<{ id: TabType; label: string }> = [
    { id: "basic", label: t("character.detail.basicInfo") },
    { id: "appearance", label: t("character.detail.appearance") },
    { id: "marks", label: t("character.detail.marks") },
    { id: "relationships", label: t("character.detail.relationships") },
  ];

  return (
    <div
      ref={containerRef}
      className={cn(
        "flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none",
      )}
    >
      {isChatOpen && (
        <div style={{ width: `${leftRatio}%` }} className="h-full overflow-hidden">
          <CharacterChatPane
            characterName={character.name}
            activeSection={t(`character.detail.${activeTab}` as any)}
            onClose={() => setIsChatOpen(false)}
          />
        </div>
      )}

      {isChatOpen && (
        <div
          onMouseDown={startDragging}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-slate-200 transition-colors group-hover:bg-blue-500 dark:bg-slate-800" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-slate-300 bg-white shadow-xs transition-opacity dark:border-slate-700 dark:bg-slate-800",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100",
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-slate-500 dark:text-slate-400" />
          </div>
        </div>
      )}

      <div
        style={{ width: isChatOpen ? `${100 - leftRatio}%` : "100%" }}
        className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8"
      >
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-slate-200 pb-4 dark:border-slate-800">
          <div className="flex items-center gap-2 truncate text-xs font-semibold text-slate-500 dark:text-slate-400">
            <Link to="/character" className="shrink-0 hover:text-slate-900 dark:hover:text-slate-100">
              {t("character.title")}
            </Link>
            <span>›</span>
            <span
              className="truncate font-bold text-slate-900 dark:text-slate-100"
              title={character.name}
            >
              {truncate(character.name, 20)}
            </span>
          </div>

          <div className="flex shrink-0 items-center gap-2">
            {!isEditing ? (
              <button
                type="button"
                onClick={handleStartEdit}
                className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
              >
                <Pencil className="h-3.5 w-3.5" />
                <span>{t("character.detail.edit")}</span>
              </button>
            ) : (
              <>
                <button
                  type="button"
                  onClick={() => setDiscardOpen(true)}
                  className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
                >
                  <X className="h-3.5 w-3.5" />
                  <span>{t("common.cancel")}</span>
                </button>
                <button
                  type="button"
                  onClick={handleSave}
                  className="flex items-center gap-1.5 rounded-xl bg-slate-900 px-3 py-1.5 text-xs font-semibold text-white shadow-xs transition-colors hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
                >
                  <Check className="h-3.5 w-3.5" />
                  <span>{t("common.save")}</span>
                </button>
              </>
            )}

            <button
              type="button"
              onClick={() => setIsChatOpen(!isChatOpen)}
              className={cn(
                "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors",
                isChatOpen
                  ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                  : "border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800",
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>{t("character.detail.chat")}</span>
            </button>
          </div>
        </div>

        <div className="mt-6 flex flex-wrap gap-2 border-b border-slate-200 pb-3 dark:border-slate-800">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id)}
              className={cn(
                "rounded-xl px-3.5 py-1.5 text-xs font-semibold transition-all",
                activeTab === tab.id
                  ? "bg-slate-900 text-white shadow-xs dark:bg-slate-100 dark:text-slate-900"
                  : "bg-white text-slate-600 hover:bg-slate-100 dark:bg-slate-900 dark:text-slate-400 dark:hover:bg-slate-800",
              )}
            >
              {tab.label}
            </button>
          ))}
        </div>

        <div className="mt-6 space-y-6">
          {activeTab === "basic" && (
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
              {renderField(t("character.detail.fullName"), "name", character.name, ["name"])}
              {renderField(t("character.detail.species"), "species", character.species, ["species"])}
              {renderField(
                t("character.detail.biologicalSex"),
                "biologicalSex",
                character.biologicalSex,
                ["biologicalSex"],
              )}
              {renderField(
                t("character.detail.genderIdentity"),
                "genderIdentity",
                character.genderIdentity,
                ["genderIdentity"],
              )}
              {renderField(
                t("character.detail.genderExpression"),
                "genderExpression",
                character.genderExpression,
                ["genderExpression"],
              )}
            </div>
          )}

          {activeTab === "appearance" && (
            <div className="space-y-6">
              <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                {renderField(
                  t("character.detail.height"),
                  "height",
                  character.appearance.height,
                  ["appearance", "height"],
                  "number",
                  {
                    currentUnit: character.appearance.heightUnit,
                    unitPath: ["appearance", "heightUnit"],
                    units: ["cm", "m", "ft", "in"],
                  },
                )}
                {renderField(
                  t("character.detail.weight"),
                  "weight",
                  character.appearance.weight,
                  ["appearance", "weight"],
                  "number",
                  {
                    currentUnit: character.appearance.weightUnit,
                    unitPath: ["appearance", "weightUnit"],
                    units: ["kg", "lbs"],
                  },
                )}
              </div>

              <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
                <div className="space-y-3">
                  {renderColorField(
                    t("character.detail.hairColor"),
                    "hairColor",
                    character.appearance.hairColor,
                    character.appearance.hairHex,
                    ["appearance", "hairColor"],
                    ["appearance", "hairHex"],
                  )}
                  {renderField(
                    t("character.detail.hairNote"),
                    "hairNote",
                    character.appearance.hairNote,
                    ["appearance", "hairNote"],
                    "textarea",
                  )}
                </div>

                <div className="space-y-3">
                  {renderColorField(
                    t("character.detail.eyeColor"),
                    "eyeColor",
                    character.appearance.eyeColor,
                    character.appearance.eyeHex,
                    ["appearance", "eyeColor"],
                    ["appearance", "eyeHex"],
                  )}
                  {renderField(
                    t("character.detail.eyeNote"),
                    "eyeNote",
                    character.appearance.eyeNote,
                    ["appearance", "eyeNote"],
                    "textarea",
                  )}
                </div>

                <div className="space-y-3">
                  {renderColorField(
                    t("character.detail.skinTone"),
                    "skinTone",
                    character.appearance.skinTone,
                    character.appearance.skinHex,
                    ["appearance", "skinTone"],
                    ["appearance", "skinHex"],
                  )}
                  {renderField(
                    t("character.detail.skinNote"),
                    "skinNote",
                    character.appearance.skinNote,
                    ["appearance", "skinNote"],
                    "textarea",
                  )}
                </div>
              </div>

              {renderField(
                t("character.detail.outfitStyle"),
                "outfitStyle",
                character.appearance.outfitStyle,
                ["appearance", "outfitStyle"],
                "textarea",
              )}
              {renderField(
                t("character.detail.defaultSummary"),
                "defaultSummary",
                character.appearance.defaultSummary,
                ["appearance", "defaultSummary"],
                "textarea",
              )}
            </div>
          )}

          {activeTab === "marks" && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <p className="text-xs text-slate-500">{t("character.detail.marksHint")}</p>
                {isEditing && (
                  <button
                    type="button"
                    onClick={handleAddMark}
                    className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-semibold text-slate-700 shadow-2xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300"
                  >
                    <Plus className="h-3.5 w-3.5" />
                    <span>{t("character.detail.addMark")}</span>
                  </button>
                )}
              </div>

              <div className="space-y-3">
                {character.distinguishingMarks.map((mark, idx) => (
                  <div
                    key={mark.id}
                    className="flex items-start justify-between rounded-xl border border-slate-200 bg-white p-4 shadow-2xs dark:border-slate-800 dark:bg-slate-900"
                  >
                    <div className="flex-1 space-y-1 pr-4">
                      {isEditing ? (
                        <div className="space-y-2">
                          <input
                            type="text"
                            value={mark.location}
                            onChange={(e) => {
                              const updated = [...character.distinguishingMarks];
                              updated[idx].location = e.target.value;
                              setCharacter((prev) => ({ ...prev, distinguishingMarks: updated }));
                            }}
                            className="w-full rounded-lg border border-slate-200 bg-slate-50 p-2 text-xs font-semibold dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200"
                            placeholder={t("character.detail.markLocationPlaceholder")}
                          />
                          <textarea
                            rows={2}
                            value={mark.description}
                            onChange={(e) => {
                              const updated = [...character.distinguishingMarks];
                              updated[idx].description = e.target.value;
                              setCharacter((prev) => ({ ...prev, distinguishingMarks: updated }));
                            }}
                            className="w-full rounded-lg border border-slate-200 bg-slate-50 p-2 text-xs leading-relaxed dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200"
                            placeholder={t("character.detail.markDescriptionPlaceholder")}
                          />
                        </div>
                      ) : (
                        <>
                          <h4 className="text-xs font-bold text-slate-900 dark:text-slate-100">
                            {mark.location || "—"}
                          </h4>
                          <p className="text-xs leading-relaxed text-slate-600 dark:text-slate-400">
                            {mark.description}
                          </p>
                        </>
                      )}
                    </div>

                    {isEditing && (
                      <button
                        type="button"
                        onClick={() => handleDeleteMark(mark.id)}
                        className="rounded-lg p-1.5 text-slate-400 hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-950/50"
                      >
                        <Trash2 className="h-4 w-4" />
                      </button>
                    )}
                  </div>
                ))}

                {character.distinguishingMarks.length === 0 && (
                  <div className="py-10 text-center text-xs text-slate-400">
                    {t("character.detail.marksEmpty")}
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === "relationships" && (
            <div className="space-y-4">
              <p className="text-xs text-slate-500">
                {t("character.detail.relationshipsHint")}
              </p>

              <div className="space-y-4">
                {character.relationships.map((rel) => (
                  <div
                    key={rel.id}
                    className="rounded-2xl border border-slate-200 bg-white p-5 shadow-xs dark:border-slate-800 dark:bg-slate-900"
                  >
                    <div className="flex flex-col gap-2 border-b border-slate-100 pb-3 dark:border-slate-800 sm:flex-row sm:items-center sm:justify-between">
                      <div>
                        <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">
                          {rel.targetName}
                        </h4>
                        <span className="text-[11px] text-slate-400">{rel.category}</span>
                      </div>

                      <span className="self-start rounded-md bg-slate-100 px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                        {t("character.detail.role")}: {rel.role}
                      </span>
                    </div>

                    <div className="mt-4 grid grid-cols-2 gap-2 text-center sm:grid-cols-4 lg:grid-cols-7">
                      {(
                        Object.entries(rel.states) as [
                          keyof typeof rel.states,
                          RelationshipStateLevel,
                        ][]
                      ).map(([stateKey, stateVal]) => (
                        <div
                          key={stateKey}
                          className="rounded-xl border border-slate-100 bg-slate-50/60 p-2 dark:border-slate-800/80 dark:bg-slate-950/50"
                        >
                          <span className="mb-1 block text-[10px] font-semibold uppercase tracking-wider text-slate-400">
                            {stateKey}
                          </span>
                          <span
                            className={cn(
                              "inline-block rounded-md px-1.5 py-0.5 text-[10px] font-bold",
                              STATE_COLORS[stateVal],
                            )}
                          >
                            {stateVal}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}

                {character.relationships.length === 0 && (
                  <div className="py-12 text-center text-xs text-slate-400">
                    {t("character.detail.relationshipsEmpty")}
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>

      <ConfirmDialog
        open={discardOpen}
        onOpenChange={setDiscardOpen}
        title={t("discard.title")}
        description={t("discard.description")}
        confirmLabel={t("discard.confirm")}
        cancelLabel={t("discard.cancel")}
        variant="danger"
        onConfirm={handleCancelConfirm}
      />
    </div>
  );
}
