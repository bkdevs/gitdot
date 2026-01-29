"use server";

import {
  createAnswer,
  createAnswerComment,
  createQuestion,
  createQuestionComment,
  createRepository,
  updateComment,
  updateQuestion,
  voteAnswer,
  voteComment,
  voteQuestion,
} from "@/lib/dal";
import { createSupabaseClient } from "@/lib/supabase";
import { VoteResponse } from "./lib/dto";
import { refresh } from "next/cache";

//////////////////////////////////////////////////////////////////////////////////////////////////////////
// note that the actions here use refresh() as opposed to revalidatePath()
// the reason why is that refresh() only ensures that the current request gets fresh data
// whereas revalidatePath invalidates the entire client-side router cache regardless of the path passed in
// this means things like instant back/forth and prefetches will not work if an action is invoked
// even though it should only selectively dump that path in the client
//
// so rather dumbly, we just use refresh() which sets FreshnessPolicy.RefreshAll for the current navigation only
//////////////////////////////////////////////////////////////////////////////////////////////////////////

export async function signup(formData: FormData) {
  const supabase = await createSupabaseClient();

  // todo: add validation
  const email = formData.get("email") as string;
  const password = formData.get("password") as string;

  const { data, error } = await supabase.auth.signUp({
    email,
    password,
  });

  console.log(data);
  console.error(error);
}

export async function login(formData: FormData) {
  const supabase = await createSupabaseClient();

  // todo: add validation
  const email = formData.get("email") as string;
  const password = formData.get("password") as string;

  const { data, error } = await supabase.auth.signInWithPassword({
    email,
    password,
  });

  console.log(data);
  console.error(error);
}

export async function signout() {
  const supabase = await createSupabaseClient();
  const { error } = await supabase.auth.signOut();

  console.log(error);
}

export async function createRepositoryAction(
  owner: string,
  name: string,
  formData: FormData,
) {
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

  return { success: true, repository: result };
}

export async function createQuestionAction(
  owner: string,
  repo: string,
  formData: FormData,
) {
  const title = formData.get("title") as string;
  const body = formData.get("body") as string;

  if (!owner || !repo || !title || !body) {
    return { error: "All fields are required" };
  }

  const result = await createQuestion(owner, repo, { title, body });

  if (!result) {
    return { error: "Failed to create question" };
  }

  return { success: true, question: result };
}

export async function updateQuestionAction(
  owner: string,
  repo: string,
  number: number,
  formData: FormData,
) {
  const title = formData.get("title") as string;
  const body = formData.get("body") as string;

  if (!title || !body) {
    return { error: "Title and body are required" };
  }

  const result = await updateQuestion(owner, repo, number, { title, body });

  if (!result) {
    return { error: "Failed to update question" };
  }

  refresh();
  return { success: true, question: result };
}

export async function createAnswerAction(
  owner: string,
  repo: string,
  number: number,
  formData: FormData,
) {
  const body = formData.get("body") as string;

  if (!body) {
    return { error: "Cannot create an empty answer" };
  }

  const result = await createAnswer(owner, repo, number, { body });

  if (!result) {
    return { error: "Failed to create answer" };
  }

  refresh();
  return { success: true, answer: result };
}

export async function createCommentAction(
  owner: string,
  repo: string,
  number: number,
  parentType: "question" | "answer",
  parentId: string | undefined,
  formData: FormData,
) {
  const body = formData.get("body") as string;

  if (parentType === "answer" && !parentId) {
    return { error: "All fields are required" };
  }

  const result =
    parentType === "question"
      ? await createQuestionComment(owner, repo, Number(number), { body })
      : await createAnswerComment(owner, repo, Number(number), parentId!, {
          body,
        });

  if (!result) {
    return { error: "Failed to create comment" };
  }

  refresh();
  return { success: true };
}

export async function voteAction(
  owner: string,
  repo: string,
  number: number,
  targetId: string | undefined,
  targetType: "question" | "answer" | "comment",
  formData: FormData,
) {
  const value = Number(formData.get("value"));

  if (!targetId && targetType !== "question") {
    return { success: false, error: "Missing required fields" };
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
    return { success: false, error: "Failed to vote" };
  }

  refresh();
  return { success: true, data: result };
}

export async function updateCommentAction(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  formData: FormData,
) {
  const body = formData.get("body") as string;

  if (!body) {
    return { error: "Comment body is required" };
  }

  const result = await updateComment(owner, repo, number, commentId, { body });

  if (!result) {
    return { error: "Failed to update comment" };
  }

  refresh();
  return { success: true };
}
