import { type DiffHunk, expandLines, type LinePair, pairLines } from "../diff";

interface TestCase {
  name: string;
  input: DiffHunk;
  expected: LinePair[];
}

/**
 * Helper to create a DiffHunk from a more readable format.
 * - { lhs: lineNum } for left-only (removed)
 * - { rhs: lineNum } for right-only (added)
 * - { lhs: lineNum, rhs: lineNum } for modified/matched
 */
function chunk(
  entries: Array<
    { lhs: number } | { rhs: number } | { lhs: number; rhs: number }
  >,
): DiffHunk {
  return entries.map((entry) => ({
    lhs: "lhs" in entry ? { line_number: entry.lhs } : undefined,
    rhs: "rhs" in entry ? { line_number: entry.rhs } : undefined,
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
        { rhs: { line_number: 733 } },
        { lhs: { line_number: 729 }, rhs: { line_number: 730 } },
        { lhs: { line_number: 731 }, rhs: { line_number: 731 } },
        { lhs: { line_number: 735 }, rhs: { line_number: 737 } },
        { lhs: { line_number: 736 }, rhs: { line_number: 738 } },
      ],
      expected: [
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

  describe("with matched lines", () => {
    test("single match surrounded by nulls", () => {
      const input: LinePair[] = [
        [1, null],
        [2, 2],
        [3, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 1],
        [1, null],
        [2, 2],
        [3, null],
        [4, 3],
        [5, 4],
        [6, 5],
        [7, 6],
      ]);
    });

    test("match at start followed by nulls", () => {
      const input: LinePair[] = [
        [5, 5],
        [6, null],
        [7, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [1, 1],
        [2, 2],
        [3, 3],
        [4, 4],
        [5, 5],
        [6, null],
        [7, null],
        [8, 6],
        [9, 7],
        [10, 8],
        [11, 9],
      ]);
    });

    test("nulls followed by match at end", () => {
      const input: LinePair[] = [
        [null, 3],
        [null, 4],
        [5, 5],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [2, 0],
        [3, 1],
        [4, 2],
        [null, 3],
        [null, 4],
        [5, 5],
        [6, 6],
        [7, 7],
        [8, 8],
        [9, 9],
      ]);
    });

    test("match with different line numbers (offset)", () => {
      const input: LinePair[] = [
        [1, null],
        [2, 5],
        [3, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 4],
        [1, null],
        [2, 5],
        [3, null],
        [4, 6],
        [5, 7],
        [6, 8],
        [7, 9],
      ]);
    });

    test("multiple matches with nulls interspersed", () => {
      const input: LinePair[] = [
        [1, 1],
        [2, null],
        [3, 3],
        [null, 4],
        [5, 5],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, 0],
        [1, 1],
        [2, null],
        [3, 3],
        [null, 4],
        [5, 5],
        [6, 6],
        [7, 7],
        [8, 8],
        [9, 9],
      ]);
    });

    test("match at line 0 - no context before", () => {
      const input: LinePair[] = [
        [0, null],
        [1, 1],
        [2, null],
      ];
      const result = expandLines(input, Infinity, Infinity);
      expect(result).toEqual([
        [0, null],
        [1, 1],
        [2, null],
        [3, 2],
        [4, 3],
        [5, 4],
        [6, 5],
      ]);
    });

    test("respects leftMax with matched lines", () => {
      const input: LinePair[] = [
        [1, null],
        [2, 2],
        [3, null],
      ];
      const result = expandLines(input, 5, Infinity);
      expect(result).toEqual([
        [0, 1],
        [1, null],
        [2, 2],
        [3, null],
        [4, 3],
      ]);
    });

    test("respects rightMax with matched lines", () => {
      const input: LinePair[] = [
        [1, null],
        [2, 2],
        [3, null],
      ];
      const result = expandLines(input, Infinity, 5);
      expect(result).toEqual([
        [0, 1],
        [1, null],
        [2, 2],
        [3, null],
        [4, 3],
        [5, 4],
      ]);
    });
  });
});
