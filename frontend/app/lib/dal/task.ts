import "server-only";

import {
  type CreateTaskRequest,
  type TaskResponse,
  TaskResponseSchema,
  type TasksResponse,
  TasksResponseSchema,
} from "../dto";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getTasks(
  owner: string,
  repo: string,
): Promise<TasksResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/tasks?owner=${encodeURIComponent(owner)}&repo=${encodeURIComponent(repo)}`,
  );

  return await handleResponse(response, TasksResponseSchema);
}

export async function createTask(
  request: CreateTaskRequest,
): Promise<TaskResponse | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/ci/task`, request);

  return await handleResponse(response, TaskResponseSchema);
}
