import "server-only";

import {
  BuildResource,
  type CreateBuildRequest,
  GetBuildResource,
} from "gitdot-api";
import { z } from "zod";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getBuilds(
  owner: string,
  repo: string,
): Promise<BuildResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/builds`,
  );

  return await handleResponse(response, z.array(BuildResource));
}

export async function createBuild(
  owner: string,
  repo: string,
  request: CreateBuildRequest,
): Promise<BuildResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build`,
    request,
  );

  return await handleResponse(response, BuildResource);
}

export async function getBuild(
  owner: string,
  repo: string,
  number: number,
): Promise<GetBuildResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build/${number}`,
  );

  return await handleResponse(response, GetBuildResource);
}
