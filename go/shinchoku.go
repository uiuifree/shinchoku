// Package shinchoku emits shinchoku (進捗, "progress") protocol events:
// a tiny NDJSON protocol for batch CLIs to report their progress on stdout.
//
// The contract is the spec (spec/SPEC.md in the repository), not this package:
// printing `{"v":1,"event":"done"}` yourself is fully compliant. This package
// is the convenience helper for Go producers.
//
//	shinchoku.Start("import", 120)
//	shinchoku.Progress(3, 120)
//	shinchoku.Done("4,520 items")
package shinchoku

import (
	"encoding/json"
	"io"
	"os"
	"sync"
)

// Version is the protocol version stamped into every line as "v".
const Version = 1

// Level is the severity of a Log event.
type Level string

const (
	LevelInfo  Level = "info"
	LevelWarn  Level = "warn"
	LevelError Level = "error"
)

var (
	mu  sync.Mutex
	out io.Writer = os.Stdout
)

// SetOutput redirects event output (default: os.Stdout). Intended for tests
// and for consumers that relay events over another transport.
func SetOutput(w io.Writer) {
	mu.Lock()
	defer mu.Unlock()
	out = w
}

// emit marshals fields (plus "v") as one NDJSON line, best-effort: write
// errors are ignored, matching the fire-and-forget nature of reporting.
// json.Marshal on a map sorts keys, so output is deterministic.
func emit(fields map[string]any) {
	fields["v"] = Version
	line, err := json.Marshal(fields)
	if err != nil {
		return
	}
	mu.Lock()
	defer mu.Unlock()
	_, _ = out.Write(append(line, '\n'))
}

// Start announces the run. Empty title / zero total are omitted.
func Start(title string, total uint64) {
	f := map[string]any{"event": "start"}
	if title != "" {
		f["title"] = title
	}
	if total > 0 {
		f["total"] = total
	}
	emit(f)
}

// Progress reports current completed units. Zero total means unknown.
func Progress(current, total uint64) {
	ProgressMsg(current, total, "")
}

// ProgressMsg is Progress with a short human-readable detail (URL, filename…).
func ProgressMsg(current, total uint64, msg string) {
	f := map[string]any{"event": "progress", "current": current}
	if total > 0 {
		f["total"] = total
	}
	if msg != "" {
		f["msg"] = msg
	}
	emit(f)
}

// Log emits a log line with a severity.
func Log(level Level, msg string) {
	emit(map[string]any{"event": "log", "level": string(level), "msg": msg})
}

// Info is Log(LevelInfo, msg).
func Info(msg string) { Log(LevelInfo, msg) }

// Warn is Log(LevelWarn, msg).
func Warn(msg string) { Log(LevelWarn, msg) }

// Error is Log(LevelError, msg) — a non-fatal error; a fatal one is Failed.
func Error(msg string) { Log(LevelError, msg) }

// Metric emits a named value (number or string) for the consumer to display
// verbatim. The key is data, not vocabulary.
func Metric(key string, value any) {
	emit(map[string]any{"event": "metric", "key": key, "value": value})
}

// Artifact declares that an output file exists. Empty label is omitted.
func Artifact(path, label string) {
	f := map[string]any{"event": "artifact", "path": path}
	if label != "" {
		f["label"] = label
	}
	emit(f)
}

// Done reports successful completion. Empty summary is omitted.
func Done(summary string) {
	f := map[string]any{"event": "done"}
	if summary != "" {
		f["summary"] = summary
	}
	emit(f)
}

// Failed reports fatal failure; the run is over.
func Failed(msg string) {
	emit(map[string]any{"event": "failed", "msg": msg})
}
