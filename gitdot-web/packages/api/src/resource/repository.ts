import { z } from "zod";

import { BuildResource } from "./build";
import { QuestionResource } from "./question";
import { ReviewResource } from "./review";
import { UserResource } from "./user";

export const RepositoryFileResource = z.object({
  commit_sha: z.string(),
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
  left_content: z.string().optional(),
  right_content: z.string().optional(),
});
export type RepositoryDiffFileResource = z.infer<
  typeof RepositoryDiffFileResource
>;

export const RepositoryCommitResource = z.object({
  owner_name: z.string(),
  repo_name: z.string(),
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
  commit_sha: z.string(),
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
  blobs: z.array(RepositoryBlobResource),
});
export type RepositoryBlobsResource = z.infer<typeof RepositoryBlobsResource>;

export const RepositoryBlobDiffsResource = z.object({
  diffs: z.record(z.string(), RepositoryDiffFileResource),
});
export type RepositoryBlobDiffsResource = z.infer<
  typeof RepositoryBlobDiffsResource
>;

export const RepositoryResource = z.object({
  id: z.uuid(),
  name: z.string(),
  owner: z.string(),
  visibility: z.string(),
  description: z.string().optional(),
  stars: z.number().int().nonnegative(),
  user_star: z.boolean(),
  readonly: z.boolean(),
  created_at: z.iso.datetime(),
});
export type RepositoryResource = z.infer<typeof RepositoryResource>;

export const RepositoryQuestionsResource = z.object({
  questions: z.array(QuestionResource),
});
export type RepositoryQuestionsResource = z.infer<
  typeof RepositoryQuestionsResource
>;

export const RepositoryReviewsResource = z.object({
  reviews: z.array(ReviewResource),
});
export type RepositoryReviewsResource = z.infer<
  typeof RepositoryReviewsResource
>;

export const RepositoryBuildsResource = z.object({
  builds: z.array(BuildResource),
});
export type RepositoryBuildsResource = z.infer<typeof RepositoryBuildsResource>;

export const RepositoryResourcesResource = z.object({
  last_commit: z.string(),
  last_updated: z.iso.datetime().optional(),
  paths: RepositoryPathsResource.optional(),
  commits: RepositoryCommitsResource.optional(),
  blobs: RepositoryBlobsResource.optional(),
  questions: RepositoryQuestionsResource.optional(),
  reviews: RepositoryReviewsResource.optional(),
  builds: RepositoryBuildsResource.optional(),
});
export type RepositoryResourcesResource = z.infer<
  typeof RepositoryResourcesResource
>;

export const RepositoryActivityEventResource = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("starred"),
    user: UserResource,
    at: z.iso.datetime(),
  }),
]);
export type RepositoryActivityEventResource = z.infer<
  typeof RepositoryActivityEventResource
>;

export const RepositoryCommitFilterResource = z.object({
  id: z.uuid(),
  repository_id: z.uuid(),
  name: z.string(),
  authors: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  paths: z.array(z.string()).optional(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type RepositoryCommitFilterResource = z.infer<
  typeof RepositoryCommitFilterResource
>;
