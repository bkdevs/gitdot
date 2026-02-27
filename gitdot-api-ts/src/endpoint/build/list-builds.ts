import { z } from "zod";
import { BuildResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListBuildsRequest = z.object({});
export type ListBuildsRequest = z.infer<typeof ListBuildsRequest>;

export const ListBuildsResponse = z.array(BuildResource);
export type ListBuildsResponse = z.infer<typeof ListBuildsResponse>;

export const ListBuilds = {
  path: "/repository/{owner}/{repo}/builds",
  method: "GET",
  request: ListBuildsRequest,
  response: ListBuildsResponse,
} as const satisfies Endpoint;
export type ListBuilds = typeof ListBuilds;
