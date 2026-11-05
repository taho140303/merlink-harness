import { useState } from "react";
import { Dialog, DialogContent, DialogTitle, DialogDescription } from "@/shared/components/ui/dialog";
import { useT } from "@/shared/i18n/useT";
import type { TKey } from "@/shared/i18n/dict";
import type { CharacterProfile, Gender } from "./types";
import { GENDER_OPTIONS } from "./types";

const GENDER_LABEL: Record<Gender, TKey> = {
  NONE: "gender.NONE",
  MALE: "gender.MALE",
  FEMALE: "gender.FEMALE",
};

interface CreateCharacterDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreate: (newChar: CharacterProfile) => void;
}

export function CreateCharacterDialog({
  open,
  onOpenChange,
  onCreate,
}: CreateCharacterDialogProps) {
  const t = useT();
  const [name, setName] = useState("");
  const [nickname, setNickname] = useState("");
  const [species, setSpecies] = useState("Human");
  const [biologicalSex, setBiologicalSex] = useState<Gender>("MALE");
  const [genderIdentity, setGenderIdentity] = useState<Gender>("NONE");
  const [genderExpression, setGenderExpression] = useState<Gender>("NONE");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmedName = name.trim();
    const trimmedNickname = nickname.trim();
    if (!trimmedName && !trimmedNickname) return;

    const newChar: CharacterProfile = {
      id: `char-${Date.now()}`,
      basicInfo: {
        name: trimmedName || trimmedNickname,
        nickname: trimmedNickname || null,
        species: species.trim() || "Human",
        biologicalSex,
        genderIdentity,
        genderExpression,
      },
      avatarBg: "bg-blue-600",
      appearance: {
        height: 175,
        weight: 65,
        hair: "Black",
        eyes: "Dark Brown",
        skin: "Fair",
        hairNote: "",
        eyesNote: "",
        skinNote: "",
        outfitStyle: "Standard traveling attire",
        defaultSummary: "",
      },
      distinguishingMarks: [],
      relationships: [],
      competences: [],
      memories: [],
    };

    onCreate(newChar);
    onOpenChange(false);
    setName("");
    setNickname("");
    setSpecies("Human");
    setBiologicalSex("MALE");
    setGenderIdentity("NONE");
    setGenderExpression("NONE");
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md p-6">
        <DialogTitle className="text-base font-bold text-fg">
          {t("character.create.title")}
        </DialogTitle>
        <DialogDescription className="text-xs text-fg-muted">{t("character.create.subtitle")}</DialogDescription>

        <form onSubmit={handleSubmit} className="mt-4 space-y-4">
          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.nameLabel")}
            </label>
            <input
              type="text"
              required={!nickname.trim()}
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t("character.create.namePlaceholder")}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs"
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.nicknameLabel")}
            </label>
            <input
              type="text"
              required={!name.trim()}
              value={nickname}
              onChange={(e) => setNickname(e.target.value)}
              placeholder={t("character.create.nicknamePlaceholder")}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs"
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.speciesLabel")}
            </label>
            <input
              type="text"
              value={species}
              onChange={(e) => setSpecies(e.target.value)}
              placeholder={t("character.create.speciesPlaceholder")}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs"
            />
          </div>

          {/* 1.2: biologicalSex dùng GENDER_OPTIONS (NONE, MALE, FEMALE) */}
          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.sexLabel")}
            </label>
            <select
              value={biologicalSex}
              onChange={(e) => setBiologicalSex(e.target.value as Gender)}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs font-medium"
            >
              {GENDER_OPTIONS.map((g) => (
                <option key={g} value={g}>
                  {t(GENDER_LABEL[g])}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.genderIdentityLabel")}
            </label>
            <select
              value={genderIdentity}
              onChange={(e) => setGenderIdentity(e.target.value as Gender)}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs font-medium"
            >
              {GENDER_OPTIONS.map((g) => (
                <option key={g} value={g}>
                  {t(GENDER_LABEL[g])}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.genderExpressionLabel")}
            </label>
            <select
              value={genderExpression}
              onChange={(e) => setGenderExpression(e.target.value as Gender)}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs font-medium"
            >
              {GENDER_OPTIONS.map((g) => (
                <option key={g} value={g}>
                  {t(GENDER_LABEL[g])}
                </option>
              ))}
            </select>
          </div>

          <div className="mt-6 flex justify-end gap-2 border-t border-line pt-3">
            <button
              type="button"
              onClick={() => onOpenChange(false)}
              className="rounded-xl px-4 py-2 text-xs font-medium text-fg-muted hover:bg-surface-muted"
            >
              {t("character.create.cancel")}
            </button>
            <button
              type="submit"
              className="rounded-xl bg-accent px-4 py-2 text-xs font-semibold text-white hover:opacity-90"
            >
              {t("character.create.submit")}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
