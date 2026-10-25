import { useState } from "react";
import { Dialog, DialogContent } from "../../../shared/components/ui/dialog";
import type { CharacterItem } from "./types";

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
  const [name, setName] = useState("");
  const [species, setSpecies] = useState("Human");
  const [biologicalSex, setBiologicalSex] = useState<CharacterItem["biologicalSex"]>("Male");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;

    const newChar: CharacterItem = {
      id: `char-${Date.now()}`,
      name: name.trim(),
      species: species.trim() || "Human",
      biologicalSex,
      genderIdentity: biologicalSex === "Male" ? "Male" : biologicalSex === "Female" ? "Female" : "Non-binary",
      genderExpression: "Neutral",
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
        defaultSummary: "Hồ sơ nhân vật mới được tạo.",
      },
      distinguishingMarks: [],
      relationships: [],
    };

    onCreate(newChar);
    onOpenChange(false);
    setName("");
    setSpecies("Human");
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md p-6">
        <h3 className="text-base font-bold text-slate-900 dark:text-slate-100">
          Khởi tạo nhân vật mới
        </h3>
        <p className="text-xs text-slate-500">
          Nhập các thông số cơ bản để khởi tạo hồ sơ nhân vật vào kho lưu trữ.
        </p>

        <form onSubmit={handleSubmit} className="mt-4 space-y-4">
          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              Tên nhân vật *
            </label>
            <input
              type="text"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Ví dụ: Kaelen Vance"
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              Chủng tộc (Species)
            </label>
            <input
              type="text"
              value={species}
              onChange={(e) => setSpecies(e.target.value)}
              placeholder="Ví dụ: High Human, Elf, Shadeborn..."
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            />
          </div>

          <div>
            <label className="text-xs font-semibold text-slate-700 dark:text-slate-300">
              Giới tính sinh học
            </label>
            <select
              value={biologicalSex}
              onChange={(e) => setBiologicalSex(e.target.value as CharacterItem["biologicalSex"])}
              className="mt-1 w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-2 text-xs font-medium focus:outline-hidden dark:border-slate-800 dark:bg-slate-950 dark:text-slate-100"
            >
              <option value="Male">Male</option>
              <option value="Female">Female</option>
              <option value="Intersex">Intersex</option>
              <option value="Unknown">Unknown</option>
            </select>
          </div>

          <div className="mt-6 flex justify-end gap-2 border-t border-slate-100 pt-3 dark:border-slate-800">
            <button
              type="button"
              onClick={() => onOpenChange(false)}
              className="rounded-xl px-4 py-2 text-xs font-medium text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800"
            >
              Hủy
            </button>
            <button
              type="submit"
              className="rounded-xl bg-slate-900 px-4 py-2 text-xs font-semibold text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
            >
              Tạo nhân vật
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
