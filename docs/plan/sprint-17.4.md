# SPRINT 17.4 — STAGE 2.2: CAST + WORLD LANES

> **Trạng thái: DONE.** Branch `sprint-17.4-stage-22`, base `main` tại `995ba0f`.
> Kết quả: `cargo test` **216 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `cargo fmt` sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.
> Route dùng: backend `openai-codex / gpt-6-luna / high` (PARTIAL, orchestrator hoàn thiện);
> board 2.2 do orchestrator viết.

## KẾT QUẢ

| Hạng mục | Nội dung |
|---|---|
| Contract | `Lane::{CastSmith, WorldSmith, BondSmith}`: validate theo `ref` slug, enum appearance/competence/memory/knowledge, `world.kind` = setting/lore/power (mỗi kind có bộ field riêng), bond `states` là **mức** `NONE..ABSOLUTE`, `roles` theo 5 category + 3 hierarchy, chặn motif code ngoài catalogue |
| Prompt | `prompts/outline_cast.md`, `outline_world.md`, `outline_bonds.md` + builder `cast_user`/`world_user`/`bonds_user` |
| Planner | cast: `max(6, chapters/3)` nhân vật, 6/lô; world: 2 shard (settings top-down, lore+power) chạy song song; bonds: 1 shard, chỉ thêm khi `characters` đã có row |
| Apply | Commit đa bảng đúng `STATE_ORDER`: characters → appearances → marks → motifs → competences/domains → capabilities/limitations → memories → knowledge → world_settings (cha trước con) → lore → power → tiers → abilities → relationships → states → roles → relationship_motifs |
| Board 2.2 | `CastWorldBoard`: cột Nhân vật | Thế giới + mục Quan hệ, nút "Sinh thế giới", `OutlineItem.payload` cho field riêng của lane |
| Test | 5 test `test_stage_22_*`: chạy cast/world lưu proposal, commit refs nhân vật + quan hệ, memories `source_scene_id = null`, settings cha-trước-con, từ chối motif code lạ |

## QUYẾT ĐỊNH TRONG SPRINT

1. **Không có bảng `character_motifs` trong `super.json`** (chỉ có trong DBML). Motif của nhân vật được
   lưu vào field mới `characters.motifs` (JSON array) — thêm field, **vẫn 81 path / 34 bảng**.
   `relationship_motifs` vẫn dùng bảng thật.
2. **`relationship_states` là enum mức**, không phải số 0..100: contract, prompt, test và UI đổi sang
   `NONE|LOW|MODERATE|HIGH|ABSOLUTE`.
3. **Sửa `super_json/tables.rs`**: tham chiếu tự trỏ (`world_settings.parent_id_json_index`) phải được
   kiểm theo chính mảng đang ghi, không theo document cũ — nếu không thì không thể ghi cây địa điểm
   trong một lần.
4. **`competence_domains.description` không được rỗng** nên domain sinh tự động nhận
   `"Lĩnh vực <tên>"`.
5. **Lỗi fan-out được nêu nguyên nhân**: khi không có proposal hợp lệ, message kèm lỗi shard đầu tiên
   thay vì câu chung, để tác giả biết contract nào hỏng.
6. **Test dùng `LaneBackend`** trả lời theo system prompt thay vì theo thứ tự gọi, vì 3 shard của world
   chạy song song nên scripted queue sẽ flaky.

## NỢ KỸ THUẬT ĐÃ BIẾT

- `characters.motifs` là giải pháp tạm; nếu sau này port DBML thì chuyển sang bảng `character_motifs`.
- 2.3 (scene binder) chưa có; tab 2.3 vẫn khoá.
- Board 2.2 chưa có test frontend (repo không có runner); kiểm bằng `pnpm build`.

## NGOÀI PHẠM VI

Không có scene binder, không có văn bản truyện, không sửa ProjectDetail.
