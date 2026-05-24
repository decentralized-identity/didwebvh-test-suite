# negative-pre-rotation-omit-updatekeys

**Bug:** #9 (pre-rotation bypass when updateKeys omitted)
**Originating audit:** Rust (also affects Java)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript ✅ · Java 🔴

## Attack pattern

When entry N commits `nextKeyHashes`, the spec intends entry N+1 to
present new `updateKeys` that hash into that committed set, rotating
control to the pre-committed key.

A vulnerable implementation accepts entry N+1 that *omits* `updateKeys`
entirely (or sends an empty array). The "absent / empty" code paths skip
the `validate_pre_rotation_keys` check and just inherit
`previous.active_update_keys` unchanged. The subsequent proof check then
authorizes against the *old* keys — defeating pre-rotation entirely.

This matters if an attacker compromises an old update key *after* the
controller has committed its replacement: the controller believes they
have safely pre-rotated, but the attacker can still forge entry N+1 by
omitting `updateKeys`.

## Why this must be rejected

Pre-rotation is the spec's defence against undetected key compromise. If
the new entry can decline to present the pre-committed key and re-use
the old key, the mechanism provides zero guarantee.

The spec should explicitly require:

> When the previous entry's `nextKeyHashes` is non-empty,
> `updateKeys` MUST be present and non-empty in this entry, and each
> entry MUST hash into the previous `nextKeyHashes` set. Implementations
> MUST reject the entry if `updateKeys` is absent or empty in this case.

## Expected error

`invalidParameters` — the parameter validation step should fail before
proof verification is attempted.

## Implementation references

- Rust: `src/parameters/mod.rs:367-393` — `None`/empty arms did not check
  `pre_rotation_previous_value`. Fixed in patch `0009`.
- Java: `LogChainValidator.java:134` — guards pre-rotation enforcement
  with `if (i > 0 && entryParams.getUpdateKeys() != null)`. Skipped when
  `updateKeys` is absent.
- Python: not vulnerable — `_check_key_rotation`
  (`state.py:236-241`) requires `self.params_update.get("updateKeys")`
  to be non-None whenever the previous entry set `nextKeyHashes`.
- TypeScript: not vulnerable — `method.v1.0.ts:234` verifies the
  signature against `parameters.updateKeys` when `meta.prerotation` is
  true; absent / empty causes `isKeyAuthorized` to see `[]` and fail.

## Generator notes

Requires the `corrupt` DSL extension with a `drop-parameter` mutation
applied before signing. The generator must produce a signed entry whose
proof is valid over a parameters block missing `updateKeys`.
