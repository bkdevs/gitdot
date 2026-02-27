import { z } from "zod";
import { TaskResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetTaskRequest = z.object({});
export type GetTaskRequest = z.infer<typeof GetTaskRequest>;

export const GetTaskResponse = TaskResource;
export type GetTaskResponse = z.infer<typeof GetTaskResponse>;

export const GetTask = {
  path: "/ci/task/{id}",
  method: "GET",
  request: GetTaskRequest,
  response: GetTaskResponse,
} as const satisfies Endpoint;
export type GetTask = typeof GetTask;
