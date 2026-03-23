import type { RepositoryDiffStatResource } from "gitdot-api";
import { computePrimaryPaths } from "../util";

function d(path: string, added = 10, removed = 0): RepositoryDiffStatResource {
  return { path, lines_added: added, lines_removed: removed };
}

function paths(diffs: RepositoryDiffStatResource[], n?: number) {
  return computePrimaryPaths(diffs, n).map((r) => r.path);
}

describe("computePrimaryPaths", () => {
  // --- empty / trivial ---

  test("empty diffs returns empty", () => {
    expect(computePrimaryPaths([])).toEqual([]);
  });

  test("single root-level file shows filename", () => {
    expect(paths([d("README.md")])).toEqual(["README.md"]);
  });

  test("single nested file shows parent/filename", () => {
    expect(paths([d("src/main.rs")])).toEqual(["src/main.rs"]);
  });

  test("single deeply nested file shows parent/filename", () => {
    expect(paths([d("src/components/ui/Button.tsx")])).toEqual([
      "ui/Button.tsx",
    ]);
  });

  // --- multi-file, single directory ---

  test("several files in same flat dir yields one group", () => {
    // All files sit directly in src/ — no subdirs to split into.
    const diffs = [d("src/a.ts"), d("src/b.ts"), d("src/c.ts"), d("src/d.ts")];
    expect(paths(diffs)).toEqual(["src/"]);
  });

  test("files in subdirs get split to fill n groups", () => {
    // src/components/ × 2  +  src/utils/ × 1  +  src/hooks/ × 1
    // Depth-1 gives 1 group; splitting repeatedly reaches 3 groups.
    // Single-file groups show parent/filename; multi-file groups show the dir.
    const diffs = [
      d("src/components/Foo.tsx"),
      d("src/components/Bar.tsx"),
      d("src/utils/format.ts"),
      d("src/hooks/useX.ts"),
    ];
    expect(paths(diffs)).toEqual(
      expect.arrayContaining([
        "src/components/",
        "utils/format.ts",
        "hooks/useX.ts",
      ]),
    );
    expect(paths(diffs)).toHaveLength(3);
  });

  // --- multiple top-level dirs ---

  test("files in 3 top-level dirs produce 3 groups", () => {
    // Each is a single-file group — shows parent/filename.
    const diffs = [d("app/main.ts"), d("lib/util.ts"), d("tests/spec.ts")];
    expect(paths(diffs)).toHaveLength(3);
    expect(paths(diffs)).toEqual(
      expect.arrayContaining(["app/main.ts", "lib/util.ts", "tests/spec.ts"]),
    );
  });

  test("more than n top-level dirs returns top n by churn", () => {
    const diffs = [
      d("alpha/a.ts", 100),
      d("beta/b.ts", 50),
      d("gamma/c.ts", 20),
      d("delta/d.ts", 5),
    ];
    expect(paths(diffs)).toEqual(["alpha/a.ts", "beta/b.ts", "gamma/c.ts"]);
  });

  // --- single-file group display ---

  test("single-file group shows parent/filename when it fits", () => {
    const result = computePrimaryPaths([d("model/review.rs")]);
    expect(result[0].path).toBe("model/review.rs");
  });

  test("single-file group falls back to filename when parent/filename is too long", () => {
    // parent dir name alone exceeds MAX_PATH_LEN when combined with filename
    const result = computePrimaryPaths([
      d("a-very-long-directory-name/another-long-name/file.ts"),
    ]);
    // should truncate to fit — either filename alone or truncated dir
    expect(result[0].path.length).toBeLessThanOrEqual(22);
  });

  // --- compact path deduplication ---

  test("two single-file groups with the same filename fall back to dir keys", () => {
    const diffs = [d("src/components/index.ts"), d("lib/components/index.ts")];
    const result = paths(diffs);
    // Both would be "components/index.ts" — must disambiguate
    expect(result[0]).not.toBe(result[1]);
    expect(result).not.toContain("components/index.ts");
  });

  test("unique compact paths are kept even when another group is a dir", () => {
    const diffs = [
      d("src/api/client.ts"),
      d("src/utils/a.ts"),
      d("src/utils/b.ts"),
    ];
    const result = paths(diffs);
    // src/utils has 2 files → stays as dir key; src/api has 1 → compact
    expect(result).toContain("api/client.ts");
    expect(result).toContain("src/utils/");
  });

  // --- depth-adaptive splitting ---

  test("two groups when files can only split into two distinct dirs", () => {
    // All files are in src/a/ or src/b/ — can't reach 3 groups.
    const diffs = [
      d("src/a/one.ts"),
      d("src/a/two.ts"),
      d("src/b/three.ts"),
      d("src/b/four.ts"),
    ];
    expect(paths(diffs)).toHaveLength(2);
    expect(paths(diffs)).toEqual(expect.arrayContaining(["src/a/", "src/b/"]));
  });

  test("prefers splitting the largest group first", () => {
    // src has 3 files: 1 in models/, 2 in routes/. lib has 1 file.
    // Depth-1 → 2 groups. Split src → src/models (1 file) + src/routes (2 files) → 3 groups.
    // src/routes stays as a dir key (2 files); src/models shows parent/filename.
    const diffs = [
      d("src/models/user.ts", 80),
      d("src/routes/auth.ts", 60),
      d("src/routes/repo.ts", 40),
      d("lib/util.ts", 5),
    ];
    const result = paths(diffs);
    expect(result).toHaveLength(3);
    expect(result).toContain("src/routes/"); // multi-file dir group
    expect(result).toContain("models/user.ts"); // single-file compact
    expect(result).toContain("lib/util.ts"); // single-file compact
  });

  // --- root-level files ---

  test("root-level file uses filename directly", () => {
    expect(paths([d("Makefile")])).toEqual(["Makefile"]);
  });

  test("mix of root-level and nested files", () => {
    const diffs = [d("README.md"), d("src/main.ts"), d("src/util.ts")];
    const result = paths(diffs);
    expect(result).toContain("README.md");
  });

  // --- churn ordering ---

  test("groups are sorted by total churn descending", () => {
    const diffs = [d("a/x.ts", 5, 0), d("b/y.ts", 0, 100), d("c/z.ts", 50, 50)];
    const result = paths(diffs);
    expect(result[0]).toBe("b/y.ts"); // 100 removed
    expect(result[1]).toBe("c/z.ts"); // 100 total
    expect(result[2]).toBe("a/x.ts"); // 5 added
  });

  // --- custom n ---

  test("n=1 returns only the highest-churn group", () => {
    const diffs = [d("a/x.ts", 100), d("b/y.ts", 10), d("c/z.ts", 1)];
    expect(paths(diffs, 1)).toEqual(["a/x.ts"]);
  });

  test("n=5 returns all groups when fewer than 5 files", () => {
    const diffs = [d("a/x.ts"), d("b/y.ts")];
    expect(paths(diffs, 5)).toHaveLength(2);
  });
});
