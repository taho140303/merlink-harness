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
const { deriveStepFromFlow } = await vite.ssrLoadModule('/src/features/draft/DraftWorkspace.tsx');

test('empty draft starts at the overview step', () => {
  assert.deepEqual(deriveStepFromFlow(null), { step: 'overview', subStep: 'appearance' });
  assert.deepEqual(deriveStepFromFlow({}), { step: 'overview', subStep: 'appearance' });
});

test('reopening resumes at the deepest phase present in the newest JSON', () => {
  assert.equal(deriveStepFromFlow({ optional: { overview_phase: { arcs: [{ code: 'a' }] } } }).step, 'overview');
  assert.equal(deriveStepFromFlow({
    optional: { chapter_phase: { arcs: [{ code: 'a', chapters: [{ code: 'c1' }] }] } },
  }).step, 'chapter');
  assert.equal(deriveStepFromFlow({
    optional: { world_building_phase: { world_settings: [{ code: 'ws' }] } },
  }).step, 'world');
  assert.equal(deriveStepFromFlow({
    optional: { scene_config_phase: { arcs: [{ code: 'a', chapters: [{ scenes: [{ code: 's1' }] }] }] } },
  }).step, 'scene');
});

test('character sub-phase and relationship phase are restored', () => {
  assert.deepEqual(
    deriveStepFromFlow({ optional: { character_appearance_phase: { character: [{ code: 'c' }] } } }),
    { step: 'character', subStep: 'appearance' },
  );
  assert.deepEqual(
    deriveStepFromFlow({ optional: { character_psychological_phase: { character: [{ code: 'c' }] } } }),
    { step: 'character', subStep: 'psychological' },
  );
  assert.deepEqual(
    deriveStepFromFlow({ optional: { character_life_experience_phase: { character: [{ code: 'c' }] } } }),
    { step: 'character', subStep: 'lifeExperience' },
  );
  assert.equal(
    deriveStepFromFlow({ optional: { relationship_phase: { relationship: [{ code: 'r' }] } } }).step,
    'relationship',
  );
});

test('later phases win over earlier ones when several exist', () => {
  const flow = {
    optional: {
      overview_phase: { arcs: [{ code: 'a' }] },
      chapter_phase: { arcs: [{ code: 'a', chapters: [{ code: 'c1' }] }] },
      world_building_phase: { world_settings: [{ code: 'ws' }] },
      scene_config_phase: { arcs: [{ chapters: [{ scenes: [{ code: 's1' }] }] }] },
      character_appearance_phase: { character: [{ code: 'c' }] },
    },
  };
  assert.deepEqual(deriveStepFromFlow(flow), { step: 'character', subStep: 'appearance' });
});
