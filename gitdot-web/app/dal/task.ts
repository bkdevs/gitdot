import "server-only";

import { TaskLogResource, TaskResource } from "gitdot-api";
import { z } from "zod";
import { authFetch, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getBuildTasks(
  owner: string,
  repo: string,
  number: number,
): Promise<TaskResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build/${number}/tasks`,
  );

  return await handleResponse(response, z.array(TaskResource));
}

export async function getTaskLogs(
  taskId: string,
): Promise<TaskLogResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/ci/task/${encodeURIComponent(taskId)}/logs`,
  );
  return await handleResponse(response, z.array(TaskLogResource));
}
