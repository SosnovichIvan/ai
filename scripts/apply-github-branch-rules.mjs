const token = process.env.GITHUB_TOKEN ?? process.env.GH_TOKEN;
const repository = process.env.GITHUB_REPOSITORY ?? "SosnovichIvan/ai";

if (!token) throw new Error("Укажите GITHUB_TOKEN или GH_TOKEN с правом Administration: read and write для repository.");
if (!/^[^/]+\/[^/]+$/.test(repository)) throw new Error("GITHUB_REPOSITORY должен иметь формат owner/repository.");

const headers = {
  Accept: "application/vnd.github+json",
  Authorization: `Bearer ${token}`,
  "X-GitHub-Api-Version": "2022-11-28",
  "Content-Type": "application/json",
};

async function request(path, options = {}) {
  const response = await fetch(`https://api.github.com${path}`, { ...options, headers: { ...headers, ...options.headers } });
  if (response.status === 204) return null;
  const body = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(`GitHub API ${response.status}: ${body.message ?? "неизвестная ошибка"}`);
  return body;
}

const [owner, name] = repository.split("/");
const repo = await request(`/repos/${owner}/${name}`);
const mainRef = await request(`/repos/${owner}/${name}/git/ref/heads/main`);

try {
  await request(`/repos/${owner}/${name}/git/ref/heads/develop`);
} catch (error) {
  if (!error.message.startsWith("GitHub API 404:")) throw error;
  await request(`/repos/${owner}/${name}/git/refs`, { method: "POST", body: JSON.stringify({ ref: "refs/heads/develop", sha: mainRef.object.sha }) });
}

const payload = {
  name: "Require owner-approved pull requests",
  target: "branch",
  enforcement: "active",
  conditions: { ref_name: { include: ["refs/heads/main", "refs/heads/develop"], exclude: [] } },
  rules: [
    { type: "deletion" },
    { type: "non_fast_forward" },
    { type: "pull_request", parameters: { dismiss_stale_reviews_on_push: true, require_code_owner_review: true, require_last_push_approval: true, required_approving_review_count: 1, required_review_thread_resolution: true } },
  ],
  bypass_actors: [],
};

const rulesets = await request(`/repos/${owner}/${name}/rulesets`);
const existing = rulesets.find((ruleset) => ruleset.name === payload.name && ruleset.target === "branch");
if (existing) await request(`/repos/${owner}/${name}/rulesets/${existing.id}`, { method: "PUT", body: JSON.stringify(payload) });
else await request(`/repos/${owner}/${name}/rulesets`, { method: "POST", body: JSON.stringify(payload) });

console.info(`Ruleset применён к main и develop в ${repo.full_name}. Прямой и force push заблокированы; требуется PR и approval code owner.`);
