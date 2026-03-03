import { z } from "zod";
import { RepositoryTreeResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryTreeRequest = z.object({
  ref_name: z.string(),
});
export type GetRepositoryTreeRequest = z.infer<typeof GetRepositoryTreeRequest>;

export const GetRepositoryTreeResponse = RepositoryTreeResource;
export type GetRepositoryTreeResponse = z.infer<
  typeof GetRepositoryTreeResponse
>;

export const GetRepositoryTree = {
  path: "/repository/{owner}/{repo}/tree",
  method: "GET",
  request: GetRepositoryTreeRequest,
  response: GetRepositoryTreeResponse,
} as const satisfies Endpoint;
export type GetRepositoryTree = typeof GetRepositoryTree;
