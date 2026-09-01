import { createHash } from 'node:crypto';
import { cp, mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { basename, extname, join } from 'node:path';

const args = Object.fromEntries(process.argv.slice(2).map((value, index, all) => value.startsWith('--') ? [value.slice(2), all[index + 1]] : null).filter(Boolean));
const required = ['platform', 'arch', 'version', 'input', 'output'];
const missing = required.filter((key) => !args[key]);
if (missing.length) throw new Error(`Не заданы параметры: ${missing.join(', ')}`);
const version = args.version.replace(/^v/, '');

const extensions = {
  macos: ['.dmg'],
  windows: ['.msi', '.exe'],
  linux: ['.AppImage', '.deb'],
};
const files = await readdir(args.input, { recursive: true });
const candidate = files
  .filter((file) => extensions[args.platform]?.includes(extname(file)))
  .sort((left, right) => extensions[args.platform].indexOf(extname(left)) - extensions[args.platform].indexOf(extname(right)))[0];
if (!candidate) throw new Error(`Не найден release-артефакт для ${args.platform} в ${args.input}`);

const source = join(args.input, candidate);
const extension = extname(candidate);
const filename = `ai_rules_version_${version}_${args.platform}-${args.arch}${extension}`;
await mkdir(args.output, { recursive: true });
await cp(source, join(args.output, filename));
const sha256 = createHash('sha256').update(await readFile(join(args.output, filename))).digest('hex');
const metadata = {
  product: 'Agent Foundry',
  technicalId: 'ai_rules_version',
  version,
  artifact: { platform: args.platform, arch: args.arch, filename, sha256, source: basename(source) },
};
await writeFile(join(args.output, `release-metadata-${args.platform}.json`), `${JSON.stringify(metadata, null, 2)}\n`);
