import assert from 'node:assert/strict';
import path from 'node:path';
import { readFileSync } from 'node:fs';
import { after, describe, it } from 'node:test';
import { createServer } from 'vite';

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: 'custom',
  resolve: { alias: { '@': path.resolve('src') } },
});
after(async () => vite.close());

const { sortRhetoricalDevices, STYLE_PRESET_CATEGORY_KEYS } =
  await vite.ssrLoadModule('/src/features/system-prompt/stylePresetUtils.ts');
const { dict } = await vite.ssrLoadModule('/src/shared/i18n/dict.ts');

describe('Style preset detail', () => {
  it('orders rhetorical devices primary, secondary, then avoid', () => {
    const input = [
      { id: 'ban', usageType: 'avoid' },
      { id: 'second', usageType: 'secondary' },
      { id: 'primary', usageType: 'primary' },
    ];
    assert.deepEqual(sortRhetoricalDevices(input).map(({ id }) => id), ['primary', 'second', 'ban']);
    assert.deepEqual(input.map(({ id }) => id), ['ban', 'second', 'primary']);
  });

  it('provides Vietnamese and English labels for every seeded category', () => {
    const seed = readFileSync('src-tauri/migrations/0004_seed_style_presets.sql', 'utf8');
    const seededCategories = [...seed.matchAll(/\('[0-9a-f-]{36}', '[^']+', '([^']+)', '(?:VERY_LOW|LOW|MODERATE|FAST|VERY_FAST)'/g)]
      .map((match) => match[1]);
    assert.ok(seededCategories.length > 0);
    assert.deepEqual([...new Set(seededCategories)].sort(), Object.keys(STYLE_PRESET_CATEGORY_KEYS).sort());
    for (const [code, key] of Object.entries(STYLE_PRESET_CATEGORY_KEYS)) {
      assert.ok(dict.vi[key], `Vietnamese label missing for ${code}`);
      assert.ok(dict.en[key], `English label missing for ${code}`);
      assert.notEqual(dict.vi[key], code);
      assert.notEqual(dict.en[key], code);
    }
  });
});
