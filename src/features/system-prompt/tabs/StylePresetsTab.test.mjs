import assert from 'node:assert/strict';
import path from 'node:path';
import { after, describe, it } from 'node:test';
import { createServer } from 'vite';

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: 'custom',
  resolve: { alias: { '@': path.resolve('src') } },
  ssr: { external: ['react', 'react-dom', 'react-router'] },
});
after(async () => vite.close());

const { sortRhetoricalDevices } = await vite.ssrLoadModule('/src/features/system-prompt/tabs/StylePresetsTab.tsx');

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
});
