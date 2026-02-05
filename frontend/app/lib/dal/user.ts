import "server-only";

import {
  type UserRepositoriesResponse,
  UserRepositoriesResponseSchema,
  type UserResponse,
  UserResponseSchema,
} from "../dto";
import { getSession } from "../supabase";
import { authFetch, GITDOT_SERVER_URL, handleResponse, NotFound } from "./util";

export type ValidateNameResult = { success: true } | { error: string };

export async function validateName(name: string): Promise<ValidateNameResult> {
  const response = await fetch(
    `${GITDOT_SERVER_URL}/user/${encodeURIComponent(name)}/validate`,
    { method: "POST" },
  );

  if (!response.ok) {
    try {
      const data = await response.json();
      return { error: data.message ?? "Invalid name" };
    } catch {
      return { error: "Invalid name" };
    }
  }

  return { success: true };
}

export async function getUser(username: string): Promise<UserResponse | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/user/${username}`);
  return await handleResponse(response, UserResponseSchema);
}

export async function listUserRepositories(
  username: string,
): Promise<UserRepositoriesResponse | NotFound | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/repositories`,
  );

  if (response.status === 404) return NotFound;
  return await handleResponse(response, UserRepositoriesResponseSchema);
}

/**
 * this should _not_ be used on any page that we intend to statically render
 * if static pages require auth, rely on useUser in client-side components instead
 */
export async function getCurrentUser(): Promise<UserResponse | null> {
  const session = await getSession();
  if (!session) return null;

  const response = await authFetch(`${GITDOT_SERVER_URL}/user`);
  return await handleResponse(response, UserResponseSchema);
}
