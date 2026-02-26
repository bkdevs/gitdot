import { z } from "zod";

import { RunnerResource } from "../../resource";

export const GetRunnerRequest = z.object({});
export type GetRunnerRequest = z.infer<typeof GetRunnerRequest>;

export const GetRunner = {
  path: "/ci/runner/{owner}/{name}",
  method: "GET",
  request: GetRunnerRequest,
  response: RunnerResource,
} as const;
export type GetRunner = typeof GetRunner;
