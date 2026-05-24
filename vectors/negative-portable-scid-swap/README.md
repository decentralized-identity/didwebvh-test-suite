# negative-portable-scid-swap

**Bug:** #15 (portable entries can change SCID)
**Originating audit:** Rust (worse-than-Rust forms in TypeScript and Java)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript 🟡 (worse: no `alsoKnownAs` check) · Java 🟡 (worse: no portability check at all)

## Attack pattern

`portable: true` is intended to allow a DID's *host* / *path* to change
while the *SCID stays constant*. A vulnerable `verify_portability` allows
the new entry's `state.id` to be anything — including a different SCID
segment — as long as the previous DID is in `alsoKnownAs`. This reopens
the same self-certifying bypass that `verify_scid` closes for the
genesis entry, at every N > 1.

## Why this must be rejected

`parameters.scid` is the genesis hash, carried forward unchanged across
the entire log. It is the cryptographic anchor of the DID; the host/path
is the network location. Portability moves the network location; the
anchor must not move.

The spec should explicitly require, for every N > 1:

> The SCID segment of `state.id` MUST equal `parameters.scid`. Portability
> changes the host/path components only; the SCID component MUST remain
> the genesis self-hash. Implementations MUST reject any entry whose
> `state.id` SCID differs from `parameters.scid`, regardless of
> `portable` / `alsoKnownAs`.

## Expected error

`invalidDid` — same family as #14.

## Implementation references

- Rust: `src/log_entry/read.rs:250` — `verify_portability` did not check
  SCID. Fixed in patch `0015`.
- TypeScript: `src/method_versions/method.v1.0.ts:219-223` only checks
  `newHost !== host` when `!portable`. When `portable=true`, there is no
  check at all — neither SCID immutability nor `alsoKnownAs` membership.
- Java: `LogChainValidator` has no portability validation. Grep for
  `portable` finds only the "cannot set portable=true after first entry"
  rule (`LogChainValidator.java:205-208`).

## Generator notes

Requires both the `migrate` DSL op and the `corrupt` extension. The
generator must produce a portable-move entry where `state.id` carries the
attacker's chosen SCID (not the inherited one) before signing.
