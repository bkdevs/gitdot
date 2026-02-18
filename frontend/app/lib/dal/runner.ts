import "server-only";

import { type RunnerResponse, RunnerResponseSchema } from "../dto/runner";
import { authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

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
