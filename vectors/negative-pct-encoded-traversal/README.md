# negative-pct-encoded-traversal

**Bug:** #13 (`%2E%2E` bypasses traversal check)
**Originating audit:** Rust (also reproduced in TypeScript)
**Affected at audit time:** Rust 🔴 · Python 🟡 (yarl) · TypeScript 🔴 · Java ✅ (double-encoded)

## Attack pattern

Companion to `negative-path-traversal-did`. Even an implementation that
checks each path segment against the literal `".."` can be defeated by
percent-encoding: `%2E%2E` decodes to `..` *after* the check runs, and
the resulting URL `https://example.com/%2E%2E/admin/did.jsonl` is then
normalised by the HTTP client.

## Why this must be rejected

Validation must be applied to the decoded form, not the raw form.

The spec should explicitly require (combined with #4):

> Path segments MUST be percent-decoded (case-insensitive per RFC 3986
> §2.1) **before** the empty / `"."` / `".."` / separator-containing
> check is applied.

## Expected error

`invalidDid`.

## Implementation references

- Rust: `src/url.rs` — fixed in patch `0013` (decode-then-check).
- TypeScript: `src/utils.ts:459` — `getBaseUrl` decodes via
  `decodeURIComponent` before joining, with no literal `..` check
  anywhere.
- Java: not vulnerable — `DidToHttpsTransformer.percentEncode` classifies
  `%` as non-unreserved and emits `%25`, producing `%252E%252E`. Still
  noted because the underlying #4 traversal works without encoding.
- Python: partially mitigated — `parse_identifier` doesn't unquote path
  segments, so `%2E%2E` survives the (currently missing) literal `..`
  check; yarl clamps later.

## Generator notes

Requires `resolve-did`.
