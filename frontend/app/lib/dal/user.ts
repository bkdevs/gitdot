import "server-only";

import {
  type UpdateCurrentUserRequest,
  type UserOrganizationsResponse,
  UserOrganizationsResponseSchema,
  type UserRepositoriesResponse,
  UserRepositoriesResponseSchema,
  type UserResponse,
  UserResponseSchema,
} from "../dto";
import { getSession } from "../supabase";
import {
  authFetch,
  authHead,
  authPatch,
  GITDOT_SERVER_URL,
  handleResponse,
  NotFound,
} from "./util";

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

export async function updateCurrentUser(
  request: UpdateCurrentUserRequest,
): Promise<UserResponse | null> {
  const response = await authPatch(`${GITDOT_SERVER_URL}/user`, request);
  return await handleResponse(response, UserResponseSchema);
}

export async function hasUser(username: string): Promise<boolean> {
  const response = await authHead(`${GITDOT_SERVER_URL}/user/${username}`);
  return response.ok;
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

export async function listUserOrganizations(
  username: string,
): Promise<UserOrganizationsResponse | NotFound | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/organizations`,
  );

  if (response.status === 404) return NotFound;
  return await handleResponse(response, UserOrganizationsResponseSchema);
}
