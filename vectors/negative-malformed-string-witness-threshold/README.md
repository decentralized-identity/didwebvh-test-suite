# negative-malformed-string-witness-threshold

**Bug:** a resolver parses a string witness `threshold` by its numeric prefix
**Originating audit:** spec review
**Affected at audit time:** TypeScript 🔴 (3.0.0); other implementations not yet assessed

## Attack pattern

The genesis entry declares `witness: { threshold: "1abc", witnesses: [wit-0] }`.
A resolver that parses the value with `parseInt`-style coercion reads `"1abc"`
as `1`. The log carries a valid witness proof from `wit-0`, so such a resolver
finds the threshold met and resolves the DID. The malformed string is the only
fault.

This goes further than [`negative-string-witness-threshold`](../negative-string-witness-threshold/):
a resolver that accepts `"1"` may still reject `"1abc"`, and one that accepts
`"1abc"` accepts any string beginning with digits.

## Why this must be rejected

The specification's Witness Lists section says `threshold` is a positive
integer (JSON number, no fractional part), and that a `witness` parameter
whose `threshold` is non-integer **MUST** be rejected, terminating resolution
with an error.

## Expected error

`invalidParameters`.

## Generator notes

The generator passes the YAML string `"1abc"` through unchanged. A library
whose `createDID` rejects it falls back to `createGenesisBypass`; the committed
artifacts were written by didwebvh-ts 3.0.0, whose `createDID` accepts it.
`sign-witness-proof` adds the genesis witness proof.
