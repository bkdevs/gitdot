import type { DiffChunk, RepositoryFile } from "@/lib/dto";
import { type LinePair, pairLines, processChunks } from "../diff";

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

describe("processChunks", () => {
  const mockFile: RepositoryFile = {
    path: "test.ts",
    content: "line1\nline2\nline3\nline4\nline5",
  };

  test("left-only chunk - all right side sentinels, no anchor", () => {
    // pairLines returns: [[1, null], [2, null], [3, null]]
    const chunks: DiffChunk[] = [chunk([{ lhs: 1 }, { lhs: 2 }, { lhs: 3 }])];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set([1, 2, 3]));
    expect(result.rightVisibleLines).toEqual(new Set());
    expect(result.leftSentinelCounts).toEqual(new Map());
    // All 3 right sentinels become trailing sentinels (no right lines to anchor)
    expect(result.rightSentinelCounts).toEqual(new Map());
  });

  test("right-only chunk - all left side sentinels, no anchor", () => {
    // pairLines returns: [[null, 1], [null, 2], [null, 3]]
    const chunks: DiffChunk[] = [chunk([{ rhs: 1 }, { rhs: 2 }, { rhs: 3 }])];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set());
    expect(result.rightVisibleLines).toEqual(new Set([1, 2, 3]));
    // All 3 left sentinels become trailing sentinels (no left lines to anchor)
    expect(result.leftSentinelCounts).toEqual(new Map());
    expect(result.rightSentinelCounts).toEqual(new Map());
  });

  test("paired lines with left sentinel before", () => {
    // pairLines returns: [[null, 1], [1, 2], [2, 3]]
    const chunks: DiffChunk[] = [chunk([{ rhs: 1 }, { lhs: 1, rhs: 2 }])];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set([1, 2]));
    expect(result.rightVisibleLines).toEqual(new Set([1, 2, 3]));
    // Before left line 1, there's 1 sentinel
    expect(result.leftSentinelCounts).toEqual(new Map([[1, 1]]));
    expect(result.rightSentinelCounts).toEqual(new Map());
  });

  test("paired lines with right sentinel before", () => {
    // pairLines returns: [[1, null], [2, 1], [3, 2]]
    const chunks: DiffChunk[] = [chunk([{ lhs: 1 }, { lhs: 2, rhs: 1 }])];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set([1, 2, 3]));
    expect(result.rightVisibleLines).toEqual(new Set([1, 2]));
    expect(result.leftSentinelCounts).toEqual(new Map());
    // Before right line 1, there's 1 sentinel
    expect(result.rightSentinelCounts).toEqual(new Map([[1, 1]]));
  });

  test("multiple consecutive sentinels on left side", () => {
    // pairLines returns: [[null, 1], [null, 2], [1, 3], [2, 4], [3, 5]]
    const chunks: DiffChunk[] = [
      chunk([{ rhs: 1 }, { rhs: 2 }, { lhs: 1, rhs: 3 }]),
    ];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set([1, 2, 3]));
    expect(result.rightVisibleLines).toEqual(new Set([1, 2, 3, 4, 5]));
    // Before left line 1, there are 2 sentinels
    expect(result.leftSentinelCounts).toEqual(new Map([[1, 2]]));
    expect(result.rightSentinelCounts).toEqual(new Map());
  });

  test("sentinels interspersed between real lines", () => {
    // pairLines returns: [[1, 1], [null, 2], [2, 3], [3, 4]]
    const chunks: DiffChunk[] = [
      chunk([{ lhs: 1, rhs: 1 }, { rhs: 2 }, { lhs: 2, rhs: 3 }]),
    ];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set([1, 2, 3]));
    expect(result.rightVisibleLines).toEqual(new Set([1, 2, 3, 4]));
    // Before left line 2, there's 1 sentinel
    expect(result.leftSentinelCounts).toEqual(new Map([[2, 1]]));
    expect(result.rightSentinelCounts).toEqual(new Map());
  });

  test("offset anchor creates leading right sentinels", () => {
    // pairLines returns: [[1, null], [2, null], [3, 1], [4, 2], [5, 3]]
    const chunks: DiffChunk[] = [chunk([{ lhs: 3, rhs: 1 }])];
    const result = processChunks(mockFile, mockFile, chunks);

    expect(result.leftVisibleLines).toEqual(new Set([1, 2, 3, 4, 5]));
    expect(result.rightVisibleLines).toEqual(new Set([1, 2, 3]));
    expect(result.leftSentinelCounts).toEqual(new Map());
    // Before right line 1, there are 2 sentinels
    expect(result.rightSentinelCounts).toEqual(new Map([[1, 2]]));
  });

  test("complex mixed case from pairLines test", () => {
    // Expected pairs: [[null,729],[729,730],[730,null],[731,731],[null,732],[null,733],[732,734],[733,735],[734,736],[735,737],[736,738],[737,739],[738,740]]
    const complexChunk: DiffChunk = [
      { rhs: { line_number: 733, changes: [] } },
      {
        lhs: { line_number: 729, changes: [] },
        rhs: { line_number: 730, changes: [] },
      },
      {
        lhs: { line_number: 731, changes: [] },
        rhs: { line_number: 731, changes: [] },
      },
      {
        lhs: { line_number: 735, changes: [] },
        rhs: { line_number: 737, changes: [] },
      },
      {
        lhs: { line_number: 736, changes: [] },
        rhs: { line_number: 738, changes: [] },
      },
    ];
    const chunks: DiffChunk[] = [complexChunk];
    const result = processChunks(mockFile, mockFile, chunks);

    // Left visible: 729, 730, 731, 732, 733, 734, 735, 736, 737, 738
    expect(result.leftVisibleLines).toEqual(
      new Set([729, 730, 731, 732, 733, 734, 735, 736, 737, 738]),
    );
    // Right visible: 729, 730, 731, 732, 733, 734, 735, 736, 737, 738, 739, 740
    expect(result.rightVisibleLines).toEqual(
      new Set([729, 730, 731, 732, 733, 734, 735, 736, 737, 738, 739, 740]),
    );

    // Left sentinels: 1 before 729, 2 before 732
    expect(result.leftSentinelCounts).toEqual(
      new Map([
        [729, 1],
        [732, 2],
      ]),
    );
    // Right sentinels: 1 before 731
    expect(result.rightSentinelCounts).toEqual(new Map([[731, 1]]));
  });
});
