import { z } from "zod";

import { CommitFilterResource } from "./settings";

export const SyntaxHighlight = z.enum([
  "delimiter",
  "normal",
  "string",
  "type",
  "comment",
  "keyword",
  "tree_sitter_error",
]);
export type SyntaxHighlight = z.infer<typeof SyntaxHighlight>;

export const DiffChangeResource = z.object({
  start: z.number().int(),
  end: z.number().int(),
  content: z.string(),
  highlight: SyntaxHighlight,
});
export type DiffChangeResource = z.infer<typeof DiffChangeResource>;

export const DiffLineResource = z.object({
  line_number: z.number().int(),
  changes: z.array(DiffChangeResource),
});
export type DiffLineResource = z.infer<typeof DiffLineResource>;

export const DiffPairResource = z.object({
  lhs: DiffLineResource.optional(),
  rhs: DiffLineResource.optional(),
});
export type DiffPairResource = z.infer<typeof DiffPairResource>;

export const DiffHunkResource = z.array(DiffPairResource);
export type DiffHunkResource = z.infer<typeof DiffHunkResource>;

export const RepositoryFileResource = z.object({
  path: z.string(),
  sha: z.string(),
  content: z.string(),
  encoding: z.string(),
});
export type RepositoryFileResource = z.infer<typeof RepositoryFileResource>;

export const CommitAuthorResource = z.object({
  id: z.uuid().optional(),
  name: z.string(),
  email: z.string(),
});
export type CommitAuthorResource = z.infer<typeof CommitAuthorResource>;

export const RepositoryDiffStatResource = z.object({
  path: z.string(),
  lines_added: z.number().int(),
  lines_removed: z.number().int(),
});
export type RepositoryDiffStatResource = z.infer<
  typeof RepositoryDiffStatResource
>;

export const RepositoryDiffFileResource = z.object({
  path: z.string(),
  lines_added: z.number().int(),
  lines_removed: z.number().int(),
  hunks: z.array(DiffHunkResource),
  left_content: z.string().optional(),
  right_content: z.string().optional(),
});
export type RepositoryDiffFileResource = z.infer<
  typeof RepositoryDiffFileResource
>;

export const RepositoryCommitResource = z.object({
  sha: z.string(),
  parent_sha: z.string(),
  message: z.string(),
  date: z.iso.datetime(),
  author: CommitAuthorResource,
  review_number: z.number().int().optional(),
  diff_position: z.number().int().optional(),
  diffs: z.array(RepositoryDiffStatResource),
});
export type RepositoryCommitResource = z.infer<typeof RepositoryCommitResource>;

export const RepositoryCommitsResource = z.object({
  commits: z.array(RepositoryCommitResource),
});
export type RepositoryCommitsResource = z.infer<
  typeof RepositoryCommitsResource
>;

export const RepositoryCommitDiffResource = z.object({
  sha: z.string(),
  parent_sha: z.string(),
  files: z.array(RepositoryDiffFileResource),
});
export type RepositoryCommitDiffResource = z.infer<
  typeof RepositoryCommitDiffResource
>;

export const PathType = z.enum(["blob", "tree", "commit", "unknown"]);
export type PathType = z.infer<typeof PathType>;

export const RepositoryPathResource = z.object({
  path: z.string(),
  name: z.string(),
  path_type: PathType,
  sha: z.string(),
});
export type RepositoryPathResource = z.infer<typeof RepositoryPathResource>;

export const RepositoryPathsResource = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
  entries: z.array(RepositoryPathResource),
});
export type RepositoryPathsResource = z.infer<typeof RepositoryPathsResource>;

export const RepositoryFolderResource = z.object({
  type: z.literal("folder"),
  path: z.string(),
  entries: z.array(RepositoryPathResource),
});
export type RepositoryFolderResource = z.infer<typeof RepositoryFolderResource>;

export const RepositoryBlobResource = z.discriminatedUnion("type", [
  RepositoryFileResource.extend({ type: z.literal("file") }),
  RepositoryFolderResource,
]);
export type RepositoryBlobResource = z.infer<typeof RepositoryBlobResource>;

export const RepositoryBlobsResource = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
  blobs: z.array(RepositoryBlobResource),
});
export type RepositoryBlobsResource = z.infer<typeof RepositoryBlobsResource>;

export const RepositoryResource = z.object({
  id: z.uuid(),
  name: z.string(),
  owner: z.string(),
  visibility: z.string(),
  created_at: z.iso.datetime(),
});
export type RepositoryResource = z.infer<typeof RepositoryResource>;

export const RepositorySettingsResource = z.object({
  commit_filters: z.array(CommitFilterResource).optional(),
});
export type RepositorySettingsResource = z.infer<
  typeof RepositorySettingsResource
>;
