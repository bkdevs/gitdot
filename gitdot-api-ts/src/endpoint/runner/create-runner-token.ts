import { z } from "zod";

import { RunnerTokenResource } from "../../resource";

export const CreateRunnerTokenRequest = z.object({});
export type CreateRunnerTokenRequest = z.infer<typeof CreateRunnerTokenRequest>;

export const CreateRunnerToken = {
  path: "/ci/runner/{owner}/{name}/token",
  method: "POST",
  request: CreateRunnerTokenRequest,
  response: RunnerTokenResource,
} as const;
export type CreateRunnerToken = typeof CreateRunnerToken;
