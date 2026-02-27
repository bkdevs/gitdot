import "server-only";

import { TaskResource } from "gitdot-api";
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
