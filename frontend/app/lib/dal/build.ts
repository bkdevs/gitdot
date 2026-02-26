import "server-only";

import {
  type BuildResponse,
  BuildResponseSchema,
  type BuildsResponse,
  BuildsResponseSchema,
  type CreateBuildRequest,
  GetBuildByNumberResponseSchema,
  type GetBuildByNumberResponse,
} from "../dto";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getBuilds(
  owner: string,
  repo: string,
): Promise<BuildsResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/builds?owner=${encodeURIComponent(owner)}&repo=${encodeURIComponent(repo)}`,
  );

  return await handleResponse(response, BuildsResponseSchema);
}

export async function createBuild(
  request: CreateBuildRequest,
): Promise<BuildResponse | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/ci/build`, request);

  return await handleResponse(response, BuildResponseSchema);
}

export async function getBuildByNumber(
  owner: string,
  repo: string,
  number: number,
): Promise<GetBuildByNumberResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/builds/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/${number}`,
  );

  return await handleResponse(response, GetBuildByNumberResponseSchema);
}
