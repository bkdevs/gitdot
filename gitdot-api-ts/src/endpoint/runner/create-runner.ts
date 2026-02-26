import { z } from "zod";

import { RunnerResource } from "../../resource";

export const CreateRunnerRequest = z.object({
  name: z.string(),
  owner_type: z.string(),
});
export type CreateRunnerRequest = z.infer<typeof CreateRunnerRequest>;

export const CreateRunner = {
  path: "/ci/runner/{owner}",
  method: "POST",
  request: CreateRunnerRequest,
  response: RunnerResource,
} as const;
export type CreateRunner = typeof CreateRunner;
