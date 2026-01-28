"use server";

import { revalidatePath } from "next/cache";
import {
  createAnswer,
  createAnswerComment,
  createQuestion,
  createQuestionComment,
  createRepository,
} from "@/lib/dal";
import { createSupabaseClient } from "@/lib/supabase";

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

export async function createRepositoryAction(formData: FormData) {
  const owner = formData.get("owner") as string;
  const name = formData.get("name") as string;
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

export async function createQuestionAction(formData: FormData) {
  const owner = formData.get("owner") as string;
  const repo = formData.get("repo") as string;
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

export async function createAnswerAction(formData: FormData) {
  const owner = formData.get("owner") as string;
  const repo = formData.get("repo") as string;
  const number = formData.get("number") as string;
  const body = formData.get("body") as string;

  if (!owner || !repo || !number || !body) {
    return { error: "All fields are required" };
  }

  const result = await createAnswer(owner, repo, Number(number), { body });

  if (!result) {
    return { error: "Failed to create answer" };
  }

  revalidatePath(`/${owner}/${repo}/questions/${number}`);
  return { success: true, answer: result };
}

export async function createCommentAction(formData: FormData) {
  const owner = formData.get("owner") as string;
  const repo = formData.get("repo") as string;
  const number = formData.get("number") as string;
  const body = formData.get("body") as string;
  const parentType = formData.get("parentType") as "question" | "answer";
  const answerId = formData.get("answerId") as string | null;

  if (!owner || !repo || !number || !body) {
    return { error: "All fields are required" };
  }

  const result =
    parentType === "question"
      ? await createQuestionComment(owner, repo, Number(number), { body })
      : await createAnswerComment(owner, repo, Number(number), answerId!, {
          body,
        });

  if (!result) {
    return { error: "Failed to create comment" };
  }

  revalidatePath(`/${owner}/${repo}/questions/${number}`);
  return { success: true };
}
