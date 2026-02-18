import "server-only";

import { z } from "zod";
import { authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

const CreateRunnerResponseSchema = z.object({
  token: z.string(),
});

export async function createRunner(
  name: string,
  ownerName: string,
  ownerType: string,
): Promise<{ token: string } | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/ci/runner`, {
    name,
    owner_name: ownerName,
    owner_type: ownerType,
  });

  return await handleResponse(response, CreateRunnerResponseSchema);
}
