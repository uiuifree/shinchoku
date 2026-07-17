# shinchoku

**shinchoku** (進捗, *"progress"*) — a tiny NDJSON protocol for batch CLIs to
report their progress. This crate is the Rust emitter/parser; the contract
itself lives in the
[spec](https://github.com/uiuifree/shinchoku/blob/master/spec/SPEC.md).

```rust
use shinchoku::Event;

Event::start().title("import").total(120).emit();
Event::progress(3).total(120).emit();
Event::done().summary("4,520 items").emit();
```

```text
{"event":"start","title":"import","total":120,"v":1}
{"current":3,"event":"progress","total":120,"v":1}
{"event":"done","summary":"4,520 items","v":1}
```

Any observer that speaks the protocol — a GUI shell, a scheduler, a log
collector — can draw the bar, send the failure notification and keep the
history, without knowing anything about your domain.

## Features

- `emit` *(default)* — producer side: build events and write them to stdout.
- `parse` — consumer side: [`parse_line`] tolerantly parses producer output
  (non-JSON lines and unknown events become `Line::Text`, never an error).

## Stability

**This crate is 0.x: breaking API changes may happen in any 0.y release until
1.0.** The wire protocol has its own version (`"v":1`, currently a draft) and
evolves additively; each crate release documents which protocol version it
speaks. Pin a minor version (`shinchoku = "0.1"`) if you need stability today.

MIT license · [repository](https://github.com/uiuifree/shinchoku) ·
[documentation site](https://uiuifree.github.io/shinchoku/)
