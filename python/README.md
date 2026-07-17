# shinchoku (Python)

**shinchoku** (進捗, *"progress"*) — a tiny NDJSON protocol for batch CLIs to
report their progress. This is the zero-dependency Python emitter; the
contract itself lives in the
[spec](https://github.com/uiuifree/shinchoku/blob/master/spec/SPEC.md).

```python
import shinchoku

shinchoku.start(title="import", total=120)
shinchoku.progress(3, total=120)
shinchoku.warn("row 42 skipped")
shinchoku.artifact("result.jsonl")
shinchoku.done("4,520 items")
```

```
{"v":1,"event":"start","title":"import","total":120}
{"v":1,"event":"progress","current":3,"total":120}
{"v":1,"event":"log","level":"warn","msg":"row 42 skipped"}
{"v":1,"event":"artifact","path":"result.jsonl"}
{"v":1,"event":"done","summary":"4,520 items"}
```

Any observer that speaks the protocol — a GUI shell, a scheduler, a log
collector — can now draw the bar, send the failure notification and keep the
history, without knowing anything about your domain.

MIT license · [repository](https://github.com/uiuifree/shinchoku) ·
[documentation](https://shinchoku.app/)
