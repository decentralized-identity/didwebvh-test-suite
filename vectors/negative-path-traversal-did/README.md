# negative-path-traversal-did

**Bug:** #4 (path traversal in DID→HTTP path)
**Originating audit:** Rust (also reproduced in TypeScript and Java)
**Affected at audit time:** Rust 🔴 · Python 🟡 (yarl-mitigated) · TypeScript 🔴 · Java 🔴

## Attack pattern

A `did:webvh` DID's colon-separated path segments are joined into the
HTTP path used to fetch `did.jsonl`:

```
did:webvh:<scid>:example.com:..:..:admin
                  ^^^^^^^^^^^^^^^^^^^^^^
                  → https://example.com/../../admin/did.jsonl
```

Without segment validation, a crafted DID reaches files outside the
intended directory on the host (or, on a path-routing reverse proxy, a
different backend entirely).

## Why this must be rejected

Identifier parsing must reject syntactically dangerous path components,
not rely on downstream URL normalisation.

The spec should explicitly require:

> Each colon-separated path segment after the host MUST NOT be empty,
> `"."`, or `".."`, and MUST NOT contain `"/"` or `"\\"`. Implementations
> MUST percent-decode segments before applying these checks.

## Expected error

`invalidDid` — parse-time failure, no HTTP fetch is attempted.

## Implementation references

- Rust: `src/url.rs:125-130` — no traversal check. Fixed in patch `0004`.
- TypeScript: `src/utils.ts:459` — `getBaseUrl` joins
  `decodeURIComponent(parts.slice(3).join('/'))` with no segment
  validation.
- Java: `DidWebVhUrl.java:89-96` accepts any non-empty segment;
  `DidToHttpsTransformer.java:101-115` treats `.` as unreserved and emits
  the literal `..` segment into the HTTPS path.
- Python: partially mitigated — `DomainPath.validate` only rejects empty
  segments; yarl's URL normalisation in aiohttp clamps `..` at host root,
  so cross-origin escape is prevented but intra-host path mutation is
  still possible.

## Generator notes

Requires the proposed `resolve-did` DSL op (resolves a literal DID URL
string instead of the DID produced by a prior `create`).
