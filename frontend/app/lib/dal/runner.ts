import "server-only";

import { toQueryString } from "@/util";
import {
  type GetRunnerQuery,
  type RunnerResponse,
  RunnerResponseSchema,
} from "../dto/runner";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function createRunner(
  name: string,
  ownerName: string,
  ownerType: string,
): Promise<RunnerResponse | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/ci/runner`, {
    name,
    owner_name: ownerName,
    owner_type: ownerType,
  });

  return await handleResponse(response, RunnerResponseSchema);
}

export async function getRunner(
  name: string,
  query: GetRunnerQuery,
): Promise<RunnerResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/runner/${name}?${toQueryString(query)}`,
  );

  return await handleResponse(response, RunnerResponseSchema);
}
