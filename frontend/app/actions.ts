"use server";

import { refresh } from "next/cache";
import { headers } from "next/headers";
import { redirect } from "next/navigation";
import {
  ApiError,
  authorizeDevice,
  createAnswer,
  createAnswerComment,
  createBuild,
  createQuestion,
  createQuestionComment,
  createRepository,
  createRunner,
  deleteRepository,
  getCurrentUser,
  hasUser,
  migrateGitHubRepositories,
  refreshRunnerToken,
  updateAnswer,
  updateComment,
  updateCurrentUser,
  updateQuestion,
  voteAnswer,
  voteComment,
  voteQuestion,
} from "@/lib/dal";
import { createSupabaseClient } from "@/lib/supabase";
import type {
  AnswerResponse,
  BuildResponse,
  CommentResponse,
  CreateRepositoryResponse,
  QuestionResponse,
  UserResponse,
  VoteResponse,
} from "./lib/dto";
import { delay, validateEmail } from "./util";

export async function getCurrentUserAction(): Promise<UserResponse | null> {
  return await getCurrentUser(false);
}

export type AuthActionResult = { success: true } | { error: string };

export async function login(
  _prev: AuthActionResult | null,
  formData: FormData,
): Promise<AuthActionResult> {
  const supabase = await createSupabaseClient();
  const email = formData.get("email") as string;
  const redirectTo = formData.get("redirect") as string;

  if (!validateEmail(email)) {
    return await delay(300, { error: "Invalid email" });
  }

  const { error } = await supabase.auth.signInWithOtp({
    email,
    options: { shouldCreateUser: false },
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
  const redirectTo = formData.get("redirect") as string;

  if (!validateEmail(email)) {
    return await delay(300, { error: "Invalid email" });
  }

  // note: this will _not_ fail if the user already exists, but instead send a sign-in link
  // we don't differentiate between new and existing for security: otherwise attackers would be able to tell what
  // user exists / doesn't exist
  const { error } = await supabase.auth.signInWithOtp({
    email,
    options: { shouldCreateUser: true },
  });

  if (error) return { error: error.message };
  if (redirectTo) redirect(redirectTo);
  return { success: true };
}

export async function loginWithGithub(): Promise<AuthActionResult> {
  const supabase = await createSupabaseClient();
  const headersList = await headers();
  const host =
    headersList.get("x-forwarded-host") ||
    headersList.get("host") ||
    "localhost:3000";
  const protocol = headersList.get("x-forwarded-proto") || "http";
  const origin = `${protocol}://${host}`;

  const { data, error } = await supabase.auth.signInWithOAuth({
    provider: "github",
    options: {
      redirectTo: `${origin}/oauth/callback`,
    },
  });

  if (error || !data.url) {
    return { error: error?.message || "Failed to auth with GitHub" };
  }

  if (data.url) redirect(data.url);
  return { success: true };
}

export type UpdateUserActionResult = { user: UserResponse } | { error: string };

export async function updateUserAction(
  _prev: UpdateUserActionResult | null,
  formData: FormData,
): Promise<UpdateUserActionResult> {
  const username = formData.get("username") as string;
  const redirectTo = formData.get("redirect") as string;

  const usernameError = await validateUsername(username);
  if (usernameError) {
    console.log(usernameError);
    return { error: usernameError };
  }

  const result = await updateCurrentUser({ name: username });
  if (!result) {
    return { error: "Failed to update user" };
  }

  refresh();
  if (redirectTo) redirect(redirectTo);
  return { user: result };
}

export async function validateUsername(
  username: string,
): Promise<string | null> {
  if (username.length < 2) {
    return await delay(300, "Username must be at least 2 characters");
  }
  if (username.length > 32) {
    return await delay(300, "Username must be at most 32 characters");
  }
  if (username.startsWith("-")) {
    return await delay(300, "Username cannot start with a hyphen");
  }
  if (username.endsWith("-")) {
    return await delay(300, "Username cannot start with a hyphen");
  }
  const invalidChars = username.match(/[^a-zA-Z0-9_-]/g);
  if (invalidChars) {
    return await delay(
      300,
      `Username cannot include '${[...new Set(invalidChars)].join("")}'`,
    );
  }
  const usernameTaken = await hasUser(username);
  if (usernameTaken) {
    return "Username taken";
  }

  return null;
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

export type CreateRunnerActionResult = { error: string };

export async function createRunnerAction(
  _prev: CreateRunnerActionResult | null,
  formData: FormData,
): Promise<CreateRunnerActionResult> {
  const name = formData.get("name") as string;
  const ownerName = formData.get("owner_name") as string;
  const ownerType = formData.get("owner_type") as string;

  if (!name || !ownerName || !ownerType) {
    return { error: "Name, owner, and owner type are required" };
  }

  if (ownerType !== "user" && ownerType !== "organization") {
    return { error: "Owner type must be user or organization" };
  }

  const result = await createRunner(name, ownerName, ownerType);
  if (!result) {
    return { error: "Failed to create runner" };
  }

  redirect(`/settings/runners/${result.name}`);
}

export type RefreshRunnerTokenActionResult =
  | { token: string }
  | { error: string };

export async function refreshRunnerTokenAction(
  runnerName: string,
  ownerName: string,
): Promise<RefreshRunnerTokenActionResult> {
  if (!runnerName || !ownerName) {
    return { error: "Runner name and owner are required" };
  }

  const result = await refreshRunnerToken(ownerName, runnerName);
  if (!result) {
    return { error: "Failed to generate token" };
  }

  return { token: result.token };
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
      : await createAnswerComment(
          owner,
          repo,
          Number(number),
          parentId as string,
          {
            body,
          },
        );

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
    result = await voteAnswer(owner, repo, number, targetId as string, {
      value,
    });
  } else {
    result = await voteComment(owner, repo, number, targetId as string, {
      value,
    });
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

export type CreateBuildActionResult =
  | { build: BuildResponse }
  | { error: string };

export async function createBuildAction(
  owner: string,
  repo: string,
  formData: FormData,
): Promise<CreateBuildActionResult> {
  const trigger = formData.get("trigger") as string;
  const commit_sha = formData.get("commit_sha") as string;

  if (!trigger || !commit_sha) {
    return { error: "Trigger and commit SHA are required" };
  }

  if (trigger !== "pull_request" && trigger !== "push_to_main") {
    return { error: "Trigger must be pull_request or push_to_main" };
  }

  let result: BuildResponse | null;
  try {
    result = await createBuild({
      repo_owner: owner,
      repo_name: repo,
      trigger,
      commit_sha,
    });
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "createBuild call failed",
    };
  }
  if (!result) {
    return { error: "createBuild call failed" };
  }

  refresh();
  return { build: result };
}

export type MigrateGitHubRepositoriesActionResult =
  | { success: true }
  | { error: string };

export async function migrateGitHubRepositoriesAction(
  installationId: number,
  origin: string,
  originType: string,
  destination: string,
  destinationType: string,
  repositories: string[],
): Promise<MigrateGitHubRepositoriesActionResult> {
  if (!destination || repositories.length === 0) {
    return { error: "Destination and repositories are required" };
  }

  try {
    await migrateGitHubRepositories(
      installationId,
      origin,
      originType,
      destination,
      destinationType,
      repositories,
    );
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to start migration",
    };
  }

  redirect("/settings/migrations");
  return { success: true };
}

export type DeleteRepositoryActionResult =
  | { success: true }
  | { error: string };

export async function deleteRepositoryAction(
  owner: string,
  repo: string,
): Promise<DeleteRepositoryActionResult> {
  try {
    await deleteRepository(owner, repo);
  } catch (e) {
    return {
      error: e instanceof Error ? e.message : "Failed to delete repository",
    };
  }

  redirect(`/${owner}`);
  return { success: true };
}
