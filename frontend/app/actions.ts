"use server";

import {
  createAnswer,
  createAnswerComment,
  createQuestion,
  createQuestionComment,
  createRepository,
  voteAnswer,
  voteComment,
  voteQuestion,
} from "@/lib/dal";
import { createSupabaseClient } from "@/lib/supabase";
import { VoteResponse } from "./lib/dto";
import { revalidatePath } from "next/cache";

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

export async function createRepositoryAction(owner: string, name: string, formData: FormData) {
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

export async function createQuestionAction(owner: string, repo: string, formData: FormData) {
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

export async function createAnswerAction(owner: string, repo: string, number: number, formData: FormData) {
  const body = formData.get("body") as string;

  if (!body) {
    return { error: "Cannot create an empty answer" };
  }

  const result = await createAnswer(owner, repo, number, { body });

  if (!result) {
    return { error: "Failed to create answer" };
  }
  return { success: true, answer: result };
}

export async function createCommentAction(
  owner: string,
  repo: string,
  number: number,
  parentType: "question" | "answer",
  parentId: string | undefined,
  formData: FormData
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

  revalidatePath(`/${owner}/${repo}/questions/${number}`);
  return { success: true, data: result };
}
