import assert from 'node:assert/strict';
import { after, test } from 'node:test';
import path from 'node:path';
import { createServer } from 'vite';

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: 'custom',
  resolve: { alias: { '@': path.resolve('src') } },
});
after(async () => { await vite.close(); });
const { mapBusinessFlowDataToDraftOverview } = await vite.ssrLoadModule('/src/features/draft/DraftWorkspace.tsx');
const { DRAFT_OVERVIEW } = await vite.ssrLoadModule('/src/features/draft/mockOverview.ts');

test('phase 3 world list and phase 4 scene config are mapped for rendering', () => {
  const bfd = {
    require: {
      project: { display_name: { value: 'Truyện' }, genres: [] },
      character: [{ code: 'hero', display_name: { value: 'Anh hùng' }, biological_sex: { value: 'MALE' } }],
    },
    optional: {
      overview_phase: {
        arcs: [{
          code: 'arc-1',
          position: { value: 1 },
          display_name: { value: 'Arc Một' },
          summary: { value: 'Tóm tắt' },
          motif: { code: { value: 'm' }, event_blocks: [] },
          themes: {},
        }],
      },
      chapter_phase: {
        arcs: [{
          code: 'arc-1',
          chapters: [{
            code: 'ch-1',
            position: { value: 1 },
            display_name: { value: 'Chương 1' },
            summary: { value: 'Tóm tắt chương' },
            scenes: [{
              code: 'sc-1',
              position: { value: 1 },
              display_name: { value: 'Cảnh 1' },
              summary: { value: 'Tóm tắt cảnh' },
            }],
          }],
        }],
      },
      character_psychological_phase: {
        character: [{
          code: 'hero',
          core_trauma_and_belief: { the_wound: { value: 'Bị bỏ rơi' } },
          cognitive_moral_framework: {
            moral_alignment: { value: 'LAWFUL' },
            moral_code: { value: 'An toàn tài chính' },
            breaking_point: { value: 'Khi niềm tin sụp đổ' },
          },
        }],
      },
      world_building_phase: {
        world_settings: [{
          code: 'world-1',
          display_name: { value: 'Thế giới 1' },
          setting_type: { value: 'WORLD' },
          description: { value: 'Mô tả' },
          lore_entries: [{ code: 'lore-1', display_name: { value: 'Lore 1' }, type: { value: 'HISTORY' } }],
        }, {
          code: 'world-2',
          display_name: { value: 'Thế giới 2' },
          setting_type: { value: 'CITY' },
          lore_entries: [],
        }],
      },
      scene_config_phase: {
        arcs: [{
          code: 'arc-1',
          chapters: [{
            code: 'ch-1',
            scenes: [{
              code: 'sc-1',
              world_setting_code: { value: 'world-1' },
              style_preset_code: { value: 'style-1' },
              narrator_config_code: { value: 'narrator-1' },
              characters: [{ character_code: 'hero', is_pov: { value: true } }],
              lore_entries: [{ lore_entry_code: 'lore-1', reveal_to_reader: { value: 'true' } }],
            }],
          }],
        }],
      },
    },
  };
  const overview = mapBusinessFlowDataToDraftOverview(bfd, DRAFT_OVERVIEW);
  assert.equal(overview.characters[0].psychological.moralFramework.code, 'An toàn tài chính');
  assert.equal(overview.characters[0].psychological.moralFramework.breakingPoint, 'Khi niềm tin sụp đổ');
  assert.equal(overview.worldSettingsList?.length, 2);
  assert.equal(overview.worldSettingsList[0].loreEntries.length, 1);
  assert.equal(overview.sceneConfig?.arcs.length, 1);
  const scene = overview.sceneConfig.arcs[0].chapters[0].scenes[0];
  assert.equal(scene.worldSettingCode, 'world-1');
  assert.equal(scene.stylePresetCode, 'style-1');
  assert.equal(scene.narratorConfigCode, 'narrator-1');
  assert.equal(scene.characters[0].isPov, true);
  assert.equal(scene.loreEntries[0].revealToReader, 'FULL');
});
