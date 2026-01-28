import { z } from "zod";

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
  author: AuthorResponseSchema,
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
  user_vote: z.number().int().nullable().default(0),
  author: AuthorResponseSchema,
  comments: z.array(CommentResponseSchema),
  answers: z.array(AnswerResponseSchema),
});

export type QuestionResponse = z.infer<typeof QuestionResponseSchema>;

export const QuestionsResponseSchema = z.array(QuestionResponseSchema);

export type QuestionsResponse = z.infer<typeof QuestionsResponseSchema>;

export const VoteResponseSchema = z.object({
  target_id: z.uuid(),
  score: z.number().int(),
  user_vote: z.number().int().nullable(),
});

export type VoteResponse = z.infer<typeof VoteResponseSchema>;
