# negative-versiontime-future

**Bug:** P1 (no future-time check) / J8 (future-time only on last entry)
**Originating audit:** Python (related Java finding J8)
**Affected at audit time:** Rust ✅ · Python 🔴 · TypeScript 🔴 · Java 🟡 (middle entries allowed far-future)

## Attack pattern

The spec implies `versionTime` should be in the past relative to the
resolver's wall clock. An attacker stamps an entry with a far-future
timestamp (year 9999) to:

- aged-out updates: legitimate later updates will appear "older" by
  comparison and may not be selected by `versionTime` queries
- break auditing: histories no longer correlate with real time

## Why this must be rejected

Every entry's `versionTime` MUST be in the past (allowing a small
clock-skew window). Checking only the last entry leaves a wide attack
surface.

The spec should explicitly require:

> Implementations MUST verify, for every entry in the log, that
> `versionTime <= now() + clockSkew`, where `clockSkew` is a small
> implementation-defined window (typically ≤ 60 seconds). The check MUST
> apply to every entry, not just the most recent.

## Expected error

`invalidDid`.

## Implementation references

- Python: no future-time check anywhere — `core/resolver.py:362-432`.
- TypeScript: same TODO as P01.
- Java: `LogChainValidator.java:158-164` — future-time check only on the
  last entry; middle entries can be year-9999.
- Rust: not vulnerable — `verify_version_time`
  (`src/log_entry/read.rs:289-309`) rejects future on every entry,
  enforced per-entry from `validate.rs:194`.

## Generator notes

Requires `corrupt` with `mutation: replace-version-time`.
