import { test, beforeEach } from "node:test";
import assert from "node:assert/strict";
import * as shinchoku from "../index.js";

let lines;
beforeEach(() => {
  lines = [];
  shinchoku.setOutput({ write: (chunk) => lines.push(chunk) });
});

test("start omits absent fields", () => {
  shinchoku.start({ title: "import", total: 120 });
  assert.deepEqual(lines, ['{"v":1,"event":"start","title":"import","total":120}\n']);

  lines.length = 0;
  shinchoku.start();
  assert.deepEqual(lines, ['{"v":1,"event":"start"}\n']);
});

test("progress keeps current zero and takes optional total/msg", () => {
  shinchoku.progress(0, 10);
  shinchoku.progress(3, 120, "page 3");
  assert.deepEqual(lines, [
    '{"v":1,"event":"progress","current":0,"total":10}\n',
    '{"v":1,"event":"progress","current":3,"total":120,"msg":"page 3"}\n',
  ]);
});

test("log levels and shorthands", () => {
  shinchoku.log("info", "a");
  shinchoku.warn("row 42 skipped");
  shinchoku.error("e");
  assert.deepEqual(lines, [
    '{"v":1,"event":"log","level":"info","msg":"a"}\n',
    '{"v":1,"event":"log","level":"warn","msg":"row 42 skipped"}\n',
    '{"v":1,"event":"log","level":"error","msg":"e"}\n',
  ]);
});

test("metric accepts numbers and strings", () => {
  shinchoku.metric("items", 4520);
  shinchoku.metric("stage", "detail");
  assert.deepEqual(lines, [
    '{"v":1,"event":"metric","key":"items","value":4520}\n',
    '{"v":1,"event":"metric","key":"stage","value":"detail"}\n',
  ]);
});

test("artifact, done, failed", () => {
  shinchoku.artifact("out.jsonl", "Save JSONL");
  shinchoku.done("4,520 items");
  shinchoku.failed("boom");
  assert.deepEqual(lines, [
    '{"v":1,"event":"artifact","path":"out.jsonl","label":"Save JSONL"}\n',
    '{"v":1,"event":"done","summary":"4,520 items"}\n',
    '{"v":1,"event":"failed","msg":"boom"}\n',
  ]);
});
