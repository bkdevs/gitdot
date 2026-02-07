"use server";

import { redirect } from "next/navigation";

import {
  authorizeDevice,
  createAnswer,
  createAnswerComment,
  createQuestion,
  createQuestionComment,
  createRepository,
  getCurrentUser,
  updateAnswer,
  updateComment,
  updateQuestion,
  validateUsername,
  voteAnswer,
  voteComment,
  voteQuestion,
} from "@/lib/dal";
import { createSupabaseClient } from "@/lib/supabase";
import { refresh } from "next/cache";
import type {
  AnswerResponse,
  CommentResponse,
  CreateRepositoryResponse,
  QuestionResponse,
  UserResponse,
  VoteResponse,
} from "./lib/dto";

export async function getCurrentUserAction(): Promise<UserResponse | null> {
  return await getCurrentUser();
}

export type AuthActionResult = { success: true } | { error: string };

export async function login(
  _prev: AuthActionResult | null,
  formData: FormData,
): Promise<AuthActionResult> {
  const supabase = await createSupabaseClient();
  const email = formData.get("email") as string;
  const password = formData.get("password") as string;
  const redirectTo = formData.get("redirect") as string;

  const { error } = await supabase.auth.signInWithPassword({
    email,
    password,
  });

  if (error) return { error: error.message };
  if (redirectTo) redirect(redirectTo);
  return { success: true };
}

export async function signup(
  _prev: AuthActionResult | null,
  formData: FormData,
): Promise<AuthActionResult> {
  const supabase = await createSupabaseClient();
  const email = formData.get("email") as string;
  const name = formData.get("name") as string;

  const valid = await validateUsername(name);
  if (!valid) {
    return { error: "Username taken" };
  }
  const { data, error } = await supabase.auth.signInWithOtp({
    email,
    options: { shouldCreateUser: true },
  });

  return error ? { error: error.message } : { success: true };
}

export async function signout() {
  const supabase = await createSupabaseClient();
  const { error } = await supabase.auth.signOut();
  console.log(error);
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////
// note that the actions here use refresh() as opposed to revalidatePath()
// the reason why is that refresh() only ensures that the current request gets fresh data
// whereas revalidatePath invalidates the entire client-side router cache regardless of the path passed in
// this means things like instant back/forth and prefetches will not work if an action is invoked
// even though it should only selectively dump that path in the client
//
// so rather dumbly, we just use refresh() which sets FreshnessPolicy.RefreshAll for the current navigation only
//////////////////////////////////////////////////////////////////////////////////////////////////////////

export type CreateRepositoryActionResult =
  | { repository: CreateRepositoryResponse }
  | { error: string };

export async function createRepositoryAction(
  formData: FormData,
): Promise<CreateRepositoryActionResult> {
  const owner = formData.get("owner") as string;
  const name = formData.get("repo-name") as string;
  const visibility = formData.get("visibility") as string;

  if (!owner || !name) {
    return { error: "Owner and repository name are required" };
  }

  const result = await createRepository(owner, name, {
    owner_type: "user",
    visibility,
  });
  if (!result) {
    return { error: "Failed to create repository" };
  }

  refresh();
  return { repository: result };
}

export type CreateQuestionActionResult =
  | { question: QuestionResponse }
  | { error: string };

export async function createQuestionAction(
  owner: string,
  repo: string,
  formData: FormData,
): Promise<CreateQuestionActionResult> {
  const title = formData.get("title") as string;
  const body = formData.get("body") as string;
  if (!title || !body) {
    return { error: "Title and body are required" };
  }

  const result = await createQuestion(owner, repo, { title, body });
  if (!result) {
    return { error: "createQuestion call failed" };
  }

  refresh();
  return { question: result };
}

export type UpdateQuestionActionResult =
  | { question: QuestionResponse }
  | { error: string };

export async function updateQuestionAction(
  owner: string,
  repo: string,
  number: number,
  formData: FormData,
): Promise<UpdateQuestionActionResult> {
  const title = formData.get("title") as string;
  const body = formData.get("body") as string;
  if (!title || !body) {
    return { error: "Title and body are required" };
  }

  const result = await updateQuestion(owner, repo, number, { title, body });
  if (!result) {
    return { error: "updateQuestion call failed" };
  }

  refresh();
  return { question: result };
}

export type CreateAnswerActionResult =
  | { answer: AnswerResponse }
  | { error: string };

export async function createAnswerAction(
  owner: string,
  repo: string,
  number: number,
  formData: FormData,
): Promise<CreateAnswerActionResult> {
  const body = formData.get("body") as string;
  if (!body) {
    return { error: "Body cannot be empty" };
  }

  const result = await createAnswer(owner, repo, number, { body });
  if (!result) {
    return { error: "createAnswer call failed" };
  }

  refresh();
  return { answer: result };
}

export type UpdateAnswerActionResult =
  | { answer: AnswerResponse }
  | { error: string };

export async function updateAnswerAction(
  owner: string,
  repo: string,
  number: number,
  answerId: string,
  formData: FormData,
): Promise<UpdateAnswerActionResult> {
  const body = formData.get("body") as string;
  if (!body) {
    return { error: "Body cannot be empty" };
  }

  const result = await updateAnswer(owner, repo, number, answerId, { body });
  if (!result) {
    return { error: "updateAnswer call failed" };
  }

  refresh();
  return { answer: result };
}

export type CreateCommentActionResult =
  | { comment: CommentResponse }
  | { error: string };

export async function createCommentAction(
  owner: string,
  repo: string,
  number: number,
  parentType: "question" | "answer",
  parentId: string | undefined,
  formData: FormData,
): Promise<CreateCommentActionResult> {
  const body = formData.get("body") as string;
  if (!body) {
    return { error: "Body cannot be empty" };
  } else if (parentType === "answer" && !parentId) {
    return { error: "parentId is required if parentType is answer" };
  }

  const result =
    parentType === "question"
      ? await createQuestionComment(owner, repo, Number(number), { body })
      : await createAnswerComment(owner, repo, Number(number), parentId!, {
          body,
        });

  if (!result) {
    return { error: "createComment call failed" };
  }

  refresh();
  return { comment: result };
}

export type UpdateCommentActionResult =
  | { comment: CommentResponse }
  | { error: string };

export async function updateCommentAction(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  formData: FormData,
): Promise<UpdateCommentActionResult> {
  const body = formData.get("body") as string;
  if (!body) {
    return { error: "Body cannot be empty" };
  }

  const result = await updateComment(owner, repo, number, commentId, { body });
  if (!result) {
    return { error: "updateComment call failed" };
  }

  refresh();
  return { comment: result };
}

export type VoteActionResult = { vote: VoteResponse } | { error: string };

export async function voteAction(
  owner: string,
  repo: string,
  number: number,
  targetId: string | undefined,
  targetType: "question" | "answer" | "comment",
  formData: FormData,
): Promise<VoteActionResult> {
  const value = Number(formData.get("value"));

  if (!targetId && targetType !== "question") {
    return { error: `targetId must be set for target type ${targetType}` };
  }

  let result: VoteResponse | null;
  if (targetType === "question") {
    result = await voteQuestion(owner, repo, number, { value });
  } else if (targetType === "answer") {
    result = await voteAnswer(owner, repo, number, targetId!, { value });
  } else {
    result = await voteComment(owner, repo, number, targetId!, { value });
  }

  if (!result) {
    return { error: "voteAction call failed" };
  }

  refresh();
  return { vote: result };
}

export type AuthorizeDeviceActionResult =
  | { success: true }
  | { success: false; error: string };

export async function authorizeDeviceAction(
  userCode: string,
): Promise<AuthorizeDeviceActionResult> {
  if (!userCode) {
    return { success: false, error: "User code is required" };
  }

  const success = await authorizeDevice({ user_code: userCode });
  if (!success) {
    return { success: false, error: "Failed to authorize device" };
  }

  return { success: true };
}
