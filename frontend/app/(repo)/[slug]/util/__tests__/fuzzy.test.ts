/**
 * Test suite ported from fzf's algo_test.go
 * Based on: https://github.com/junegunn/fzf/blob/master/src/algo/algo_test.go
 */

import { fuzzyMatch } from "../fuzzy";

const SCORE_MATCH = 16;
const SCORE_GAP_START = -3;
const SCORE_GAP_EXTENSION = -1;

const BONUS_BOUNDARY = SCORE_MATCH / 2; // 8
const BONUS_NON_WORD = SCORE_MATCH / 2; // 8
const BONUS_CAMEL123 = BONUS_BOUNDARY + SCORE_GAP_EXTENSION; // 7
const BONUS_CONSECUTIVE = -(SCORE_GAP_START + SCORE_GAP_EXTENSION); // 4
const BONUS_FIRST_CHAR_MULTIPLIER = 2;

const BONUS_BOUNDARY_WHITE = BONUS_BOUNDARY + 2; // 10
const BONUS_BOUNDARY_DELIMITER = BONUS_BOUNDARY + 1; // 9

function max(...values: number[]): number {
  return Math.max(...values);
}

function assertMatch(
  caseSensitive: boolean,
  input: string,
  pattern: string,
  sidx: number,
  eidx: number,
  expectedScore: number,
) {
  const res = fuzzyMatch(pattern, input, true, caseSensitive);

  if (sidx === -1 && eidx === -1) {
    expect(res).toBeNull();
    return;
  }

  expect(res).not.toBeNull();
  if (!res) return;

  let start: number;
  let end: number;

  if (!res.positions || res.positions.length === 0) {
    start = res.start;
    end = res.end;
  } else {
    const sorted = [...res.positions].sort((a, b) => a - b);
    start = sorted[0];
    end = sorted[sorted.length - 1] + 1;
  }

  expect(start).toBe(sidx);
  expect(end).toBe(eidx);
  expect(res.score).toBe(expectedScore);
}

// ============================================================================
// Tests
// ============================================================================

describe("FuzzyMatch", () => {
  test.each([
    [
      "fooBarbaz1",
      "oBZ",
      2,
      9,
      SCORE_MATCH * 3 +
        BONUS_CAMEL123 +
        SCORE_GAP_START +
        SCORE_GAP_EXTENSION * 3,
    ],
    [
      "foo bar baz",
      "fbb",
      0,
      9,
      SCORE_MATCH * 3 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_WHITE) * 2 +
        2 * SCORE_GAP_START +
        4 * SCORE_GAP_EXTENSION,
    ],
    [
      "/AutomatorDocument.icns",
      "rdoc",
      9,
      13,
      SCORE_MATCH * 4 + BONUS_CAMEL123 + BONUS_CONSECUTIVE * 2,
    ],
    [
      "/man1/zshcompctl.1",
      "zshc",
      6,
      10,
      SCORE_MATCH * 4 +
        Math.floor(BONUS_BOUNDARY_DELIMITER) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_DELIMITER) * 3,
    ],
    [
      "/.oh-my-zsh/cache",
      "zshc",
      8,
      13,
      SCORE_MATCH * 4 +
        BONUS_BOUNDARY * BONUS_FIRST_CHAR_MULTIPLIER +
        BONUS_BOUNDARY * 2 +
        SCORE_GAP_START +
        Math.floor(BONUS_BOUNDARY_DELIMITER),
    ],
    [
      "ab0123 456",
      "12356",
      3,
      10,
      SCORE_MATCH * 5 +
        BONUS_CONSECUTIVE * 3 +
        SCORE_GAP_START +
        SCORE_GAP_EXTENSION,
    ],
    [
      "abc123 456",
      "12356",
      3,
      10,
      SCORE_MATCH * 5 +
        BONUS_CAMEL123 * BONUS_FIRST_CHAR_MULTIPLIER +
        BONUS_CAMEL123 * 2 +
        BONUS_CONSECUTIVE +
        SCORE_GAP_START +
        SCORE_GAP_EXTENSION,
    ],
    [
      "foo/bar/baz",
      "fbb",
      0,
      9,
      SCORE_MATCH * 3 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_DELIMITER) * 2 +
        2 * SCORE_GAP_START +
        4 * SCORE_GAP_EXTENSION,
    ],
    [
      "fooBarBaz",
      "fbb",
      0,
      7,
      SCORE_MATCH * 3 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        BONUS_CAMEL123 * 2 +
        2 * SCORE_GAP_START +
        2 * SCORE_GAP_EXTENSION,
    ],
    [
      "foo barbaz",
      "fbb",
      0,
      8,
      SCORE_MATCH * 3 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_WHITE) +
        SCORE_GAP_START * 2 +
        SCORE_GAP_EXTENSION * 3,
    ],
    [
      "fooBar Baz",
      "foob",
      0,
      4,
      SCORE_MATCH * 4 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_WHITE) * 3,
    ],
    [
      "xFoo-Bar Baz",
      "foo-b",
      1,
      6,
      SCORE_MATCH * 5 +
        BONUS_CAMEL123 * BONUS_FIRST_CHAR_MULTIPLIER +
        BONUS_CAMEL123 * 2 +
        BONUS_NON_WORD +
        BONUS_BOUNDARY,
    ],
  ])("case-insensitive: %s / %s", (input, pattern, sidx, eidx, score) => {
    assertMatch(false, input, pattern, sidx, eidx, score);
  });

  test.each([
    [
      "fooBarbaz",
      "oBz",
      2,
      9,
      SCORE_MATCH * 3 +
        BONUS_CAMEL123 +
        SCORE_GAP_START +
        SCORE_GAP_EXTENSION * 3,
    ],
    [
      "Foo/Bar/Baz",
      "FBB",
      0,
      9,
      SCORE_MATCH * 3 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_DELIMITER) * 2 +
        SCORE_GAP_START * 2 +
        SCORE_GAP_EXTENSION * 4,
    ],
    [
      "FooBarBaz",
      "FBB",
      0,
      7,
      SCORE_MATCH * 3 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        BONUS_CAMEL123 * 2 +
        SCORE_GAP_START * 2 +
        SCORE_GAP_EXTENSION * 2,
    ],
    [
      "FooBar Baz",
      "FooB",
      0,
      4,
      SCORE_MATCH * 4 +
        Math.floor(BONUS_BOUNDARY_WHITE) * BONUS_FIRST_CHAR_MULTIPLIER +
        Math.floor(BONUS_BOUNDARY_WHITE) * 2 +
        max(BONUS_CAMEL123, Math.floor(BONUS_BOUNDARY_WHITE)),
    ],
    ["foo-bar", "o-ba", 2, 6, SCORE_MATCH * 4 + BONUS_BOUNDARY * 3],
  ])("case-sensitive: %s / %s", (input, pattern, sidx, eidx, score) => {
    assertMatch(true, input, pattern, sidx, eidx, score);
  });

  test.each([
    ["fooBarbaz", "oBZ"],
    ["Foo Bar Baz", "fbb"],
    ["fooBarbaz", "fooBarbazz"],
  ])("case-sensitive non-match: %s / %s", (input, pattern) => {
    assertMatch(true, input, pattern, -1, -1, 0);
  });
});

describe("EmptyPattern", () => {
  test("should handle empty patterns", () => {
    const res = fuzzyMatch("", "foobar", true, true);
    expect(res).not.toBeNull();
    expect(res?.start).toBe(0);
    expect(res?.end).toBe(0);
    expect(res?.score).toBe(0);
  });
});

describe("LongString", () => {
  test("should handle very long strings", () => {
    const bytes = new Array(2 ** 16 * 2).fill("x");
    bytes[2 ** 16] = "z";
    const str = bytes.join("");

    assertMatch(
      true,
      str,
      "zx",
      2 ** 16,
      2 ** 16 + 2,
      SCORE_MATCH * 2 + BONUS_CONSECUTIVE,
    );
  });
});
