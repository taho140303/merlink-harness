import { useState } from "react";
import { Dialog, DialogContent, DialogTitle, DialogDescription } from "../../../shared/components/ui/dialog";
import { useT } from "../../../shared/i18n/useT";
import type { TKey } from "../../../shared/i18n/dict";
import type { CharacterItem, Gender } from "./types";
import { GENDER_OPTIONS } from "./types";

const GENDER_LABEL: Record<Gender, TKey> = {
  NONE: "gender.NONE",
  MALE: "gender.MALE",
  FEMALE: "gender.FEMALE",
};

interface CreateCharacterDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreate: (newChar: CharacterItem) => void;
}

export function CreateCharacterDialog({
  open,
  onOpenChange,
  onCreate,
}: CreateCharacterDialogProps) {
  const t = useT();
  const [name, setName] = useState("");
  const [species, setSpecies] = useState("Human");
  const [biologicalSex, setBiologicalSex] = useState<CharacterItem["biologicalSex"]>("Male");
  const [genderIdentity, setGenderIdentity] = useState<Gender>("NONE");
  const [genderExpression, setGenderExpression] = useState<Gender>("NONE");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;

    const newChar: CharacterItem = {
      id: `char-${Date.now()}`,
      name: name.trim(),
      species: species.trim() || "Human",
      biologicalSex,
      genderIdentity,
      genderExpression,
      avatarBg: "bg-blue-600",
      appearance: {
        height: 175,
        heightUnit: "cm",
        weight: 65,
        weightUnit: "kg",
        hairColor: "Black",
        hairHex: "#000000",
        hairNote: "",
        eyeColor: "Dark Brown",
        eyeHex: "#3d2314",
        eyeNote: "",
        skinTone: "Fair",
        skinHex: "#f0d5be",
        skinNote: "",
        outfitStyle: "Standard traveling attire",
        defaultSummary: "",
      },
      distinguishingMarks: [],
      relationships: [],
    };

    onCreate(newChar);
    onOpenChange(false);
    setName("");
    setSpecies("Human");
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
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t("character.create.namePlaceholder")}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs focus-ring   "
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
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs focus-ring   "
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.sexLabel")}
            </label>
            <select
              value={biologicalSex}
              onChange={(e) =>
                setBiologicalSex(e.target.value as CharacterItem["biologicalSex"])
              }
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs font-medium focus-ring   "
            >
              <option value="Male">{t("sex.male")}</option>
              <option value="Female">{t("sex.female")}</option>
              <option value="Intersex">{t("sex.intersex")}</option>
              <option value="Unknown">{t("sex.unknown")}</option>
            </select>
          </div>

          <div>
            <label className="text-xs font-semibold text-fg-muted">
              {t("character.create.genderIdentityLabel")}
            </label>
            <select
              value={genderIdentity}
              onChange={(e) => setGenderIdentity(e.target.value as Gender)}
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs font-medium focus-ring   "
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
              className="mt-1 w-full rounded-xl tier-input focus-ring px-3 py-2 text-xs font-medium focus-ring   "
            >
              {GENDER_OPTIONS.map((g) => (
                <option key={g} value={g}>
                  {t(GENDER_LABEL[g])}
                </option>
              ))}
            </select>
          </div>

          <div className="mt-6 flex justify-end gap-2 border-t border-line pt-3 ">
            <button
              type="button"
              onClick={() => onOpenChange(false)}
              className="rounded-xl px-4 py-2 text-xs font-medium text-fg-muted hover:bg-surface-muted  "
            >
              {t("character.create.cancel")}
            </button>
            <button
              type="submit"
              className="rounded-xl bg-accent px-4 py-2 text-xs font-semibold text-white  hover:opacity-90   "
            >
              {t("character.create.submit")}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
