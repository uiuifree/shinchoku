# Changelog

All notable changes to this repository are documented here, per package.
Until 1.0, every 0.x release may contain breaking changes (see README
"Versioning and stability").

## Unreleased

### spec
- v1 (draft): initial vocabulary — `start`, `progress`, `log`, `metric`,
  `artifact`, `done`, `failed`; envelope (`v`, `event`); producer/consumer
  rules; reserved names `ask`, `row`, `heartbeat`.
- v1 (draft): added §10 Non-goals — scope statement vs. observability stacks
  (OpenTelemetry et al.); no traces/aggregation, no network transport, no
  result data.

### rust 0.1.0
- Initial release. Builder-style emitters (`emit` feature, default) and a
  tolerant line parser (`parse` feature). Speaks protocol v1. MSRV 1.70.

### rust (unreleased)
- MSRV raised from 1.70 to 1.71: the serde / proc-macro ecosystem
  (serde_json, zmij, quote, syn, unicode-ident) now requires rustc 1.71.

### go, node, php, python
- Initial implementations (unpublished; live in-repo). Speak protocol v1.
