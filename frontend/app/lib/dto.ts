import { z } from "zod";

export const CreateRepositoryRequestSchema = z.object({
  owner_type: z.string(),
  visibility: z.string().default("public"),
});

export type CreateRepositoryRequest = z.infer<
  typeof CreateRepositoryRequestSchema
>;

export const CreateRepositoryResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  owner_name: z.string(),
  visibility: z.string(),
  created_at: z.iso.datetime(),
});

export type CreateRepositoryResponse = z.infer<
  typeof CreateRepositoryResponseSchema
>;

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

export const RepositoryTreeQuerySchema = z.object({
  ref_name: z.string().default("HEAD").optional(),
  path: z.string().default("").optional(),
});

export type RepositoryTreeQuery = z.infer<typeof RepositoryTreeQuerySchema>;

export const RepositoryTreeEntrySchema = z.object({
  path: z.string(),
  name: z.string(),
  entry_type: z.enum(["blob", "tree"]),
  sha: z.string(),
  commit: RepositoryCommitSchema,
  preview: z.string().optional(),
});

export type RepositoryTreeEntry = z.infer<typeof RepositoryTreeEntrySchema>;

export const RepositoryTreeSchema = z.object({
  ref_name: z.string(),
  commit_sha: z.string(),
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

export const RepositoryFileCommitsQuerySchema = z.object({
  path: z.string(),
  ref_name: z.string().default("HEAD").optional(),
  page: z.number().int().positive().default(1).optional(),
  per_page: z.number().int().positive().default(30).optional(),
});

export type RepositoryFileCommitsQuery = z.infer<
  typeof RepositoryFileCommitsQuerySchema
>;

export const SyntaxHighlightSchema = z.enum([
  "delimiter",
  "normal",
  "string",
  "type",
  "comment",
  "keyword",
  "tree_sitter_error",
]);

export type SyntaxHighlight = z.infer<typeof SyntaxHighlightSchema>;

export const DiffChangeSchema = z.object({
  start: z.number(),
  end: z.number(),
  content: z.string(),
  highlight: SyntaxHighlightSchema,
});

export type DiffChange = z.infer<typeof DiffChangeSchema>;

export const DiffSideSchema = z.object({
  line_number: z.number(),
  changes: z.array(DiffChangeSchema),
});

export type DiffSide = z.infer<typeof DiffSideSchema>;

export const DiffLineSchema = z.object({
  lhs: DiffSideSchema.optional(),
  rhs: DiffSideSchema.optional(),
});

export type DiffLine = z.infer<typeof DiffLineSchema>;

export const DiffHunkSchema = z.array(DiffLineSchema);

export type DiffHunk = z.infer<typeof DiffHunkSchema>;

export const RepositoryFileDiffSchema = z.object({
  left: RepositoryFileSchema.optional(),
  right: RepositoryFileSchema.optional(),
  lines_added: z.number(),
  lines_removed: z.number(),
  hunks: z.array(DiffHunkSchema),
});

export type RepositoryFileDiff = z.infer<typeof RepositoryFileDiffSchema>;

export const RepositoryCommitDiffsSchema = z.object({
  sha: z.string(),
  parent_sha: z.string().optional(),
  commit: RepositoryCommitSchema,
  diffs: z.array(RepositoryFileDiffSchema),
});

export type RepositoryCommitDiffs = z.infer<typeof RepositoryCommitDiffsSchema>;

export const CreateQuestionRequestSchema = z.object({
  title: z.string(),
  body: z.string(),
});

export type CreateQuestionRequest = z.infer<typeof CreateQuestionRequestSchema>;

export const UpdateQuestionRequestSchema = z.object({
  title: z.string(),
  body: z.string(),
});

export type UpdateQuestionRequest = z.infer<typeof UpdateQuestionRequestSchema>;

export const CreateAnswerRequestSchema = z.object({
  body: z.string(),
});

export type CreateAnswerRequest = z.infer<typeof CreateAnswerRequestSchema>;

export const UpdateAnswerRequestSchema = z.object({
  body: z.string(),
});

export type UpdateAnswerRequest = z.infer<typeof UpdateAnswerRequestSchema>;

export const CreateCommentRequestSchema = z.object({
  body: z.string(),
});

export type CreateCommentRequest = z.infer<typeof CreateCommentRequestSchema>;

export const UpdateCommentRequestSchema = z.object({
  body: z.string(),
});

export type UpdateCommentRequest = z.infer<typeof UpdateCommentRequestSchema>;

export const VoteRequestSchema = z.object({
  value: z.number().int(),
});

export type VoteRequest = z.infer<typeof VoteRequestSchema>;

export const AuthorResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
});

export type AuthorResponse = z.infer<typeof AuthorResponseSchema>;

export const CommentResponseSchema = z.object({
  id: z.uuid(),
  parent_id: z.uuid(),
  author_id: z.uuid(),
  body: z.string(),
  upvote: z.number().int(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResponseSchema.nullable(),
});

export type CommentResponse = z.infer<typeof CommentResponseSchema>;

export const AnswerResponseSchema = z.object({
  id: z.uuid(),
  question_id: z.uuid(),
  author_id: z.uuid(),
  body: z.string(),
  upvote: z.number().int(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResponseSchema.nullable(),
  comments: z.array(CommentResponseSchema),
});

export type AnswerResponse = z.infer<typeof AnswerResponseSchema>;

export const QuestionResponseSchema = z.object({
  id: z.uuid(),
  number: z.number().int(),
  author_id: z.uuid(),
  repository_id: z.uuid(),
  title: z.string(),
  body: z.string(),
  upvote: z.number().int(),
  impression: z.number().int(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResponseSchema.nullable(),
  comments: z.array(CommentResponseSchema),
  answers: z.array(AnswerResponseSchema),
});

export type QuestionResponse = z.infer<typeof QuestionResponseSchema>;

export const QuestionsResponseSchema = z.object({
  questions: z.array(QuestionResponseSchema),
});

export type QuestionsResponse = z.infer<typeof QuestionsResponseSchema>;

export const VoteResponseSchema = z.object({
  target_id: z.uuid(),
  score: z.number().int(),
  user_vote: z.number().int().nullable(),
});

export type VoteResponse = z.infer<typeof VoteResponseSchema>;
