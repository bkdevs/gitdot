import { z } from "zod";

export const AuthorResource = z.object({
  id: z.string().uuid(),
  name: z.string(),
});
export type AuthorResource = z.infer<typeof AuthorResource>;

export const CommentResource = z.object({
  id: z.string().uuid(),
  parent_id: z.string().uuid(),
  author_id: z.string().uuid(),
  body: z.string(),
  upvote: z.number().int(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResource.nullable(),
});
export type CommentResource = z.infer<typeof CommentResource>;

export const AnswerResource = z.object({
  id: z.string().uuid(),
  question_id: z.string().uuid(),
  author_id: z.string().uuid(),
  body: z.string(),
  upvote: z.number().int(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResource.nullable(),
  comments: z.array(CommentResource),
});
export type AnswerResource = z.infer<typeof AnswerResource>;

export const QuestionResource = z.object({
  id: z.string().uuid(),
  number: z.number().int(),
  author_id: z.string().uuid(),
  repository_id: z.string().uuid(),
  title: z.string(),
  body: z.string(),
  upvote: z.number().int(),
  impression: z.number().int(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResource.nullable(),
  comments: z.array(CommentResource),
  answers: z.array(AnswerResource),
});
export type QuestionResource = z.infer<typeof QuestionResource>;

export const VoteResource = z.object({
  target_id: z.string().uuid(),
  score: z.number().int(),
  user_vote: z.number().int().nullable(),
});
export type VoteResource = z.infer<typeof VoteResource>;
