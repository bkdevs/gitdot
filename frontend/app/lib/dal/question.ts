import "server-only";

import {
  type AnswerResponse,
  AnswerResponseSchema,
  type CommentResponse,
  CommentResponseSchema,
  type CreateAnswerRequest,
  type CreateCommentRequest,
  type CreateQuestionRequest,
  type QuestionResponse,
  QuestionResponseSchema,
  type QuestionsResponse,
  QuestionsResponseSchema,
  type UpdateAnswerRequest,
  type UpdateCommentRequest,
  type UpdateQuestionRequest,
  type VoteRequest,
  type VoteResponse,
  VoteResponseSchema,
} from "../dto";
import {
  API_BASE_URL,
  authFetch,
  authPatch,
  authPost,
  handleResponse,
} from "./util";

export async function createQuestion(
  owner: string,
  repo: string,
  request: CreateQuestionRequest,
): Promise<QuestionResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question`,
    request,
  );

  return await handleResponse(response, QuestionResponseSchema);
}

export async function getQuestion(
  owner: string,
  repo: string,
  number: number,
): Promise<QuestionResponse | null> {
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}`,
  );

  return await handleResponse(response, QuestionResponseSchema);
}

export async function updateQuestion(
  owner: string,
  repo: string,
  number: number,
  request: UpdateQuestionRequest,
): Promise<QuestionResponse | null> {
  const response = await authPatch(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}`,
    request,
  );

  return await handleResponse(response, QuestionResponseSchema);
}

export async function getQuestions(
  owner: string,
  repo: string,
): Promise<QuestionsResponse | null> {
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/questions`,
  );

  return await handleResponse(response, QuestionsResponseSchema);
}

export async function createAnswer(
  owner: string,
  repo: string,
  number: number,
  request: CreateAnswerRequest,
): Promise<AnswerResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/answer`,
    request,
  );

  return await handleResponse(response, AnswerResponseSchema);
}

export async function updateAnswer(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  request: UpdateAnswerRequest,
): Promise<AnswerResponse | null> {
  const response = await authPatch(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/answer/${answerId}`,
    request,
  );

  return await handleResponse(response, AnswerResponseSchema);
}

export async function createQuestionComment(
  owner: string,
  repo: string,
  number: number,
  request: CreateCommentRequest,
): Promise<CommentResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/comment`,
    request,
  );

  return await handleResponse(response, CommentResponseSchema);
}

export async function createAnswerComment(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  request: CreateCommentRequest,
): Promise<CommentResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/answer/${answerId}/comment`,
    request,
  );

  return await handleResponse(response, CommentResponseSchema);
}

export async function updateComment(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  request: UpdateCommentRequest,
): Promise<CommentResponse | null> {
  const response = await authPatch(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/comment/${commentId}`,
    request,
  );

  return await handleResponse(response, CommentResponseSchema);
}

export async function voteQuestion(
  owner: string,
  repo: string,
  number: number,
  request: VoteRequest,
): Promise<VoteResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/vote`,
    request,
  );

  return await handleResponse(response, VoteResponseSchema);
}

export async function voteAnswer(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  request: VoteRequest,
): Promise<VoteResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/answer/${answerId}/vote`,
    request,
  );

  return await handleResponse(response, VoteResponseSchema);
}

export async function voteComment(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  request: VoteRequest,
): Promise<VoteResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${number}/comment/${commentId}/vote`,
    request,
  );

  return await handleResponse(response, VoteResponseSchema);
}
