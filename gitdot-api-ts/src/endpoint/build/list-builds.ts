import { z } from "zod";

import { BuildResource } from "../../resource";

export const ListBuildsRequest = z.object({});
export type ListBuildsRequest = z.infer<typeof ListBuildsRequest>;

export const ListBuilds = {
  path: "/repository/{owner}/{repo}/builds",
  method: "GET",
  request: ListBuildsRequest,
  response: z.array(BuildResource),
} as const;
export type ListBuilds = typeof ListBuilds;
