# negative-pct-encoded-ip-host

**Bug:** #12 (percent-encoded IP host bypass)
**Originating audit:** Rust (also reproduced in TypeScript and Java)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript 🔴 · Java 🔴

## Attack pattern

The spec forbids IP addresses as the host portion of a `did:webvh`. A
vulnerable implementation runs the IP-rejection check on the *raw*,
still-percent-encoded host segment — so `127%2E0%2E0%2E1` does not match
the IPv4 regex and slips through. The URL builder then percent-decodes
the host (`%2E` → `.`) and the HTTP client connects to `127.0.0.1`. The
exact same bypass works against `169.254.169.254` (AWS metadata) and any
RFC1918 address.

## Why this must be rejected

The IP-rejection rule must be applied to what the HTTP client will
actually connect to, not to the raw DID text. A naïve check that runs
before normalisation is defeated by the very normalisation it relies on.

The spec should explicitly require:

> Implementations MUST reject any DID whose host segment, after
> percent-decoding (case-insensitive per RFC 3986 §2.1) and IDNA
> normalisation, parses as an IPv4 or IPv6 literal.

## Expected error

`invalidDid` — parse-time failure, no fetch attempted.

## Implementation references

- Rust: `src/url.rs:349` — no post-parse host check. Fixed in patch `0012`.
- TypeScript: `src/utils.ts:67` and `:459, 463` — `isIPAddress` on raw
  segment; `getBaseUrl` decodes twice without re-checking.
- Java: `DidWebVhUrl.java:163` — IPv4 regex on raw segment; OkHttp /
  `IDN.toASCII` later normalises.
- Python: not vulnerable — `DomainPath.parse_identifier`
  (`domain_path.py:44`) unquotes before `validate()`, and `validate()`
  rejects labels not starting with an alpha character.

## Generator notes

Requires `resolve-did`. No log generation needed.
