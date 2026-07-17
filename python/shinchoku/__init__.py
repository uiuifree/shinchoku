"""shinchoku (進捗, "progress") — a tiny NDJSON protocol for batch CLIs to
report their progress. One JSON object per line on stdout.

The contract is the spec (spec/SPEC.md in the repository), not this package:
``print('{"v":1,"event":"done"}')`` is fully compliant. This package is the
convenience helper for Python producers.

    import shinchoku

    shinchoku.start(title="import", total=120)
    shinchoku.progress(3, total=120)
    shinchoku.done("4,520 items")
"""

from __future__ import annotations

import json
import sys
from typing import Any, Protocol, Union

#: Protocol version stamped into every line as ``"v"``.
VERSION = 1


class Output(Protocol):
    """Anything with ``write(text)`` and ``flush()`` (e.g. a text stream)."""

    def write(self, text: str) -> object: ...

    def flush(self) -> object: ...


_out: Output = sys.stdout


def set_output(stream: Output) -> None:
    """Redirect event output (default: ``sys.stdout``).

    Intended for tests and for consumers that relay events over another
    transport.
    """
    global _out
    _out = stream


def _emit(fields: dict[str, Any]) -> None:
    """Emit one NDJSON line with ``"v"`` first, dropping ``None`` fields.

    Best-effort: write errors (e.g. the reader went away) are ignored,
    matching the fire-and-forget nature of reporting.
    """
    pruned = {key: value for key, value in fields.items() if value is not None}
    line = json.dumps(
        {"v": VERSION, **pruned}, ensure_ascii=False, separators=(",", ":")
    )
    try:
        _out.write(line + "\n")
        _out.flush()
    except (OSError, ValueError):
        pass


def start(title: Union[str, None] = None, total: Union[int, None] = None) -> None:
    """Announce the run. Should be the first event."""
    _emit({"event": "start", "title": title, "total": total})


def progress(
    current: int,
    total: Union[int, None] = None,
    msg: Union[str, None] = None,
) -> None:
    """Report ``current`` completed units. ``total`` and ``msg`` are optional."""
    _emit({"event": "progress", "current": current, "total": total, "msg": msg})


def log(level: str, msg: str) -> None:
    """Emit a log line with a severity (``"info"`` | ``"warn"`` | ``"error"``)."""
    _emit({"event": "log", "level": level, "msg": msg})


def info(msg: str) -> None:
    """``log("info", msg)``"""
    log("info", msg)


def warn(msg: str) -> None:
    """``log("warn", msg)``"""
    log("warn", msg)


def error(msg: str) -> None:
    """``log("error", msg)`` — non-fatal; a fatal error is :func:`failed`."""
    log("error", msg)


def metric(key: str, value: Union[int, float, str]) -> None:
    """Emit a named value for the consumer to display verbatim.

    The key is data, not vocabulary.
    """
    _emit({"event": "metric", "key": key, "value": value})


def artifact(path: str, label: Union[str, None] = None) -> None:
    """Declare that an output file exists. ``label`` is optional."""
    _emit({"event": "artifact", "path": path, "label": label})


def done(summary: Union[str, None] = None) -> None:
    """Report successful completion. ``summary`` is optional."""
    _emit({"event": "done", "summary": summary})


def failed(msg: str) -> None:
    """Report fatal failure; the run is over."""
    _emit({"event": "failed", "msg": msg})
