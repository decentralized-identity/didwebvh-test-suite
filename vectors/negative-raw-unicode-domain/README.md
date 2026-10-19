# negative-raw-unicode-domain

**Origin:** IDN review. Both DID Core `idchar` and the did:webvh
`encoded-domain-label` ABNF exclude raw non-ASCII characters. The
internationalised-domain example in the v1.0 specification uses this invalid
form.

**Expected error:** `invalidDid`, before any network request.

**Generator notes:** `resolve-did` only; use the valid 46-character SCID so
rejection cannot be attributed to an invalid SCID.
