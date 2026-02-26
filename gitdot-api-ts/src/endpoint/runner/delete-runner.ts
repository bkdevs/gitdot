import { z } from "zod";

export const DeleteRunnerRequest = z.object({});
export type DeleteRunnerRequest = z.infer<typeof DeleteRunnerRequest>;

export const DeleteRunner = {
  path: "/ci/runner/{owner}/{name}",
  method: "DELETE",
  request: DeleteRunnerRequest,
  response: z.void(),
} as const;
export type DeleteRunner = typeof DeleteRunner;
