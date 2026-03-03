import { z } from "zod";

export const AuthorResource = z.object({
  id: z.uuid(),
  name: z.string(),
});
export type AuthorResource = z.infer<typeof AuthorResource>;

export const CommentResource = z.object({
  id: z.uuid(),
  parent_id: z.uuid(),
  author_id: z.uuid(),
  body: z.string(),
  upvote: z.number().int(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResource.nullable(),
});
export type CommentResource = z.infer<typeof CommentResource>;

export const AnswerResource = z.object({
  id: z.uuid(),
  question_id: z.uuid(),
  author_id: z.uuid(),
  body: z.string(),
  upvote: z.number().int(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  user_vote: z.number().int().nullable(),
  author: AuthorResource.nullable(),
  comments: z.array(CommentResource),
});
export type AnswerResource = z.infer<typeof AnswerResource>;

export const QuestionResource = z.object({
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
  author: AuthorResource.nullable(),
  comments: z.array(CommentResource),
  answers: z.array(AnswerResource),
});
export type QuestionResource = z.infer<typeof QuestionResource>;

export const VoteResource = z.object({
  target_id: z.uuid(),
  score: z.number().int(),
  user_vote: z.number().int().nullable(),
});
export type VoteResource = z.infer<typeof VoteResource>;
