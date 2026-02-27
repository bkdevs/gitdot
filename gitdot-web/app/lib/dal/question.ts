import "server-only";

import {
  AnswerResource,
  CommentResource,
  QuestionResource,
  VoteResource,
} from "gitdot-api-ts";
import { z } from "zod";
import {
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
} from "./util";

type CreateQuestionRequest = { title: string; body: string };
type UpdateQuestionRequest = { title: string; body: string };
type CreateAnswerRequest = { body: string };
type UpdateAnswerRequest = { body: string };
type CreateCommentRequest = { body: string };
type UpdateCommentRequest = { body: string };
type VoteRequest = { value: number };

export async function createQuestion(
  owner: string,
  repo: string,
  request: CreateQuestionRequest,
): Promise<QuestionResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question`,
    request,
  );

  return await handleResponse(response, QuestionResource);
}

export async function getQuestion(
  owner: string,
  repo: string,
  number: number,
): Promise<QuestionResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}`,
  );

  return await handleResponse(response, QuestionResource);
}

export async function updateQuestion(
  owner: string,
  repo: string,
  number: number,
  request: UpdateQuestionRequest,
): Promise<QuestionResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}`,
    request,
  );

  return await handleResponse(response, QuestionResource);
}

export async function getQuestions(
  owner: string,
  repo: string,
): Promise<QuestionResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/questions`,
  );

  return await handleResponse(response, z.array(QuestionResource));
}

export async function createAnswer(
  owner: string,
  repo: string,
  number: number,
  request: CreateAnswerRequest,
): Promise<AnswerResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/answer`,
    request,
  );

  return await handleResponse(response, AnswerResource);
}

export async function updateAnswer(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  request: UpdateAnswerRequest,
): Promise<AnswerResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/answer/${answerId}`,
    request,
  );

  return await handleResponse(response, AnswerResource);
}

export async function createQuestionComment(
  owner: string,
  repo: string,
  number: number,
  request: CreateCommentRequest,
): Promise<CommentResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/comment`,
    request,
  );

  return await handleResponse(response, CommentResource);
}

export async function createAnswerComment(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  request: CreateCommentRequest,
): Promise<CommentResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/answer/${answerId}/comment`,
    request,
  );

  return await handleResponse(response, CommentResource);
}

export async function updateComment(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  request: UpdateCommentRequest,
): Promise<CommentResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/comment/${commentId}`,
    request,
  );

  return await handleResponse(response, CommentResource);
}

export async function voteQuestion(
  owner: string,
  repo: string,
  number: number,
  request: VoteRequest,
): Promise<VoteResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/vote`,
    request,
  );

  return await handleResponse(response, VoteResource);
}

export async function voteAnswer(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  request: VoteRequest,
): Promise<VoteResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/answer/${answerId}/vote`,
    request,
  );

  return await handleResponse(response, VoteResource);
}

export async function voteComment(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  request: VoteRequest,
): Promise<VoteResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/question/${number}/comment/${commentId}/vote`,
    request,
  );

  return await handleResponse(response, VoteResource);
}
