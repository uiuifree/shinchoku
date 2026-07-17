# shinchoku protocol — Specification

**Status: v1 (draft)** — the wire format is stable-additive: fields and events may be
added, existing ones are never renamed or removed. This document is the single
source of truth for the protocol. Implementations (the `rust/`, `go/`, `node/`,
`php/` helpers in this repository) follow this document, not the other way around.

shinchoku (進捗, "progress") is a tiny NDJSON protocol that lets a batch CLI
report its progress on stdout, so that any observer — a GUI shell, a scheduler,
a log collector — can display, notify and record the run **without knowing
anything about the domain** of the work.

```
$ your-cli --out result.jsonl
{"v":1,"event":"start","title":"import","total":120}
{"v":1,"event":"progress","current":3,"total":120}
{"v":1,"event":"log","level":"warn","msg":"row 42 skipped"}
{"v":1,"event":"artifact","path":"result.jsonl"}
{"v":1,"event":"done","summary":"4,520 items"}
```

## 1. Terminology

- **Producer** — the process doing the work (a CLI, a batch job). It writes events.
- **Consumer** — the process observing the run (a GUI, a runner, a collector).
  It reads events.
- The key words MUST, SHOULD, MAY are to be interpreted as in RFC 2119.

## 2. Transport

- Events are written to the producer's **stdout**, one event per line,
  encoded as a single JSON object in UTF-8, terminated by `\n` (NDJSON).
- stdout is the *reporting* channel. Data output (the actual result of the work)
  MUST go elsewhere: a file, a different stream, a network call.
- **stderr is not part of the protocol.** Consumers SHOULD show stderr lines as
  plain logs.
- The transport is deliberately dumb: anything that can carry lines of text —
  a pipe, SSH, a WebSocket relay — can carry the protocol unchanged.

## 3. Envelope

Every line is a JSON object with two required fields:

| field | type | meaning |
|---|---|---|
| `v` | integer | protocol version. Currently `1`. |
| `event` | string | event type, see below. |

All other fields belong to the event type. Consumers MUST ignore unknown
fields, and MUST NOT fail on unknown event types (see §6).

## 4. Events

### 4.1 `start`

Announces the run. SHOULD be the first event.

| field | type | required | meaning |
|---|---|---|---|
| `title` | string | no | human-readable name of the run |
| `total` | integer | no | total number of work units, if known |

### 4.2 `progress`

Reports advancement. `current`/`total` are in **work units** (pages, files,
rows, seconds — whatever the producer counts; the consumer only needs the ratio).

| field | type | required | meaning |
|---|---|---|---|
| `current` | integer | yes | units completed so far |
| `total` | integer | no | total units, if known (overrides `start.total`) |
| `msg` | string | no | short human-readable detail (current URL, filename…) |

### 4.3 `log`

A log line with a severity.

| field | type | required | meaning |
|---|---|---|---|
| `level` | `"info"` \| `"warn"` \| `"error"` | yes | severity |
| `msg` | string | yes | the message |

`error` reports a non-fatal error (the run continues). A fatal error is `failed`.

### 4.4 `metric`

A named value the consumer may display as a counter/tile. The key is **data,
not vocabulary**: the consumer displays it verbatim and MUST NOT interpret it.

| field | type | required | meaning |
|---|---|---|---|
| `key` | string | yes | metric name, shown verbatim |
| `value` | number \| string | yes | metric value |

### 4.5 `artifact`

Declares that an output file exists. Consumers MAY offer it (open / save / upload).

| field | type | required | meaning |
|---|---|---|---|
| `path` | string | yes | path of the artifact, as visible to the producer |
| `label` | string | no | human-readable label for the offer (e.g. a button) |

### 4.6 `done`

The run finished successfully. MUST be emitted at most once, SHOULD be the
last event.

| field | type | required | meaning |
|---|---|---|---|
| `summary` | string | no | one-line result ("4,520 items") |

### 4.7 `failed`

The run failed fatally. MUST be emitted at most once, SHOULD be the last event.
`done` and `failed` are mutually exclusive.

| field | type | required | meaning |
|---|---|---|---|
| `msg` | string | yes | what went wrong |

## 5. Producer requirements

1. One event per line; no pretty-printing; flush after each line.
2. SHOULD emit `start` first and `done` or `failed` last.
3. MUST NOT put domain meaning into the vocabulary — domain words travel as
   *values* (`title`, `msg`, `metric.key`), never as event types or field names.
4. Anything else the producer prints to stdout that is not a protocol line is
   allowed — consumers will treat it as a plain log (§6.1). This makes adoption
   incremental: a legacy tool that logs plain text is already a valid producer.
5. A producer needs no library: `printf '{"v":1,"event":"done"}\n'` is compliant.
   The helpers in this repository are convenience, not the contract.

## 6. Consumer requirements

The consumer rules make the protocol *tolerant by contract*:

1. A line that is not valid JSON MUST be treated as a plain log line (info).
2. An object with an unknown `event` MUST be ignored or shown as a log —
   never an error. (This is what makes adding events a compatible change.)
3. Unknown fields on a known event MUST be ignored.
4. stderr output SHOULD be shown as plain logs.
5. **The exit code is authoritative.** If the process exits non-zero and no
   `failed` was seen, the consumer MUST synthesize a failure. If it exits zero
   without `done`, the consumer SHOULD treat the run as done without summary.
6. If `v` is greater than the consumer's supported version, the consumer
   SHOULD still process known events and MAY surface a version notice.

## 7. Versioning and extension

- `v` bumps **only** on a breaking change, which this spec intends never to make.
- Adding events or optional fields is a compatible change and does not bump `v`
  (guaranteed by rules §6.2 and §6.3).
- Reserved event names for future extensions: `ask` (bidirectional prompts),
  `row` (streamed tabular data), `heartbeat` (long-running services).
  Producers MUST NOT use these names with other meanings.

## 8. Relationship to exit codes

The protocol *narrates* the run; the exit code *decides* it. A consumer that
implements only §6.5 and treats every stdout line as a log is already a valid,
minimal consumer — progress bars, metrics and artifacts are progressive
enhancement on top.

## 9. Machine validation

[`schema/shinchoku.schema.json`](schema/shinchoku.schema.json) validates a
single event line. Note that per §6, a consumer is more tolerant than the
schema: the schema describes what producers SHOULD write, not what consumers
may reject.

## 10. Non-goals

shinchoku is not an observability stack, and does not try to become one:

- **No traces, spans or aggregation.** The unit of interest is one run of one
  process, narrated to the consumer that spawned it. Fleet-level questions
  (latency percentiles, cross-service correlation) belong to systems like
  OpenTelemetry. A consumer MAY bridge events into such a pipeline; the
  protocol itself stays ignorant of it.
- **No network transport.** The protocol is defined over stdout only (§2).
  Relaying lines over SSH, WebSockets or anything else that carries text is a
  consumer concern and needs no extra specification.
- **No structured result data.** The result of the work travels outside the
  protocol (§2): `artifact` points at it, `metric` summarizes it, neither
  replaces it.
