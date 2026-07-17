/**
 * shinchoku (進捗, "progress") — a tiny NDJSON protocol for batch CLIs to
 * report their progress. See spec/SPEC.md in the repository.
 */

/** Protocol version stamped into every line as `"v"`. */
export declare const VERSION: 1;

/** Severity of a log event. */
export type Level = "info" | "warn" | "error";

/** Anything with a `write(chunk: string)` method (e.g. a Writable stream). */
export interface Output {
  write(chunk: string): unknown;
}

/**
 * Redirect event output (default: `process.stdout`). Intended for tests and
 * for consumers that relay events over another transport.
 */
export declare function setOutput(stream: Output): void;

/** Announce the run. Should be the first event. */
export declare function start(options?: { title?: string; total?: number }): void;

/** Report `current` completed units. */
export declare function progress(current: number, total?: number, msg?: string): void;

/** Emit a log line with a severity. */
export declare function log(level: Level, msg: string): void;

/** `log("info", msg)` */
export declare function info(msg: string): void;

/** `log("warn", msg)` */
export declare function warn(msg: string): void;

/** `log("error", msg)` — non-fatal; a fatal error is `failed()`. */
export declare function error(msg: string): void;

/** Emit a named value for the consumer to display verbatim. */
export declare function metric(key: string, value: number | string): void;

/** Declare that an output file exists. */
export declare function artifact(path: string, label?: string): void;

/** Report successful completion. */
export declare function done(summary?: string): void;

/** Report fatal failure; the run is over. */
export declare function failed(msg: string): void;
