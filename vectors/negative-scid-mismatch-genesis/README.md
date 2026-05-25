# negative-scid-mismatch-genesis

**Bug:** #14 (genesis `state.id` SCID not bound to `parameters.scid`)
**Originating audit:** Rust (also reproduced in TypeScript and Java)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript 🔴 · Java 🔴

## Attack pattern

A vulnerable implementation verifies that `parameters.scid` is the genesis
self-hash but never checks that the SCID segment embedded in `state.id`
(the field the resolver matches the requested DID against) is the same
value. An attacker publishes:

```jsonc
{
  "state": { "id": "did:webvh:QmAttackerChosen:example.com", ... },
  "parameters": { "scid": "<real-genesis-self-hash>", ... }
}
```

`verify_scid` passes (because `parameters.scid` does match the genesis
hash). A user resolving `did:webvh:QmAttackerChosen:example.com` matches
against `state.id` and accepts the "validated" log — even though the SCID
they typed has zero cryptographic binding to the genesis.

## Why this must be rejected

The "self-certifying" in SCID is meant to bind the DID a user types to
the genesis log entry. If `state.id`'s SCID is unverified, an attacker who
controls the hosting can mint arbitrarily-named DIDs that all point to a
single underlying log.

The spec should explicitly require:

> Every entry MUST contain a `state.id` whose SCID segment is exactly
> equal to `parameters.scid`. Implementations MUST reject the log if the
> two values differ.

## Expected error

`invalidDid` — the binding check is part of DID validation, not parameter
validation, because the corrupted field is `state.id`.

## Implementation references

- Rust: `src/log_entry/read.rs:312-348` — `verify_scid` only verified
  `parameters.scid`. Fixed in patch `0014`.
- TypeScript: `src/method_versions/method.v1.0.ts:181-184` extracts
  `meta.scid` from `parameters.scid`; the SCID segment in `state.id` is
  never compared. The only `state.id`-based check is
  `host = newDoc.id.split(':').at(-1)` (`method.v1.0.ts:180`) — last
  segment only.
- Java: `LogChainValidator.java:73-78` derives SCID from
  `parameters.scid`; `state.id` only checked via
  `docId.equals(expectedDid)` (line 128). When `resolveFromLog` /
  `resolveFromFile` is called with `expectedDid=null`, the binding check
  is bypassed entirely.

## Generator notes

Requires the `corrupt` DSL extension (`when: before-sign`,
`mutation: replace-state-id-scid`). The generator's normal `createDID`
path applies a `{SCID}`→`<real-hash>` substitution that affects
*both* `parameters.scid` and `state.id`. For this vector the generator
must build the entry with `state.id` carrying a literal (non-`{SCID}`)
attacker value before applying the placeholder substitution to the
parameters.
