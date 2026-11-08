import assert from 'node:assert/strict';
import { after, test } from 'node:test';
import path from 'node:path';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { createServer } from 'vite';

// Vite transforms the project's TSX and @ alias; no DOM or test-runner dependency needed.
const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: 'custom',
  resolve: { alias: { '@': path.resolve('src') } },
});
after(async () => { await vite.close(); });
const { SubagentOrchestrationTab } = await vite.ssrLoadModule('/src/features/settings/SubagentOrchestrationTab.tsx');
const { SettingsSidebar } = await vite.ssrLoadModule('/src/features/settings/SettingsSidebar.tsx');
const { DEFAULT_SUBAGENT_CONFIG, toDraft, validateSubagentDraft } =
  await vite.ssrLoadModule('/src/features/settings/subagentConfig.ts');
const { dict } = await vite.ssrLoadModule('/src/shared/i18n/dict.ts');

function render(draft = toDraft(DEFAULT_SUBAGENT_CONFIG)) {
  return renderToStaticMarkup(createElement(SubagentOrchestrationTab, {
    draft, saved: DEFAULT_SUBAGENT_CONFIG, onDraftChange: () => {}, onSave: () => {},
  }));
}

test('renders localized settings, all numeric controls, and disabled unchanged Save', () => {
  const html = render();
  assert.match(html, /Điều phối subagent/);
  assert.match(html, /Theo model của cuộc chat/);
  assert.match(html, /ví dụ cục bộ/);
  assert.equal((html.match(/type="number"/g) ?? []).length, 9);
  assert.match(html, /aria-label="Giảm Số subagent tối đa mỗi vòng"/);
  assert.match(html, /<button[^>]*disabled=""[^>]*>Lưu<\/button>/);
  assert.equal(dict.en['settings.subagent.title'], 'Subagent orchestration');
  assert.equal(dict.en['settings.subagent.useChatModel'], 'Use chat model');
});

test('settings sidebar renders orchestration between providers and recycle bin with active state', () => {
  const html = renderToStaticMarkup(createElement(SettingsSidebar, {
    activeTab: 'subagent-orchestration', onSelectTab: () => {},
  }));
  const providers = html.indexOf(dict.vi['settings.aiProviders']);
  const orchestration = html.indexOf(dict.vi['settings.subagent']);
  const recycleBin = html.indexOf(dict.vi['settings.recycleBin']);
  assert.ok(providers >= 0 && providers < orchestration && orchestration < recycleBin);
  assert.match(html, /<button[^>]*aria-current="page"[^>]*>.*?Điều phối subagent<\/span><\/button>/);
  assert.equal((html.match(/aria-current="page"/g) ?? []).length, 1);
});

test('lookback must be strictly below capacity; invalid row links error and disables Save', () => {
  const valid = toDraft(DEFAULT_SUBAGENT_CONFIG);
  assert.deepEqual(validateSubagentDraft(valid), {});
  assert.deepEqual(validateSubagentDraft({ ...valid, outlineCapacity: '10', outlineLookback: '9' }), {});
  assert.equal(validateSubagentDraft({ ...valid, outlineCapacity: '10', outlineLookback: '10' }).outlineLookback, 'lookback');
  assert.equal(validateSubagentDraft({ ...valid, outlineCapacity: '10', outlineLookback: '11' }).outlineLookback, 'lookback');
  const html = render({ ...valid, outlineLookback: '50' });
  assert.match(html, /Số chương nhìn lại phải nhỏ hơn sức chứa dàn ý/);
  assert.match(html, /aria-invalid="true" aria-describedby="subagent-outlineLookback-description subagent-outlineLookback-error"/);
  assert.match(html, /<div role="group" aria-label="Số chương nhìn lại"[^>]*>.*?<\/div><\/div><p id="subagent-outlineLookback-error"/);
  assert.match(html, /<button[^>]*disabled=""[^>]*>Lưu<\/button>/);
});

test('rejects missing, fractional, negative, out-of-range and unsafe inputs', () => {
  const draft = toDraft(DEFAULT_SUBAGENT_CONFIG);
  assert.equal(validateSubagentDraft({ ...draft, outlineLookback: '' }).outlineLookback, 'required');
  assert.equal(validateSubagentDraft({ ...draft, outlineLookback: '2.5' }).outlineLookback, 'integer');
  assert.equal(validateSubagentDraft({ ...draft, contextMargin: '-1' }).contextMargin, 'minimum');
  assert.equal(validateSubagentDraft({ ...draft, maxSubagents: '7' }).maxSubagents, 'maximum');
  assert.equal(validateSubagentDraft({ ...draft, maxSubagents: '0' }).maxSubagents, 'minimum');
  assert.equal(validateSubagentDraft({ ...draft, outlineCapacity: '999999999999999999999' }).outlineCapacity, 'integer');
});
