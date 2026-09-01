import { readFile } from 'node:fs/promises';

const packageJson = JSON.parse(await readFile(new URL('../package.json', import.meta.url), 'utf8'));
const tauri = JSON.parse(await readFile(new URL('../src-tauri/tauri.conf.json', import.meta.url), 'utf8'));
const cargoToml = await readFile(new URL('../src-tauri/Cargo.toml', import.meta.url), 'utf8');
// Явный аргумент нужен для воспроизводимой локальной проверки и тестов.
// В GitHub Actions он также передаётся workflow, а переменная остаётся fallback.
const tag = process.argv[2] || process.env.GITHUB_REF_NAME;
if (!tag) throw new Error('Передайте SemVer tag (например v1.0.0)');
const version = tag.replace(/^v/, '');
if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(version)) throw new Error(`Недопустимая версия: ${tag}`);
const cargoVersion = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
if (packageJson.version !== version || tauri.version !== version || cargoVersion !== version) throw new Error(`Версия tag ${version} должна совпадать с package.json (${packageJson.version}), tauri.conf.json (${tauri.version}) и Cargo.toml (${cargoVersion ?? 'не найдена'})`);
