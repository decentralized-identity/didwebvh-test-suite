# negative-duplicate-witness-ids

**Bug:** #3 (duplicate witness IDs bypass threshold)
**Originating audit:** Rust (also reproduced in Python and Java)
**Affected at audit time:** Rust 🔴 · Python 🔴 · TypeScript ✅ · Java 🔴

## Attack pattern

The DID controller publishes a `witness` configuration containing the same
witness ID multiple times with a threshold equal to the duplication count.
A vulnerable resolver iterates the configured witness list and counts one
match per listed entry, so a single proof from that witness satisfies the
entire threshold.

```yaml
witness:
  threshold: 2
  witnesses:
    - id: wit-0
    - id: wit-0   # duplicate
```

With threshold=2 and a single proof from `wit-0`, a vulnerable resolver
counts the proof twice (once per list entry) and considers the witness
threshold satisfied.

## Why this must be rejected

Witnesses exist precisely to constrain a (possibly compromised) DID
controller. If the controller can weaken its own witness threshold by
duplicating witness IDs, the entire witness mechanism is meaningless.

The spec should explicitly require:

- `witness.witnesses` MUST contain distinct IDs
- `witness.threshold` MUST be `≤ count(distinct witnesses)`

Parsers MUST reject configurations that violate either rule.

## Expected error

`invalidParameters` — the validator should refuse the witness configuration
at parse time and never proceed to log-entry verification.

## Implementation references

- Rust: `src/witness/mod.rs:117-141` — `Witnesses::validate` checked
  threshold ≤ count, never deduped. Fixed in patch `0003`.
- Python: `did_webvh/core/witness.py:38-62` — `WitnessRule.deserialize`
  builds `tuple(WitnessEntry.deserialize(w) for w in witnesses)` with no
  dedup; `WitnessChecks.verify` (lines 87-96) iterates and counts.
- Java: `LogChainValidator.java:218` and `WitnessValidator.java:67-79` —
  threshold checked against raw `size()`; per-key dedup missing.
- TypeScript: not vulnerable — `validateWitnessParameter`
  (`src/witness.ts:37-46`) rejects duplicates via a Set, and
  `verifyWitnessProofs` (`src/witness.ts:76, 91, 147`) independently
  dedups counted approvals.
