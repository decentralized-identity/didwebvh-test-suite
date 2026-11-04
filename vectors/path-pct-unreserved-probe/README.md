# path-pct-unreserved-probe

**Interoperability probe, not a conformance test.** Step 2 of the v1.0
DID-to-HTTPS transformation decodes each path segment and re-encodes it
"according to [RFC3986], using uppercase hexadecimal digits". DID Core `idchar`
does not allow a raw `~`, so a DID must carry it as `%7E`. The specification
does not say whether the HTTPS URL keeps `%7E` or restores `~`.

The comparison targets follow RFC 3986 sections 2.3 and 6.2.2.2: unreserved
characters (`ALPHA`, `DIGIT`, `-`, `.`, `_`, `~`) are not percent-encoded in
the URL. A URL that keeps them encoded, such as `/a%7Eb/`, is evidence for
discussion, **not** a conformance failure.

The HTTP request is intercepted and no network is used.
