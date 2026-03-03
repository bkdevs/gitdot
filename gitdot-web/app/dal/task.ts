import "server-only";

import { TaskResource, TaskTokenResource } from "gitdot-api";
import { z } from "zod";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

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

export async function issueTaskToken(taskId: string): Promise<string | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/ci/task/${encodeURIComponent(taskId)}/token`,
    {},
  );
  const result = await handleResponse(response, TaskTokenResource);
  return result?.token ?? null;
}
