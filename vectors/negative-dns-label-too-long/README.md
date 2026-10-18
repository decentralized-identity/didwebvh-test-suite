# negative-dns-label-too-long

**Origin:** IDN review. The first domain label is 64 ASCII octets; did:webvh
v1.0 limits a DNS label to 63 octets after IDNA processing.

**Expected error:** `invalidDid`, before any network request.

**Generator notes:** `resolve-did` only, with a valid SCID so the domain is
the reason for rejection.
