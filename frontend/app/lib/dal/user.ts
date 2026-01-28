import "server-only";

import {
  type UserRepositoriesResponse,
  UserRepositoriesResponseSchema,
  type UserResponse,
  UserResponseSchema,
} from "../dto";
import { API_BASE_URL, authFetch, handleResponse } from "./util";

export async function getUser(username: string): Promise<UserResponse | null> {
  const response = await authFetch(`${API_BASE_URL}/user/${username}`);

  return await handleResponse(response, UserResponseSchema);
}

export async function listUserRepositories(
  username: string,
): Promise<UserRepositoriesResponse | null> {
  const response = await authFetch(
    `${API_BASE_URL}/user/${username}/repositories`,
  );

  return await handleResponse(response, UserRepositoriesResponseSchema);
}

export async function getCurrentUser(): Promise<UserResponse | null> {
  const response = await authFetch(`${API_BASE_URL}/user`);
  return await handleResponse(response, UserResponseSchema);
}
