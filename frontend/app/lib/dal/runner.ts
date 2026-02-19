import "server-only";

import {
  type RunnerListResponse,
  RunnerListResponseSchema,
  type RunnerResponse,
  RunnerResponseSchema,
  type RunnerTokenResponse,
  RunnerTokenResponseSchema,
} from "../dto/runner";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function createRunner(
  name: string,
  ownerName: string,
  ownerType: string,
): Promise<RunnerResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/ci/runner/${ownerName}`,
    {
      name,
      owner_type: ownerType,
    },
  );

  return await handleResponse(response, RunnerResponseSchema);
}

export async function getRunner(
  owner: string,
  name: string,
): Promise<RunnerResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/runner/${owner}/${name}`,
  );

  return await handleResponse(response, RunnerResponseSchema);
}

export async function listRunners(
  ownerName: string,
): Promise<RunnerListResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/runner/${ownerName}`,
  );

  return await handleResponse(response, RunnerListResponseSchema);
}

export async function refreshRunnerToken(
  ownerName: string,
  name: string,
): Promise<RunnerTokenResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/ci/runner/${ownerName}/${name}/token`,
    {},
  );

  return await handleResponse(response, RunnerTokenResponseSchema);
}
