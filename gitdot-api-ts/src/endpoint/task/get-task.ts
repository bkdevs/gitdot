import { z } from "zod";

import { TaskResource } from "../../resource";

export const GetTaskRequest = z.object({});
export type GetTaskRequest = z.infer<typeof GetTaskRequest>;

export const GetTask = {
  path: "/ci/task/{id}",
  method: "GET",
  request: GetTaskRequest,
  response: TaskResource,
} as const;
export type GetTask = typeof GetTask;
