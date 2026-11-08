import assert from 'node:assert/strict';
import { after, test } from 'node:test';
import { createServer } from 'vite';

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false },
  appType: 'custom',
});
after(async () => { await vite.close(); });

const {
  buildSceneBlocks,
  buildSceneBlocksFromReadModel,
  buildSceneRegenerationRequest,
  isSceneDirty,
  isSceneContentDirty,
  normalizeNote,
  sourceForSceneAction,
  canBulkChangeLocks,
  applyBulkLock,
  reviewGeneratedScene,
  restoreSceneVersion,
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

test('REQ-FE-005: isSceneContentDirty treats lock toggles as NOT content-dirty', () => {
  const baseline = { text: 'Nội dung ban đầu', locked: false };
  assert.equal(isSceneContentDirty(baseline, baseline), false);
  assert.equal(isSceneContentDirty({ text: 'Nội dung ban đầu', locked: true }, baseline), false);
  assert.equal(isSceneContentDirty({ text: 'Nội dung đã sửa', locked: false }, baseline), true);
  assert.equal(isSceneContentDirty({ text: 'Nội dung đã sửa', locked: true }, baseline), true);
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

test('buildSceneBlocksFromReadModel maps real UUIDv7 scenes.id directly', () => {
  const readModel = {
    projectId: 'proj-123',
    chapterId: 'chap-456',
    chapterTitle: 'Chương 1',
    position: 1,
    scenes: [
      {
        id: '0192e2b3-7649-7000-8000-000000000001',
        chapterId: 'chap-456',
        position: 1,
        displayName: 'Cảnh mở đầu',
        content: 'Bình minh trên đỉnh núi.',
        wordCount: 5,
        isLocked: false,
        headVersionNumber: 1,
        currentSceneSource: 'USER',
      },
      {
        id: '0192e2b3-7649-7000-8000-000000000002',
        chapterId: 'chap-456',
        position: 2,
        displayName: 'Cảnh chuyển tiếp',
        content: 'Bóng tối ập xuống thung lũng.',
        wordCount: 6,
        isLocked: true,
        headVersionNumber: 2,
        currentSceneSource: 'AI',
      },
    ],
  };

  const blocks = buildSceneBlocksFromReadModel(readModel);
  assert.equal(blocks.length, 2);
  assert.equal(blocks[0].sceneId, '0192e2b3-7649-7000-8000-000000000001');
  assert.equal(blocks[0].id, '0192e2b3-7649-7000-8000-000000000001');
  assert.equal(blocks[0].locked, false);
  assert.equal(blocks[0].headVersionNumber, 1);
  assert.equal(blocks[1].sceneId, '0192e2b3-7649-7000-8000-000000000002');
  assert.equal(blocks[1].locked, true);
  assert.equal(blocks[1].headVersionNumber, 2);
});

test('F03: bulk lock changes are blocked while generating or reviewing', () => {
  assert.equal(canBulkChangeLocks(null), true);
  assert.equal(canBulkChangeLocks({ status: 'generating' }), false);
  assert.equal(canBulkChangeLocks({ status: 'review' }), false);
});

test('F03: Unlock All during active generation/review leaves blocks untouched', () => {
  const blocks = [
    { sceneId: 's1', text: 'a edited', locked: true },
    { sceneId: 's2', text: 'b', locked: false },
  ];
  for (const status of ['generating', 'review']) {
    const out = applyBulkLock(blocks, false, { status });
    assert.equal(out, blocks);
    assert.deepEqual(out.map((b) => b.locked), [true, false]);
    assert.equal(out[0].text, 'a edited');
  }
});

test('F03: Unlock All when idle unlocks every scene and preserves content', () => {
  const blocks = [
    { sceneId: 's1', text: 'a edited', locked: true },
    { sceneId: 's2', text: 'b', locked: true },
  ];
  const out = applyBulkLock(blocks, false, null);
  assert.deepEqual(out.map((b) => b.locked), [false, false]);
  assert.deepEqual(out.map((b) => b.text), ['a edited', 'b']);
  assert.deepEqual(blocks.map((b) => b.locked), [true, true]);
  assert.equal(isSceneDirty(out[0], blocks[0]), true);
});

test('P5S5-P & P5S5-N: reviewGeneratedScene accepts valid candidate and rejects invalidated', () => {
  const scene = {
    id: 's1',
    sceneId: 's1',
    text: 'Old text',
    locked: false,
    headVersionNumber: 1,
  };
  const validCandidate = {
    requestId: 'req-1',
    generationId: 'gen-1',
    sceneId: 's1',
    baselineVersionNumber: 1,
    authorizationValid: true,
    status: 'review',
    text: 'AI generated content text',
  };

  // Valid candidate acceptance
  const accepted = reviewGeneratedScene({ action: 'accept', candidate: validCandidate, scene });
  assert.equal(accepted.accepted, true);
  assert.equal(accepted.updatedText, 'AI generated content text');
  assert.equal(accepted.source, 'AI_GENERATED');

  // Discard
  const discarded = reviewGeneratedScene({ action: 'discard', candidate: validCandidate, scene });
  assert.equal(discarded.accepted, false);

  // Invalidated candidate: authorizationValid === false
  const invalidAuth = { ...validCandidate, authorizationValid: false };
  assert.throws(
    () => reviewGeneratedScene({ action: 'accept', candidate: invalidAuth, scene }),
    /CONFLICT_INVALIDATED/
  );

  // Invalidated candidate: scene is locked
  assert.throws(
    () => reviewGeneratedScene({ action: 'accept', candidate: validCandidate, scene: { ...scene, locked: true } }),
    /CONFLICT_INVALIDATED/
  );

  // Invalidated candidate: version mismatch
  assert.throws(
    () => reviewGeneratedScene({ action: 'accept', candidate: validCandidate, scene: { ...scene, headVersionNumber: 2 } }),
    /CONFLICT_INVALIDATED/
  );
});

test('CI-08 / REQ-WRITE-004: restoreSceneVersion restores into dirty buffer and blocks locked scene', () => {
  const unlockedScene = {
    id: 's1',
    sceneId: 's1',
    text: 'Current text',
    locked: false,
    headVersionNumber: 2,
  };
  const version = {
    versionNumber: 1,
    content: 'Historical version 1 content',
  };

  const res = restoreSceneVersion(unlockedScene, version);
  assert.equal(res.success, true);
  assert.equal(res.newText, 'Historical version 1 content');
  assert.equal(res.source, 'RESTORE');

  const lockedScene = { ...unlockedScene, locked: true };
  assert.throws(
    () => restoreSceneVersion(lockedScene, version),
    /SCENE_LOCKED_RESTORE_BLOCKED/
  );
});
