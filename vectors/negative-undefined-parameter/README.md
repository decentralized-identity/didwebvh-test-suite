# negative-undefined-parameter

**Bug:** a resolver ignores `parameters` properties that did:webvh v1.0 does not define
**Originating audit:** spec review
**Affected at audit time:** TypeScript 🔴 (3.0.0); other implementations not yet assessed

## Attack pattern

The second log entry carries `"foo": "bar"` in its `parameters`. Everything
else about the log is valid: the entry is re-signed after the parameter is
added, so its hash chain and controller proof verify. A vulnerable resolver
skips the unknown property and resolves the DID.

`foo` stands for any property outside the v1.0 parameter list. The same rule
covers names that look meaningful, such as top-level `witnesses` or
`witnessThreshold` written by pre-v1.0 drafts. A resolver that ignores those
resolves a log whose author meant it to be witnessed as if it were
unwitnessed.

## Why this must be rejected

The specification's DID Method Parameters section says:

> The `parameters` object **MUST** only include properties defined in the
> version of the `did:webvh` DID Method specification being used.

and the read algorithm says the `parameters` **MUST** adhere to that section.
The parameters did:webvh v1.0 defines are `method`, `scid`, `updateKeys`,
`nextKeyHashes`, `witness`, `watchers`, `portable`, `deactivated` and `ttl`.

## Expected error

`invalidParameters`.

## Generator notes

Requires `corrupt` with `mutation: replace-parameter` on a non-genesis entry
(re-signing after). `replace-parameter` adds the property when it is absent.
