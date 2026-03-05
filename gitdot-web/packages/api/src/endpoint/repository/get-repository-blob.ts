import { z } from "zod";
import { RepositoryBlobResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryBlobRequest = z.object({
  path: z.string(),
  ref_name: z.string().optional(),
});
export type GetRepositoryBlobRequest = z.infer<typeof GetRepositoryBlobRequest>;

export const GetRepositoryBlobResponse = RepositoryBlobResource;
export type GetRepositoryBlobResponse = z.infer<
  typeof GetRepositoryBlobResponse
>;

export const GetRepositoryBlob = {
  path: "/repository/{owner}/{repo}/blob",
  method: "GET",
  request: GetRepositoryBlobRequest,
  response: GetRepositoryBlobResponse,
} as const satisfies Endpoint;
export type GetRepositoryBlob = typeof GetRepositoryBlob;
