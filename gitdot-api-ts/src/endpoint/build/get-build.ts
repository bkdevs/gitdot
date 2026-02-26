import { z } from "zod";

import { GetBuildResource } from "../../resource";

export const GetBuildRequest = z.object({});
export type GetBuildRequest = z.infer<typeof GetBuildRequest>;

export const GetBuild = {
  path: "/repository/{owner}/{repo}/build/{number}",
  method: "GET",
  request: GetBuildRequest,
  response: GetBuildResource,
} as const;
export type GetBuild = typeof GetBuild;
