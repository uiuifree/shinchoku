# Contributing

Thank you for your interest. A few rules keep this project coherent:

## The spec is the source of truth

[`spec/SPEC.md`](spec/SPEC.md) defines the protocol; the language helpers
follow it. **Any change to the wire format starts as a spec PR**, not a code
PR. Once the spec change is agreed, all five helpers (rust / go / node / php /
python) are updated in the same PR — no helper may get ahead of or fall behind
the spec.

Compatible changes are additions only: new events, new optional fields.
Renaming or removing anything is a breaking change and is intended never to
happen (see SPEC §7).

## Running the tests

```
cd rust   && cargo test --all-features
cd go     && go test ./...
cd node   && node --test
cd php    && php tests/run.php
cd python && python -m unittest discover -s tests
```

All five suites assert the exact same expected lines. If you change expected
output in one language, change it in all five.

## Scope

This project stays small on purpose. The helpers are ~100-line emitters —
convenience, not frameworks. Features that grow the vocabulary beyond what a
domain-agnostic consumer can render (see SPEC §4.4: "the key is data, not
vocabulary") are out of scope, as are transport layers, schedulers and UIs:
those belong to consumers built *on* the protocol, not to the protocol.

## Reporting issues

Bug reports with a reproducing NDJSON line (or the line you expected) are the
most useful kind. For protocol questions, quote the SPEC section.
