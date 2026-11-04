# idn-alabel-create

Supply the ASCII A-labels for `jp納豆.例.jp` to each controller, then
cross-resolve the logs they create. A-label DIDs are valid under did:webvh
v1.0. A controller that emits the given A-labels will produce a `state.id`
domain of `xn--jp-cd2fp15c.xn--fsq.jp`; do not treat a controller's different,
v1.0-valid domain encoding as a conformance failure. Report its cross-resolve
differences for discussion. This vector does **not** prescribe what form a
controller must emit from Unicode input.
