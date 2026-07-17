<?php

declare(strict_types=1);

/**
 * Dependency-free test runner: `php tests/run.php` (or `composer test`).
 * Captures emitted lines into a memory stream and compares them exactly.
 */

require __DIR__ . '/../src/Shinchoku.php';

use Shinchoku\Shinchoku;

/** @return string[] emitted lines produced by $fn */
function capture(callable $fn): array
{
    $stream = fopen('php://memory', 'r+');
    if ($stream === false) {
        fwrite(STDERR, "cannot open memory stream\n");
        exit(1);
    }
    Shinchoku::setOutput($stream);
    $fn();
    rewind($stream);
    $raw = stream_get_contents($stream);
    fclose($stream);
    Shinchoku::setOutput(STDOUT);
    return $raw === '' || $raw === false ? [] : explode("\n", rtrim($raw, "\n"));
}

$failures = 0;

/** @param string[] $expected */
function assertLines(string $name, array $expected, callable $fn): void
{
    global $failures;
    $actual = capture($fn);
    if ($actual === $expected) {
        echo "ok - {$name}\n";
        return;
    }
    $failures++;
    echo "FAIL - {$name}\n";
    echo '  expected: ' . var_export($expected, true) . "\n";
    echo '  actual:   ' . var_export($actual, true) . "\n";
}

assertLines('start omits absent fields', [
    '{"v":1,"event":"start","title":"import","total":120}',
    '{"v":1,"event":"start"}',
], static function (): void {
    Shinchoku::start('import', 120);
    Shinchoku::start();
});

assertLines('progress keeps current zero and takes optional total/msg', [
    '{"v":1,"event":"progress","current":0,"total":10}',
    '{"v":1,"event":"progress","current":3,"total":120,"msg":"page 3"}',
], static function (): void {
    Shinchoku::progress(0, 10);
    Shinchoku::progress(3, 120, 'page 3');
});

assertLines('log levels and shorthands', [
    '{"v":1,"event":"log","level":"info","msg":"a"}',
    '{"v":1,"event":"log","level":"warn","msg":"row 42 skipped"}',
    '{"v":1,"event":"log","level":"error","msg":"e"}',
], static function (): void {
    Shinchoku::log('info', 'a');
    Shinchoku::warn('row 42 skipped');
    Shinchoku::error('e');
});

assertLines('metric accepts numbers and strings', [
    '{"v":1,"event":"metric","key":"items","value":4520}',
    '{"v":1,"event":"metric","key":"stage","value":"detail"}',
], static function (): void {
    Shinchoku::metric('items', 4520);
    Shinchoku::metric('stage', 'detail');
});

assertLines('artifact, done, failed', [
    '{"v":1,"event":"artifact","path":"out.jsonl","label":"Save JSONL"}',
    '{"v":1,"event":"done","summary":"4,520 items"}',
    '{"v":1,"event":"failed","msg":"boom"}',
], static function (): void {
    Shinchoku::artifact('out.jsonl', 'Save JSONL');
    Shinchoku::done('4,520 items');
    Shinchoku::failed('boom');
});

assertLines('utf-8 passes through unescaped', [
    '{"v":1,"event":"start","title":"求人取り込み"}',
], static function (): void {
    Shinchoku::start('求人取り込み');
});

if ($failures > 0) {
    echo "{$failures} test(s) failed\n";
    exit(1);
}
echo "all tests passed\n";
