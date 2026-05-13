import "server-only";

import {
  CurrentUserResource,
  OrganizationMemberResource,
  RepositoryCommitResource,
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
): Promise<CurrentUserResource | null> {
  const session = await getSession();
  if (!session) {
    if (required) notFound();
    return null;
  }

  const response = await authFetch(`${GITDOT_SERVER_URL}/user`);
  const currentUser = await handleResponse(response, CurrentUserResource);
  if (!currentUser) {
    if (required) notFound();
    return null;
  }
  return currentUser;
}

export async function updateCurrentUser(request: {
  name?: string;
  location?: string | null;
  readme?: string | null;
  links?: string[];
  display_name?: string | null;
}): Promise<UserResource | null> {
  const response = await authPatch(`${GITDOT_SERVER_URL}/user`, request);
  return await handleResponse(response, UserResource);
}

export async function uploadUserImage(file: File): Promise<boolean> {
  const url = `${GITDOT_SERVER_URL}/user/image`;
  console.log("[uploadUserImage] POST", url, {
    type: file.type,
    size: file.size,
  });

  let bytes: ArrayBuffer;
  try {
    bytes = await file.arrayBuffer();
  } catch (e) {
    console.error("[uploadUserImage] file.arrayBuffer() failed:", e);
    throw e;
  }
  console.log("[uploadUserImage] read bytes", { byteLength: bytes.byteLength });

  let response: Response;
  try {
    response = await authFetch(url, {
      method: "POST",
      headers: { "Content-Type": file.type },
      body: bytes,
    });
  } catch (e) {
    console.error("[uploadUserImage] authFetch threw (network/proxy?):", e);
    throw e;
  }

  if (!response.ok) {
    let body = "";
    try {
      body = await response.text();
    } catch (e) {
      console.error("[uploadUserImage] failed to read error body:", e);
    }
    console.error("[uploadUserImage] non-OK response", {
      status: response.status,
      statusText: response.statusText,
      contentType: response.headers.get("content-type"),
      contentLength: response.headers.get("content-length"),
      body: body.slice(0, 500),
    });
  } else {
    console.log("[uploadUserImage] ok", { status: response.status });
  }
  return response.ok;
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
): Promise<OrganizationMemberResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/organizations`,
  );

  return await handleResponse(response, z.array(OrganizationMemberResource));
}

export async function listUserCommits(
  username: string,
): Promise<RepositoryCommitResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/commits`,
  );

  return await handleResponse(response, z.array(RepositoryCommitResource));
}

export async function listUserStars(
  username: string,
): Promise<RepositoryResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/user/${username}/stars`,
  );

  return await handleResponse(response, z.array(RepositoryResource));
}
