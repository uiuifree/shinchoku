//! shinchoku (進捗, "progress") — a tiny NDJSON protocol for batch CLIs to
//! report their progress. One JSON object per line on stdout.
//!
//! The contract is the spec, not this crate: a producer that does
//! `printf '{"v":1,"event":"done"}\n'` is fully compliant. This crate is the
//! convenience helper for Rust producers (`emit` feature, default) and
//! consumers (`parse` feature).
//!
//! ```
//! use shinchoku::Event;
//!
//! Event::start().title("import").total(120).emit();
//! Event::progress(3).total(120).emit();
//! Event::done().summary("4,520 items").emit();
//! ```
//!
//! # Stability
//!
//! This crate is 0.x: **breaking API changes may happen in any 0.y release
//! until 1.0**. The wire protocol has its own version (`"v":1`) and evolves
//! additively; each crate release documents which protocol version it speaks.

#![forbid(unsafe_code)]

use serde::Serialize;
use std::io::Write;

#[cfg(feature = "parse")]
use serde::Deserialize;

/// Protocol version stamped into every line as `"v"`.
pub const VERSION: u8 = 1;

/// Severity for [`Event::Log`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "parse", derive(Deserialize))]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Info,
    Warn,
    Error,
}

/// A protocol event. One event = one NDJSON line.
///
/// Field semantics are defined by `spec/SPEC.md`; this enum mirrors it 1:1.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "parse", derive(Deserialize))]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    Start {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        total: Option<u64>,
    },
    Progress {
        current: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        total: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        msg: Option<String>,
    },
    Log {
        level: Level,
        msg: String,
    },
    Metric {
        key: String,
        /// number | string (the spec allows both; the key is data, not vocabulary)
        value: serde_json::Value,
    },
    Artifact {
        path: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    Done {
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<String>,
    },
    Failed {
        msg: String,
    },
}

impl Event {
    /// Begin building a `start` event.
    pub fn start() -> Start {
        Start {
            title: None,
            total: None,
        }
    }

    /// Begin building a `progress` event for `current` completed units.
    pub fn progress(current: u64) -> Progress {
        Progress {
            current,
            total: None,
            msg: None,
        }
    }

    /// A `log` event.
    pub fn log(level: Level, msg: impl Into<String>) -> Event {
        Event::Log {
            level,
            msg: msg.into(),
        }
    }

    /// A `metric` event. `value` may be a number or a string.
    pub fn metric(key: impl Into<String>, value: impl Into<serde_json::Value>) -> Event {
        Event::Metric {
            key: key.into(),
            value: value.into(),
        }
    }

    /// An `artifact` event pointing at an output file.
    pub fn artifact(path: impl Into<String>) -> Artifact {
        Artifact {
            path: path.into(),
            label: None,
        }
    }

    /// Begin building a `done` event.
    pub fn done() -> Done {
        Done { summary: None }
    }

    /// A `failed` event (fatal; the run is over).
    pub fn failed(msg: impl Into<String>) -> Event {
        Event::Failed { msg: msg.into() }
    }

    /// Serialize as one NDJSON line (with `"v"`) and write it to `w`, flushing.
    pub fn write_to<W: Write>(&self, mut w: W) -> std::io::Result<()> {
        let mut value = serde_json::to_value(self).expect("Event always serializes");
        value
            .as_object_mut()
            .expect("Event serializes to an object")
            .insert("v".to_string(), VERSION.into());
        serde_json::to_writer(&mut w, &value)?;
        w.write_all(b"\n")?;
        w.flush()
    }

    /// Emit to stdout, best-effort: IO errors (e.g. the reader went away) are
    /// ignored. Use [`Event::write_to`] when you need to observe them.
    pub fn emit(&self) {
        let _ = self.write_to(std::io::stdout().lock());
    }
}

/// Builder for [`Event::Start`].
#[derive(Debug, Clone)]
pub struct Start {
    title: Option<String>,
    total: Option<u64>,
}

impl Start {
    /// Human-readable name of the run.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Total number of work units, if known.
    pub fn total(mut self, total: u64) -> Self {
        self.total = Some(total);
        self
    }
    /// Finish building the event.
    pub fn build(self) -> Event {
        Event::Start {
            title: self.title,
            total: self.total,
        }
    }
    /// Build and emit to stdout (best-effort).
    pub fn emit(self) {
        self.build().emit();
    }
}

/// Builder for [`Event::Progress`].
#[derive(Debug, Clone)]
pub struct Progress {
    current: u64,
    total: Option<u64>,
    msg: Option<String>,
}

impl Progress {
    /// Total number of work units, if known (overrides `start.total`).
    pub fn total(mut self, total: u64) -> Self {
        self.total = Some(total);
        self
    }
    /// Short human-readable detail (current URL, filename…).
    pub fn msg(mut self, msg: impl Into<String>) -> Self {
        self.msg = Some(msg.into());
        self
    }
    /// Finish building the event.
    pub fn build(self) -> Event {
        Event::Progress {
            current: self.current,
            total: self.total,
            msg: self.msg,
        }
    }
    /// Build and emit to stdout (best-effort).
    pub fn emit(self) {
        self.build().emit();
    }
}

/// Builder for [`Event::Artifact`].
#[derive(Debug, Clone)]
pub struct Artifact {
    path: String,
    label: Option<String>,
}

impl Artifact {
    /// Human-readable label for the offer (e.g. a save button).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Finish building the event.
    pub fn build(self) -> Event {
        Event::Artifact {
            path: self.path,
            label: self.label,
        }
    }
    /// Build and emit to stdout (best-effort).
    pub fn emit(self) {
        self.build().emit();
    }
}

/// Builder for [`Event::Done`].
#[derive(Debug, Clone)]
pub struct Done {
    summary: Option<String>,
}

impl Done {
    /// One-line result ("4,520 items").
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
    /// Finish building the event.
    pub fn build(self) -> Event {
        Event::Done {
            summary: self.summary,
        }
    }
    /// Build and emit to stdout (best-effort).
    pub fn emit(self) {
        self.build().emit();
    }
}

/// One parsed line of producer stdout (consumer side, `parse` feature).
#[cfg(feature = "parse")]
#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    /// A protocol event.
    Event(Event),
    /// Not a protocol line: plain text, invalid JSON, or an unknown event.
    /// Per SPEC §6 the consumer shows it as a plain log — never an error.
    Text(String),
}

/// Parse one line of producer stdout, tolerantly (SPEC §6).
#[cfg(feature = "parse")]
pub fn parse_line(line: &str) -> Line {
    let trimmed = line.trim_end();
    match serde_json::from_str::<Event>(trimmed) {
        Ok(event) => Line::Event(event),
        Err(_) => Line::Text(trimmed.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(event: &Event) -> String {
        let mut buf = Vec::new();
        event.write_to(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn start_serializes_with_version_and_omits_absent_fields() {
        assert_eq!(
            line(&Event::start().title("import").total(120).build()),
            "{\"event\":\"start\",\"title\":\"import\",\"total\":120,\"v\":1}\n"
        );
        assert_eq!(
            line(&Event::start().build()),
            "{\"event\":\"start\",\"v\":1}\n"
        );
    }

    #[test]
    fn progress_keeps_current_zero() {
        assert_eq!(
            line(&Event::progress(0).total(10).build()),
            "{\"current\":0,\"event\":\"progress\",\"total\":10,\"v\":1}\n"
        );
    }

    #[test]
    fn log_metric_artifact_failed_serialize() {
        assert_eq!(
            line(&Event::log(Level::Warn, "row 42 skipped")),
            "{\"event\":\"log\",\"level\":\"warn\",\"msg\":\"row 42 skipped\",\"v\":1}\n"
        );
        assert_eq!(
            line(&Event::metric("items", 4520)),
            "{\"event\":\"metric\",\"key\":\"items\",\"v\":1,\"value\":4520}\n"
        );
        assert_eq!(
            line(&Event::metric("stage", "detail")),
            "{\"event\":\"metric\",\"key\":\"stage\",\"v\":1,\"value\":\"detail\"}\n"
        );
        assert_eq!(
            line(&Event::artifact("out.jsonl").label("Save JSONL").build()),
            "{\"event\":\"artifact\",\"label\":\"Save JSONL\",\"path\":\"out.jsonl\",\"v\":1}\n"
        );
        assert_eq!(
            line(&Event::failed("boom")),
            "{\"event\":\"failed\",\"msg\":\"boom\",\"v\":1}\n"
        );
        assert_eq!(
            line(&Event::done().summary("4,520 items").build()),
            "{\"event\":\"done\",\"summary\":\"4,520 items\",\"v\":1}\n"
        );
    }

    #[cfg(feature = "parse")]
    mod parse {
        use super::super::*;

        #[test]
        fn roundtrips_every_event() {
            let events = [
                Event::start().title("t").total(9).build(),
                Event::progress(3).total(9).msg("page 3").build(),
                Event::log(Level::Error, "e"),
                Event::metric("k", 1.5),
                Event::artifact("a.txt").build(),
                Event::done().build(),
                Event::failed("f"),
            ];
            for event in events {
                let mut buf = Vec::new();
                event.write_to(&mut buf).unwrap();
                let parsed = parse_line(std::str::from_utf8(&buf).unwrap());
                assert_eq!(parsed, Line::Event(event));
            }
        }

        #[test]
        fn non_json_becomes_text() {
            assert_eq!(
                parse_line("plain output\n"),
                Line::Text("plain output".to_string())
            );
        }

        #[test]
        fn unknown_event_becomes_text() {
            assert_eq!(
                parse_line("{\"v\":1,\"event\":\"teleport\"}"),
                Line::Text("{\"v\":1,\"event\":\"teleport\"}".to_string())
            );
        }

        #[test]
        fn unknown_fields_are_ignored() {
            assert_eq!(
                parse_line("{\"v\":1,\"event\":\"done\",\"future_field\":true}"),
                Line::Event(Event::done().build())
            );
        }
    }
}
