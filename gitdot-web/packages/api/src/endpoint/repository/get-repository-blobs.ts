import { z } from "zod";
import { RepositoryBlobsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryBlobsRequest = z.object({
  refs: z.array(z.string()).default(["HEAD"]),
  paths: z.array(z.string()),
});
export type GetRepositoryBlobsRequest = z.infer<
  typeof GetRepositoryBlobsRequest
>;

export const GetRepositoryBlobsResponse = RepositoryBlobsResource;
export type GetRepositoryBlobsResponse = z.infer<
  typeof GetRepositoryBlobsResponse
>;

export const GetRepositoryBlobs = {
  path: "/repository/{owner}/{repo}/blobs",
  method: "POST",
  request: GetRepositoryBlobsRequest,
  response: GetRepositoryBlobsResponse,
} as const satisfies Endpoint;
export type GetRepositoryBlobs = typeof GetRepositoryBlobs;
