import "server-only";

import {
  OrganizationResource,
  RepositoryResource,
  UserResource,
} from "gitdot-api";
import { notFound } from "next/navigation";
import { z } from "zod";
import { getSession } from "@/lib/auth";
import {
  authFetch,
  authHead,
  authPatch,
  GITDOT_SERVER_URL,
  handleResponse,
} from "./util";

export async function getCurrentUser(
  required = true,
): Promise<UserResource | null> {
  const session = await getSession();
  if (!session) {
    if (required) notFound();
    return null;
  }

  const response = await authFetch(`${GITDOT_SERVER_URL}/user`);
  const user = await handleResponse(response, UserResource);
  if (!user) {
    if (required) notFound();
    return null;
  }
  return user;
}

export async function updateCurrentUser(request: {
  name: string;
}): Promise<UserResource | null> {
  const response = await authPatch(`${GITDOT_SERVER_URL}/user`, request);
  return await handleResponse(response, UserResource);
}

export async function hasUser(username: string): Promise<boolean> {
  const response = await authHead(`${GITDOT_SERVER_URL}/user/${username}`);
  return response.ok;
}

export async function getUser(username: string): Promise<UserResource | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/user/${username}`);
  return await handleResponse(response, UserResource);
}

export async function listUserRepositories(
  username: string,
): Promise<RepositoryResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/repositories`,
  );

  return await handleResponse(response, z.array(RepositoryResource));
}

export async function listUserOrganizations(
  username: string,
): Promise<OrganizationResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/organizations`,
  );

  return await handleResponse(response, z.array(OrganizationResource));
}
