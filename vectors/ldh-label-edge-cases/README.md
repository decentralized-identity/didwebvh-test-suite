# ldh-label-edge-cases

RFC 1123 allows single-character and digit-first labels. The did:webvh v1.0
specification does not prohibit them. Check the URL before the HTTP client is
called; do not access the network.
