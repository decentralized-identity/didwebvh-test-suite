# negative-lowercase-pct-port-ip

**Bug:** #5 (lowercase `%3a` bypasses IP rejection)
**Originating audit:** Rust (also reproduced in TypeScript)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript 🔴 · Java ✅

## Attack pattern

A vulnerable implementation splits the host on the literal `%3A` only.
`127.0.0.1%3a8080` (lowercase) is not split, leaving the "host" as
`127.0.0.1%3a8080` — which fails any IP-literal regex and so passes the
IP-rejection check. The URL builder later percent-decodes `%3a` → `:`,
producing `https://127.0.0.1:8080/...`.

## Why this must be rejected

RFC 3986 §2.1 makes percent-encoding case-insensitive. Implementations
that only match the uppercase form silently miss the lowercase variant.

The spec should explicitly require (or reference RFC 3986 §2.1):

> Implementations MUST treat percent-encoded characters as
> case-insensitive when parsing the host:port separator and the IP
> address detection.

## Expected error

`invalidDid`.

## Implementation references

- Rust: `src/url.rs:106` — only matched `%3A`. Fixed in patch `0005`.
- TypeScript: `src/utils.ts:57, 128` — only matches uppercase `%3A`.
- Python / Java: not vulnerable.

## Generator notes

Requires `resolve-did`.
