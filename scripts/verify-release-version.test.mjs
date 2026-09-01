import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { describe, expect, it } from 'vitest';

const run = promisify(execFile);
const root = new URL('..', import.meta.url);

describe('release version verification', () => {
  it('принимает baseline immutable tag', async () => {
    await expect(run('node', ['scripts/verify-release-version.mjs', 'v1.0.0'], { cwd: root.pathname })).resolves.toBeDefined();
  });

  it('отклоняет не-SemVer tag до публикации', async () => {
    await expect(run('node', ['scripts/verify-release-version.mjs', 'latest'], { cwd: root.pathname })).rejects.toMatchObject({ stderr: expect.stringContaining('Недопустимая версия') });
  });
});
