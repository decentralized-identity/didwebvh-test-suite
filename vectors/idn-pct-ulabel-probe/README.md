# idn-pct-ulabel-probe

**Interoperability probe, not a conformance test.** did:webvh v1.0 allows
percent-encoded U-labels but does not specify a Unicode normalisation form or
IDNA profile. The comparison targets assume NFC and non-transitional UTS #46
ToASCII. A different captured URL or a pre-fetch rejection is evidence for
discussion, **not** a conformance failure.

Compare the URL at the library-to-HTTP-client boundary, before client-specific
IDNA conversion. The HTTP request is intercepted; no network is used.
