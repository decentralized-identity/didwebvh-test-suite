# negative-zero-witness-threshold

**Bug:** R2 (witness threshold parsed via untagged enum silently treats
invalid values as `Empty`)
**Originating audit:** Rust (also affects TypeScript in different form)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript 🟡 · Java ✅

## Attack pattern

The DID controller publishes a witness config with `threshold: 0` (or a
negative integer, or a non-integer such as a string or `null`) alongside a
non-empty witnesses list. In implementations that use a permissive parser
(serde untagged enum, Zod `.or()`, etc.), the malformed-but-non-empty value
falls through to the "empty / no witnesses" arm and witness validation is
silently disabled for the DID — even though the controller is presenting a
witness list as if witnessing is in effect.

## Why this must be rejected

A controller advertises "this DID is witnessed by W1, W2, W3" but ships a
configuration that, due to parser leniency, disables witnessing entirely.
Upstream consumers who trusted the witnessing claim are deceived.

The spec should explicitly require:

- `witness.threshold` MUST be a positive integer (≥ 1)
- Parsers MUST reject malformed witness configurations rather than
  silently treating them as `{}` / "no witnesses"

## Expected error

`invalidParameters` — the parser should reject the malformed config and
not proceed to evaluate the entry.

## Implementation references

- Rust: `src/witness/mod.rs:75-89` — `Witnesses` is `#[serde(untagged)]`
  with a `Value` arm then `Empty {}`; when the `Value` arm fails to
  deserialize, serde falls through and `Empty {}` matches.
- TypeScript: `src/witness.ts:33` checks `parseInt(...).toString() < 1`
  but unvalidated `parameters.witnessThreshold` at
  `src/method_versions/method.v1.0.ts:266` silently coerces non-numeric
  strings to `NaN` → `0`.
