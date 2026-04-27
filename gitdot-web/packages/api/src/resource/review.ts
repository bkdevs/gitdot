import { z } from "zod";

export const ReviewAuthorResource = z.object({
  id: z.uuid(),
  name: z.string(),
});
export type ReviewAuthorResource = z.infer<typeof ReviewAuthorResource>;

export const ReviewVerdictResource = z.object({
  id: z.uuid(),
  diff_id: z.uuid(),
  revision_id: z.uuid(),
  reviewer_id: z.uuid(),
  verdict: z.string(),
  created_at: z.iso.datetime(),
});
export type ReviewVerdictResource = z.infer<typeof ReviewVerdictResource>;

export const RevisionResource = z.object({
  id: z.uuid(),
  diff_id: z.uuid(),
  number: z.number().int(),
  commit_hash: z.string(),
  parent_hash: z.string(),
  created_at: z.iso.datetime(),
  verdicts: z.array(ReviewVerdictResource),
});
export type RevisionResource = z.infer<typeof RevisionResource>;

export const ReviewStatus = z.enum(["draft", "in_progress", "closed"]);
export type ReviewStatus = z.infer<typeof ReviewStatus>;

export const DiffStatus = z.enum(["pending", "open", "merged"]);
export type DiffStatus = z.infer<typeof DiffStatus>;

export const ReviewDiffResource = z.object({
  id: z.uuid(),
  review_id: z.uuid(),
  position: z.number().int(),
  message: z.string(),
  status: DiffStatus,
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  revisions: z.array(RevisionResource),
});
export type ReviewDiffResource = z.infer<typeof ReviewDiffResource>;

export const ReviewerResource = z.object({
  id: z.uuid(),
  review_id: z.uuid(),
  reviewer_id: z.uuid(),
  created_at: z.iso.datetime(),
  user: ReviewAuthorResource.nullable(),
});
export type ReviewerResource = z.infer<typeof ReviewerResource>;

export const ReviewCommentResource = z.object({
  id: z.uuid(),
  review_id: z.uuid(),
  diff_id: z.uuid(),
  revision_id: z.uuid(),
  author_id: z.uuid(),
  parent_id: z.uuid().nullable(),
  body: z.string(),
  file_path: z.string().nullable(),
  line_number_start: z.number().int().nullable(),
  line_number_end: z.number().int().nullable(),
  start_character: z.number().int().nullable(),
  end_character: z.number().int().nullable(),
  side: z.string().nullable(),
  resolved: z.boolean(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  author: ReviewAuthorResource.nullable(),
});
export type ReviewCommentResource = z.infer<typeof ReviewCommentResource>;

export const ReviewResource = z.object({
  id: z.uuid(),
  number: z.number().int(),
  author_id: z.uuid(),
  repository_id: z.uuid(),
  title: z.string(),
  description: z.string(),
  target_branch: z.string(),
  status: ReviewStatus,
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  author: ReviewAuthorResource.nullable(),
  diffs: z.array(ReviewDiffResource),
  reviewers: z.array(ReviewerResource),
  comments: z.array(ReviewCommentResource),
});
export type ReviewResource = z.infer<typeof ReviewResource>;
