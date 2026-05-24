# negative-fragment-leaks-into-domain

**Bug:** #6 (fragment leaks into domain when URL has no query)
**Originating audit:** Rust
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript ⚪ (N/A — different parser) · Java ✅

## Attack pattern

A specific parser bug in the Rust implementation (a swapped variable in
the query-split fallback) caused the already-stripped fragment to be
re-included in the host when the DID URL had a fragment but no query.
`did:webvh:<scid>:127.0.0.1#x` became `host="127.0.0.1#x"`, which is not
an IPv4 literal and so bypassed the IP-rejection check.

This vector is included in the test suite to ensure the bug stays fixed
and that any future implementation choosing a similar parsing approach
catches it.

## Expected error

`invalidDid`.

## Implementation references

- Rust: `src/url.rs:88` — `None => (url, None)` should have been
  `(prefix, None)`. Fixed in patch `0006`.

## Generator notes

Requires `resolve-did`. This is essentially a regression test against a
specific Rust-side bug shape, but the cost of including it in the shared
suite is trivial.
