import { z } from "zod";
import { RepositoryResourcesResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryResourcesRequest = z.object({
  last_commit: z.string().optional(),
  last_updated: z.iso.datetime().optional(),
});
export type GetRepositoryResourcesRequest = z.infer<
  typeof GetRepositoryResourcesRequest
>;

export const GetRepositoryResourcesResponse = RepositoryResourcesResource;
export type GetRepositoryResourcesResponse = z.infer<
  typeof GetRepositoryResourcesResponse
>;

export const GetRepositoryResources = {
  path: "/repository/{owner}/{repo}/resources",
  method: "POST",
  request: GetRepositoryResourcesRequest,
  response: GetRepositoryResourcesResponse,
} as const satisfies Endpoint;
export type GetRepositoryResources = typeof GetRepositoryResources;
