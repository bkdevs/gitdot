import { z } from "zod";
import { RepositoryPathsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryPathsRequest = z.object({
  ref_name: z.string().optional(),
});
export type GetRepositoryPathsRequest = z.infer<
  typeof GetRepositoryPathsRequest
>;

export const GetRepositoryPathsResponse = RepositoryPathsResource;
export type GetRepositoryPathsResponse = z.infer<
  typeof GetRepositoryPathsResponse
>;

export const GetRepositoryPaths = {
  path: "/repository/{owner}/{repo}/paths",
  method: "GET",
  request: GetRepositoryPathsRequest,
  response: GetRepositoryPathsResponse,
} as const satisfies Endpoint;
export type GetRepositoryPaths = typeof GetRepositoryPaths;
