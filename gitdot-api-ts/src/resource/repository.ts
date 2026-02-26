import { z } from "zod";

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

export const RepositoryDiffResource = z.object({
  lines_added: z.number().int(),
  lines_removed: z.number().int(),
  hunks: z.array(DiffHunkResource),
});
export type RepositoryDiffResource = z.infer<typeof RepositoryDiffResource>;

export const RepositoryFileResource = z.object({
  ref_name: z.string(),
  path: z.string(),
  commit_sha: z.string(),
  sha: z.string(),
  content: z.string(),
  encoding: z.string(),
});
export type RepositoryFileResource = z.infer<typeof RepositoryFileResource>;

export const RepositoryCommitDiffResource = z.object({
  diff: RepositoryDiffResource,
  left: RepositoryFileResource.nullable(),
  right: RepositoryFileResource.nullable(),
});
export type RepositoryCommitDiffResource = z.infer<
  typeof RepositoryCommitDiffResource
>;

export const CommitAuthorResource = z.object({
  id: z.string().uuid().optional(),
  name: z.string(),
  email: z.string(),
});
export type CommitAuthorResource = z.infer<typeof CommitAuthorResource>;

export const RepositoryCommitResource = z.object({
  sha: z.string(),
  parent_sha: z.string().optional(),
  message: z.string(),
  date: z.string().datetime(),
  author: CommitAuthorResource,
});
export type RepositoryCommitResource = z.infer<typeof RepositoryCommitResource>;

export const RepositoryCommitsResource = z.object({
  commits: z.array(RepositoryCommitResource),
  has_next: z.boolean(),
});
export type RepositoryCommitsResource = z.infer<
  typeof RepositoryCommitsResource
>;

export const FilePreviewResource = z.object({
  content: z.string(),
  total_lines: z.number().int(),
  preview_lines: z.number().int(),
  truncated: z.boolean(),
  encoding: z.string(),
});
export type FilePreviewResource = z.infer<typeof FilePreviewResource>;

export const RepositoryPreviewEntryResource = z.object({
  path: z.string(),
  name: z.string(),
  sha: z.string(),
  preview: FilePreviewResource.nullable(),
});
export type RepositoryPreviewEntryResource = z.infer<
  typeof RepositoryPreviewEntryResource
>;

export const RepositoryPreviewResource = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
  entries: z.array(RepositoryPreviewEntryResource),
});
export type RepositoryPreviewResource = z.infer<
  typeof RepositoryPreviewResource
>;

export const RepositoryTreeEntryResource = z.object({
  path: z.string(),
  name: z.string(),
  entry_type: z.string(),
  sha: z.string(),
  commit: RepositoryCommitResource,
});
export type RepositoryTreeEntryResource = z.infer<
  typeof RepositoryTreeEntryResource
>;

export const RepositoryTreeResource = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
  entries: z.array(RepositoryTreeEntryResource),
});
export type RepositoryTreeResource = z.infer<typeof RepositoryTreeResource>;

export const RepositoryCommitStatResource = z.object({
  path: z.string(),
  lines_added: z.number().int(),
  lines_removed: z.number().int(),
});
export type RepositoryCommitStatResource = z.infer<
  typeof RepositoryCommitStatResource
>;

export const RepositoryPermissionResource = z.object({
  permission: z.string(),
});
export type RepositoryPermissionResource = z.infer<
  typeof RepositoryPermissionResource
>;

export const RepositoryResource = z.object({
  id: z.string().uuid(),
  name: z.string(),
  owner: z.string(),
  visibility: z.string(),
  created_at: z.string().datetime(),
});
export type RepositoryResource = z.infer<typeof RepositoryResource>;
