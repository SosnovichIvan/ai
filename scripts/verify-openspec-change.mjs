import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const requiredProposalSections = ['## Why', '## What Changes', '## Scope', '## Non-Goals', '## Version'];
const requiredVersionFields = ['**Previous:**', '**Target:**', '**Level:**', '**Rationale:**', '**Compatibility:**'];
const requiredDesignTerms = ['rollback', 'regression'];

export function validateChangeContents(proposal, design) {
  const errors = [];
  for (const section of requiredProposalSections) if (!proposal.includes(section)) errors.push(`В proposal отсутствует раздел ${section}.`);
  for (const field of requiredVersionFields) if (!proposal.includes(field)) errors.push(`В proposal отсутствует поле ${field}.`);
  const lowerDesign = design.toLowerCase();
  for (const term of requiredDesignTerms) if (!lowerDesign.includes(term)) errors.push(`В design отсутствует план ${term}.`);
  return errors;
}

async function main() {
  const index = process.argv.indexOf('--change');
  const change = index >= 0 ? process.argv[index + 1] : undefined;
  if (!change) throw new Error('Передайте --change openspec/changes/<change>.');
  const directory = resolve(change);
  const [proposal, design] = await Promise.all([
    readFile(resolve(directory, 'proposal.md'), 'utf8'),
    readFile(resolve(directory, 'design.md'), 'utf8'),
  ]);
  const errors = validateChangeContents(proposal, design);
  if (errors.length) throw new Error(`OpenSpec change не соответствует SDD-политике:\n${errors.join('\n')}`);
  process.stdout.write(`OpenSpec change проверен: ${change}\n`);
}

if (import.meta.url === `file://${process.argv[1]}`) main();
