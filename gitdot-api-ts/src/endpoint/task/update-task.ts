import { z } from "zod";
import { TaskResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateTaskRequest = z.object({
  status: z.string(),
});
export type UpdateTaskRequest = z.infer<typeof UpdateTaskRequest>;

export const UpdateTaskResponse = TaskResource;
export type UpdateTaskResponse = z.infer<typeof UpdateTaskResponse>;

export const UpdateTask = {
  path: "/ci/task/{id}",
  method: "PATCH",
  request: UpdateTaskRequest,
  response: UpdateTaskResponse,
} satisfies Endpoint;
export type UpdateTask = typeof UpdateTask;
