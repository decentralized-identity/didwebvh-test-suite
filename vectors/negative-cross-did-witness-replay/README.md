# negative-cross-did-witness-replay

**Bug:** #11 (cross-DID witness-proof replay)
**Originating audit:** Rust (also affects TypeScript and Java)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript 🔴 · Java 🔴

This is the most serious **spec-level gap** in the audit set: three of
four implementations reproduce it because the spec under-specifies the
binding between a witness proof and a particular DID.

## Attack pattern

A witness signs the canonical JSON of `{"versionId": "<entry-hash>"}`.
That payload contains no DID identifier. So a genuine signature W made
for DID-B's entry `3-<some-hash>` is *cryptographically valid* against
DID-A's log too, if DID-A and DID-B share any witness.

Attack steps:

1. The attacker controls the hosting for DID-A (or controls DID-A's
   controller key) but DID-A is configured with a real witness W.
2. DID-A also publishes a `did-witness.json` file from the same host.
3. The attacker downloads DID-B's genuine witness proof (DID-B is some
   other DID that shares witness W) and pastes it into DID-A's
   `did-witness.json` verbatim.
4. The attacker turns witnessing off in an early forged DID-A entry so
   no later entry direct-verifies the proof against DID-A's actual
   entry hash.
5. The replayed proof's `versionId` is from DID-B's log, but the
   "later-version" branch of a vulnerable resolver verifies the
   signature against the *claimed* `versionId` — which checks out
   because W really did sign it. The threshold is "met", and DID-A
   resolves with no W having actually witnessed anything in it.

## Why this must be rejected

The witness signing payload was meant to authenticate "W has seen this
specific entry". Without DID binding, W's signature is portable — once
W has signed a single `versionId` for any DID, that signature
authenticates that `versionId` for *every* DID where it appears.

Possible spec fixes (not in scope for this PR, but for context):

- **Recommended**: add a normative requirement that resolvers MUST
  filter witness proofs to those whose `versionId` is present in *this*
  log before counting them toward the threshold. This is the fix Rust
  patch `0011` applies and Python's `WitnessChecks.verify` happens to
  do correctly.
- **Future**: bind the DID into the witness signing payload itself
  (would break the wire format and is therefore out of scope for the
  proposed spec PR).

## Expected error

`invalidDid` — typically surfacing as a witness threshold failure
because the replayed proof is filtered out.

## Implementation references

- Rust: `src/validate.rs:274` had no versionId filter. Fixed in patch
  `0011` by filtering `witness_version` to entries whose versionId is
  in this log.
- Python: not vulnerable — `WitnessChecks.verify`
  (`witness.py:93`) requires `self.versions[check_num - 1] == check_ver`
  — i.e. the proof's `versionId` must equal the actual versionId at
  that position in *this* log.
- TypeScript: `src/method_versions/method.v1.0.ts:334-336` filters by
  `versionId` string equality only, never against this DID's log.
- Java: `WitnessValidator.java:63-65` — `signedDoc = {"versionId": entry.getVersionId()}`
  with no DID binding; same shape as TS.

## Generator notes

Requires three DSL extensions:

1. A `did_label` parameter on every op to disambiguate multi-DID
   scenarios (default `A` if absent, for backward compatibility).
2. A `replay-witness-proof` op that copies a witness proof from one
   DID's witness file to another's verbatim.
3. The existing `resolve` op gains an optional `did:` field naming
   which DID to resolve (default: the last-created DID).

This is the most complex vector to support in the generator; multi-DID
support may be a candidate for a separate enabling PR.
