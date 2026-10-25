import { useState } from "react";
import { Dialog, DialogContent } from "../../../shared/components/ui/dialog";
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
        <h3 className="text-base font-bold text-slate-900 dark:text-slate-100">
          {t("character.create.title")}
        </h3>
        <p className="text-xs text-slate-500">{t("character.create.subtitle")}</p>

        <form onSubmit={handleSubmit} className="mt-4 space-y-4">
          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              {t("character.create.nameLabel")}
            </label>
            <input
              type="text"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t("character.create.namePlaceholder")}
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              {t("character.create.speciesLabel")}
            </label>
            <input
              type="text"
              value={species}
              onChange={(e) => setSpecies(e.target.value)}
              placeholder={t("character.create.speciesPlaceholder")}
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              {t("character.create.sexLabel")}
            </label>
            <select
              value={biologicalSex}
              onChange={(e) =>
                setBiologicalSex(e.target.value as CharacterItem["biologicalSex"])
              }
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs font-medium focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            >
              <option value="Male">{t("sex.male")}</option>
              <option value="Female">{t("sex.female")}</option>
              <option value="Intersex">{t("sex.intersex")}</option>
              <option value="Unknown">{t("sex.unknown")}</option>
            </select>
          </div>

          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              {t("character.create.genderIdentityLabel")}
            </label>
            <select
              value={genderIdentity}
              onChange={(e) => setGenderIdentity(e.target.value as Gender)}
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs font-medium focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            >
              {GENDER_OPTIONS.map((g) => (
                <option key={g} value={g}>
                  {t(GENDER_LABEL[g])}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              {t("character.create.genderExpressionLabel")}
            </label>
            <select
              value={genderExpression}
              onChange={(e) => setGenderExpression(e.target.value as Gender)}
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs font-medium focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            >
              {GENDER_OPTIONS.map((g) => (
                <option key={g} value={g}>
                  {t(GENDER_LABEL[g])}
                </option>
              ))}
            </select>
          </div>

          <div className="mt-6 flex justify-end gap-2 border-t border-slate-100 pt-3 dark:border-slate-800">
            <button
              type="button"
              onClick={() => onOpenChange(false)}
              className="rounded-xl px-4 py-2 text-xs font-medium text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800"
            >
              {t("character.create.cancel")}
            </button>
            <button
              type="submit"
              className="rounded-xl bg-slate-900 px-4 py-2 text-xs font-semibold text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
            >
              {t("character.create.submit")}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
