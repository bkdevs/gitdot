import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const VerifyRunnerRequest = z.object({});
export type VerifyRunnerRequest = z.infer<typeof VerifyRunnerRequest>;

export const VerifyRunnerResponse = z.void();
export type VerifyRunnerResponse = z.infer<typeof VerifyRunnerResponse>;

export const VerifyRunner = {
  path: "/ci/runner/{id}/verify",
  method: "POST",
  request: VerifyRunnerRequest,
  response: VerifyRunnerResponse,
} satisfies Endpoint;
export type VerifyRunner = typeof VerifyRunner;
