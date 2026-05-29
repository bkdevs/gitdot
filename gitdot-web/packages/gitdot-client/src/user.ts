import "server-only";

import {
  CurrentUserResource,
  ListUserCommitsResponse,
  ListUserContributedRepositoriesResponse,
  ListUserOrganizationsResponse,
  ListUserRepositoriesResponse,
  ListUserStarredRepositoriesResponse,
  UserResource,
} from "gitdot-api";
import { notFound } from "next/navigation";
import { getSession } from "./auth";
import {
  authFetch,
  authHead,
  authPatch,
  GITDOT_SERVER_URL,
  handleResponse,
  toQueryString,
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

// TODO: also needs pagination logic here beyond the 10K
export async function listUserRepositories(
  username: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListUserRepositoriesResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/user/${username}/repositories${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListUserRepositoriesResponse);
}

export async function listUserOrganizations(
  username: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListUserOrganizationsResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/user/${username}/organizations${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListUserOrganizationsResponse);
}

export async function listUserCommits(
  username: string,
  opts?: { from?: string; to?: string; cursor?: string; limit?: number },
): Promise<ListUserCommitsResponse | null> {
  const qs = toQueryString({
    from: opts?.from,
    to: opts?.to,
    cursor: opts?.cursor,
    limit: opts?.limit,
  });
  const url = `${GITDOT_SERVER_URL}/user/${username}/commits${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListUserCommitsResponse);
}

export async function listUserStarredRepositories(
  username: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListUserStarredRepositoriesResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/user/${username}/repositories-starred${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListUserStarredRepositoriesResponse);
}

export async function listUserContributedRepositories(
  username: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListUserContributedRepositoriesResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/user/${username}/repositories-contributed${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(
    response,
    ListUserContributedRepositoriesResponse,
  );
}
