import assert from 'node:assert/strict';
import { after, test } from 'node:test';
import { createServer } from 'vite';

const vite = await createServer({ configFile: false, server: { middlewareMode: true, hmr: false, ws: false }, appType: 'custom' });
after(async () => { await vite.close(); });

const m = await vite.ssrLoadModule('/src/features/draft/commitPreview.ts');
const { DRAFT_OVERVIEW } = await vite.ssrLoadModule('/src/features/draft/mockOverview.ts');

const ready = m.deriveValidation({ validation: { ready: true } });
const zero = m.derivePendingPatches({ pendingPatches: { count: 0 } });

test('C01 missing validation is unknown', () => {
  assert.equal(m.deriveValidation(undefined).status, 'unknown');
  assert.equal(m.deriveValidation({}).status, 'unknown');
  assert.equal(m.deriveValidation({ validation: {} }).status, 'unknown');
});
test('C02 missing patch data is unknown', () => {
  assert.deepEqual(m.derivePendingPatches(undefined), { status: 'unknown' });
  assert.deepEqual(m.derivePendingPatches({ pendingPatches: null }), { status: 'unknown' });
  assert.deepEqual(m.derivePendingPatches({ pendingPatches: { count: -1 } }), { status: 'unknown' });
});
test('C03 known zero patches eligible only if validation ready', () => {
  assert.deepEqual(zero, { status: 'known', count: 0 });
  assert.equal(m.isCommitEligible(ready, zero, false), true);
  assert.equal(m.isCommitEligible(ready, zero, true), false);
});
test('C04 known pending patches block commit', () => {
  const p = m.derivePendingPatches({ pendingPatches: { count: 2 } });
  assert.equal(m.isCommitEligible(ready, p, false), false);
  assert.ok(m.getCommitBlockReasons(ready, p, false).includes('patchesPending'));
  assert.ok(m.getCommitBlockReasons(ready, { status: 'unknown' }, false).includes('patchesUnknown'));
});
test('C05 blocked validation blocks commit', () => {
  const v = m.deriveValidation({ validation: { ready: false, blockingReasons: ['x'] } });
  assert.equal(v.status, 'blocked');
  assert.equal(m.isCommitEligible(v, zero, false), false);
});
test('C06 unknown validation blocks commit', () => {
  assert.equal(m.isCommitEligible(m.deriveValidation(undefined), zero, false), false);
});
test('C07 plan summary derives from draft fixture', () => {
  const s = m.summarizePlan(DRAFT_OVERVIEW);
  assert.equal(s.projectName, DRAFT_OVERVIEW.displayName);
  assert.equal(s.arcs, DRAFT_OVERVIEW.arcs.length);
  assert.equal(s.characters, DRAFT_OVERVIEW.characters.length);
  assert.equal(s.relationships, DRAFT_OVERVIEW.relationships.length);
  assert.equal(s.worldbuildingAvailable, true);
});
test('C08 missing summary fields are unavailable, not fabricated', () => {
  const s = m.summarizePlan({ displayName: ' ', summary: '', arcs: [], characters: [] });
  assert.equal(s.projectName, null);
  assert.equal(s.description, null);
  assert.equal(s.chapters, null);
  assert.equal(s.scenes, null);
  assert.equal(s.relationships, null);
  assert.equal(s.worldbuildingAvailable, false);
});
test('C09 duplicate submit yields one operation', () => {
  let s = m.reduceCommitOp({ status: 'idle' }, 'submit');
  const again = m.reduceCommitOp(s, 'submit');
  assert.equal(again, s);
  assert.equal(again.status, 'pending');
});
test('C10 pending dismissal is prevented', () => {
  assert.equal(m.canDismissCommitDialog({ status: 'pending' }), false);
  assert.equal(m.canDismissCommitDialog({ status: 'idle' }), true);
  assert.equal(m.canDismissCommitDialog({ status: 'error' }), true);
  assert.equal(m.reduceCommitOp({ status: 'pending' }, 'reset').status, 'pending');
});
test('C11 error preserves draft and permits retry', () => {
  const before = structuredClone(DRAFT_OVERVIEW);
  let s = m.reduceCommitOp({ status: 'idle' }, 'submit');
  s = m.reduceCommitOp(s, 'fail');
  assert.equal(s.status, 'error');
  assert.deepEqual(DRAFT_OVERVIEW, before);
  assert.equal(m.reduceCommitOp(s, 'submit').status, 'pending');
});
test('C12 success preview is a distinct non-persisted state', () => {
  let s = m.reduceCommitOp({ status: 'idle' }, 'submit');
  s = m.reduceCommitOp(s, 'succeed');
  assert.equal(s.status, 'success-preview');
  assert.equal(m.reduceCommitOp(s, 'submit').status, 'success-preview');
  assert.deepEqual(Object.keys(s), ['status']);
});

test('unknown result is dismissable, retryable and not success', () => {
  let s = m.reduceCommitOp({ status: 'idle' }, 'submit');
  s = m.reduceCommitOp(s, 'unknown');
  assert.equal(s.status, 'unknown');
  assert.equal(m.canDismissCommitDialog(s), true);
  assert.equal(m.reduceCommitOp(s, 'submit').status, 'pending');
});
