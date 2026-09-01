import { describe, expect, it } from 'vitest';
import { validateChangeContents } from './verify-openspec-change.mjs';

const proposal = `## Why\n\nПричина\n\n## What Changes\n\nИзменения\n\n## Scope\n\nScope\n\n## Non-Goals\n\nNon-goals\n\n## Version\n\n- **Previous:** v1.0.0\n- **Target:** v1.0.1\n- **Level:** PATCH\n- **Rationale:** Исправление\n- **Compatibility:** Без потери данных`;
const design = 'План rollback и regression проверок.';

describe('OpenSpec SDD policy', () => {
  it('принимает change с SemVer и compatibility планом', () => {
    expect(validateChangeContents(proposal, design)).toEqual([]);
  });

  it('отклоняет change без compatibility и regression', () => {
    expect(validateChangeContents(proposal.replace('**Compatibility:** Без потери данных', ''), 'Только rollback')).toEqual(expect.arrayContaining([
      expect.stringContaining('**Compatibility:**'),
      expect.stringContaining('regression'),
    ]));
  });
});
