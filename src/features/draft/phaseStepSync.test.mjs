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

const { deriveStepFromRuntimePhase, DraftBackendController } = await vite.ssrLoadModule(
  '/src/features/draft/DraftWorkspace.tsx',
);

test('runtime phase maps to the tab that should stay open across remounts', () => {
  assert.deepEqual(deriveStepFromRuntimePhase(2, null, false), { step: 'chapter', subStep: 'appearance' });
  // A completed phase shows the next step, so reopening does not fall back to tab 1.
  assert.deepEqual(deriveStepFromRuntimePhase(1, null, true), { step: 'chapter', subStep: 'appearance' });
  assert.deepEqual(deriveStepFromRuntimePhase(5, 2, false), {
    step: 'character',
    subStep: 'psychological',
  });
  assert.deepEqual(deriveStepFromRuntimePhase(5, 3, true), {
    step: 'relationship',
    subStep: 'lifeExperience',
  });
  assert.deepEqual(deriveStepFromRuntimePhase(0, null, false), {
    step: 'overview',
    subStep: 'appearance',
  });
});

test('version label follows the sealed head instead of staying at v.01', async () => {
  const controller = new DraftBackendController({
    listDraftVersions: async () => ({
      headVersionSeq: 10,
      versions: [
        { seq: 10, displayLabel: 'v.10' },
        { seq: 9, displayLabel: 'v.09' },
      ],
    }),
    getDraftWorkspaceState: async () => ({
      draftSessionId: 'draft-1',
      status: 'editing',
      phase: 2,
      subphase: null,
      headVersionSeq: 10,
      businessFlowData: {},
      progress: { canAdvancePhase: false },
      pendingPatches: [],
    }),
  });

  await controller.loadVersions();
  assert.equal(controller.getState().currentVersionLabel, 'v.10');

  await controller.loadWorkspaceState();
  assert.equal(controller.getState().phase, 2);
  assert.equal(controller.getState().headVersionSeq, 10);
  assert.equal(controller.getState().currentVersionLabel, 'v.10');
});
