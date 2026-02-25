import "server-only";

import { type TasksResponse, TasksResponseSchema } from "../dto";
import { authFetch, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getBuildTasks(
  buildId: string,
): Promise<TasksResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/build/${encodeURIComponent(buildId)}/tasks`,
  );

  return await handleResponse(response, TasksResponseSchema);
}
