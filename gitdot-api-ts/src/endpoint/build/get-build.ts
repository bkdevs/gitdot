import { z } from "zod";
import { GetBuildResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetBuildRequest = z.object({});
export type GetBuildRequest = z.infer<typeof GetBuildRequest>;

export const GetBuildResponse = GetBuildResource;
export type GetBuildResponse = z.infer<typeof GetBuildResponse>;

export const GetBuild = {
  path: "/repository/{owner}/{repo}/build/{number}",
  method: "GET",
  request: GetBuildRequest,
  response: GetBuildResponse,
} as const satisfies Endpoint;
export type GetBuild = typeof GetBuild;
