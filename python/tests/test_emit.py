import io
import sys
import unittest

import shinchoku


class EmitTest(unittest.TestCase):
    def setUp(self) -> None:
        self.buf = io.StringIO()
        shinchoku.set_output(self.buf)
        self.addCleanup(shinchoku.set_output, sys.stdout)

    def lines(self) -> list[str]:
        return self.buf.getvalue().splitlines()

    def test_start_omits_absent_fields(self) -> None:
        shinchoku.start(title="import", total=120)
        shinchoku.start()
        self.assertEqual(
            self.lines(),
            [
                '{"v":1,"event":"start","title":"import","total":120}',
                '{"v":1,"event":"start"}',
            ],
        )

    def test_progress_keeps_current_zero(self) -> None:
        shinchoku.progress(0, total=10)
        shinchoku.progress(3, total=120, msg="page 3")
        self.assertEqual(
            self.lines(),
            [
                '{"v":1,"event":"progress","current":0,"total":10}',
                '{"v":1,"event":"progress","current":3,"total":120,"msg":"page 3"}',
            ],
        )

    def test_log_levels_and_shorthands(self) -> None:
        shinchoku.log("info", "a")
        shinchoku.warn("row 42 skipped")
        shinchoku.error("e")
        self.assertEqual(
            self.lines(),
            [
                '{"v":1,"event":"log","level":"info","msg":"a"}',
                '{"v":1,"event":"log","level":"warn","msg":"row 42 skipped"}',
                '{"v":1,"event":"log","level":"error","msg":"e"}',
            ],
        )

    def test_metric_accepts_numbers_and_strings(self) -> None:
        shinchoku.metric("items", 4520)
        shinchoku.metric("stage", "detail")
        self.assertEqual(
            self.lines(),
            [
                '{"v":1,"event":"metric","key":"items","value":4520}',
                '{"v":1,"event":"metric","key":"stage","value":"detail"}',
            ],
        )

    def test_artifact_done_failed(self) -> None:
        shinchoku.artifact("out.jsonl", label="Save JSONL")
        shinchoku.done("4,520 items")
        shinchoku.failed("boom")
        self.assertEqual(
            self.lines(),
            [
                '{"v":1,"event":"artifact","path":"out.jsonl","label":"Save JSONL"}',
                '{"v":1,"event":"done","summary":"4,520 items"}',
                '{"v":1,"event":"failed","msg":"boom"}',
            ],
        )

    def test_utf8_passes_through_unescaped(self) -> None:
        shinchoku.start(title="求人取り込み")
        self.assertEqual(
            self.lines(),
            ['{"v":1,"event":"start","title":"求人取り込み"}'],
        )


if __name__ == "__main__":
    unittest.main()
