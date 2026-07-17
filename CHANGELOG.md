# Changelog

All notable changes to this repository are documented here, per package.
Until 1.0, every 0.x release may contain breaking changes (see README
"Versioning and stability").

## Unreleased

### spec
- v1 (draft): initial vocabulary — `start`, `progress`, `log`, `metric`,
  `artifact`, `done`, `failed`; envelope (`v`, `event`); producer/consumer
  rules; reserved names `ask`, `row`, `heartbeat`.

### rust 0.1.0
- Initial release. Builder-style emitters (`emit` feature, default) and a
  tolerant line parser (`parse` feature). Speaks protocol v1. MSRV 1.70.

### go, node, php, python
- Initial implementations (unpublished; live in-repo). Speak protocol v1.
