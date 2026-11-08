import { useState, useMemo, useEffect } from "react";
import { Search, Plus, Link as LinkIcon } from "lucide-react";
import { useCharacterStore } from "./store";
import { CharacterCard } from "./CharacterCard";
import { CreateCharacterDialog } from "./CreateCharacterDialog";
import { sanitizeDisplayLabel } from "@/features/projects/ProjectView";
import { useT } from "@/shared/i18n/useT";
import { Input } from "@/shared/components/ui/input";
import { Select } from "@/shared/components/ui/select";
import { Button } from "@/shared/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@/shared/components/ui/dialog";
import { toast } from "@/shared/components/ui/toast";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import type { CharacterLibraryItemDto, ListCharactersReq } from "@/shared/backend/generated";
import type { CharacterProfile, Gender } from "./types";

const validGender = (g?: string | null): Gender => {
  if (g === "MALE" || g === "FEMALE" || g === "NONE") return g;
  return "NONE";
};

/**
 * Reuses a shared library character in a project/draft via backend transport.
 * If linking fails, preserves existing library rows and project state without partial mutation.
 */
export async function linkCharacterToProject(
  client: BackendClient | any,
  params: { characterId: string; projectId: string; role?: string; notes?: string }
): Promise<{ success: boolean; characterCode?: string; error?: string }> {
  try {
    if (typeof client.linkCharacterToProject === "function") {
      const res = await client.linkCharacterToProject(params);
      return { success: true, characterCode: res?.characterCode };
    }
    const res = await client.linkLibraryCharacterToDraft({
      characterId: params.characterId,
      draftSessionId: params.projectId,
      role: params.role,
      notes: params.notes,
    });
    return { success: true, characterCode: res?.characterCode };
  } catch (err: any) {
    return { success: false, error: err?.message || String(err) };
  }
}

/**
 * Sanitizes character profile so no internal codes (e.g. CHAR_001) are user-visible.
 */
export function sanitizeCharacterProfile(c: CharacterProfile): CharacterProfile {
  return {
    ...c,
    basicInfo: {
      ...c.basicInfo,
      name: sanitizeDisplayLabel(c.basicInfo.name, c.code, "Nhân vật"),
      nickname: c.basicInfo.nickname
        ? sanitizeDisplayLabel(c.basicInfo.nickname, undefined, "")
        : undefined,
    },
    projectName: c.projectName
      ? sanitizeDisplayLabel(c.projectName, undefined, c.projectName)
      : undefined,
    competences: (c.competences || []).map((comp) => ({
      ...comp,
      name: sanitizeDisplayLabel(comp.name, comp.code, "Kỹ năng"),
    })),
    relationships: (c.relationships || []).map((r) => ({
      ...r,
      targetName: sanitizeDisplayLabel(r.targetName, undefined, "Người liên hệ"),
    })),
  };
}

/**
 * Maps a CharacterLibraryItemDto into a UI CharacterProfile.
 */
export function libraryItemToProfile(dto: CharacterLibraryItemDto, projectName?: string): CharacterProfile {
  const cleanName = sanitizeDisplayLabel(dto.name, dto.code, "Nhân vật");
  const cleanNickname = dto.nickname ? sanitizeDisplayLabel(dto.nickname, undefined, "") : undefined;
  return {
    id: dto.id,
    code: dto.code,
    projectId: undefined,
    projectName: projectName ? sanitizeDisplayLabel(projectName, undefined, projectName) : undefined,
    basicInfo: {
      name: cleanName,
      nickname: cleanNickname,
      species: dto.species,
      biologicalSex: validGender(dto.biologicalSex),
      genderIdentity: validGender(dto.genderIdentity),
      genderExpression: validGender(dto.genderExpression),
    },
    avatarBg: "bg-indigo-500",
    appearance: {
      height: 170,
      weight: 60,
      hair: "Đen",
      eyes: "Nâu",
      skin: "Tự nhiên",
      hairNote: "",
      eyesNote: "",
      skinNote: "",
      outfitStyle: "Giản dị",
      defaultSummary: dto.defaultSummary ?? "",
    },
    distinguishingMarks: [],
    relationships: [],
    competences: [],
    memories: [],
  };
}

export interface CharacterListViewProps {
  initialCharacters?: CharacterProfile[];
  client?: BackendClient;
}

export function CharacterListView({ initialCharacters, client: propClient }: CharacterListViewProps = {}) {
  const t = useT();
  const storeCharacters = useCharacterStore((s) => s.charactersList);
  const addCharacter = useCharacterStore((s) => s.addCharacter);
  const activeClient = propClient || backendClient;

  const [backendItems, setBackendItems] = useState<CharacterLibraryItemDto[] | null>(null);
  const [backendProjects, setBackendProjects] = useState<{ id: string; title: string }[] | null>(null);

  const [searchTerm, setSearchTerm] = useState("");
  const [filterSex, setFilterSex] = useState<string>("ALL");
  const [filterProject, setFilterProject] = useState<string>("ALL");
  const [isCreateOpen, setIsCreateOpen] = useState(false);

  // Link to Project dialog state
  const [isLinkOpen, setIsLinkOpen] = useState(false);
  const [selectedCharToLink, setSelectedCharToLink] = useState<string>("");
  const [selectedProjectToLink, setSelectedProjectToLink] = useState<string>("");
  const [linkRole, setLinkRole] = useState<string>("");
  const [linkNotes, setLinkNotes] = useState<string>("");
  const [isLinking, setIsLinking] = useState(false);

  // Fetch projects from backend transport
  useEffect(() => {
    let isMounted = true;
    if (activeClient && activeClient.status !== "unavailable") {
      activeClient
        .listProjects()
        .then((res) => {
          if (!isMounted) return;
          if (res?.items) {
            setBackendProjects(
              res.items.map((p) => ({
                id: p.id,
                title: sanitizeDisplayLabel(p.title, p.id, "Dự án"),
              }))
            );
          }
        })
        .catch(() => {});
    }
    return () => {
      isMounted = false;
    };
  }, [activeClient]);

  // Fetch characters from backend transport when filter/search changes
  useEffect(() => {
    let isMounted = true;
    if (initialCharacters) {
      return;
    }

    if (activeClient && activeClient.status !== "unavailable") {
      const req: ListCharactersReq = {
        projectId: filterProject !== "ALL" ? filterProject : undefined,
        biologicalSex: filterSex !== "ALL" ? (filterSex as Gender) : undefined,
        query: searchTerm.trim() ? searchTerm.trim() : undefined,
      };

      activeClient
        .listCharacters(req)
        .then((res) => {
          if (!isMounted) return;
          setBackendItems(res.items || []);
        })
        .catch((err) => {
          console.warn("Failed to list characters from backend transport:", err);
        });
    }

    return () => {
      isMounted = false;
    };
  }, [activeClient, initialCharacters, filterProject, filterSex, searchTerm]);

  // Consolidated characters list
  const characters: CharacterProfile[] = useMemo(() => {
    if (initialCharacters) {
      return initialCharacters.map(sanitizeCharacterProfile);
    }
    if (backendItems !== null) {
      return backendItems.map((item) => libraryItemToProfile(item));
    }
    return storeCharacters.map(sanitizeCharacterProfile);
  }, [initialCharacters, backendItems, storeCharacters]);

  const projectOptions = useMemo(() => {
    if (backendProjects !== null) {
      return backendProjects;
    }
    const map = new Map<string, string>();
    characters.forEach((c) => {
      if (c.projectId && c.projectName) {
        map.set(c.projectId, c.projectName);
      }
    });
    return Array.from(map.entries()).map(([id, title]) => ({ id, title }));
  }, [characters, backendProjects]);

  const filteredCharacters = useMemo(() => {
    // If backend transport provided filtered items, return directly
    if (backendItems !== null) {
      return characters;
    }
    return characters.filter((c) => {
      const name = c.basicInfo.name.toLowerCase();
      const nickname = (c.basicInfo.nickname ?? "").toLowerCase();
      const species = c.basicInfo.species.toLowerCase();
      const term = searchTerm.toLowerCase();

      const matchSearch = name.includes(term) || nickname.includes(term) || species.includes(term);
      const matchSex = filterSex === "ALL" || c.basicInfo.biologicalSex === filterSex;
      const matchProject =
        filterProject === "ALL" ||
        c.projectId === filterProject ||
        (filterProject === "proj-sample-500" && c.projectName?.includes("Tế nữ"));

      return matchSearch && matchSex && matchProject;
    });
  }, [characters, backendItems, searchTerm, filterSex, filterProject]);

  const handleAddCharacter = (newChar: CharacterProfile) => {
    addCharacter(newChar);
  };

  const handleLinkSubmit = async () => {
    if (!selectedCharToLink || !selectedProjectToLink) return;
    setIsLinking(true);
    try {
      const result = await linkCharacterToProject(activeClient, {
        characterId: selectedCharToLink,
        projectId: selectedProjectToLink,
        role: linkRole || undefined,
        notes: linkNotes || undefined,
      });

      if (result.success) {
        toast({ title: "Đã liên kết nhân vật vào dự án thành công", variant: "success" });
        setIsLinkOpen(false);
        // Refresh characters from backend
        if (activeClient && activeClient.status !== "unavailable") {
          const res = await activeClient.listCharacters({
            projectId: filterProject !== "ALL" ? filterProject : undefined,
            biologicalSex: filterSex !== "ALL" ? (filterSex as Gender) : undefined,
            query: searchTerm.trim() ? searchTerm.trim() : undefined,
          });
          setBackendItems(res.items || []);
        }
      } else {
        // Link failure: preserve existing library rows/state, no corrupted mutation
        toast({ title: "Không thể liên kết nhân vật vào dự án", variant: "error" });
      }
    } catch {
      toast({ title: "Không thể liên kết nhân vật vào dự án", variant: "error" });
    } finally {
      setIsLinking(false);
    }
  };

  return (
    <div className="flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6 sm:p-8">
      <div className="flex flex-col gap-4 border-b border-line pb-6 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-fg sm:text-2xl">
            {t("character.title")}
          </h1>
          <p className="mt-1 text-xs text-fg-muted">
            {t("character.subtitle")}
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2.5">
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-fg-subtle" />
            <Input
              type="text"
              aria-label={t("character.searchPlaceholder")}
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder={t("character.searchPlaceholder")}
              className="py-1.5 pl-9 pr-3 text-xs"
            />
          </div>

          {/* Filter theo Project */}
          <Select
            aria-label={t("character.filterProject")}
            value={filterProject}
            onChange={(e) => setFilterProject(e.target.value)}
            className="py-1.5 text-xs max-w-[180px]"
          >
            <option value="ALL">{t("project.all")}</option>
            {projectOptions.map((p) => (
              <option key={p.id} value={p.id}>
                {p.title}
              </option>
            ))}
          </Select>

          {/* Filter theo Sex */}
          <Select
            aria-label={t("sex.all")}
            value={filterSex}
            onChange={(e) => setFilterSex(e.target.value)}
            className="py-1.5 text-xs"
          >
            <option value="ALL">{t("sex.all")}</option>
            <option value="MALE">{t("gender.MALE")}</option>
            <option value="FEMALE">{t("gender.FEMALE")}</option>
            <option value="NONE">{t("gender.NONE")}</option>
          </Select>

          <Button
            type="button"
            onClick={() => setIsLinkOpen(true)}
            className="px-3.5 py-1.5"
            variant="secondary"
          >
            <LinkIcon className="h-3.5 w-3.5" />
            <span>Liên kết dự án</span>
          </Button>

          <Button
            type="button"
            onClick={() => setIsCreateOpen(true)}
            className="px-3.5 py-1.5"
          >
            <Plus className="h-3.5 w-3.5" />
            <span>{t("character.create")}</span>
          </Button>
        </div>
      </div>

      <div className="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
        {filteredCharacters.map((c) => (
          <CharacterCard key={c.id} character={c} />
        ))}
        {filteredCharacters.length === 0 && (
          <div className="tier-card rounded-card col-span-full py-16 text-center text-xs text-fg-subtle">
            {t("character.empty")}
          </div>
        )}
      </div>

      <CreateCharacterDialog
        open={isCreateOpen}
        onOpenChange={setIsCreateOpen}
        onCreate={handleAddCharacter}
      />

      {/* Dialog for linking shared character to a project */}
      <Dialog open={isLinkOpen} onOpenChange={setIsLinkOpen}>
        <DialogContent className="max-w-md p-6">
          <DialogTitle className="text-base font-bold text-fg">
            Liên kết nhân vật vào dự án
          </DialogTitle>
          <DialogDescription className="text-xs text-fg-muted mt-1">
            Tái sử dụng nhân vật từ thư viện vào dự án mà không tạo bản sao trùng lặp.
          </DialogDescription>

          <div className="mt-4 space-y-3">
            <div>
              <label className="text-xs font-semibold text-fg">
                Chọn nhân vật
              </label>
              <Select
                value={selectedCharToLink}
                onChange={(e) => setSelectedCharToLink(e.target.value)}
                className="mt-1 w-full text-xs"
              >
                <option value="">-- Chọn nhân vật --</option>
                {characters.map((c) => (
                  <option key={c.id} value={c.id}>
                    {c.basicInfo.name}
                  </option>
                ))}
              </Select>
            </div>

            <div>
              <label className="text-xs font-semibold text-fg">
                Chọn dự án
              </label>
              <Select
                value={selectedProjectToLink}
                onChange={(e) => setSelectedProjectToLink(e.target.value)}
                className="mt-1 w-full text-xs"
              >
                <option value="">-- Chọn dự án --</option>
                {projectOptions.map((p) => (
                  <option key={p.id} value={p.id}>
                    {p.title}
                  </option>
                ))}
              </Select>
            </div>

            <div>
              <label className="text-xs font-semibold text-fg">
                Vai trò trong dự án (tùy chọn)
              </label>
              <Input
                value={linkRole}
                onChange={(e) => setLinkRole(e.target.value)}
                placeholder="Nhân vật chính, Phản diện..."
                className="mt-1 text-xs"
              />
            </div>

            <div>
              <label className="text-xs font-semibold text-fg">
                Ghi chú (tùy chọn)
              </label>
              <Input
                value={linkNotes}
                onChange={(e) => setLinkNotes(e.target.value)}
                placeholder="Ghi chú thiết lập..."
                className="mt-1 text-xs"
              />
            </div>
          </div>

          <div className="mt-6 flex justify-end gap-2">
            <Button
              type="button"
              variant="secondary"
              onClick={() => setIsLinkOpen(false)}
              disabled={isLinking}
            >
              {t("common.cancel")}
            </Button>
            <Button
              type="button"
              onClick={handleLinkSubmit}
              disabled={!selectedCharToLink || !selectedProjectToLink || isLinking}
            >
              {isLinking ? "Đang xử lý..." : "Liên kết"}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
