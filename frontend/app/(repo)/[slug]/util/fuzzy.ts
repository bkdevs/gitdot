/**
 * FuzzyMatchV2 - Port of fzf's fuzzy matching algorithm, courtesy of Claude
 * Based on fzf's algo package implementation: https://github.com/junegunn/fzf/blob/master/src/algo/algo.go
 *
 * This implements a modified Smith-Waterman algorithm for fuzzy string matching
 * with special consideration for word boundaries, camelCase, consecutive matches, etc.
 */

// ============================================================================
// Type Definitions
// ============================================================================

export interface FuzzyResult {
  start: number; // Start index of match in target
  end: number; // End index of match in target
  score: number; // Match score (higher is better)
  positions?: number[]; // Character positions (optional)
}

enum CharClass {
  White = 0,
  NonWord = 1,
  Delimiter = 2,
  Lower = 3,
  Upper = 4,
  Letter = 5,
  Number = 6,
}

// ============================================================================
// Constants (from fzf)
// ============================================================================

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

const DELIMITER_CHARS = "/,:;|";
const WHITE_CHARS = " \t\n\v\f\r\x85\xA0";

// ============================================================================
// Character Classification
// ============================================================================

// Pre-computed ASCII character classes for performance
const asciiCharClasses: CharClass[] = new Array(128);

// Initialize ASCII character class lookup table
function initAsciiCharClasses() {
  for (let i = 0; i < 128; i++) {
    const char = String.fromCharCode(i);
    let c = CharClass.NonWord;

    if (char >= "a" && char <= "z") {
      c = CharClass.Lower;
    } else if (char >= "A" && char <= "Z") {
      c = CharClass.Upper;
    } else if (char >= "0" && char <= "9") {
      c = CharClass.Number;
    } else if (WHITE_CHARS.indexOf(char) >= 0) {
      c = CharClass.White;
    } else if (DELIMITER_CHARS.indexOf(char) >= 0) {
      c = CharClass.Delimiter;
    }

    asciiCharClasses[i] = c;
  }
}

initAsciiCharClasses();

function charClassOfNonAscii(char: string): CharClass {
  const code = char.charCodeAt(0);

  // Check Unicode categories
  if (/[a-z]/.test(char.toLowerCase()) && char === char.toLowerCase()) {
    return CharClass.Lower;
  } else if (/[A-Z]/.test(char.toUpperCase()) && char === char.toUpperCase()) {
    return CharClass.Upper;
  } else if (/\d/.test(char)) {
    return CharClass.Number;
  } else if (/\p{L}/u.test(char)) {
    return CharClass.Letter;
  } else if (/\s/.test(char)) {
    return CharClass.White;
  } else if (DELIMITER_CHARS.indexOf(char) >= 0) {
    return CharClass.Delimiter;
  }

  return CharClass.NonWord;
}

function charClassOf(char: string): CharClass {
  const code = char.charCodeAt(0);
  if (code < 128) {
    return asciiCharClasses[code];
  }
  return charClassOfNonAscii(char);
}

// ============================================================================
// Bonus Calculation
// ============================================================================

function bonusFor(prevClass: CharClass, currClass: CharClass): number {
  if (currClass > CharClass.NonWord) {
    switch (prevClass) {
      case CharClass.White:
        return BONUS_BOUNDARY_WHITE;
      case CharClass.Delimiter:
        return BONUS_BOUNDARY_DELIMITER;
      case CharClass.NonWord:
        return BONUS_BOUNDARY;
    }
  }

  if (
    (prevClass === CharClass.Lower && currClass === CharClass.Upper) ||
    (prevClass !== CharClass.Number && currClass === CharClass.Number)
  ) {
    return BONUS_CAMEL123;
  }

  switch (currClass) {
    case CharClass.NonWord:
    case CharClass.Delimiter:
      return BONUS_NON_WORD;
    case CharClass.White:
      return BONUS_BOUNDARY_WHITE;
  }

  return 0;
}

// Pre-computed bonus matrix for performance
const bonusMatrix: number[][] = [];

function initBonusMatrix() {
  for (let i = 0; i <= CharClass.Number; i++) {
    bonusMatrix[i] = [];
    for (let j = 0; j <= CharClass.Number; j++) {
      bonusMatrix[i][j] = bonusFor(i as CharClass, j as CharClass);
    }
  }
}

initBonusMatrix();

function bonusAt(text: string, idx: number): number {
  if (idx === 0) {
    return BONUS_BOUNDARY_WHITE;
  }
  return bonusMatrix[charClassOf(text[idx - 1])][charClassOf(text[idx])];
}

// ============================================================================
// Helper Functions
// ============================================================================

function indexAt(index: number, max: number, forward: boolean): number {
  if (forward) {
    return index;
  }
  return max - index - 1;
}

function max(...values: number[]): number {
  return Math.max(...values);
}

function min(a: number, b: number): number {
  return Math.min(a, b);
}

// ============================================================================
// ASCII Optimization
// ============================================================================

function isAscii(pattern: string[]): boolean {
  for (const char of pattern) {
    if (char.charCodeAt(0) >= 128) {
      return false;
    }
  }
  return true;
}

function trySkip(text: string, b: string, from: number): number {
  const idx = text.indexOf(b, from);

  if (idx === from) {
    return from; // Can't skip any further
  }

  // Check for uppercase version
  const upperB = b.toUpperCase();
  if (b !== upperB) {
    const searchText =
      idx > 0 ? text.substring(from, idx) : text.substring(from);
    const uidx = searchText.indexOf(upperB);
    if (uidx >= 0) {
      return from + uidx;
    }
  }

  if (idx < 0) {
    return -1;
  }
  return idx;
}

function asciiFuzzyIndex(text: string, pattern: string[]): [number, number] {
  // Not ASCII pattern
  if (!isAscii(pattern)) {
    return [-1, -1];
  }

  let firstIdx = 0;
  let idx = 0;
  let lastIdx = 0;

  for (let pidx = 0; pidx < pattern.length; pidx++) {
    const b = pattern[pidx].toLowerCase();
    idx = trySkip(text, b, idx);

    if (idx < 0) {
      return [-1, -1];
    }

    if (pidx === 0 && idx > 0) {
      // Step back to find the right bonus point
      firstIdx = idx - 1;
    }

    lastIdx = idx;
    idx++;
  }

  // Find the last appearance of the last character to limit search scope
  const lastChar = pattern[pattern.length - 1].toLowerCase();
  const lastCharUpper = lastChar.toUpperCase();
  const scope = text.substring(lastIdx);

  for (let offset = scope.length - 1; offset > 0; offset--) {
    const c = scope[offset];
    if (c === lastChar || c === lastCharUpper) {
      return [firstIdx, lastIdx + offset + 1];
    }
  }

  return [firstIdx, lastIdx + 1];
}

// ============================================================================
// Main FuzzyMatchV2 Algorithm
// ============================================================================

function fuzzyMatchV2(
  text: string,
  pattern: string[],
  withPos: boolean,
  caseSensitive: boolean = false,
): FuzzyResult | null {
  const M = pattern.length;
  const N = text.length;

  if (M === 0) {
    return { start: 0, end: 0, score: 0 };
  }

  if (M > N) {
    return null;
  }

  // Phase 1: ASCII optimization
  const [minIdx, maxIdx] = asciiFuzzyIndex(text, pattern);

  if (minIdx < 0) {
    return null;
  }

  const searchLen = maxIdx - minIdx;

  // Phase 2: Calculate bonuses and first pass
  const H0 = new Array<number>(searchLen);
  const C0 = new Array<number>(searchLen);
  const B = new Array<number>(searchLen);
  const F = new Array<number>(M);
  const T = new Array<string>(searchLen);

  let maxScore = 0;
  let maxScorePos = 0;
  let pidx = 0;
  let lastIdx = 0;
  const pchar0 = pattern[0];
  let pchar = pattern[0];
  let prevH0 = 0;
  let prevClass = minIdx > 0 ? charClassOf(text[minIdx - 1]) : CharClass.White;
  let inGap = false;

  // Process each character in the search range
  for (let off = 0; off < searchLen; off++) {
    let char = text[minIdx + off];

    // Classify and optionally convert to lowercase
    const currClass = charClassOf(char);
    if (!caseSensitive && currClass === CharClass.Upper) {
      char = char.toLowerCase();
    }

    T[off] = char;

    // Calculate bonus
    const bonus = bonusMatrix[prevClass][currClass];
    B[off] = bonus;
    prevClass = currClass;

    // Track first occurrence of each pattern character
    if (char === pchar) {
      if (pidx < M) {
        F[pidx] = off;
        pidx++;
        pchar = pattern[min(pidx, M - 1)];
      }
      lastIdx = off;
    }

    // Calculate H0 and C0 for first pattern character
    if (char === pchar0) {
      const score = SCORE_MATCH + bonus * BONUS_FIRST_CHAR_MULTIPLIER;
      H0[off] = score;
      C0[off] = 1;

      // Prefer earlier match for forward search
      if (M === 1 && score > maxScore) {
        maxScore = score;
        maxScorePos = off;

        // Early exit for single-char pattern with boundary bonus
        if (bonus >= BONUS_BOUNDARY) {
          break;
        }
      }
      inGap = false;
    } else {
      if (inGap) {
        H0[off] = max(prevH0 + SCORE_GAP_EXTENSION, 0);
      } else {
        H0[off] = max(prevH0 + SCORE_GAP_START, 0);
      }
      C0[off] = 0;
      inGap = true;
    }

    prevH0 = H0[off];
  }

  // Not all pattern characters found
  if (pidx !== M) {
    return null;
  }

  // Single character pattern
  if (M === 1) {
    const result: FuzzyResult = {
      start: minIdx + maxScorePos,
      end: minIdx + maxScorePos + 1,
      score: maxScore,
    };

    if (withPos) {
      result.positions = [minIdx + maxScorePos];
    }

    return result;
  }

  // Phase 3: Fill in score matrix (dynamic programming)
  const f0 = F[0];
  const width = lastIdx - f0 + 1;
  const H = new Array<number>(width * M);
  const C = new Array<number>(width * M);

  // Copy first row
  for (let i = 0; i < width; i++) {
    H[i] = H0[f0 + i];
    C[i] = C0[f0 + i];
  }

  // Fill remaining rows
  for (let i = 1; i < M; i++) {
    const f = F[i];
    const pchar = pattern[i];
    const row = i * width;
    inGap = false;

    for (let j = f; j <= lastIdx; j++) {
      const col = j - f0;
      const char = T[j];

      let s1 = 0;
      let s2 = 0;
      let consecutive = 0;

      // Gap extension/start from left
      if (j > f) {
        if (inGap) {
          s2 = H[row + col - 1] + SCORE_GAP_EXTENSION;
        } else {
          s2 = H[row + col - 1] + SCORE_GAP_START;
        }
      }

      // Match from diagonal
      if (pchar === char) {
        s1 = H[row - width + col - 1] + SCORE_MATCH;
        let b = B[j];
        consecutive = C[row - width + col - 1] + 1;

        if (consecutive > 1) {
          const fb = B[j - consecutive + 1];
          // Break consecutive chunk
          if (b >= BONUS_BOUNDARY && b > fb) {
            consecutive = 1;
          } else {
            b = max(b, BONUS_CONSECUTIVE, fb);
          }
        }

        if (s1 + b < s2) {
          s1 += B[j];
          consecutive = 0;
        } else {
          s1 += b;
        }
      }

      C[row + col] = consecutive;
      inGap = s1 < s2;
      const score = max(s1, s2, 0);

      // Prefer earlier match for forward search
      if (i === M - 1 && score > maxScore) {
        maxScore = score;
        maxScorePos = j;
      }

      H[row + col] = score;
    }
  }

  // Phase 4: Backtrace to find character positions
  const positions: number[] = [];

  if (withPos) {
    let i = M - 1;
    let j = maxScorePos;
    let preferMatch = true;

    while (true) {
      const I = i * width;
      const j0 = j - f0;
      const s = H[I + j0];

      let s1 = 0;
      let s2 = 0;

      if (i > 0 && j >= F[i]) {
        s1 = H[I - width + j0 - 1];
      }

      if (j > F[i]) {
        s2 = H[I + j0 - 1];
      }

      if (s > s1 && (s > s2 || (s === s2 && preferMatch))) {
        positions.push(j + minIdx);
        if (i === 0) {
          break;
        }
        i--;
      }

      preferMatch =
        C[I + j0] > 1 ||
        (I + width + j0 + 1 < C.length && C[I + width + j0 + 1] > 0);
      j--;
    }

    // Reverse to get forward order
    positions.reverse();
  }

  return {
    start: minIdx + F[0],
    end: minIdx + maxScorePos + 1,
    score: maxScore,
    positions: withPos ? positions : undefined,
  };
}

/**
 * Fuzzy match a query against a target string
 *
 * @param query - The search query
 * @param target - The string to search in
 * @param withPos - Whether to return character positions
 * @param caseSensitive - Whether to perform case-sensitive matching (default: false)
 * @returns Match result with score and positions, or null if no match
 */
export function fuzzyMatch(
  query: string,
  target: string,
  withPos: boolean = false,
  caseSensitive: boolean = false,
): FuzzyResult | null {
  if (query.length === 0) {
    return { start: 0, end: 0, score: 0 };
  } else if (target.length === 0) {
    return null;
  }

  const pattern = caseSensitive
    ? query.split("")
    : query.toLowerCase().split("");

  return fuzzyMatchV2(target, pattern, withPos, caseSensitive);
}
