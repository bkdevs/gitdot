import { z } from "zod";

import { PollTaskResource } from "../../resource";

export const PollTaskRequest = z.object({});
export type PollTaskRequest = z.infer<typeof PollTaskRequest>;

export const PollTask = {
  path: "/ci/task/poll",
  method: "GET",
  request: PollTaskRequest,
  response: PollTaskResource.nullable(),
} as const;
export type PollTask = typeof PollTask;
