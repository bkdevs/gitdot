import "server-only";

import {
  type CreateUserRequest,
  type UserRepositoriesResponse,
  UserRepositoriesResponseSchema,
  type UserResponse,
  UserResponseSchema,
} from "../dto";
import { getSession } from "../supabase";
import {
  authFetch,
  authHead,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
  NotFound,
} from "./util";

export async function hasUser(username: string): Promise<boolean> {
  const response = await authHead(`${GITDOT_SERVER_URL}/user/${username}`);
  return response.ok;
}

export async function getUser(username: string): Promise<UserResponse | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/user/${username}`);
  return await handleResponse(response, UserResponseSchema);
}

export async function createUser(username: string): Promise<UserResponse | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/user`, { name: username });
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
