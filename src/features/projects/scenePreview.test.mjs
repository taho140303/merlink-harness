import assert from 'node:assert/strict';
import { after, test } from 'node:test';
import { createServer } from 'vite';

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, hmr: false },
  appType: 'custom',
});
after(async () => { await vite.close(); });

const {
  buildSceneBlocks,
  buildSceneRegenerationRequest,
  isSceneDirty,
  normalizeNote,
  sourceForSceneAction,
} = await vite.ssrLoadModule('/src/features/projects/scenePreview.ts');

test('T09/T10: normalizes empty, whitespace-only, and non-empty notes', () => {
  assert.equal(normalizeNote(''), null);
  assert.equal(normalizeNote('   \n\t '), null);
  assert.equal(normalizeNote('  Giảm lời thoại  '), 'Giảm lời thoại');
  assert.equal(normalizeNote('Giữ nguyên diễn biến chính'), 'Giữ nguyên diễn biến chính');
  assert.deepEqual(buildSceneRegenerationRequest('scn-c-1-blk-1', '  hay hơn  '), {
    sceneId: 'scn-c-1-blk-1',
    note: 'hay hơn',
  });
  assert.deepEqual(buildSceneRegenerationRequest('scn-c-1-blk-2', '   '), {
    sceneId: 'scn-c-1-blk-2',
    note: null,
  });
});

test('T05: edit then revert is clean; source alone never creates dirty', () => {
  const baseline = { text: 'A', locked: false };
  assert.equal(isSceneDirty(baseline, baseline), false);
  assert.equal(isSceneDirty({ text: 'B', locked: false }, baseline), true);
  assert.equal(isSceneDirty({ text: 'A', locked: false }, baseline), false);
});

test('T06: lock change is dirty only when it differs from baseline', () => {
  const baseline = { text: 'A', locked: true };
  assert.equal(isSceneDirty({ text: 'A', locked: false }, baseline), true);
  assert.equal(isSceneDirty({ text: 'A', locked: true }, baseline), false);
  const unlocked = { text: 'A', locked: false };
  assert.equal(isSceneDirty({ text: 'A', locked: true }, unlocked), true);
  assert.equal(isSceneDirty({ text: 'A', locked: false }, unlocked), false);
});

test('content and lock changes are combined in dirty calculation', () => {
  const baseline = { text: 'A', locked: false };
  assert.equal(isSceneDirty({ text: 'B', locked: true }, baseline), true);
  assert.equal(isSceneDirty({ text: 'A', locked: true }, baseline), true);
  assert.equal(isSceneDirty({ text: 'B', locked: false }, baseline), true);
});

test('T07/T08: source follows the final scene action', () => {
  assert.equal(sourceForSceneAction('accept'), 'AI_GENERATED');
  assert.equal(sourceForSceneAction('edit'), 'HUMAN_EDITED');
  assert.equal(sourceForSceneAction('restore'), 'RESTORE');
  assert.equal(sourceForSceneAction('edit'), 'HUMAN_EDITED');
});

test('scene identity follows block id, not array position', () => {
  const chapter = {
    id: 'c-1',
    title: 'Chapter',
    date: '',
    blocks: [
      { id: 'blk-1', text: 'A', locked: false },
      { id: 'blk-2', text: 'B', locked: false },
    ],
  };
  assert.deepEqual(buildSceneBlocks(chapter).map((block) => block.sceneId), [
    'scn-c-1-blk-1',
    'scn-c-1-blk-2',
  ]);

  const reversed = { ...chapter, blocks: [...chapter.blocks].reverse() };
  assert.deepEqual(buildSceneBlocks(reversed).map((block) => block.sceneId), [
    'scn-c-1-blk-2',
    'scn-c-1-blk-1',
  ]);
});
