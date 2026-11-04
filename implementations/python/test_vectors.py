"""
Compliance test harness: runs each committed vector through the Python did_webvh
resolver and compares results.

Parametrized over (scenario, impl, result_file) so every implementation's
committed logs are tested against the Python resolver.

Normalization functions labelled "TS COMPAT" compensate for known differences
between what the TypeScript generator writes into vectors/ and what the Python
library produces.  Each one is a candidate for community discussion — see the
inline notes for context.

Known hard failures are marked xfail rather than skipped so the suite stays
green while the incompatibilities are being resolved upstream.
"""

import asyncio
import json
import pytest
from pathlib import Path

import aiohttp
import jcs
import yaml

from did_webvh.resolver import resolve_did

VECTORS_ROOT = Path(__file__).parent.parent.parent / "vectors"


# ---------------------------------------------------------------------------
# Test collection helpers
# ---------------------------------------------------------------------------

def _collect_cases() -> list[tuple[str, str, str]]:
    """Return (scenario_name, impl_name, result_filename) for all impl vectors."""
    cases = []
    for vdir in sorted(VECTORS_ROOT.iterdir()):
        if not vdir.is_dir():
            continue
        if vdir.name.startswith("negative-"):
            continue
        for impl_dir in sorted(vdir.iterdir()):
            if not impl_dir.is_dir():
                continue
            if not (impl_dir / "did.jsonl").exists():
                continue
            for rf in sorted(impl_dir.glob("resolutionResult*.json")):
                cases.append((vdir.name, impl_dir.name, rf.name))
    return cases


def _collect_negative_cases() -> list[tuple[str, str]]:
    """Return (scenario_name, expected_error) for all negative test vectors with a ts/ log.

    URL-only scenarios whose resolve-did steps are checked by
    test_resolve_did_step are excluded so they are not reported twice.
    """
    url_scenarios = {scenario for scenario, _ in _collect_resolve_did_steps()}
    cases = []
    for vdir in sorted(VECTORS_ROOT.iterdir()):
        if not vdir.is_dir() or not vdir.name.startswith("negative-"):
            continue
        if vdir.name in url_scenarios:
            continue
        ts_dir = vdir / "ts"
        jsonl = ts_dir / "did.jsonl"
        result_json = ts_dir / "resolutionResult.json"
        if not jsonl.exists() or not result_json.exists():
            continue
        expected_error = json.loads(result_json.read_text()).get(
            "didResolutionMetadata", {}
        ).get("error", "?")
        cases.append((vdir.name, expected_error))
    return cases


def _load_script(vdir: Path) -> dict | None:
    script_path = vdir / "script.yaml"
    if not script_path.exists():
        return None
    return yaml.safe_load(script_path.read_text()) or {}


def _has_ts_log(vdir: Path) -> bool:
    log = vdir / "ts" / "did.jsonl"
    return log.exists() and bool(log.read_text().strip())


def _is_resolve_did_assertion(step: dict, has_log: bool) -> bool:
    """True for resolve-did steps checked by test_resolve_did_step.

    expectError steps in scenarios with a non-empty ts/ log are exercised by
    test_negative_resolution against that log instead.
    """
    if not isinstance(step, dict) or step.get("op") != "resolve-did":
        return False
    if "expectFetchUrl" in step:
        return True
    return "expectError" in step and not has_log


def _collect_resolve_did_steps() -> list[tuple[str, int]]:
    """Return (scenario_name, 1-based step number) for resolve-did assertions."""
    cases = []
    for vdir in sorted(VECTORS_ROOT.iterdir()):
        if not vdir.is_dir():
            continue
        script = _load_script(vdir)
        if not script:
            continue
        has_log = _has_ts_log(vdir)
        for index, step in enumerate(script.get("steps") or []):
            if _is_resolve_did_assertion(step, has_log):
                cases.append((vdir.name, index + 1))
    return cases


def _version_number_from_stem(stem: str) -> int | None:
    """Extract the version number from a filename stem like 'resolutionResult.2'."""
    parts = stem.split(".")
    if len(parts) > 1:
        try:
            return int(parts[1])
        except ValueError:
            pass
    return None



# ---------------------------------------------------------------------------
# Parametrised test
# ---------------------------------------------------------------------------

@pytest.fixture
def _status_data(request):
    """Passes actual/expected to conftest.py for diff generation in status files."""
    data: dict = {}
    request.node._status_data = data
    return data


@pytest.mark.parametrize("scenario,impl,result_file", _collect_cases())
def test_vector(scenario: str, impl: str, result_file: str, _status_data: dict) -> None:
    log = VECTORS_ROOT / scenario / impl / "did.jsonl"
    expected = json.loads((VECTORS_ROOT / scenario / impl / result_file).read_text())

    did = json.loads(log.read_text().splitlines()[0])["state"]["id"]

    version_number = _version_number_from_stem(Path(result_file).stem)
    kwargs = {"version_number": version_number} if version_number is not None else {}

    result = asyncio.run(resolve_did(did, local_history=log, **kwargs))
    actual = result.serialize()

    _status_data["actual"] = actual
    _status_data["expected"] = expected
    assert jcs.canonicalize(actual) == jcs.canonicalize(expected)


# ---------------------------------------------------------------------------
# Negative resolution tests
# ---------------------------------------------------------------------------

@pytest.mark.parametrize("scenario,expected_error", _collect_negative_cases())
def test_negative_resolution(scenario: str, expected_error: str, _status_data: dict) -> None:
    """Resolver must reject each negative test vector.

    PASS = resolver returned an error or raised an exception.
    FAIL = resolver accepted the invalid log (conformance bug).
    """
    log = VECTORS_ROOT / scenario / "ts" / "did.jsonl"
    log_text = log.read_text().strip()

    if not log_text:
        _status_data["neg_scenario"] = scenario
        _status_data["neg_expected_error"] = expected_error
        _status_data["neg_outcome"] = "skip"
        pytest.skip("URL-only test (no log)")

    did = json.loads(log_text.splitlines()[0])["state"]["id"]

    try:
        result = asyncio.run(resolve_did(did, local_history=log))
        actual_error = result.serialize().get("didResolutionMetadata", {}) or {}
        if actual_error.get("error"):
            # Resolver returned an error — correct behaviour.
            _status_data["neg_outcome"] = "pass"
            _status_data["neg_scenario"] = scenario
            _status_data["neg_expected_error"] = expected_error
        else:
            # Resolver succeeded — it should have rejected this log.
            _status_data["neg_outcome"] = "fail"
            _status_data["neg_scenario"] = scenario
            _status_data["neg_expected_error"] = expected_error
            pytest.fail("resolver accepted invalid log (should have returned an error)")
    except Exception:
        # Any exception counts as rejection — correct behaviour.
        _status_data["neg_outcome"] = "pass"
        _status_data["neg_scenario"] = scenario
        _status_data["neg_expected_error"] = expected_error


# ---------------------------------------------------------------------------
# resolve-did script steps (DID-to-HTTPS URL assertions and probes)
# ---------------------------------------------------------------------------

class _FetchIntercepted(Exception):
    """Raised in place of the HTTP request so no network access occurs."""


async def _resolve_capturing_fetch(did: str) -> tuple[str | None, str | None, str | None]:
    """Resolve ``did`` with aiohttp requests intercepted.

    Returns (fetched_url, error_code, exception). The URL is the argument exactly as the
    library passed it to ``aiohttp.ClientSession.get``, before aiohttp converts
    it to a ``yarl.URL`` (which would re-encode / IDNA-normalise the host).
    """
    fetched: list[str] = []

    def _get(self, url, *args, **kwargs):
        fetched.append(str(url))
        raise _FetchIntercepted(str(url))

    async def _request(self, method, str_or_url, *args, **kwargs):
        # Backstop for any request path that bypasses ClientSession.get.
        if not fetched:
            fetched.append(str(str_or_url))
        raise _FetchIntercepted(str(str_or_url))

    mp = pytest.MonkeyPatch()
    mp.setattr(aiohttp.ClientSession, "get", _get)
    mp.setattr(aiohttp.ClientSession, "_request", _request)
    error: str | None = None
    exception: str | None = None
    try:
        result = await resolve_did(did)
        metadata = result.serialize().get("didResolutionMetadata") or {}
        error = metadata.get("error")
    except _FetchIntercepted:
        pass
    except Exception as exc:
        exception = f"{type(exc).__name__}: {exc}"
    finally:
        mp.undo()
    return (fetched[0] if fetched else None), error, exception


def _evaluate_resolve_did_step(step: dict) -> tuple[str, str, str, str | None]:
    """Return (outcome, expected, notes, diff) for a resolve-did step.

    outcome is one of "pass", "diff", "fail", "skip".
    """
    did = step.get("did")
    expected_url = step.get("expectFetchUrl")
    expected_error = step.get("expectError")
    expected = str(expected_url if expected_url is not None else expected_error)
    if not isinstance(did, str):
        return "fail", expected, "resolve-did step has no did", None
    if expected_url is not None and expected_error is not None:
        return "fail", expected, "step specifies both expectFetchUrl and expectError", None
    if not isinstance(expected_url or expected_error, str):
        return "fail", expected, "expected URL or error must be a string", None

    fetched_url, error, exception = asyncio.run(_resolve_capturing_fetch(did))
    rejection = error or exception or "no error reported"

    if expected_url is not None:
        if fetched_url == expected_url:
            return "pass", expected, f"fetch URL: {fetched_url}", None
        if fetched_url is not None:
            return ("diff", expected, f"expected {expected_url}; got {fetched_url}",
                    f"expected: {expected_url}\nactual:   {fetched_url}")
        return ("fail", expected, f"rejected before fetch: {rejection}",
                f"expected: {expected_url}\nactual:   (no fetch; {rejection})")

    # expectError
    if fetched_url is not None:
        if expected_error != "invalidDid":
            return ("skip", expected,
                    f"{expected_error} requires resolution beyond URL construction", None)
        return ("fail", expected, f"resolver fetched URL: {fetched_url}",
                f"expected: error {expected_error}\nactual:   fetched {fetched_url}")
    if error == expected_error:
        return "pass", expected, "rejected before fetch", None
    if error is None and exception in {
        "ValueError: Invalid DID URL",
        "ValueError: Invalid DID",
        "ValueError: Invalid method-specific ID",
        "ValueError: Invalid domain name in method-specific ID",
        "ValueError: Invalid path in method-specific ID",
        "ValueError: Invalid port specification",
    } and expected_error == "invalidDid":
        # This implementation raises these parser errors instead of returning
        # invalidDid resolution metadata.
        return "pass", expected, f"rejected before fetch: {exception}", None
    return ("diff", expected, f"expected {expected_error}; got {rejection}",
            f"expected: error {expected_error}\nactual:   {rejection}")


@pytest.mark.parametrize("scenario,step_number", _collect_resolve_did_steps())
def test_resolve_did_step(scenario: str, step_number: int, _status_data: dict) -> None:
    """Check the URL a resolve-did step fetches (expectFetchUrl) or that it is
    rejected before any fetch (expectError), without network access.

    Steps in a scenario (or individual steps) marked ``probe: true`` are
    reported separately and never fail the run.
    """
    script = _load_script(VECTORS_ROOT / scenario) or {}
    step = (script.get("steps") or [])[step_number - 1]
    probe = bool(script.get("probe")) or bool(step.get("probe"))

    outcome, expected, notes, diff = _evaluate_resolve_did_step(step)
    if probe and outcome == "fail":
        outcome = "diff"
    _status_data.update({
        "url_case": f"{scenario}/resolve-did/{step_number}",
        "url_expected": expected,
        "url_outcome": outcome,
        "url_notes": notes,
        "url_diff": diff,
        "url_probe": probe,
    })

    if outcome == "skip":
        pytest.skip(notes)
    if outcome in ("diff", "fail") and not probe:
        pytest.fail(f"{outcome.upper()}: {notes}")
