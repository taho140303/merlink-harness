import assert from 'node:assert/strict';
import { after, test } from 'node:test';
import { readFileSync } from 'node:fs';
import { createServer } from 'vite';

const vite = await createServer({ configFile: false, resolve: { alias: { '@': new URL('../../', import.meta.url).pathname } }, server: { middlewareMode: true, watch: null, hmr: false, ws: false }, appType: 'custom' });
after(async () => { await vite.close(); });

const n = await vite.ssrLoadModule('/src/features/draft/commitNavigation.ts');
const { MOCK_PROJECTS } = await vite.ssrLoadModule('/src/features/projects/mockProjects.ts');
const projects = (MOCK_PROJECTS.length ? MOCK_PROJECTS : [{ id: 'proj-1', title: 'Test Project' }]).map((p) => ({ id: p.id, title: p.title }));
const okId = projects[0].id;
const base = { pending: false, simulationActive: true, projects };
const go = (over) => n.resolveSimulatedNavigation({ ...base, ...over });

test('N01 failed commit does not navigate', () => {
  assert.equal(go({ result: { status: 'failed', reason: 'x' } }).ok, false);
});
test('N02 unknown / missing result does not navigate', () => {
  assert.equal(go({ result: { status: 'unknown' } }).ok, false);
  assert.equal(go({ result: null }).ok, false);
});
test('N03 pending does not navigate', () => {
  const d = go({ pending: true, result: { status: 'committed', projectId: okId } });
  assert.deepEqual(d, { ok: false, reason: 'pending' });
});
test('N04 committed without project id does not navigate', () => {
  assert.equal(go({ result: { status: 'committed', projectId: '' } }).reason, 'missingProjectId');
  assert.equal(go({ result: { status: 'committed' } }).reason, 'missingProjectId');
});
test('N05 invalid project id does not navigate and does not fall back', () => {
  const d = go({ result: { status: 'committed', projectId: 'does-not-exist' } });
  assert.deepEqual(d, { ok: false, reason: 'invalidProject' });
});
test('N06 committed with existing mock project is eligible', () => {
  const d = go({ result: { status: 'committed', projectId: okId } });
  assert.equal(d.ok, true);
  assert.equal(d.projectId, okId);
});
test('N07 simulation not activated does not navigate', () => {
  assert.equal(go({ simulationActive: false, result: { status: 'committed', projectId: okId } }).reason, 'notSimulated');
});
test('N08 duplicate navigation request runs one transition', () => {
  const once = n.createOnceGuard();
  let calls = 0;
  assert.equal(once(() => { calls++; }), true);
  assert.equal(once(() => { calls++; }), false);
  assert.equal(calls, 1);
});
test('N09 missing mock project list blocks navigation', () => {
  const d = go({ projects: [], result: { status: 'committed', projectId: okId } });
  assert.deepEqual(d, { ok: false, reason: 'invalidProject' });
});
test('N10 valid navigation targets the View Mode route', () => {
  const d = go({ result: { status: 'committed', projectId: okId } });
  assert.equal(d.path, `/project/${okId}`);
  assert.ok(!d.path.includes('mode='));
});
test('N11 creator chapter selection uses the existing chapter route', () => {
  assert.equal(n.buildCreatorChapterPath('proj-1', 'c-1'), '/project/proj-1/chapter/c-1?mode=creator');
});
test('N12 ProjectCreatorPanel navigates through the tested creator route builder', () => {
  const src = readFileSync(new URL('../projects/ProjectCreatorPanel.tsx', import.meta.url), 'utf8');
  assert.match(src, /import \{ buildCreatorChapterPath \} from "@\/features\/draft\/commitNavigation"/);
  assert.match(src, /navigate\(buildCreatorChapterPath\(project\.id, chapter\.id\)\)/);
  assert.ok(!src.includes('?mode=creator'), 'no inline creator route template');
});
test('N13 commit dialog hides the close affordance and blocks dismissal while pending', () => {
  const src = readFileSync(new URL('./CommitConfirmationDialog.tsx', import.meta.url), 'utf8');
  assert.match(src, /pending && "\[&>button:last-child\]:hidden"/);
  assert.match(src, /onEscapeKeyDown=\{\(e\) => \{ if \(pending\) e\.preventDefault\(\); \}\}/);
  assert.match(src, /onInteractOutside=\{\(e\) => \{ if \(pending\) e\.preventDefault\(\); \}\}/);
  assert.match(src, /disabled=\{pending\} onClick=\{\(\) => handleOpenChange\(false\)\}/);
});
