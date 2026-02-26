import { z } from "zod";

import { RunnerResource } from "../../resource";

export const ListRunnersRequest = z.object({});
export type ListRunnersRequest = z.infer<typeof ListRunnersRequest>;

export const ListRunners = {
  path: "/ci/runner/{owner}",
  method: "GET",
  request: ListRunnersRequest,
  response: z.array(RunnerResource),
} as const;
export type ListRunners = typeof ListRunners;
