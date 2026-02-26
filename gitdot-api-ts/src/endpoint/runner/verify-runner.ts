import { z } from "zod";

export const VerifyRunnerRequest = z.object({});
export type VerifyRunnerRequest = z.infer<typeof VerifyRunnerRequest>;

export const VerifyRunner = {
  path: "/ci/runner/{id}/verify",
  method: "POST",
  request: VerifyRunnerRequest,
  response: z.void(),
} as const;
export type VerifyRunner = typeof VerifyRunner;
