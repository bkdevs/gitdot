import { z } from "zod";
import { RunnerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRunnerRequest = z.object({});
export type GetRunnerRequest = z.infer<typeof GetRunnerRequest>;

export const GetRunnerResponse = RunnerResource;
export type GetRunnerResponse = z.infer<typeof GetRunnerResponse>;

export const GetRunner = {
  path: "/ci/runner/{owner}/{name}",
  method: "GET",
  request: GetRunnerRequest,
  response: GetRunnerResponse,
} satisfies Endpoint;
export type GetRunner = typeof GetRunner;
