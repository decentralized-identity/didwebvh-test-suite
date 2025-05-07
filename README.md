# didwebvh-test-suite

An initial test suite for [`did:webvh`](https://github.com/decentralized-identity/didwebvh). Only resolver tests are currently included, and more are needed.

This repo currently includes a script for running the tests against `didwebvh-py` as an example.

The script `gen_resolver_suite.py` is used to generate the test suite, placing output inside of the `test-suite` subdirectory. To add new tests, edit this script and run it again (the `didwebvh` Python library is required).

## License

[Apache License Version 2.0](LICENSE)
