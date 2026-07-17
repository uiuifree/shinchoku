# shinchoku

[![ci](https://github.com/uiuifree/shinchoku/actions/workflows/ci.yml/badge.svg)](https://github.com/uiuifree/shinchoku/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/shinchoku.svg)](https://crates.io/crates/shinchoku)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**shinchoku** (進捗, *"progress"*) — a tiny NDJSON protocol for batch CLIs to
report their progress.

日本語のガイドは [docs/ja/](https://uiuifree.github.io/shinchoku/ja/)（またはリポジトリ内 `docs/ja/index.html`）へ。

```
$ your-cli --out result.jsonl
{"v":1,"event":"start","title":"import","total":120}
{"v":1,"event":"progress","current":3,"total":120}
{"v":1,"event":"log","level":"warn","msg":"row 42 skipped"}
{"v":1,"event":"artifact","path":"result.jsonl"}
{"v":1,"event":"done","summary":"4,520 items"}
```

A CLI that writes these lines to stdout can be driven by any observer — a GUI
shell, a scheduler, a log collector — that draws progress bars, sends failure
notifications and keeps run history **without knowing anything about the
domain** of the work. Think of it as the tracking barcode on a parcel: every
parcel carries different contents, but one tracking screen works for all of
them, because the *status format* is shared — not the contents.

- **Pipe-native.** stdout is the transport. Anything that carries lines —
  a pipe, SSH, a WebSocket relay — carries the protocol unchanged.
- **printf-compliant.** `printf '{"v":1,"event":"done"}\n'` is a valid
  producer. The libraries here are convenience, not the contract.
- **Tolerant by contract.** Non-JSON lines are plain logs. Unknown events and
  fields are ignored. A legacy tool that logs plain text is already a valid
  producer — it just doesn't get a progress bar yet.
- **Seven events.** `start` `progress` `log` `metric` `artifact` `done`
  `failed`. That's the whole vocabulary. Domain words (URLs, filenames,
  metric names) travel as *values*, never as vocabulary.

The full contract lives in **[spec/SPEC.md](spec/SPEC.md)** (status: v1 draft)
with a JSON Schema in [spec/schema/](spec/schema/). The spec is the single
source of truth; implementations follow it.

## Why not OpenTelemetry?

Different question. OpenTelemetry answers *"what is happening across my
services?"* — traces, metrics and logs, pushed over the network to a collector
for later analysis. shinchoku answers *"how far along is this one process?"* —
narrated on stdout to whoever spawned it.

- **Progress is first-class.** OTel has no `current/total`, no `artifact`, no
  terminal `done`/`failed` — and no standard consumer that would draw a
  progress bar from a span.
- **The transport points the other way.** OTLP pushes to a network endpoint
  you must configure. shinchoku writes to stdout and is read by the parent
  process — no endpoint, no network, unchanged over a pipe or SSH.
- **Nothing to install.** OTel needs an SDK, an exporter and usually a
  collector. A shinchoku producer can be `printf`; a consumer is a tolerant
  NDJSON parser.
- **Crashes can't lose the verdict.** Push telemetry drops the tail when a
  process dies. Here the parent holds the exit code, and
  ["the exit code decides"](#consumer-rules-the-short-version).

They compose rather than compete: a consumer may forward events into an OTel
pipeline. In the parcel metaphor — shinchoku is the tracking barcode;
OpenTelemetry is the logistics company's back office. See
[SPEC §10](spec/SPEC.md#10-non-goals) for the normative scope statement.

## Nothing here is new — on purpose

shinchoku combines four old, proven ideas and adds none of its own:

- **From test harness protocols** — a process reports by writing plain lines
  to stdout, and the reader parses them tolerantly. Decades of proof behind
  the mechanism; but there the vocabulary is bound to pass/fail.
- **From CI runners' status lines** — progress, artifacts and annotations
  emitted as specially formatted lines that a runner turns into UI. The right
  vocabulary; but each dialect belongs to one vendor's runner.
- **From tools' `--json` modes** — machine-readable events streamed one JSON
  line at a time. The right transport; but each tool invents a private
  vocabulary, so no consumer works across tools.
- **From process supervisors** — the child narrates its state, the parent
  holds the verdict.
  ["The exit code decides"](#consumer-rules-the-short-version) comes straight
  from this tradition.

The only contribution is the combination: the same old transport, one
tolerant, domain-neutral vocabulary — so a single consumer can finally serve
every tool.

## Quickstart

### Rust

```toml
[dependencies]
shinchoku = "0.1"
```

```rust
use shinchoku::Event;

Event::start().title("import").total(120).emit();
Event::progress(3).total(120).emit();
Event::done().summary("4,520 items").emit();
```

Consumers enable the `parse` feature and get a tolerant line parser:

```rust
use shinchoku::{parse_line, Line};

match parse_line(&line) {
    Line::Event(event) => update_ui(event),
    Line::Text(text) => show_as_log(text), // non-JSON / unknown events
}
```

### Go

```go
import shinchoku "github.com/uiuifree/shinchoku/go"

shinchoku.Start("import", 120)
shinchoku.Progress(3, 120)
shinchoku.Done("4,520 items")
```

### Node

```js
import * as shinchoku from "shinchoku";

shinchoku.start({ title: "import", total: 120 });
shinchoku.progress(3, 120);
shinchoku.done("4,520 items");
```

### PHP

```php
use Shinchoku\Shinchoku;

Shinchoku::start('import', 120);
Shinchoku::progress(3, 120);
Shinchoku::done('4,520 items');
```

### Python

```python
import shinchoku

shinchoku.start(title="import", total=120)
shinchoku.progress(3, total=120)
shinchoku.done("4,520 items")
```

## For AI coding agents

The protocol is sized for AI adoption: the whole spec fits in one context
window, and a compliant producer needs no library — an agent can add progress
reporting to any CLI without new dependencies. Paste this into your agent's
instructions (`CLAUDE.md`, `AGENTS.md`, a system prompt):

> When you write a batch CLI, make it report progress with the shinchoku
> protocol: print one JSON object per line to stdout and flush after each
> line. Emit `{"v":1,"event":"start","title":"<run name>","total":<units>}`
> first; `{"v":1,"event":"progress","current":<n>,"total":<units>}` as work
> advances; `{"v":1,"event":"log","level":"info|warn|error","msg":"..."}` for
> logs; `{"v":1,"event":"metric","key":"...","value":...}` for named
> counters; `{"v":1,"event":"artifact","path":"..."}` when an output file is
> written; and end with `{"v":1,"event":"done","summary":"..."}` on success
> or `{"v":1,"event":"failed","msg":"..."}` plus a non-zero exit code on
> fatal error. The run's data output goes to a file or another stream, never
> stdout. Put domain words in values, never in event names. Full spec:
> https://shinchoku.app/llms.txt

Agents browsing the web get the same pointers from
[shinchoku.app/llms.txt](https://shinchoku.app/llms.txt), and emitted lines
can be machine-validated against
[the JSON Schema](https://raw.githubusercontent.com/uiuifree/shinchoku/master/spec/schema/shinchoku.schema.json).

## Consumer rules (the short version)

1. A line that isn't valid JSON → show it as a plain log. Never an error.
2. Unknown `event` → ignore or show as a log. Never an error.
3. Unknown fields → ignore.
4. stderr → plain logs.
5. **The exit code decides.** Non-zero exit with no `failed` seen → synthesize
   a failure. The protocol narrates the run; the exit code judges it.

These rules are what make additions compatible: new events and fields never
break an old consumer. See [SPEC §6–7](spec/SPEC.md#6-consumer-requirements).

## Versioning and stability

Two version numbers exist, on purpose:

- **The protocol** — every line carries `"v":1`. `v` bumps only on a breaking
  change, which this spec intends never to make; additions don't bump it.
  The spec is currently a **draft**: it may still change based on real-world
  use before being frozen.
- **The libraries** — each helper follows its own semver, and **all of them
  are 0.x: breaking API changes may happen in any 0.y release until 1.0.**
  Pin a minor version if you need stability today. Each release documents
  which protocol version it speaks.

## Repository layout

```
spec/    the contract (SPEC.md + JSON Schema) — the single source of truth
rust/    crate  `shinchoku`         (features: emit [default], parse)
go/      module `github.com/uiuifree/shinchoku/go`
node/    npm    `shinchoku`         (ESM, zero-dependency)
php/     composer `shinchoku/shinchoku` (PSR-4, zero-dependency)
python/  PyPI   `shinchoku`         (zero-dependency)
docs/    documentation site (GitHub Pages)
```

Reserved for future protocol extensions (not implemented): `ask`
(bidirectional prompts), `row` (streamed tabular data), `heartbeat`
(long-running services). See [SPEC §7](spec/SPEC.md#7-versioning-and-extension).

## License

[MIT](LICENSE)
