import { readdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const args = Object.fromEntries(process.argv.slice(2).map((value, index, all) => value.startsWith('--') ? [value.slice(2), all[index + 1]] : null).filter(Boolean));
if (!args.input || !args.output || !args.version || !args.baseUrl) throw new Error('Нужны --input, --output, --version и --baseUrl');
const version = args.version.replace(/^v/, '');
const entries = await Promise.all((await readdir(args.input)).filter((file) => file.startsWith('release-metadata-')).map(async (file) => JSON.parse(await readFile(join(args.input, file), 'utf8'))));
const artifacts = Object.fromEntries(entries.map(({ artifact }) => [artifact.platform, {
  label: `Скачать ${artifact.filename.split('.').pop()}`,
  url: `${args.baseUrl}/${artifact.filename}`,
  platform: `${artifact.platform} · ${artifact.arch}`,
  sha256: artifact.sha256,
  signature: 'Не подписано',
}]));
const payload = { product: 'Agent Foundry', technicalId: 'ai_rules_version', channels: { stable: [{ version, status: 'latest', artifacts }] } };
const sums = entries.map(({ artifact }) => `${artifact.sha256}  ${artifact.filename}`).sort().join('\n');
await writeFile(join(args.output, 'releases.json'), `${JSON.stringify(payload, null, 2)}\n`);
await writeFile(join(args.output, 'SHA-256SUMS'), `${sums}\n`);
