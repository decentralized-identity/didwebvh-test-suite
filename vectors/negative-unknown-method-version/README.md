# negative-unknown-method-version

**Bug:** R6 (`parameters.method` silently coerces unknown values to default)
**Originating audit:** Rust
**Affected at audit time:** Rust 🔴 · Python 🔴 · TypeScript 🔴 · Java 🔴

This bug is reproduced by **all four** audited implementations.

## Attack pattern

`parameters.method` is supposed to identify the did:webvh version the
entry conforms to (e.g., `did:webvh:1.0`). A vulnerable implementation
maps any non-matching string to a default version (typically v1.0 or
the latest known) instead of rejecting the entry.

Today this is benign — there is only one version. But once v1.1, v2.0,
etc. add breaking changes, an attacker who publishes a log claiming a
future version can downgrade the resolver to interpret it under v1.0
rules, defeating any new validation the future spec introduces.

## Why this must be rejected

Algorithm-version negotiation must fail closed.

The spec should explicitly require:

> The `method` parameter MUST exactly match a method version identifier
> from a registry of known did:webvh versions. Implementations MUST
> reject any entry whose `method` value is unknown, regardless of
> whether the unknown value resembles a future version.

## Expected error

`invalidDid`.

## Implementation references

- Rust: `src/parameters/spec_1_0.rs:100` — `value.method.map(|_| Version::V1_0)`.
- Python: `did_webvh/core/state.py:640-646` — `_update_params` accepts any
  non-empty string; strict check only in `verify.py:54` which is bypassed
  when `WebvhVerifier` isn't used.
- TypeScript: `src/method.ts:8-12` — `getWebvhVersionFromMethod` maps any
  non-matching `method` string to `LATEST_VERSION`.
- Java: `LogChainValidator.java:227-247` — `compareMethod` returns 0
  (equal) when `extractMethodVersion` returns null.

## Generator notes

Requires `corrupt` with `mutation: replace-parameter` (re-signing after).
