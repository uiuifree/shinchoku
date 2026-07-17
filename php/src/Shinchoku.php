<?php

declare(strict_types=1);

namespace Shinchoku;

/**
 * shinchoku (進捗, "progress") — a tiny NDJSON protocol for batch CLIs to
 * report their progress. One JSON object per line on stdout.
 *
 * The contract is the spec (spec/SPEC.md in the repository), not this class:
 * `echo '{"v":1,"event":"done"}' . "\n";` is fully compliant. This class is
 * the convenience helper for PHP producers.
 */
final class Shinchoku
{
    /** Protocol version stamped into every line as "v". */
    public const VERSION = 1;

    /** @var resource|null Output stream; defaults to STDOUT. */
    private static $out = null;

    private function __construct()
    {
    }

    /**
     * Redirect event output (default: STDOUT). Intended for tests and for
     * consumers that relay events over another transport.
     *
     * @param resource $stream
     */
    public static function setOutput($stream): void
    {
        self::$out = $stream;
    }

    /** Announce the run. Should be the first event. */
    public static function start(?string $title = null, ?int $total = null): void
    {
        self::emit(['event' => 'start', 'title' => $title, 'total' => $total]);
    }

    /** Report `current` completed units. Null total means unknown. */
    public static function progress(int $current, ?int $total = null, ?string $msg = null): void
    {
        self::emit(['event' => 'progress', 'current' => $current, 'total' => $total, 'msg' => $msg]);
    }

    /** Emit a log line with a severity ("info" | "warn" | "error"). */
    public static function log(string $level, string $msg): void
    {
        self::emit(['event' => 'log', 'level' => $level, 'msg' => $msg]);
    }

    /** log("info", $msg) */
    public static function info(string $msg): void
    {
        self::log('info', $msg);
    }

    /** log("warn", $msg) */
    public static function warn(string $msg): void
    {
        self::log('warn', $msg);
    }

    /** log("error", $msg) — non-fatal; a fatal error is failed(). */
    public static function error(string $msg): void
    {
        self::log('error', $msg);
    }

    /**
     * Emit a named value (number or string) for the consumer to display
     * verbatim. The key is data, not vocabulary.
     */
    public static function metric(string $key, int|float|string $value): void
    {
        self::emit(['event' => 'metric', 'key' => $key, 'value' => $value]);
    }

    /** Declare that an output file exists. */
    public static function artifact(string $path, ?string $label = null): void
    {
        self::emit(['event' => 'artifact', 'path' => $path, 'label' => $label]);
    }

    /** Report successful completion. */
    public static function done(?string $summary = null): void
    {
        self::emit(['event' => 'done', 'summary' => $summary]);
    }

    /** Report fatal failure; the run is over. */
    public static function failed(string $msg): void
    {
        self::emit(['event' => 'failed', 'msg' => $msg]);
    }

    /**
     * Emit one NDJSON line with "v" first, dropping null fields. Best-effort:
     * write errors are ignored, matching the fire-and-forget nature of reporting.
     *
     * @param array<string, mixed> $fields
     */
    private static function emit(array $fields): void
    {
        $fields = array_filter($fields, static fn (mixed $value): bool => $value !== null);
        $line = json_encode(
            ['v' => self::VERSION] + $fields,
            JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES
        );
        if ($line === false) {
            return;
        }
        @fwrite(self::$out ?? STDOUT, $line . "\n");
    }
}
