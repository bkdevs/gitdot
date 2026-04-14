"use client";

import { useState } from "react";
import { cn } from "@/util";
import {
  Dialog,
  DialogContent,
  DialogClose,
} from "@/ui/dialog";

type DiffLine = {
  type: "add" | "remove" | "context" | "hunk";
  content: string;
  lineNo?: number;
};

type FileDiff = {
  path: string;
  isNew?: boolean;
  additions: number;
  deletions: number;
  lines: DiffLine[];
};

type CommitStatus = "pending" | "approved" | "merged" | "changes_requested";

type Commit = {
  sha: string;
  title: string;
  message: string;
  status: CommitStatus;
  files: FileDiff[];
};

const MOCK_COMMITS: Commit[] = [
  {
    sha: "a1b2c3d",
    title: "Extract diff actions to dedicated module",
    status: "merged",
    message:
      "Move DiffData, renderDiff, renderDiffs, renderCommitDiffAction, and getReviewAllDiffDataAction out of repository.ts and into a new app/actions/diff.ts. The review diff action is updated to call renderDiffs directly using the embedded left_content/right_content fields already present on RepositoryDiffFileResource, eliminating the redundant blob fetches that computeDiffDataFromBlobs was doing. computeDiffDataFromBlobs is deleted. Re-export from index.ts so existing @/actions imports are unaffected.",
    files: [
      {
        path: "app/actions/diff.ts",
        isNew: true,
        additions: 62,
        deletions: 0,
        lines: [
          { type: "add", content: '"use server";', lineNo: 1 },
          { type: "add", content: "", lineNo: 2 },
          {
            type: "add",
            content:
              'import type { RepositoryDiffFileResource } from "gitdot-api";',
            lineNo: 3,
          },
          {
            type: "add",
            content: 'import type { DiffHunkResource } from "gitdot-api";',
            lineNo: 4,
          },
          {
            type: "add",
            content: 'import type { Element } from "hast";',
            lineNo: 5,
          },
          { type: "add", content: "import {", lineNo: 6 },
          { type: "add", content: "  createChangeMaps,", lineNo: 7 },
          { type: "add", content: "  fileToHast,", lineNo: 8 },
          { type: "add", content: "  inferLanguage,", lineNo: 9 },
          { type: "add", content: "  mergeHunks,", lineNo: 10 },
          { type: "add", content: "  renderSpans,", lineNo: 11 },
          {
            type: "add",
            content: '} from "@/(main)/[owner]/[repo]/util";',
            lineNo: 12,
          },
          {
            type: "add",
            content: 'import { getRepositoryCommitDiff } from "@/dal";',
            lineNo: 13,
          },
          { type: "add", content: "", lineNo: 14 },
          { type: "add", content: "export type DiffData =", lineNo: 15 },
          {
            type: "add",
            content:
              '  | { kind: "split"; leftSpans: Element[]; rightSpans: Element[]; hunks: DiffHunkResource[] }',
            lineNo: 16,
          },
          {
            type: "add",
            content: '  | { kind: "single"; spans: Element[] }',
            lineNo: 17,
          },
          {
            type: "add",
            content: '  | { kind: "no-change" };',
            lineNo: 18,
          },
          { type: "hunk", content: "@@ +20,18 @@" },
          {
            type: "add",
            content: "async function renderDiff(",
            lineNo: 20,
          },
          {
            type: "add",
            content: "  file: RepositoryDiffFileResource,",
            lineNo: 21,
          },
          { type: "add", content: "): Promise<DiffData> {", lineNo: 22 },
          {
            type: "add",
            content: "  const lang = inferLanguage(file.path);",
            lineNo: 23,
          },
          {
            type: "add",
            content: "  if (!file.left_content && !file.right_content) {",
            lineNo: 24,
          },
          {
            type: "add",
            content: '    return { kind: "no-change" };',
            lineNo: 25,
          },
          { type: "add", content: "  }", lineNo: 26 },
          {
            type: "add",
            content: "  const hunks = mergeHunks(file.hunks);",
            lineNo: 27,
          },
          {
            type: "add",
            content: "  const { leftMap, rightMap } = createChangeMaps(hunks);",
            lineNo: 28,
          },
          { type: "hunk", content: "@@ +47,16 @@" },
          {
            type: "add",
            content: "export async function renderCommitDiffAction(",
            lineNo: 47,
          },
          { type: "add", content: "  owner: string,", lineNo: 48 },
          { type: "add", content: "  repo: string,", lineNo: 49 },
          { type: "add", content: "  sha: string,", lineNo: 50 },
          {
            type: "add",
            content: "): Promise<Record<string, DiffData>> {",
            lineNo: 51,
          },
          {
            type: "add",
            content:
              "  const result = await getRepositoryCommitDiff(owner, repo, sha);",
            lineNo: 52,
          },
          { type: "add", content: "  if (!result) return {};", lineNo: 53 },
          {
            type: "add",
            content: "  return renderDiffs(result.files);",
            lineNo: 54,
          },
          { type: "add", content: "}", lineNo: 55 },
          { type: "add", content: "", lineNo: 56 },
          {
            type: "add",
            content: "export async function renderReviewDiffAction(",
            lineNo: 57,
          },
          {
            type: "add",
            content: "  files: RepositoryDiffFileResource[],",
            lineNo: 58,
          },
          {
            type: "add",
            content: "): Promise<Record<string, DiffData>> {",
            lineNo: 59,
          },
          {
            type: "add",
            content: "  return renderDiffs(files);",
            lineNo: 60,
          },
          { type: "add", content: "}", lineNo: 61 },
          { type: "hunk", content: "@@ private helpers (unchanged) @@" },
          { type: "context", content: "// inferLanguage: maps file extension to shiki lang token", lineNo: 65 },
          { type: "context", content: "// mergeHunks: flattens adjacent hunks into contiguous ranges", lineNo: 66 },
          { type: "context", content: "// createChangeMaps: builds left/right line-number → change maps", lineNo: 67 },
          { type: "context", content: "// fileToHast: tokenises source text into a HAST tree via shiki", lineNo: 68 },
          { type: "context", content: "// renderSpans: walks HAST and emits <span> elements with inline styles", lineNo: 69 },
          { type: "context", content: "", lineNo: 70 },
          { type: "context", content: "// All helpers above are pure functions with no side-effects.", lineNo: 71 },
          { type: "context", content: "// They are intentionally kept private to this module so callers", lineNo: 72 },
          { type: "context", content: "// cannot depend on their signatures changing in the future.", lineNo: 73 },
          { type: "context", content: "", lineNo: 74 },
          { type: "context", content: "// The two exported actions follow the same contract:", lineNo: 75 },
          { type: "context", content: "//   input  → list of RepositoryDiffFileResource", lineNo: 76 },
          { type: "context", content: "//   output → Record<path, DiffData> resolved in parallel", lineNo: 77 },
          { type: "context", content: "", lineNo: 78 },
          { type: "context", content: "// DiffData discriminated union:", lineNo: 79 },
          { type: "context", content: '//   { kind: "split" }     — two tokenised sides + hunk ranges', lineNo: 80 },
          { type: "context", content: '//   { kind: "single" }    — single tokenised side (binary or large)', lineNo: 81 },
          { type: "context", content: '//   { kind: "no-change" } — file present in diff but content identical', lineNo: 82 },
        ],
      },
      {
        path: "app/actions/repository.ts",
        additions: 1,
        deletions: 26,
        lines: [
          { type: "context", content: '"use server";', lineNo: 1 },
          { type: "context", content: "", lineNo: 2 },
          {
            type: "remove",
            content:
              'import type { RepositoryDiffFileResource } from "gitdot-api";',
            lineNo: 3,
          },
          {
            type: "remove",
            content: 'import type { DiffHunkResource } from "gitdot-api";',
            lineNo: 4,
          },
          {
            type: "remove",
            content: 'import type { Element } from "hast";',
            lineNo: 5,
          },
          { type: "remove", content: "import {", lineNo: 6 },
          { type: "remove", content: "  createChangeMaps,", lineNo: 7 },
          { type: "remove", content: "  fileToHast,", lineNo: 8 },
          { type: "remove", content: "  inferLanguage,", lineNo: 9 },
          { type: "remove", content: "  mergeHunks,", lineNo: 10 },
          { type: "remove", content: "  renderSpans,", lineNo: 11 },
          {
            type: "remove",
            content: '} from "@/(main)/[owner]/[repo]/util";',
            lineNo: 12,
          },
          {
            type: "remove",
            content:
              'import { getRepositoryBlob, getRepositoryCommitDiff } from "@/dal";',
            lineNo: 13,
          },
          {
            type: "add",
            content: 'import { getRepositoryBlob } from "@/dal";',
            lineNo: 13,
          },
          {
            type: "context",
            content: 'import { createRepository } from "@/dal";',
            lineNo: 14,
          },
          { type: "hunk", content: "@@ -42,26 +28,0 @@" },
          {
            type: "remove",
            content: "export type DiffData =",
            lineNo: 42,
          },
          {
            type: "remove",
            content:
              '  | { kind: "split"; leftSpans: Element[]; rightSpans: Element[]; hunks: DiffHunkResource[] }',
            lineNo: 43,
          },
          {
            type: "remove",
            content: '  | { kind: "single"; spans: Element[] }',
            lineNo: 44,
          },
          {
            type: "remove",
            content: '  | { kind: "no-change" };',
            lineNo: 45,
          },
          { type: "remove", content: "", lineNo: 46 },
          {
            type: "remove",
            content: "async function computeDiffDataFromBlobs(",
            lineNo: 47,
          },
          { type: "remove", content: "  owner: string,", lineNo: 48 },
          { type: "remove", content: "  repo: string,", lineNo: 49 },
          {
            type: "remove",
            content: "  files: RepositoryDiffFileResource[],",
            lineNo: 50,
          },
          { type: "remove", content: "  sha: string,", lineNo: 51 },
          { type: "remove", content: "  parentSha: string,", lineNo: 52 },
          {
            type: "remove",
            content: "): Promise<Record<string, DiffData>> {",
            lineNo: 53,
          },
          {
            type: "remove",
            content: "  // fetch blobs for each file and compute diffs",
            lineNo: 54,
          },
          { type: "remove", content: "}", lineNo: 55 },
          { type: "remove", content: "", lineNo: 56 },
          {
            type: "remove",
            content: "export async function renderCommitDiffAction(",
            lineNo: 57,
          },
          {
            type: "remove",
            content:
              "): Promise<Record<string, DiffData>> { ... }",
            lineNo: 61,
          },
          { type: "remove", content: "", lineNo: 62 },
          {
            type: "remove",
            content: "export async function getReviewAllDiffDataAction(",
            lineNo: 63,
          },
          {
            type: "remove",
            content:
              "): Promise<Record<string, DiffData>> { ... }",
            lineNo: 67,
          },
          { type: "hunk", content: "@@ remaining repository actions (unchanged) @@" },
          { type: "context", content: "export async function createRepositoryAction(", lineNo: 70 },
          { type: "context", content: "  owner: string,", lineNo: 71 },
          { type: "context", content: "  input: CreateRepositoryInput,", lineNo: 72 },
          { type: "context", content: "): Promise<{ success: true } | { error: string }> {", lineNo: 73 },
          { type: "context", content: "  const result = await createRepository(owner, input);", lineNo: 74 },
          { type: "context", content: '  if (!result) return { error: "failed to create repository" };', lineNo: 75 },
          { type: "context", content: "  refresh();", lineNo: 76 },
          { type: "context", content: "  return { success: true };", lineNo: 77 },
          { type: "context", content: "}", lineNo: 78 },
          { type: "context", content: "", lineNo: 79 },
          { type: "context", content: "export async function deleteRepositoryAction(", lineNo: 80 },
          { type: "context", content: "  owner: string,", lineNo: 81 },
          { type: "context", content: "  repo: string,", lineNo: 82 },
          { type: "context", content: "): Promise<{ success: true } | { error: string }> {", lineNo: 83 },
          { type: "context", content: "  await deleteRepository(owner, repo);", lineNo: 84 },
          { type: "context", content: "  refresh();", lineNo: 85 },
          { type: "context", content: "  return { success: true };", lineNo: 86 },
          { type: "context", content: "}", lineNo: 87 },
          { type: "context", content: "", lineNo: 88 },
          { type: "context", content: "export async function getRepositoryBlobAction(", lineNo: 89 },
          { type: "context", content: "  owner: string,", lineNo: 90 },
          { type: "context", content: "  repo: string,", lineNo: 91 },
          { type: "context", content: "  path: string,", lineNo: 92 },
          { type: "context", content: "  ref: string,", lineNo: 93 },
          { type: "context", content: ") {", lineNo: 94 },
          { type: "context", content: "  return getRepositoryBlob(owner, repo, path, ref);", lineNo: 95 },
          { type: "context", content: "}", lineNo: 96 },
        ],
      },
      {
        path: "app/actions/index.ts",
        additions: 1,
        deletions: 0,
        lines: [
          {
            type: "context",
            content: 'export * from "./repository";',
            lineNo: 1,
          },
          {
            type: "add",
            content: 'export * from "./diff";',
            lineNo: 2,
          },
        ],
      },
    ],
  },
  {
    sha: "e4f5a6b",
    title: "Add ReviewDiffBody client component",
    status: "changes_requested",
    message:
      "Introduce ReviewDiffBody as a client component modeled after CommitBody. It accepts the files array and a diffData promise, unwraps the promise with use(), and maps each file through DiffBody. RepositoryDiffFileResource is structurally compatible with RepositoryDiffStatResource so no casting is needed.",
    files: [
      {
        path: "reviews/[number]/review-diff-body.tsx",
        isNew: true,
        additions: 24,
        deletions: 0,
        lines: [
          { type: "add", content: '"use client";', lineNo: 1 },
          { type: "add", content: "", lineNo: 2 },
          {
            type: "add",
            content:
              'import type { RepositoryDiffFileResource } from "gitdot-api";',
            lineNo: 3,
          },
          { type: "add", content: 'import { use } from "react";', lineNo: 4 },
          {
            type: "add",
            content: 'import type { DiffData } from "@/actions";',
            lineNo: 5,
          },
          {
            type: "add",
            content:
              'import { DiffBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-body";',
            lineNo: 6,
          },
          { type: "add", content: "", lineNo: 7 },
          {
            type: "add",
            content: "export function ReviewDiffBody({",
            lineNo: 8,
          },
          { type: "add", content: "  files,", lineNo: 9 },
          { type: "add", content: "  diffData,", lineNo: 10 },
          { type: "add", content: "}: {", lineNo: 11 },
          {
            type: "add",
            content: "  files: RepositoryDiffFileResource[];",
            lineNo: 12,
          },
          {
            type: "add",
            content: "  diffData: Promise<Record<string, DiffData>>;",
            lineNo: 13,
          },
          { type: "add", content: "}) {", lineNo: 14 },
          {
            type: "add",
            content: "  const diffMap = use(diffData);",
            lineNo: 15,
          },
          { type: "add", content: "", lineNo: 16 },
          { type: "add", content: "  return (", lineNo: 17 },
          {
            type: "add",
            content: '    <div className="flex flex-col">',
            lineNo: 18,
          },
          { type: "add", content: "      {files.map((stat) => (", lineNo: 19 },
          {
            type: "add",
            content:
              "        <DiffBody key={stat.path} stat={stat} diff={diffMap[stat.path]} />",
            lineNo: 20,
          },
          { type: "add", content: "      ))}", lineNo: 21 },
          { type: "add", content: "    </div>", lineNo: 22 },
          { type: "add", content: "  );", lineNo: 23 },
          { type: "add", content: "}", lineNo: 24 },
          { type: "hunk", content: "@@ usage notes @@" },
          { type: "context", content: "// ReviewDiffBody is intentionally decoupled from data fetching.", lineNo: 26 },
          { type: "context", content: "// The caller (ReviewDiffContent) is responsible for calling", lineNo: 27 },
          { type: "context", content: "// renderReviewDiffAction and passing the resulting promise in.", lineNo: 28 },
          { type: "context", content: "// This mirrors how CommitBody receives its diffData promise,", lineNo: 29 },
          { type: "context", content: "// keeping the two components structurally symmetric.", lineNo: 30 },
          { type: "context", content: "", lineNo: 31 },
          { type: "context", content: "// RepositoryDiffFileResource is a superset of", lineNo: 32 },
          { type: "context", content: "// RepositoryDiffStatResource — it carries the same fields plus", lineNo: 33 },
          { type: "context", content: "// left_content and right_content, so DiffBody accepts it without", lineNo: 34 },
          { type: "context", content: "// any casting or adapter layer.", lineNo: 35 },
          { type: "context", content: "", lineNo: 36 },
          { type: "context", content: "// Suspense boundary lives in ReviewDiffContent, not here,", lineNo: 37 },
          { type: "context", content: "// so the loading fallback can span the full list rather than", lineNo: 38 },
          { type: "context", content: "// appearing once per file.", lineNo: 39 },
        ],
      },
    ],
  },
  {
    sha: "c7d8e9f",
    title: "Refactor review diff to use renderReviewDiffAction",
    status: "pending",
    message:
      "Replace the old blob-fetching flow in ReviewDiffContent with renderReviewDiffAction + ReviewDiffBody wrapped in Suspense. Removes imports of DiffBody, DiffFileClient, and the NULL_SHA constant which are no longer referenced. The per-file DiffFileClient wrapper is gone — ReviewDiffBody handles the full list.",
    files: [
      {
        path: "reviews/[number]/review-diff-content.tsx",
        additions: 7,
        deletions: 8,
        lines: [
          {
            type: "remove",
            content:
              'import { getReviewAllDiffDataAction } from "@/actions/repository";',
            lineNo: 1,
          },
          {
            type: "remove",
            content:
              'import { DiffBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-body";',
            lineNo: 2,
          },
          {
            type: "remove",
            content:
              'import { DiffFileClient } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-file-client";',
            lineNo: 3,
          },
          {
            type: "remove",
            content:
              'const NULL_SHA = "0000000000000000000000000000000000000000";',
            lineNo: 4,
          },
          {
            type: "add",
            content: 'import { renderReviewDiffAction } from "@/actions";',
            lineNo: 1,
          },
          {
            type: "add",
            content: 'import { Suspense } from "react";',
            lineNo: 2,
          },
          {
            type: "add",
            content: 'import { ReviewDiffBody } from "./review-diff-body";',
            lineNo: 3,
          },
          { type: "hunk", content: "@@ -18,12 +14,8 @@" },
          {
            type: "context",
            content:
              "export function ReviewDiffContent({ owner, repo, review }) {",
            lineNo: 14,
          },
          {
            type: "context",
            content: "  const diffResponse = use(review);",
            lineNo: 15,
          },
          {
            type: "context",
            content: "  if (!diffResponse) return null;",
            lineNo: 16,
          },
          {
            type: "remove",
            content:
              "  const allDiffDataPromise = getReviewAllDiffDataAction(",
            lineNo: 17,
          },
          {
            type: "remove",
            content: "    owner, repo, diffResponse.files, sha, parentSha,",
            lineNo: 18,
          },
          { type: "remove", content: "  );", lineNo: 19 },
          {
            type: "remove",
            content: "  return diffResponse.files.map((stat) => (",
            lineNo: 20,
          },
          {
            type: "remove",
            content: "    <DiffFileClient key={stat.path}>",
            lineNo: 21,
          },
          {
            type: "remove",
            content:
              "      <DiffBody path={stat.path} allDiffDataPromise={allDiffDataPromise} />",
            lineNo: 22,
          },
          {
            type: "remove",
            content: "    </DiffFileClient>",
            lineNo: 23,
          },
          { type: "remove", content: "  ));", lineNo: 24 },
          {
            type: "add",
            content:
              "  const diffData = renderReviewDiffAction(diffResponse.files);",
            lineNo: 17,
          },
          { type: "add", content: "  return (", lineNo: 18 },
          {
            type: "add",
            content:
              '    <Suspense fallback={<div className="p-4 text-sm text-muted-foreground">loading...</div>}>',
            lineNo: 19,
          },
          {
            type: "add",
            content:
              "      <ReviewDiffBody files={diffResponse.files} diffData={diffData} />",
            lineNo: 20,
          },
          { type: "add", content: "    </Suspense>", lineNo: 21 },
          { type: "add", content: "  );", lineNo: 22 },
          { type: "add", content: "}", lineNo: 23 },
          { type: "hunk", content: "@@ surrounding context (unchanged) @@" },
          { type: "context", content: "// ReviewDiffContent is a client component because it calls", lineNo: 25 },
          { type: "context", content: "// use() on the review promise passed down from the page.", lineNo: 26 },
          { type: "context", content: "", lineNo: 27 },
          { type: "context", content: "// The Suspense wrapper here is intentional — ReviewDiffBody", lineNo: 28 },
          { type: "context", content: "// calls use(diffData) which suspends until renderReviewDiffAction", lineNo: 29 },
          { type: "context", content: "// resolves. Without Suspense the component would throw.", lineNo: 30 },
          { type: "context", content: "", lineNo: 31 },
          { type: "context", content: '// The fallback "loading..." div matches the minimum height of', lineNo: 32 },
          { type: "context", content: "// a single diff file card to avoid layout shift on resolution.", lineNo: 33 },
          { type: "context", content: "", lineNo: 34 },
          { type: "context", content: "// owner and repo are still threaded through for potential", lineNo: 35 },
          { type: "context", content: "// future use (e.g. comment submission endpoints).", lineNo: 36 },
        ],
      },
    ],
  },
  {
    sha: "0a1b2c3",
    title: "Fix DiffData import paths",
    status: "approved",
    message:
      "Update the DiffData type import in commit-body.tsx and commit-client.tsx from @/actions/repository to @/actions now that the type is exported from the new diff.ts module. No logic changes.",
    files: [
      {
        path: "commits/[sha]/ui/commit-body.tsx",
        additions: 1,
        deletions: 1,
        lines: [
          { type: "context", content: '"use client";', lineNo: 1 },
          { type: "context", content: "", lineNo: 2 },
          {
            type: "remove",
            content: 'import type { DiffData } from "@/actions/repository";',
            lineNo: 3,
          },
          {
            type: "add",
            content: 'import type { DiffData } from "@/actions";',
            lineNo: 3,
          },
          {
            type: "context",
            content:
              'import type { RepositoryDiffStatResource } from "gitdot-api";',
            lineNo: 4,
          },
          {
            type: "context",
            content: 'import { use } from "react";',
            lineNo: 5,
          },
          { type: "context", content: 'import { Suspense } from "react";', lineNo: 6 },
          { type: "context", content: 'import type { DiffFile } from "@/(main)/[owner]/[repo]/util";', lineNo: 7 },
          { type: "context", content: "", lineNo: 8 },
          { type: "context", content: "type CommitBodyProps = {", lineNo: 9 },
          { type: "context", content: "  files: RepositoryDiffStatResource[];", lineNo: 10 },
          { type: "context", content: "  diffData: Promise<Record<string, DiffData>>;", lineNo: 11 },
          { type: "context", content: "};", lineNo: 12 },
          { type: "context", content: "", lineNo: 13 },
          { type: "context", content: "export function CommitBody({ files, diffData }: CommitBodyProps) {", lineNo: 14 },
          { type: "context", content: "  const diffMap = use(diffData);", lineNo: 15 },
          { type: "context", content: "  return (", lineNo: 16 },
          { type: "context", content: '    <div className="flex flex-col">', lineNo: 17 },
          { type: "context", content: "      {files.map((stat) => (", lineNo: 18 },
          { type: "context", content: "        <DiffFile key={stat.path} diff={stat} data={diffMap[stat.path]} />", lineNo: 19 },
          { type: "context", content: "      ))}", lineNo: 20 },
          { type: "context", content: "    </div>", lineNo: 21 },
          { type: "context", content: "  );", lineNo: 22 },
          { type: "context", content: "}", lineNo: 23 },
        ],
      },
      {
        path: "commits/[sha]/ui/commit-client.tsx",
        additions: 1,
        deletions: 1,
        lines: [
          { type: "context", content: '"use client";', lineNo: 1 },
          { type: "context", content: "", lineNo: 2 },
          {
            type: "remove",
            content: 'import type { DiffData } from "@/actions/repository";',
            lineNo: 3,
          },
          {
            type: "add",
            content: 'import type { DiffData } from "@/actions";',
            lineNo: 3,
          },
          {
            type: "context",
            content: 'import { renderCommitDiffAction } from "@/actions";',
            lineNo: 4,
          },
          {
            type: "context",
            content: 'import type { CommitResource } from "gitdot-api";',
            lineNo: 5,
          },
          { type: "context", content: 'import type { RepositoryDiffStatResource } from "gitdot-api";', lineNo: 6 },
          { type: "context", content: "", lineNo: 7 },
          { type: "context", content: "export function CommitClient({", lineNo: 8 },
          { type: "context", content: "  owner,", lineNo: 9 },
          { type: "context", content: "  repo,", lineNo: 10 },
          { type: "context", content: "  commit,", lineNo: 11 },
          { type: "context", content: "}: {", lineNo: 12 },
          { type: "context", content: "  owner: string;", lineNo: 13 },
          { type: "context", content: "  repo: string;", lineNo: 14 },
          { type: "context", content: "  commit: CommitResource;", lineNo: 15 },
          { type: "context", content: "}) {", lineNo: 16 },
          { type: "context", content: "  const diffData = renderCommitDiffAction(owner, repo, commit.sha);", lineNo: 17 },
          { type: "context", content: "  return (", lineNo: 18 },
          { type: "context", content: "    <Suspense fallback={null}>", lineNo: 19 },
          { type: "context", content: "      <CommitBody files={commit.files} diffData={diffData} />", lineNo: 20 },
          { type: "context", content: "    </Suspense>", lineNo: 21 },
          { type: "context", content: "  );", lineNo: 22 },
          { type: "context", content: "}", lineNo: 23 },
        ],
      },
    ],
  },
];

function StatusBadge({ status }: { status: CommitStatus }) {
  const styles: Record<CommitStatus, string> = {
    pending: "text-foreground",
    approved: "text-green-600 dark:text-green-400",
    merged: "text-muted-foreground underline underline-offset-2",
    changes_requested: "text-red-500 dark:text-red-400",
  };
  const labels: Record<CommitStatus, string> = {
    pending: "pending",
    approved: "approved",
    merged: "merged",
    changes_requested: "changes",
  };
  return (
    <span className={cn("text-[10px] font-mono shrink-0", styles[status])}>
      {labels[status]}
    </span>
  );
}

type SplitRow =
  | { kind: "hunk"; content: string }
  | {
      kind: "line";
      left: { lineNo: number; content: string; type: "context" | "remove" } | null;
      right: { lineNo: number; content: string; type: "context" | "add" } | null;
    };

function toSplitRows(lines: DiffLine[]): SplitRow[] {
  const rows: SplitRow[] = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    if (line.type === "hunk") {
      rows.push({ kind: "hunk", content: line.content });
      i++;
      continue;
    }
    if (line.type === "context") {
      rows.push({
        kind: "line",
        left: { lineNo: line.lineNo ?? 0, content: line.content, type: "context" },
        right: { lineNo: line.lineNo ?? 0, content: line.content, type: "context" },
      });
      i++;
      continue;
    }
    const removes: DiffLine[] = [];
    const adds: DiffLine[] = [];
    while (i < lines.length && lines[i].type === "remove") { removes.push(lines[i]); i++; }
    while (i < lines.length && lines[i].type === "add") { adds.push(lines[i]); i++; }
    const maxLen = Math.max(removes.length, adds.length);
    for (let j = 0; j < maxLen; j++) {
      const rem = removes[j];
      const add = adds[j];
      rows.push({
        kind: "line",
        left: rem ? { lineNo: rem.lineNo ?? 0, content: rem.content, type: "remove" } : null,
        right: add ? { lineNo: add.lineNo ?? 0, content: add.content, type: "add" } : null,
      });
    }
  }
  return rows;
}

function FileDiffDialog({ file, open, onClose }: { file: FileDiff; open: boolean; onClose: () => void }) {
  const splitRows = toSplitRows(file.lines);
  return (
    <Dialog open={open} onOpenChange={(v) => { if (!v) onClose(); }}>
      <DialogContent
        showOverlay
        className="w-[80vw] !max-w-[80vw] sm:!max-w-[80vw] h-[90vh] p-0 flex flex-col gap-0 rounded-lg overflow-hidden"
      >
        {/* Header */}
        <div className="flex items-center gap-3 px-4 py-3 border-b border-border shrink-0">
          {file.isNew && (
            <span className="text-[10px] font-sans bg-green-500/15 text-green-600 dark:text-green-400 px-1.5 rounded leading-4 shrink-0">
              new
            </span>
          )}
          <span className="font-mono text-xs text-foreground/70 flex-1 truncate">
            {file.path}
          </span>
          <span className="text-[10px] font-mono text-green-600 dark:text-green-400 shrink-0">
            +{file.additions}
          </span>
          <span className="text-[10px] font-mono text-red-500 dark:text-red-400 shrink-0">
            -{file.deletions}
          </span>
          <DialogClose className="text-muted-foreground hover:text-foreground transition-colors text-xs cursor-pointer shrink-0">
            ✕
          </DialogClose>
        </div>

        {/* Split diff */}
        <div className="flex-1 overflow-y-auto font-mono text-xs leading-5">
          {/* Column headers */}
          <div className="flex divide-x divide-border/50 border-b border-border bg-muted/20 select-none">
            <div className="w-1/2 px-3 py-1 text-[10px] text-muted-foreground">before</div>
            <div className="w-1/2 px-3 py-1 text-[10px] text-muted-foreground">after</div>
          </div>
          {splitRows.map((row, i) => {
            if (row.kind === "hunk") {
              return (
                <div
                  key={i}
                  className="px-3 py-0.5 bg-blue-500/10 text-blue-500 dark:text-blue-400 select-none"
                >
                  {row.content}
                </div>
              );
            }
            return (
              <div key={i} className="flex divide-x divide-border/40">
                {/* Left */}
                <div
                  className={cn(
                    "flex items-start w-1/2 min-w-0",
                    row.left?.type === "remove" && "bg-diff-red",
                    !row.left && "bg-muted/10",
                  )}
                >
                  {row.left ? (
                    <>
                      <span className="w-10 text-right shrink-0 pr-2 text-primary/30 select-none">
                        {row.left.lineNo || ""}
                      </span>
                      <span
                        className={cn(
                          "w-4 shrink-0 select-none text-center",
                          row.left.type === "remove" && "text-red-600 dark:text-red-400 font-bold",
                          row.left.type === "context" && "text-transparent",
                        )}
                      >
                        {row.left.type === "remove" ? "-" : " "}
                      </span>
                      <span className="whitespace-pre pr-3 overflow-hidden">{row.left.content}</span>
                    </>
                  ) : null}
                </div>
                {/* Right */}
                <div
                  className={cn(
                    "flex items-start w-1/2 min-w-0",
                    row.right?.type === "add" && "bg-diff-green",
                    !row.right && "bg-muted/10",
                  )}
                >
                  {row.right ? (
                    <>
                      <span className="w-10 text-right shrink-0 pr-2 text-primary/30 select-none">
                        {row.right.lineNo || ""}
                      </span>
                      <span
                        className={cn(
                          "w-4 shrink-0 select-none text-center",
                          row.right.type === "add" && "text-green-600 dark:text-green-400 font-bold",
                          row.right.type === "context" && "text-transparent",
                        )}
                      >
                        {row.right.type === "add" ? "+" : " "}
                      </span>
                      <span className="whitespace-pre pr-3 overflow-hidden">{row.right.content}</span>
                    </>
                  ) : null}
                </div>
              </div>
            );
          })}
        </div>
      </DialogContent>
    </Dialog>
  );
}

function FileDiffView({ file }: { file: FileDiff }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <div className="rounded-lg border border-border overflow-hidden">
        <button
          type="button"
          onClick={() => setOpen(true)}
          className="w-full flex items-center gap-2 px-3 py-1.5 border-b border-border hover:bg-muted/30 transition-colors cursor-pointer text-left"
        >
          {file.isNew && (
            <span className="text-[10px] font-sans bg-green-500/15 text-green-600 dark:text-green-400 px-1 rounded leading-4">
              new
            </span>
          )}
          <span className="font-mono text-[11px] text-muted-foreground flex-1 truncate">
            {file.path}
          </span>
          <span className="text-[10px] font-mono text-green-600 dark:text-green-400 shrink-0">
            +{file.additions}
          </span>
          <span className="text-[10px] font-mono text-red-500 dark:text-red-400 shrink-0">
            -{file.deletions}
          </span>
          <span className="text-[10px] text-muted-foreground shrink-0">↗</span>
        </button>
        <div className="font-mono text-xs leading-5">
          {file.lines.map((line, i) => {
            if (line.type === "hunk") {
              return (
                <div
                  key={i}
                  className="px-3 py-0.5 bg-blue-500/10 text-blue-500 dark:text-blue-400 select-none"
                >
                  {line.content}
                </div>
              );
            }
            return (
              <div
                key={i}
                className={cn(
                  "flex items-start",
                  line.type === "add" && "bg-diff-green",
                  line.type === "remove" && "bg-diff-red",
                )}
              >
                <span className="w-10 text-right shrink-0 pr-2 text-primary/30 select-none">
                  {line.lineNo ?? ""}
                </span>
                <span
                  className={cn(
                    "w-4 shrink-0 select-none text-center",
                    line.type === "add" && "text-green-600 dark:text-green-400 font-bold",
                    line.type === "remove" && "text-red-600 dark:text-red-400 font-bold",
                    line.type === "context" && "text-transparent",
                  )}
                >
                  {line.type === "add" ? "+" : line.type === "remove" ? "-" : " "}
                </span>
                <span className="whitespace-pre pr-4">{line.content}</span>
              </div>
            );
          })}
        </div>
      </div>
      <FileDiffDialog file={file} open={open} onClose={() => setOpen(false)} />
    </>
  );
}

export function DiffTabs() {
  const [activeIndex, setActiveIndex] = useState(0);
  const [scrolled, setScrolled] = useState(false);
  const [statuses, setStatuses] = useState<Record<string, CommitStatus>>(
    () => Object.fromEntries(MOCK_COMMITS.map((c) => [c.sha, c.status])),
  );
  const activeCommit = MOCK_COMMITS[activeIndex];

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* Commit list — collapses to active row on scroll */}
      <div className="shrink-0 border-b border-border overflow-hidden transition-all duration-200">
        {MOCK_COMMITS.map((commit, i) => {
          const isActive = i === activeIndex;
          if (scrolled && !isActive) return null;
          return (
            <button
              key={commit.sha}
              type="button"
              onClick={() => { setActiveIndex(i); setScrolled(false); }}
              className={cn(
                "w-full flex items-center gap-1.5 px-2 h-8 text-left border-b border-border last:border-b-0 cursor-pointer transition-colors",
                isActive
                  ? "bg-sidebar text-foreground"
                  : "text-muted-foreground hover:text-foreground hover:bg-sidebar/60",
              )}
            >
              <span className="font-mono text-sm text-muted-foreground shrink-0 w-5 text-right">
                {i + 1}.
              </span>
              <span className="text-xs flex-1 truncate">{commit.title}</span>
              <StatusBadge status={statuses[commit.sha]} />
            </button>
          );
        })}
      </div>

      {/* Stacked file diffs */}
      <div
        className="flex-1 overflow-y-auto"
        onScroll={(e) => setScrolled(e.currentTarget.scrollTop > 10)}
      >
        <div className="max-w-3xl mx-auto px-1 pt-6">
            <div className="flex items-center gap-2 mb-2">
              <p className="text-sm font-semibold flex-1">{activeCommit.title}</p>
              {statuses[activeCommit.sha] === "pending" && (
                <div className="flex items-stretch border border-border rounded-xs overflow-hidden shrink-0">
                  <button
                    type="button"
                    onClick={() => setStatuses((prev) => ({ ...prev, [activeCommit.sha]: "changes_requested" }))}
                    className="text-xs font-mono px-2.5 py-1 bg-background hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer transition-colors"
                  >
                    Reject
                  </button>
                  <div className="w-px bg-border shrink-0" />
                  <button
                    type="button"
                    onClick={() => setStatuses((prev) => ({ ...prev, [activeCommit.sha]: "approved" }))}
                    className="text-xs font-mono px-2.5 py-1 bg-primary text-primary-foreground hover:bg-primary/90 cursor-pointer transition-colors"
                  >
                    Approve
                  </button>
                </div>
              )}
              {statuses[activeCommit.sha] === "approved" && (
                <button
                  type="button"
                  onClick={() => setStatuses((prev) => ({ ...prev, [activeCommit.sha]: "merged" }))}
                  className="text-xs font-mono px-2.5 py-1 rounded-xs bg-primary text-primary-foreground hover:bg-primary/90 cursor-pointer transition-colors shrink-0"
                >
                  Merge
                </button>
              )}
              {statuses[activeCommit.sha] === "merged" && (
                <span className="text-xs font-mono text-muted-foreground underline underline-offset-2 shrink-0">
                  merged
                </span>
              )}
            </div>
            <p className="text-xs text-muted-foreground leading-relaxed">
              {activeCommit.message}
            </p>
          </div>
          <div className="max-w-3xl mx-auto flex flex-col gap-3 py-4">
            {activeCommit.files.map((file) => (
              <FileDiffView key={file.path} file={file} />
            ))}
          </div>
      </div>
    </div>
  );
}
