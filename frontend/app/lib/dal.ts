import "server-only";

import { toQueryString } from "@/util";
import { ZodType } from "zod";
import {
  type AnswerResponse,
  AnswerResponseSchema,
  type CommentResponse,
  CommentResponseSchema,
  type CreateAnswerRequest,
  type CreateCommentRequest,
  type CreateQuestionRequest,
  type CreateRepositoryRequest,
  type CreateRepositoryResponse,
  CreateRepositoryResponseSchema,
  type QuestionResponse,
  QuestionResponseSchema,
  type QuestionsResponse,
  QuestionsResponseSchema,
  type RepositoryCommitDiffs,
  RepositoryCommitDiffsSchema,
  type RepositoryCommits,
  type RepositoryCommitsQuery,
  RepositoryCommitsSchema,
  type RepositoryFile,
  type RepositoryFileCommitsQuery,
  type RepositoryFileQuery,
  RepositoryFileSchema,
  type RepositoryTree,
  type RepositoryTreeQuery,
  RepositoryTreeSchema,
  type UpdateAnswerRequest,
  type UpdateCommentRequest,
  type UpdateQuestionRequest,
  type VoteRequest,
  type VoteResponse,
  VoteResponseSchema,
} from "./dto";
import { getSession } from "./supabase";

const API_BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";

async function authFetch(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  const session = await getSession();
  if (!session) throw new Error("User not authenticated!");

  return fetch(url, {
    ...options,
    headers: {
      ...options?.headers,
      Authorization: `Bearer ${session.access_token}`,
    },
  });
}

async function authPost(url: string, request: any): Promise<Response> {
  return await authFetch(
    url,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(request),
    },
  );
}

async function authPatch(url: string, request: any): Promise<Response> {
  return await authFetch(
    url,
    {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(request),
    },
  );
}

async function handleResponse<T>(response: Response, schema: ZodType<T>): Promise<T | null> {
  if (!response.ok) {
    console.error(`${response.url} failed:`, response?.status, response?.statusText);
    return null;
  }

  const data = await response.json();
  return schema.parse(data);
}


export async function createRepository(
  owner: string,
  repo: string,
  request: CreateRepositoryRequest,
): Promise<CreateRepositoryResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}`,
    request,
  );

  return await handleResponse(response, CreateRepositoryResponseSchema);
}

export async function getRepositoryFile(
  owner: string,
  repo: string,
  query: RepositoryFileQuery,
): Promise<RepositoryFile | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/file?${queryString}`,
  );

  return await handleResponse(response, RepositoryFileSchema);
}

export async function getRepositoryTree(
  owner: string,
  repo: string,
  query?: RepositoryTreeQuery,
): Promise<RepositoryTree | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/tree?${queryString}`,
  );

  return await handleResponse(response, RepositoryTreeSchema);
}

export async function getRepositoryCommits(
  owner: string,
  repo: string,
  query?: RepositoryCommitsQuery,
): Promise<RepositoryCommits | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/commits?${queryString}`,
  );

  return await handleResponse(response, RepositoryCommitsSchema);
}

export async function getRepositoryFileCommits(
  owner: string,
  repo: string,
  query: RepositoryFileCommitsQuery,
): Promise<RepositoryCommits | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/file/commits?${queryString}`,
  );

  return await handleResponse(response, RepositoryCommitsSchema);
}

export async function getRepositoryCommitStats(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffs | null> {
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/commits/${sha}/stats`,
  );

  return await handleResponse(response, RepositoryCommitDiffsSchema);
}

export async function getRepositoryCommitDiffs(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffs | null> {
  const response = await authFetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/commits/${sha}/diffs`,
  );

  return await handleResponse(response, RepositoryCommitDiffsSchema);
}

export async function createQuestion(
  owner: string,
  repo: string,
  request: CreateQuestionRequest,
): Promise<QuestionResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question`,
    JSON.stringify(request)
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
    JSON.stringify(request)
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
  questionNumber: number,
  request: CreateAnswerRequest,
): Promise<AnswerResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/answer`,
    request,
  );

  return await handleResponse(response, AnswerResponseSchema);
}

export async function updateAnswer(
  owner: string,
  repo: string,
  questionNumber: number,
  answerId: string,
  request: UpdateAnswerRequest,
): Promise<AnswerResponse | null> {
  const response = await authPatch(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/answer/${answerId}`,
    request,
  );

  return await handleResponse(response, AnswerResponseSchema);
}

export async function createQuestionComment(
  owner: string,
  repo: string,
  questionNumber: number,
  request: CreateCommentRequest,
): Promise<CommentResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/comment`,
    request,
  );

  return await handleResponse(response, CommentResponseSchema);
}

export async function createAnswerComment(
  owner: string,
  repo: string,
  questionNumber: number,
  answerId: string,
  request: CreateCommentRequest,
): Promise<CommentResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/answer/${answerId}/comment`,
    request,
  );

  return await handleResponse(response, CommentResponseSchema);
}

export async function updateComment(
  owner: string,
  repo: string,
  questionNumber: number,
  commentId: string,
  request: UpdateCommentRequest,
): Promise<CommentResponse | null> {
  const response = await authPatch(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/comment/${commentId}`,
    request,
  );

  return await handleResponse(response, CommentResponseSchema);
}

export async function voteQuestion(
  owner: string,
  repo: string,
  questionNumber: number,
  request: VoteRequest,
): Promise<VoteResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/vote`,
    request,
  );

  return await handleResponse(response, VoteResponseSchema);
}

export async function voteAnswer(
  owner: string,
  repo: string,
  questionNumber: number,
  answerId: string,
  request: VoteRequest,
): Promise<VoteResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/answer/${answerId}/vote`,
    request,
  );

  return await handleResponse(response, VoteResponseSchema);
}

export async function voteComment(
  owner: string,
  repo: string,
  questionNumber: number,
  commentId: string,
  request: VoteRequest,
): Promise<VoteResponse | null> {
  const response = await authPost(
    `${API_BASE_URL}/repository/${owner}/${repo}/question/${questionNumber}/comment/${commentId}/vote`,
    request,
  );

  return await handleResponse(response, VoteResponseSchema);
}
