import { z } from "zod";

import { TaskResource } from "../../resource";

export const UpdateTaskRequest = z.object({
  status: z.string(),
});
export type UpdateTaskRequest = z.infer<typeof UpdateTaskRequest>;

export const UpdateTask = {
  path: "/ci/task/{id}",
  method: "PATCH",
  request: UpdateTaskRequest,
  response: TaskResource,
} as const;
export type UpdateTask = typeof UpdateTask;
