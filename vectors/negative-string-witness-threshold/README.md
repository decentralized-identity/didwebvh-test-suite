# negative-string-witness-threshold

**Bug:** a resolver coerces a string witness `threshold` to an integer
**Originating audit:** spec review
**Affected at audit time:** TypeScript 🔴 (3.0.0); other implementations not yet assessed

## Attack pattern

The genesis entry declares `witness: { threshold: "1", witnesses: [wit-0] }`,
with `threshold` as a JSON string. The log carries a valid witness proof from
`wit-0`, so a resolver that coerces `"1"` to `1` finds the threshold met and
resolves the DID. The string type is the fault.

A coercing resolver also accepts strings such as `"1abc"`, and a writer that
passes a caller's string through publishes logs that strict resolvers reject.

## Why this must be rejected

The specification's Witness Lists section says `threshold` is a positive
integer (JSON number, no fractional part), and that a `witness` parameter
whose `threshold` is non-integer **MUST** be rejected, terminating resolution
with an error.

The string form belongs only to DID resolution metadata, where
`didDocumentMetadata.witness.threshold` is a string holding the integer
because DID Resolution does not allow integers in DID metadata. It is never
valid in a log entry.

## Expected error

`invalidParameters`.

## Generator notes

The generator passes the YAML string `"1"` through unchanged. A library whose
`createDID` rejects it falls back to `createGenesisBypass`; the committed
artifacts were written by didwebvh-ts 3.0.0, whose `createDID` accepts it.
`sign-witness-proof` adds the genesis witness proof.
