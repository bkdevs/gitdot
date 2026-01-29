import "server-only";

import {
  type UserRepositoriesResponse,
  UserRepositoriesResponseSchema,
  type UserResponse,
  UserResponseSchema,
} from "../dto";
import { GITDOT_SERVER_URL, authFetch, handleResponse } from "./util";

export async function getUser(username: string): Promise<UserResponse | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/user/${username}`);

  return await handleResponse(response, UserResponseSchema);
}

export async function listUserRepositories(
  username: string,
): Promise<UserRepositoriesResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/repositories`,
  );

  return await handleResponse(response, UserRepositoriesResponseSchema);
}
