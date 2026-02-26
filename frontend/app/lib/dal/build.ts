import "server-only";

import {
  type BuildResponse,
  BuildResponseSchema,
  type BuildsResponse,
  BuildsResponseSchema,
  type CreateBuildRequest,
  type GetBuildResponse,
  GetBuildResponseSchema,
} from "../dto";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getBuilds(
  owner: string,
  repo: string,
): Promise<BuildsResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/builds`,
  );

  return await handleResponse(response, BuildsResponseSchema);
}

export async function createBuild(
  owner: string,
  repo: string,
  request: CreateBuildRequest,
): Promise<BuildResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build`,
    request,
  );

  return await handleResponse(response, BuildResponseSchema);
}

export async function getBuild(
  owner: string,
  repo: string,
  number: number,
): Promise<GetBuildResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build/${number}`,
  );

  return await handleResponse(response, GetBuildResponseSchema);
}
