package shinchoku

import (
	"bytes"
	"os"
	"testing"
)

// capture redirects output to a buffer for the duration of fn.
func capture(t *testing.T, fn func()) string {
	t.Helper()
	var buf bytes.Buffer
	SetOutput(&buf)
	defer SetOutput(os.Stdout)
	fn()
	return buf.String()
}

func TestStartOmitsAbsentFields(t *testing.T) {
	got := capture(t, func() { Start("import", 120) })
	want := `{"event":"start","title":"import","total":120,"v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}

	got = capture(t, func() { Start("", 0) })
	want = `{"event":"start","v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestProgressKeepsCurrentZero(t *testing.T) {
	got := capture(t, func() { Progress(0, 10) })
	want := `{"current":0,"event":"progress","total":10,"v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestProgressMsg(t *testing.T) {
	got := capture(t, func() { ProgressMsg(3, 120, "page 3") })
	want := `{"current":3,"event":"progress","msg":"page 3","total":120,"v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestLogLevels(t *testing.T) {
	got := capture(t, func() { Warn("row 42 skipped") })
	want := `{"event":"log","level":"warn","msg":"row 42 skipped","v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestMetricNumberAndString(t *testing.T) {
	got := capture(t, func() { Metric("items", 4520) })
	want := `{"event":"metric","key":"items","v":1,"value":4520}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}

	got = capture(t, func() { Metric("stage", "detail") })
	want = `{"event":"metric","key":"stage","v":1,"value":"detail"}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestArtifactDoneFailed(t *testing.T) {
	got := capture(t, func() { Artifact("out.jsonl", "Save JSONL") })
	want := `{"event":"artifact","label":"Save JSONL","path":"out.jsonl","v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}

	got = capture(t, func() { Done("4,520 items") })
	want = `{"event":"done","summary":"4,520 items","v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}

	got = capture(t, func() { Failed("boom") })
	want = `{"event":"failed","msg":"boom","v":1}` + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}
