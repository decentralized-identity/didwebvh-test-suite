# idn-did-to-https

Check the URL handed to the HTTP client for an A-label DID. The domain is
already an ASCII DNS name; step 4 of the v1.0 transformation requires
percent-encoded path segments to be decoded and re-encoded with uppercase
hexadecimal digits. The HTTP request is intercepted and no network is used.
