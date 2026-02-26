import { z } from "zod";
import { RunnerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateRunnerRequest = z.object({
  name: z.string(),
  owner_type: z.string(),
});
export type CreateRunnerRequest = z.infer<typeof CreateRunnerRequest>;

export const CreateRunnerResponse = RunnerResource;
export type CreateRunnerResponse = z.infer<typeof CreateRunnerResponse>;

export const CreateRunner = {
  path: "/ci/runner/{owner}",
  method: "POST",
  request: CreateRunnerRequest,
  response: CreateRunnerResponse,
} satisfies Endpoint;
export type CreateRunner = typeof CreateRunner;
