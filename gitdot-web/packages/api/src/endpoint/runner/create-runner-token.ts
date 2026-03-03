import { z } from "zod";
import { RunnerTokenResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateRunnerTokenRequest = z.object({});
export type CreateRunnerTokenRequest = z.infer<typeof CreateRunnerTokenRequest>;

export const CreateRunnerTokenResponse = RunnerTokenResource;
export type CreateRunnerTokenResponse = z.infer<
  typeof CreateRunnerTokenResponse
>;

export const CreateRunnerToken = {
  path: "/ci/runner/{owner}/{name}/token",
  method: "POST",
  request: CreateRunnerTokenRequest,
  response: CreateRunnerTokenResponse,
} as const satisfies Endpoint;
export type CreateRunnerToken = typeof CreateRunnerToken;
