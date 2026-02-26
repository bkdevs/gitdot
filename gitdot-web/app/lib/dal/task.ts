import "server-only";

import { type TasksResponse, TasksResponseSchema } from "../dto";
import { authFetch, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getBuildTasks(
  owner: string,
  repo: string,
  number: number,
): Promise<TasksResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build/${number}/tasks`,
  );

  return await handleResponse(response, TasksResponseSchema);
}
