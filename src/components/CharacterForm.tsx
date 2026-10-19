import { useState, type ChangeEvent, type FormEvent } from "react";
import { ImagePlus } from "lucide-react";
import {
  EMPTY_FORM,
  MAX_UPLOAD_BYTES,
  PERSONALITY_TRAITS,
  type CharacterFormValues,
} from "../lib/characters";
import type { Character, Project } from "../types";

interface CharacterFormProps {
  projects: Project[];
  onClose: () => void;
  onCreate: (character: Character) => void;
}

export function CharacterForm({ projects, onClose, onCreate }: CharacterFormProps) {
  const [form, setForm] = useState<CharacterFormValues>(EMPTY_FORM);
  const [uploadError, setUploadError] = useState("");

  function field<K extends keyof CharacterFormValues>(name: K) {
    return {
      value: form[name] as string,
      onChange: (event: ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) =>
        setForm((current) => ({ ...current, [name]: event.target.value })),
    };
  }

  function toggleTrait(trait: string) {
    setForm((current) => ({
      ...current,
      tags: current.tags.includes(trait)
        ? current.tags.filter((value) => value !== trait)
        : [...current.tags, trait].slice(0, 5),
    }));
  }

  function upload(event: ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) return;
    if (!file.type.startsWith("image/")) {
      setUploadError("Choose a JPG, PNG, or WebP image.");
      return;
    }
    if (file.size > MAX_UPLOAD_BYTES) {
      setUploadError("Choose an image smaller than 1.5 MB.");
      return;
    }
    setUploadError("");
    const reader = new FileReader();
    reader.onload = () =>
      setForm((current) => ({ ...current, uploadedImage: String(reader.result) }));
    reader.readAsDataURL(file);
  }

  function submit(event: FormEvent) {
    event.preventDefault();
    if (!form.name.trim() || !form.role.trim()) return;
    const name = form.name.trim();
    const role = form.role.trim();
    const appearance = form.appearance.trim();
    const character: Character = {
      id: `custom-${Date.now()}`,
      name,
      role,
      age: form.age.trim(),
      gender: form.gender,
      story: form.story || projects[0]?.id || "",
      tags: form.tags,
      appearance,
      bio:
        form.bio.trim() ||
        appearance ||
        `${name} is a ${role.toLowerCase()} whose story is just beginning.`,
      backstory: form.backstory.trim(),
      relationships: [],
      dialogueCount: 0,
      uploadedImage: form.uploadedImage,
    };
    onCreate(character);
  }

  return (
    <form className="ch-dialog-body ch-form" onSubmit={submit}>
      <p className="ch-form-note">
        Give someone a place in your world. You can start with a few details and
        let the rest emerge as you write.
      </p>
      <div className="ch-form-grid">
        <div className="ch-form-field">
          <label htmlFor="ch-name">Full name *</label>
          <input
            id="ch-name"
            autoFocus
            required
            maxLength={65}
            placeholder="e.g. Mara Voss"
            {...field("name")}
          />
        </div>
        <div className="ch-form-field">
          <label htmlFor="ch-role">Role *</label>
          <select id="ch-role" required {...field("role")}>
            <option value="Main">Main character</option>
            <option value="Supporting">Supporting character</option>
            <option value="Antagonist">Antagonist</option>
          </select>
        </div>
        <div className="ch-form-field">
          <label htmlFor="ch-age">
            Age <span>optional</span>
          </label>
          <input
            id="ch-age"
            type="number"
            min="0"
            max="999"
            placeholder="28"
            {...field("age")}
          />
        </div>
        <div className="ch-form-field">
          <label htmlFor="ch-gender">
            Gender <span>optional</span>
          </label>
          <input
            id="ch-gender"
            maxLength={40}
            placeholder="e.g. Woman, nonbinary"
            {...field("gender")}
          />
        </div>
        <div className="ch-form-field ch-form-field--full">
          <label htmlFor="ch-story">Story</label>
          <select id="ch-story" {...field("story")}>
            <option value="">Select a story</option>
            {projects.map((project) => (
              <option key={project.id} value={project.id}>
                {project.title}
              </option>
            ))}
          </select>
        </div>
        <div className="ch-form-field ch-form-field--full">
          <label id="ch-traits-label">
            Personality traits <span>choose up to five</span>
          </label>
          <div
            className="ch-trait-options"
            role="group"
            aria-labelledby="ch-traits-label"
          >
            {PERSONALITY_TRAITS.map((trait) => (
              <button
                key={trait}
                type="button"
                className="ch-trait-option pill"
                aria-pressed={form.tags.includes(trait)}
                onClick={() => toggleTrait(trait)}
              >
                {trait}
              </button>
            ))}
          </div>
        </div>
        <div className="ch-form-field ch-form-field--full">
          <label htmlFor="ch-appearance">Appearance</label>
          <textarea
            id="ch-appearance"
            maxLength={600}
            placeholder="The details that make them recognizable…"
            {...field("appearance")}
          />
        </div>
        <div className="ch-form-field ch-form-field--full">
          <label htmlFor="ch-backstory">Backstory</label>
          <textarea
            id="ch-backstory"
            maxLength={1500}
            placeholder="What happened before the first page?"
            {...field("backstory")}
          />
        </div>
        <div className="ch-form-field ch-form-field--full">
          <label htmlFor="ch-upload">
            Portrait <span>optional, up to 1.5 MB</span>
          </label>
          <label className="ch-upload" htmlFor="ch-upload">
            {form.uploadedImage ? (
              <img
                className="ch-upload-preview"
                src={form.uploadedImage}
                alt="Portrait preview"
              />
            ) : (
              <ImagePlus size={18} />
            )}
            <span>
              {form.uploadedImage
                ? "Portrait ready · click to replace"
                : "Upload a portrait, or use the illustrated silhouette"}
            </span>
            <input
              id="ch-upload"
              type="file"
              accept="image/png,image/jpeg,image/webp"
              onChange={upload}
            />
          </label>
          {uploadError && (
            <span role="alert" style={{ fontSize: 11, color: "#bb4353" }}>
              {uploadError}
            </span>
          )}
        </div>
      </div>
      <div className="ch-form-actions">
        <button type="button" className="ch-cancel btn" onClick={onClose}>
          Cancel
        </button>
        <button type="submit" className="ch-submit btn btn-primary">
          Create character
        </button>
      </div>
    </form>
  );
}
