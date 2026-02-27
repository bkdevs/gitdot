import "server-only";

import { RunnerResource, RunnerTokenResource } from "gitdot-api-ts";
import { z } from "zod";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function createRunner(
  name: string,
  ownerName: string,
  ownerType: string,
): Promise<RunnerResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/ci/runner/${ownerName}`,
    {
      name,
      owner_type: ownerType,
    },
  );

  return await handleResponse(response, RunnerResource);
}

export async function getRunner(
  owner: string,
  name: string,
): Promise<RunnerResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/runner/${owner}/${name}`,
  );

  return await handleResponse(response, RunnerResource);
}

export async function listRunners(
  ownerName: string,
): Promise<RunnerResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/runner/${ownerName}`,
  );

  return await handleResponse(response, z.array(RunnerResource));
}

export async function refreshRunnerToken(
  ownerName: string,
  name: string,
): Promise<RunnerTokenResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/ci/runner/${ownerName}/${name}/token`,
    {},
  );

  return await handleResponse(response, RunnerTokenResource);
}
