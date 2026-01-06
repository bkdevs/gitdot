import { z } from "zod";

export const CreateRepositoryRequestSchema = z.object({
  default_branch: z.string().default("main"),
});

export type CreateRepositoryRequest = z.infer<
  typeof CreateRepositoryRequestSchema
>;

export const CreateRepositoryResponseSchema = z.object({
  owner: z.string(),
  name: z.string(),
  default_branch: z.string(),
});

export type CreateRepositoryResponse = z.infer<
  typeof CreateRepositoryResponseSchema
>;

export const RepositoryTreeQuerySchema = z.object({
  ref_name: z.string().default("HEAD").optional(),
  path: z.string().default("").optional(),
});

export type RepositoryTreeQuery = z.infer<typeof RepositoryTreeQuerySchema>;

export const RepositoryTreeEntrySchema = z.object({
  path: z.string(),
  name: z.string(),
  entry_type: z.string(),
  sha: z.string(),
});

export type RepositoryTreeEntry = z.infer<typeof RepositoryTreeEntrySchema>;

export const RepositoryTreeSchema = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
  path: z.string(),
  entries: z.array(RepositoryTreeEntrySchema),
});

export type RepositoryTree = z.infer<typeof RepositoryTreeSchema>;

export const RepositoryFileQuerySchema = z.object({
  ref_name: z.string().default("HEAD").optional(),
  path: z.string(),
});

export type RepositoryFileQuery = z.infer<typeof RepositoryFileQuerySchema>;

export const RepositoryFileSchema = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
  path: z.string(),
  sha: z.string(),
  content: z.string(),
  encoding: z.string(),
});

export type RepositoryFile = z.infer<typeof RepositoryFileSchema>;

export const RepositoryCommitsQuerySchema = z.object({
  ref_name: z.string().default("HEAD"),
  page: z.number().int().positive().default(1),
  per_page: z.number().int().positive().default(30),
});

export type RepositoryCommitsQuery = z.infer<
  typeof RepositoryCommitsQuerySchema
>;

export const RepositoryCommitSchema = z.object({
  sha: z.string(),
  message: z.string(),
  author: z.string(),
  date: z.iso.datetime(),
});

export type RepositoryCommit = z.infer<typeof RepositoryCommitSchema>;

export const RepositoryCommitsSchema = z.object({
  commits: z.array(RepositoryCommitSchema),
  has_next: z.boolean(),
});

export type RepositoryCommits = z.infer<typeof RepositoryCommitsSchema>;
