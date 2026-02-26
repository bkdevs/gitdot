import { z } from "zod";

import { RepositoryFileResource } from "../../resource";

export const GetRepositoryFileRequest = z.object({
  ref_name: z.string(),
  path: z.string(),
});
export type GetRepositoryFileRequest = z.infer<typeof GetRepositoryFileRequest>;

export const GetRepositoryFile = {
  path: "/repository/{owner}/{repo}/file",
  method: "GET",
  request: GetRepositoryFileRequest,
  response: RepositoryFileResource,
} as const;
export type GetRepositoryFile = typeof GetRepositoryFile;
