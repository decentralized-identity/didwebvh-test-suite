# negative-versiontime-non-monotonic

**Bug:** P1 (no `versionTime` monotonicity / future-time check) — also related to R5
**Originating audit:** Python (also affects TypeScript)
**Affected at audit time:** Rust ✅ · Python 🔴 · TypeScript 🔴 · Java ✅ (monotonicity only)

## Attack pattern

The spec requires `versionTime` to be strictly later than the previous
entry's `versionTime`. A vulnerable implementation does not enforce
monotonicity at log-validation time. An attacker who controls the
hosting (or a compromised controller) writes an entry with
`versionTime` equal to (or earlier than) the previous entry's. This:

- breaks "resolve at `versionTime = T`" queries (the resolver picks an
  attacker-chosen entry)
- removes the linearisable history guarantee that downstream auditors
  rely on

## Why this must be rejected

`versionTime` is the spec's primary auditability hook. Without
monotonicity enforcement, the log becomes an unordered set, not an
append-only history.

The spec should explicitly require:

> Implementations MUST verify, for every entry N > 0, that
> `entry[N].versionTime > entry[N-1].versionTime`. The check MUST apply
> to every entry in the log, not just the last.

## Expected error

`invalidDid`.

## Implementation references

- Python: `did_webvh/core/state.py:318-454` / `resolver.py:296-492` —
  never compares timestamps across entries.
- TypeScript: `src/method_versions/method.v1.0.ts:172` and
  `method.v0.5.ts:148` — literal `// TODO check timestamps make sense`.
- Rust: `src/log_entry/read.rs:289-309` enforces both
  `versionTime > Utc::now()` rejection AND
  `current > previous` per-entry monotonicity.
- Java: `LogChainValidator.java:120-123` enforces monotonicity on every
  entry (so this script only catches the second variant — the first is
  Java-safe).

## Generator notes

Requires `corrupt` with `mutation: replace-version-time`.
