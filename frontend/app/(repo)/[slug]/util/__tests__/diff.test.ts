import type { DiffChunk } from "@/lib/dto";
import { expandLines, type LinePair, pairLines } from "../diff";

interface TestCase {
  name: string;
  input: DiffChunk;
  expected: LinePair[];
}

/**
 * Helper to create a DiffChunk from a more readable format.
 * - { lhs: lineNum } for left-only (removed)
 * - { rhs: lineNum } for right-only (added)
 * - { lhs: lineNum, rhs: lineNum } for modified/matched
 */
function chunk(
  entries: Array<
    { lhs: number } | { rhs: number } | { lhs: number; rhs: number }
  >,
): DiffChunk {
  return entries.map((entry) => ({
    lhs: "lhs" in entry ? { line_number: entry.lhs, changes: [] } : undefined,
    rhs: "rhs" in entry ? { line_number: entry.rhs, changes: [] } : undefined,
  }));
}

describe("pairLines", () => {
  const testCases: TestCase[] = [
    // LHS only tests
    {
      name: "lhs only - simple consecutive lines",
      input: chunk([{ lhs: 1 }, { lhs: 2 }, { lhs: 3 }, { lhs: 4 }]),
      expected: [
        [1, null],
        [2, null],
        [3, null],
        [4, null],
      ],
    },
    {
      name: "lhs only - advanced with gaps (1, 3, 5)",
      input: chunk([{ lhs: 1 }, { lhs: 3 }, { lhs: 5 }]),
      expected: [
        [1, null],
        [2, null],
        [3, null],
        [4, null],
        [5, null],
      ],
    },
    // RHS only tests
    {
      name: "rhs only - simple consecutive lines",
      input: chunk([{ rhs: 1 }, { rhs: 2 }, { rhs: 3 }, { rhs: 4 }]),
      expected: [
        [null, 1],
        [null, 2],
        [null, 3],
        [null, 4],
      ],
    },
    {
      name: "rhs only - advanced with gaps (1, 3, 5)",
      input: chunk([{ rhs: 1 }, { rhs: 3 }, { rhs: 5 }]),
      expected: [
        [null, 1],
        [null, 2],
        [null, 3],
        [null, 4],
        [null, 5],
      ],
    },
    {
      name: "testing smart sentinel addition - should add sentinel near existing one",
      input: [
        {
          rhs: {
            line_number: 733,
            changes: [
              { start: 2, end: 7, content: "const", highlight: "keyword" },
              { start: 8, end: 9, content: "{", highlight: "delimiter" },
              { start: 10, end: 16, content: "commit", highlight: "normal" },
              { start: 16, end: 17, content: ",", highlight: "normal" },
              { start: 18, end: 23, content: "diffs", highlight: "normal" },
              { start: 24, end: 25, content: "}", highlight: "delimiter" },
              { start: 26, end: 27, content: "=", highlight: "keyword" },
              {
                start: 28,
                end: 39,
                content: "commitDiffs",
                highlight: "normal",
              },
            ],
          },
        },
        {
          lhs: {
            line_number: 729,
            changes: [
              { start: 8, end: 9, content: "{", highlight: "delimiter" },
              { start: 10, end: 16, content: "commit", highlight: "normal" },
              { start: 16, end: 17, content: ",", highlight: "normal" },
              { start: 18, end: 23, content: "diffs", highlight: "normal" },
              { start: 24, end: 25, content: "}", highlight: "delimiter" },
            ],
          },
          rhs: {
            line_number: 730,
            changes: [
              {
                start: 8,
                end: 19,
                content: "commitDiffs",
                highlight: "normal",
              },
            ],
          },
        },
        {
          lhs: {
            line_number: 731,
            changes: [
              { start: 2, end: 9, content: "console", highlight: "normal" },
              { start: 9, end: 10, content: ".", highlight: "normal" },
              { start: 10, end: 13, content: "log", highlight: "normal" },
              { start: 13, end: 14, content: "(", highlight: "delimiter" },
              { start: 14, end: 18, content: "JSON", highlight: "keyword" },
              { start: 18, end: 19, content: ".", highlight: "normal" },
              { start: 19, end: 28, content: "stringify", highlight: "normal" },
              { start: 28, end: 29, content: "(", highlight: "delimiter" },
              { start: 29, end: 34, content: "diffs", highlight: "normal" },
              { start: 34, end: 35, content: ",", highlight: "normal" },
              { start: 40, end: 41, content: ",", highlight: "normal" },
              { start: 42, end: 43, content: "2", highlight: "normal" },
              { start: 43, end: 44, content: ")", highlight: "delimiter" },
              { start: 44, end: 45, content: ")", highlight: "delimiter" },
            ],
          },
          rhs: {
            line_number: 731,
            changes: [
              { start: 2, end: 4, content: "if", highlight: "keyword" },
              { start: 5, end: 6, content: "(", highlight: "delimiter" },
              { start: 6, end: 7, content: "!", highlight: "keyword" },
              {
                start: 7,
                end: 18,
                content: "commitDiffs",
                highlight: "normal",
              },
              { start: 18, end: 19, content: ")", highlight: "delimiter" },
              { start: 20, end: 26, content: "return", highlight: "keyword" },
              { start: 31, end: 32, content: ";", highlight: "normal" },
            ],
          },
        },
        {
          lhs: { line_number: 735, changes: [] },
          rhs: {
            line_number: 737,
            changes: [
              { start: 36, end: 41, content: "diffs", highlight: "normal" },
              { start: 41, end: 42, content: "=", highlight: "keyword" },
              { start: 42, end: 43, content: "{", highlight: "delimiter" },
              { start: 43, end: 48, content: "diffs", highlight: "normal" },
              { start: 48, end: 49, content: "}", highlight: "delimiter" },
            ],
          },
        },
        {
          lhs: {
            line_number: 736,
            changes: [
              { start: 21, end: 22, content: '"', highlight: "string" },
              { start: 22, end: 26, content: "flex", highlight: "string" },
              { start: 26, end: 27, content: "-", highlight: "string" },
              { start: 27, end: 28, content: "1", highlight: "string" },
              { start: 28, end: 29, content: " ", highlight: "string" },
              { start: 29, end: 37, content: "overflow", highlight: "string" },
              { start: 37, end: 38, content: "-", highlight: "string" },
              { start: 38, end: 42, content: "auto", highlight: "string" },
              { start: 42, end: 43, content: '"', highlight: "string" },
            ],
          },
          rhs: {
            line_number: 738,
            changes: [
              { start: 21, end: 22, content: '"', highlight: "string" },
              { start: 22, end: 26, content: "flex", highlight: "string" },
              { start: 26, end: 27, content: "-", highlight: "string" },
              { start: 27, end: 28, content: "1", highlight: "string" },
              { start: 28, end: 29, content: " ", highlight: "string" },
              { start: 29, end: 37, content: "overflow", highlight: "string" },
              { start: 37, end: 38, content: "-", highlight: "string" },
              { start: 38, end: 42, content: "auto", highlight: "string" },
              { start: 43, end: 52, content: "scrollbar", highlight: "string" },
              { start: 52, end: 53, content: "-", highlight: "string" },
              { start: 53, end: 57, content: "thin", highlight: "string" },
              { start: 57, end: 58, content: '"', highlight: "string" },
            ],
          },
        },
      ],
      expected: [
        [null, 729],
        [729, 730],
        [730, null],
        [731, 731],
        [null, 732],
        [null, 733],
        [732, 734],
        [733, 735],
        [734, 736],
        [735, 737],
        [736, 738],
        [737, 739],
        [738, 740],
      ],
    },
  ];

  test.each(testCases)("$name", ({ input, expected }) => {
    const result = pairLines(input);
    expect(result).toEqual(expected);
  });
});

describe("expandLines", () => {
  describe("all one-sided pairs", () => {
    test("lhs only - adds context before and after", () => {
      const input: LinePair[] = [
        [5, null],
        [6, null],
        [7, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 0],
        [1, 1],
        [2, 2],
        [3, 3],
        [4, 4],
        [5, null],
        [6, null],
        [7, null],
        [8, 5],
        [9, 6],
        [10, 7],
        [11, 8],
      ]);
    });

    test("rhs only - adds context before and after", () => {
      const input: LinePair[] = [
        [null, 5],
        [null, 6],
        [null, 7],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 0],
        [1, 1],
        [2, 2],
        [3, 3],
        [4, 4],
        [null, 5],
        [null, 6],
        [null, 7],
        [5, 8],
        [6, 9],
        [7, 10],
        [8, 11],
      ]);
    });

    test("lhs only starting at line 1 - limited context before", () => {
      const input: LinePair[] = [
        [1, null],
        [2, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 0],
        [1, null],
        [2, null],
        [3, 1],
        [4, 2],
        [5, 3],
        [6, 4],
      ]);
    });

    test("lhs only starting at line 0 - no context before", () => {
      const input: LinePair[] = [
        [0, null],
        [1, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, null],
        [1, null],
        [2, 0],
        [3, 1],
        [4, 2],
        [5, 3],
      ]);
    });

    test("single lhs line", () => {
      const input: LinePair[] = [[10, null]];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [5, 5],
        [6, 6],
        [7, 7],
        [8, 8],
        [9, 9],
        [10, null],
        [11, 10],
        [12, 11],
        [13, 12],
        [14, 13],
      ]);
    });

    test("single rhs line", () => {
      const input: LinePair[] = [[null, 10]];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [5, 5],
        [6, 6],
        [7, 7],
        [8, 8],
        [9, 9],
        [null, 10],
        [10, 11],
        [11, 12],
        [12, 13],
        [13, 14],
      ]);
    });

    test("lhs only - consecutive lines starting at 3", () => {
      const input: LinePair[] = [
        [3, null],
        [4, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 0],
        [1, 1],
        [2, 2],
        [3, null],
        [4, null],
        [5, 3],
        [6, 4],
        [7, 5],
        [8, 6],
      ]);
    });

    test("respects left max", () => {
      const input: LinePair[] = [
        [3, null],
        [4, null],
      ];
      const result = expandLines(input, 5, Infinity);
      expect(result).toEqual([
        [0, 0],
        [1, 1],
        [2, 2],
        [3, null],
        [4, null],
      ]);
    });

    test("respects right max", () => {
      const input: LinePair[] = [
        [3, null],
        [4, null],
      ];
      const result = expandLines(input, Infinity, 4);
      expect(result).toEqual([
        [0, 0],
        [1, 1],
        [2, 2],
        [3, null],
        [4, null],
        [5, 3],
      ]);
    });
  });
});
