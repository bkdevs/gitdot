import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const DeleteRunnerRequest = z.object({});
export type DeleteRunnerRequest = z.infer<typeof DeleteRunnerRequest>;

export const DeleteRunnerResponse = z.void();
export type DeleteRunnerResponse = z.infer<typeof DeleteRunnerResponse>;

export const DeleteRunner = {
  path: "/ci/runner/{owner}/{name}",
  method: "DELETE",
  request: DeleteRunnerRequest,
  response: DeleteRunnerResponse,
} satisfies Endpoint;
export type DeleteRunner = typeof DeleteRunner;
