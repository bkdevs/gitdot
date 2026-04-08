"use server";

import type { UserResource } from "gitdot-api";
import { refresh } from "next/cache";
import { redirect } from "next/navigation";
import { getCurrentUser, hasUser, updateCurrentUser } from "@/dal";
import { getGitHubRedirectUrl, logout, sendAuthEmail } from "@/lib/auth";
import { delay, validateEmail } from "../util";

export async function getCurrentUserAction(): Promise<UserResource | null> {
  return await getCurrentUser(false);
}

export type AuthActionResult = { success: true } | { error: string };

export async function login(
  _prev: AuthActionResult | null,
  formData: FormData,
): Promise<AuthActionResult> {
  const email = formData.get("email") as string;
  const redirectTo = formData.get("redirect") as string;

  if (!validateEmail(email)) {
    return await delay(300, { error: "Invalid email" });
  }

  await sendAuthEmail(email);
  if (redirectTo) redirect(redirectTo);
  return { success: true };
}

// TODO: remove this as it's the same as login
export async function signup(
  _prev: AuthActionResult | null,
  formData: FormData,
): Promise<AuthActionResult> {
  const email = formData.get("email") as string;
  const redirectTo = formData.get("redirect") as string;

  if (!validateEmail(email)) {
    return await delay(300, { error: "Invalid email" });
  }

  await sendAuthEmail(email);
  if (redirectTo) redirect(redirectTo);
  return { success: true };
}

export async function loginWithGithub(): Promise<AuthActionResult> {
  const url = await getGitHubRedirectUrl();
  if (!url) return { error: "Failed to initiate GitHub login" };
  redirect(url);
}

export type UpdateUserActionResult = { user: UserResource } | { error: string };

export async function updateUserAction(
  _prev: UpdateUserActionResult | null,
  formData: FormData,
): Promise<UpdateUserActionResult> {
  const username = formData.get("username") as string | null;
  const location = formData.get("location") as string | null;
  const redirectTo = formData.get("redirect") as string;

  let name: string | undefined;
  if (username) {
    const usernameError = await validateUsername(username);
    if (usernameError) {
      console.log(usernameError);
      return { error: usernameError };
    }
    name = username;
  }

  const result = await updateCurrentUser({
    name,
    location: location !== null ? location || "" : undefined,
  });

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
  await logout();
}
