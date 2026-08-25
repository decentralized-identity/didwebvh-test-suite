import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { load as yamlLoad } from 'js-yaml';
import { canonicalize } from 'json-canonicalize';
import { getResolver } from '@thisdid/webvh-did-resolver';

const REPO_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../..');
const IMPL_DIR = path.join(REPO_ROOT, 'implementations/thisdid');
const VECTORS_DIR = path.join(REPO_ROOT, 'vectors');

// ---------------------------------------------------------------------------
// thisdid is a resolver-only wrapper around didwebvh-ts's resolveDID(): it has
// no local-log entry point, so every resolution here goes through the real
// `did:webvh` network path with `fetch` intercepted to serve the committed
// vector files instead of hitting the network — the same technique the
// wrapper's own test suite uses (src/__tests__/resolver.test.ts).
// ---------------------------------------------------------------------------

function stubFetch(log: string, witness?: string): () => void {
  const original = globalThis.fetch;
  globalThis.fetch = (async (url: unknown) => {
    const u = String(url);
    if (u.endsWith('did-witness.json')) {
      return witness !== undefined
        ? new Response(witness, { status: 200 })
        : new Response('not found', { status: 404 });
    }
    if (u.endsWith('did.jsonl')) {
      return new Response(log, { status: 200 });
    }
    return new Response('not found', { status: 404 });
  }) as typeof fetch;
  return () => { globalThis.fetch = original; };
}

function stubFetchTracking(): { restore: () => void; attempted: boolean; url: string } {
  const original = globalThis.fetch;
  const state = { restore: () => { globalThis.fetch = original; }, attempted: false, url: '' };
  globalThis.fetch = (async (url: unknown) => {
    state.attempted = true;
    state.url = String(url);
    throw new Error('fetch intercepted by test harness');
  }) as typeof fetch;
  return state;
}

async function resolveViaThisdid(did: string, resolutionOptions: Record<string, unknown> = {}): Promise<any> {
  const registry = getResolver() as unknown as Record<string, (...args: any[]) => Promise<any>>;
  return registry.webvh(did, null, null, resolutionOptions);
}

interface ParsedLog { lines: string[]; entries: any[]; did: string }

function parseLog(logPath: string): ParsedLog | null {
  const content = fs.readFileSync(logPath, 'utf8').trim();
  if (!content) return null;
  const lines = content.split('\n').filter(l => l.trim());
  const entries = lines.map(l => JSON.parse(l));
  const did = entries[entries.length - 1]?.state?.id;
  if (!did) throw new Error('could not determine DID from did.jsonl (missing state.id)');
  return { lines, entries, did };
}

function readWitness(implDir: string): string | undefined {
  const p = path.join(implDir, 'did-witness.json');
  return fs.existsSync(p) ? fs.readFileSync(p, 'utf8') : undefined;
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

function readConfig(): { version: string } {
  try {
    const cfg = yamlLoad(fs.readFileSync(path.join(IMPL_DIR, 'config.yaml'), 'utf8')) as any;
    return { version: cfg.version ?? '' };
  } catch {
    return { version: '' };
  }
}

function readWrappedCoreVersion(): string {
  // node_modules lives at REPO_ROOT/node_modules both locally (npm install run
  // from implementations/thisdid/ with no parent package.json in the way once
  // this dir has its own) and in Docker (installed at /workspace so it
  // survives the implementations/thisdid runtime mount) — try both.
  for (const base of [REPO_ROOT, IMPL_DIR]) {
    try {
      const pkg = JSON.parse(fs.readFileSync(
        path.join(base, 'node_modules/@thisdid/webvh-did-resolver/package.json'), 'utf8'
      ));
      if (pkg.dependencies?.['didwebvh-ts']) return pkg.dependencies['didwebvh-ts'];
    } catch { /* try next */ }
  }
  return '';
}

// ---------------------------------------------------------------------------
// Cross-resolution
// ---------------------------------------------------------------------------

function extractVersionNumber(filename: string): number | null {
  const parts = path.basename(filename, '.json').split('.');
  if (parts.length > 1) {
    const n = parseInt(parts[1]);
    if (!isNaN(n)) return n;
  }
  return null;
}

function computeUnifiedDiff(expected: unknown, actual: unknown): string {
  const expLines = JSON.stringify(expected, null, 2).split('\n');
  const actLines = JSON.stringify(actual, null, 2).split('\n');
  const m = expLines.length, n = actLines.length;

  const dp: number[][] = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));
  for (let i = m - 1; i >= 0; i--) {
    for (let j = n - 1; j >= 0; j--) {
      dp[i][j] = expLines[i] === actLines[j]
        ? dp[i + 1][j + 1] + 1
        : Math.max(dp[i + 1][j], dp[i][j + 1]);
    }
  }

  type Edit = { t: ' ' | '-' | '+'; l: string };
  const edits: Edit[] = [];
  let i = 0, j = 0;
  while (i < m || j < n) {
    if (i < m && j < n && expLines[i] === actLines[j]) {
      edits.push({ t: ' ', l: expLines[i++] }); j++;
    } else if (j < n && (i >= m || dp[i][j + 1] >= dp[i + 1][j])) {
      edits.push({ t: '+', l: actLines[j++] });
    } else {
      edits.push({ t: '-', l: expLines[i++] });
    }
  }

  const CONTEXT = 3;
  const changeIdxs = edits.map((e, k) => e.t !== ' ' ? k : -1).filter(k => k >= 0);
  if (changeIdxs.length === 0) return '';

  const hunks: Array<[number, number]> = [];
  let hs = -1, he = -1;
  for (const idx of changeIdxs) {
    const s = Math.max(0, idx - CONTEXT);
    const e = Math.min(edits.length - 1, idx + CONTEXT);
    if (hs < 0) { hs = s; he = e; }
    else if (s <= he + 1) { he = Math.max(he, e); }
    else { hunks.push([hs, he]); hs = s; he = e; }
  }
  if (hs >= 0) hunks.push([hs, he]);

  const out: string[] = ['--- expected', '+++ actual (thisdid resolver)'];
  for (const [start, end] of hunks) {
    const slice = edits.slice(start, end + 1);
    const beforeOld = edits.slice(0, start).filter(e => e.t !== '+').length;
    const beforeNew = edits.slice(0, start).filter(e => e.t !== '-').length;
    out.push(`@@ -${beforeOld + 1},${slice.filter(e => e.t !== '+').length} +${beforeNew + 1},${slice.filter(e => e.t !== '-').length} @@`);
    for (const e of slice) out.push(`${e.t}${e.l}`);
  }
  return out.join('\n');
}

async function runVectorTest(
  implDir: string,
  resultFile: string,
): Promise<{ type: 'pass' | 'fail' | 'diff'; diff?: string; reason?: string }> {
  try {
    const parsed = parseLog(path.join(implDir, 'did.jsonl'));
    if (!parsed) return { type: 'fail', reason: 'empty did.jsonl' };
    const expected = JSON.parse(fs.readFileSync(path.join(implDir, resultFile), 'utf8'));

    const versionNumber = extractVersionNumber(resultFile);
    const resolutionOptions: Record<string, unknown> = {};
    if (versionNumber !== null) {
      const entry = parsed.entries[versionNumber - 1];
      if (!entry) return { type: 'fail', reason: `no log entry for version ${versionNumber}` };
      resolutionOptions.versionId = entry.versionId;
    }

    const witness = readWitness(implDir);
    const restore = stubFetch(parsed.lines.join('\n') + '\n', witness);
    let actual: unknown;
    try {
      actual = await resolveViaThisdid(parsed.did, resolutionOptions);
    } finally {
      restore();
    }

    if (canonicalize(actual as any) === canonicalize(expected)) return { type: 'pass' };
    return { type: 'diff', diff: computeUnifiedDiff(expected, actual) };
  } catch (e: any) {
    return { type: 'fail', reason: e.message };
  }
}

interface RowEntry { logSource: string; result: string; notes: string }
interface DiffEntry { logSource: string; filename: string; diff: string }
interface NegRow { testCase: string; expectedError: string; result: string; notes: string }

async function crossResolveStatus(
  scenarioName: string
): Promise<{ rows: RowEntry[]; diffs: DiffEntry[] }> {
  const scenarioDir = path.join(VECTORS_DIR, scenarioName);

  const implDirs = fs.readdirSync(scenarioDir)
    .filter((d: string) => fs.statSync(path.join(scenarioDir, d)).isDirectory())
    .sort();

  const rows: RowEntry[] = [];
  const diffs: DiffEntry[] = [];

  for (const implName of implDirs) {
    const implDir = path.join(scenarioDir, implName);
    if (!fs.existsSync(path.join(implDir, 'did.jsonl'))) {
      rows.push({ logSource: implName, result: '⚠️ SKIP', notes: 'no did.jsonl' });
      continue;
    }

    const resultFiles = fs.readdirSync(implDir)
      .filter((f: string) => f.startsWith('resolutionResult') && f.endsWith('.json'))
      .sort();

    if (resultFiles.length === 0) {
      rows.push({ logSource: implName, result: '⚠️ SKIP', notes: 'no resolutionResult files' });
      continue;
    }

    let outcome: 'pass' | 'fail' | 'diff' = 'pass';
    let failReason = '';

    for (const rf of resultFiles) {
      const r = await runVectorTest(implDir, rf);
      if (r.type === 'fail') {
        if (outcome !== 'diff') { outcome = 'fail'; failReason = r.reason ?? 'error'; }
      } else if (r.type === 'diff') {
        if (outcome === 'pass') outcome = 'diff';
        diffs.push({ logSource: implName, filename: rf, diff: r.diff! });
      }
    }

    rows.push({
      logSource: implName,
      result: outcome === 'pass' ? '✅ PASS' : outcome === 'diff' ? '🔶 DIFF' : '❌ FAIL',
      notes: outcome === 'fail' ? failReason : outcome === 'diff' ? 'see diffs.txt' : '',
    });
  }

  return { rows, diffs };
}

// ---------------------------------------------------------------------------
// Negative resolution
// ---------------------------------------------------------------------------

async function runURLResolutionTest(did: string): Promise<{ outcome: 'pass' | 'fail'; fetchedUrl?: string }> {
  const tracker = stubFetchTracking();
  // Suppress the library's "Error fetching DID log" console.error that fires
  // when our fetch intercept throws — expected noise in this test context.
  const originalConsoleError = console.error;
  console.error = (...args: any[]) => {
    if (typeof args[0] === 'string' && args[0].includes('Error fetching DID log')) return;
    originalConsoleError(...args);
  };
  try {
    await resolveViaThisdid(did, {});
  } finally {
    tracker.restore();
    console.error = originalConsoleError;
  }
  return tracker.attempted ? { outcome: 'fail', fetchedUrl: tracker.url } : { outcome: 'pass' };
}

async function runNegativeResolutionTest(
  scenarioName: string,
): Promise<{ outcome: 'pass' | 'fail' | 'skip'; expectedError: string; reason?: string }> {
  // Negative artifacts are only ever generated by the ts harness.
  const dir = path.join(VECTORS_DIR, scenarioName, 'ts');
  const scriptPath = path.join(VECTORS_DIR, scenarioName, 'script.yaml');

  if (!fs.existsSync(path.join(dir, 'did.jsonl'))) {
    return { outcome: 'skip', expectedError: '—', reason: 'not generated' };
  }

  const resultPath = path.join(dir, 'resolutionResult.json');
  if (!fs.existsSync(resultPath)) {
    return { outcome: 'skip', expectedError: '—', reason: 'no resolutionResult.json' };
  }

  const expectedResult = JSON.parse(fs.readFileSync(resultPath, 'utf8'));
  const expectedError: string = expectedResult.didResolutionMetadata?.error ?? '?';

  const logContent = fs.readFileSync(path.join(dir, 'did.jsonl'), 'utf8').trim();
  if (!logContent) {
    // URL-only test: no log in the file. Read the DID URLs from the script's
    // resolve-did ops and test each one via the resolver with a fetch
    // intercept. PASS = rejected before touching the network (correct).
    const script = yamlLoad(fs.readFileSync(scriptPath, 'utf8')) as any;
    const resolveDIDOps = (script.steps ?? []).filter((s: any) => s.op === 'resolve-did');

    for (const op of resolveDIDOps) {
      const r = await runURLResolutionTest(op.did as string);
      if (r.outcome === 'fail') {
        return { outcome: 'fail', expectedError, reason: `resolver fetched URL: ${r.fetchedUrl}` };
      }
    }
    return { outcome: 'pass', expectedError };
  }

  let did: string;
  try {
    did = parseLog(path.join(dir, 'did.jsonl'))!.did;
  } catch (e: any) {
    return { outcome: 'skip', expectedError, reason: `cannot determine DID: ${e.message}` };
  }

  const witness = readWitness(dir);
  const restore = stubFetch(logContent + '\n', witness);
  let result: any;
  try {
    result = await resolveViaThisdid(did, {});
  } finally {
    restore();
  }

  if (result.didResolutionMetadata?.error) {
    return { outcome: 'pass', expectedError };
  }
  return { outcome: 'fail', expectedError, reason: `expected error "${expectedError}" but resolution succeeded` };
}

async function negativeResolutionStatus(): Promise<NegRow[]> {
  const scenarios = fs.readdirSync(VECTORS_DIR)
    .filter((d: string) => {
      const scriptPath = path.join(VECTORS_DIR, d, 'script.yaml');
      if (!fs.existsSync(scriptPath)) return false;
      const s = yamlLoad(fs.readFileSync(scriptPath, 'utf8')) as any;
      return s.negative === true;
    })
    .sort();

  const rows: NegRow[] = [];
  for (const name of scenarios) {
    const r = await runNegativeResolutionTest(name);
    rows.push({
      testCase: name,
      expectedError: r.expectedError,
      result: r.outcome === 'pass' ? '✅ PASS' : r.outcome === 'fail' ? '❌ FAIL' : '⚠️ SKIP',
      notes: r.reason ?? '',
    });
  }
  return rows;
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

async function main() {
  const scenarioNames = fs.readdirSync(VECTORS_DIR)
    .filter((d: string) => {
      const p = path.join(VECTORS_DIR, d);
      return fs.statSync(p).isDirectory() && fs.existsSync(path.join(p, 'script.yaml'));
    })
    .sort();

  const allRows: Array<{ testCase: string } & RowEntry> = [];
  const allDiffs: Array<{ testCase: string } & DiffEntry> = [];

  for (const name of scenarioNames) {
    const scriptPath = path.join(VECTORS_DIR, name, 'script.yaml');
    const script = yamlLoad(fs.readFileSync(scriptPath, 'utf8')) as any;
    if (script.negative === true) continue;

    process.stdout.write(`Cross-resolving ${name}... `);
    const { rows, diffs } = await crossResolveStatus(name);
    for (const row of rows) allRows.push({ testCase: name, ...row });
    for (const diff of diffs) allDiffs.push({ testCase: name, ...diff });
    console.log('done');
  }

  process.stdout.write('Running negative resolution tests... ');
  const negRows = await negativeResolutionStatus();
  console.log('done');

  const cfg = readConfig();
  const coreVersion = readWrappedCoreVersion();
  const header = `Implementation: @thisdid/webvh-did-resolver ${cfg.version}`
    + (coreVersion ? ` (wraps didwebvh-ts ${coreVersion})\n\n` : '\n\n');

  const genNote = '## DID Creation\n\n'
    + 'thisdid is a resolver-only wrapper — it has no DID-log generator, so DID Creation is '
    + 'not applicable and does not run here.\n\n';

  const metaNote = '**Note on didDocumentMetadata:** the wrapper deliberately maps only '
    + '`created`, `updated`, `versionId`, and `deactivated` into `didDocumentMetadata` (see its '
    + 'README) rather than the full metadata surface (`scid`, `updateKeys`, `nextKeyHashes`, '
    + '`witness`, `watchers`, `portable`, `previousLogEntryHash`, ...) that the committed '
    + '`resolutionResult*.json` vectors carry. Because comparisons below are against the full '
    + 'resolution-result envelope, essentially every cross-resolution row will show DIFF on '
    + '`didDocumentMetadata` for this reason alone — a deliberate minimal-surface design choice '
    + 'in the wrapper, not a resolution-correctness bug. Rows may also carry the same '
    + '`didDocument`-level differences (e.g. relative vs. absolute service `id`s) already visible '
    + 'between other implementations in their own status.md files — thisdid wraps didwebvh-ts '
    + '2.8.0, an older version than some vectors were generated with. See diffs.txt for details.\n\n';

  const negTable = negRows.length > 0
    ? `## Negative Resolution\n\n| Test Case | Expected Error | Result | Notes |\n|---|---|---|---|\n${negRows.map(r => `| ${r.testCase} | ${r.expectedError} | ${r.result} | ${r.notes} |`).join('\n')}\n\n`
    : '';

  const tableRows = allRows
    .map(r => `| ${r.testCase} | ${r.logSource} | ${r.result} | ${r.notes} |`)
    .join('\n');
  const crossTable = allRows.length > 0
    ? `## Cross-Resolution\n\n| Test Case | Log Source | Result | Notes |\n|---|---|---|---|\n${tableRows}\n`
    : '';

  const statusPath = path.join(IMPL_DIR, 'status.md');
  const diffsPath = path.join(IMPL_DIR, 'diffs.txt');

  fs.writeFileSync(statusPath,
    `# thisdid status\n\n${header}${genNote}${metaNote}${negTable}${crossTable}`
  );

  if (allDiffs.length > 0) {
    const diffContent = allDiffs
      .map(d => `=== ${d.testCase} / ${d.logSource} — ${d.filename} ===\n${d.diff}`)
      .join('\n\n') + '\n';
    fs.writeFileSync(diffsPath, diffContent);
  } else if (fs.existsSync(diffsPath)) {
    fs.unlinkSync(diffsPath);
  }
}

main();
