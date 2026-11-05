# negative-fractional-witness-threshold

**Bug:** a resolver truncates a fractional witness `threshold` to an integer
**Originating audit:** spec review
**Affected at audit time:** TypeScript 🔴 (3.0.0); other implementations not yet assessed

## Attack pattern

The genesis entry declares `witness: { threshold: 1.5, witnesses: [wit-0] }`,
with `threshold` as a JSON number that has a fractional part. A resolver that
truncates or `parseInt`s the value reads it as `1`. The log carries a valid
witness proof from `wit-0`, so such a resolver finds the threshold met and
resolves the DID. The fractional value is the only fault.

Unlike [`negative-string-witness-threshold`](../negative-string-witness-threshold/),
the value here is a JSON number, so a resolver that checks only the JSON type
still accepts it.

## Why this must be rejected

The specification's Witness Lists section says `threshold` is a positive
integer (JSON number, no fractional part), and that a `witness` parameter
whose `threshold` is non-integer **MUST** be rejected, terminating resolution
with an error.

## Expected error

`invalidParameters`.

## Generator notes

The generator passes the YAML number `1.5` through unchanged. A library whose
`createDID` rejects it falls back to `createGenesisBypass`; the committed
artifacts were written by didwebvh-ts 3.0.0, whose `createDID` accepts it.
`sign-witness-proof` adds the genesis witness proof.
