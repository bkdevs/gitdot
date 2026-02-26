import { z } from "zod";

import { BuildResource } from "../../resource";

export const CreateBuildRequest = z.object({
  trigger: z.string(),
  commit_sha: z.string(),
});
export type CreateBuildRequest = z.infer<typeof CreateBuildRequest>;

export const CreateBuild = {
  path: "/repository/{owner}/{repo}/build",
  method: "POST",
  request: CreateBuildRequest,
  response: BuildResource,
} as const;
export type CreateBuild = typeof CreateBuild;
