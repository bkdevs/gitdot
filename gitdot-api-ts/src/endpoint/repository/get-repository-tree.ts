import { z } from "zod";

import { RepositoryTreeResource } from "../../resource";

export const GetRepositoryTreeRequest = z.object({
  ref_name: z.string(),
});
export type GetRepositoryTreeRequest = z.infer<typeof GetRepositoryTreeRequest>;

export const GetRepositoryTree = {
  path: "/repository/{owner}/{repo}/tree",
  method: "GET",
  request: GetRepositoryTreeRequest,
  response: RepositoryTreeResource,
} as const;
export type GetRepositoryTree = typeof GetRepositoryTree;
