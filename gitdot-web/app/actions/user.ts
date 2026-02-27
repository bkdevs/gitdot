"use server";

import type { UserResource } from "gitdot-api";
import { refresh } from "next/cache";
import { headers } from "next/headers";
import { redirect } from "next/navigation";
import { getCurrentUser, hasUser, updateCurrentUser } from "@/dal";
import { createSupabaseClient } from "@/lib/supabase";
import { delay, validateEmail } from "../util";

export async function getCurrentUserAction(): Promise<UserResource | null> {
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

export type UpdateUserActionResult = { user: UserResource } | { error: string };

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
