import { diffFiles } from "../algo";

describe("diffFiles", () => {
  test("returns already-paired pairs plus per-side change sets", () => {
    // hunk:
    //   - alpha          (line 0 in left)
    //   + gamma          (line 0 in right)
    //     common_one     (line 1 in both)  <-- context anchor
    //     common_two     (line 2 in both)  <-- context anchor
    //     common_three   (line 3 in both)  <-- context anchor
    //   - beta           (line 4 in left)
    const left = `alpha\ncommon_one\ncommon_two\ncommon_three\nbeta\n`;
    const right = `gamma\ncommon_one\ncommon_two\ncommon_three\n`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [0, 0],
      [1, 1],
      [2, 2],
      [3, 3],
      [4, null],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set([0, 4]));
    expect(hunks[0].addedLines).toEqual(new Set([0]));
  });

  test("identical content produces no hunks", () => {
    const identical = "a\nb\nc\nd\n";
    expect(diffFiles(identical, identical)).toEqual([]);
  });
});
