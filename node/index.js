/**
 * shinchoku (進捗, "progress") — a tiny NDJSON protocol for batch CLIs to
 * report their progress. One JSON object per line on stdout.
 *
 * The contract is the spec (spec/SPEC.md in the repository), not this package:
 * `console.log('{"v":1,"event":"done"}')` is fully compliant. This package is
 * the convenience helper for Node producers.
 */

/** Protocol version stamped into every line as "v". */
export const VERSION = 1;

let out = process.stdout;

/**
 * Redirect event output (default: process.stdout). Intended for tests and for
 * consumers that relay events over another transport.
 */
export function setOutput(stream) {
  out = stream;
}

/** Emit one NDJSON line with "v" first, dropping undefined fields. */
function emit(fields) {
  for (const key of Object.keys(fields)) {
    if (fields[key] === undefined) delete fields[key];
  }
  out.write(JSON.stringify({ v: VERSION, ...fields }) + "\n");
}

/** Announce the run. Should be the first event. */
export function start({ title, total } = {}) {
  emit({ event: "start", title, total });
}

/** Report `current` completed units. `total` and `msg` are optional. */
export function progress(current, total, msg) {
  emit({ event: "progress", current, total, msg });
}

/** Emit a log line with a severity ("info" | "warn" | "error"). */
export function log(level, msg) {
  emit({ event: "log", level, msg });
}

/** log("info", msg) */
export function info(msg) {
  log("info", msg);
}

/** log("warn", msg) */
export function warn(msg) {
  log("warn", msg);
}

/** log("error", msg) — non-fatal; a fatal error is failed(). */
export function error(msg) {
  log("error", msg);
}

/**
 * Emit a named value (number or string) for the consumer to display verbatim.
 * The key is data, not vocabulary.
 */
export function metric(key, value) {
  emit({ event: "metric", key, value });
}

/** Declare that an output file exists. `label` is optional. */
export function artifact(path, label) {
  emit({ event: "artifact", path, label });
}

/** Report successful completion. `summary` is optional. */
export function done(summary) {
  emit({ event: "done", summary });
}

/** Report fatal failure; the run is over. */
export function failed(msg) {
  emit({ event: "failed", msg });
}
