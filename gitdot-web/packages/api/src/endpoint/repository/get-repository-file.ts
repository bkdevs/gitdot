import { z } from "zod";
import { RepositoryFileResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryFileRequest = z.object({
  path: z.string(),
  ref_name: z.string().optional(),
});
export type GetRepositoryFileRequest = z.infer<typeof GetRepositoryFileRequest>;

export const GetRepositoryFileResponse = RepositoryFileResource;
export type GetRepositoryFileResponse = z.infer<
  typeof GetRepositoryFileResponse
>;

export const GetRepositoryFile = {
  path: "/repository/{owner}/{repo}/file",
  method: "GET",
  request: GetRepositoryFileRequest,
  response: GetRepositoryFileResponse,
} as const satisfies Endpoint;
export type GetRepositoryFile = typeof GetRepositoryFile;
