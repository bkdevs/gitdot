import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateRepositoryRequest = z.object({
  owner_type: z.string(),
  visibility: z.string(),
});
export type CreateRepositoryRequest = z.infer<typeof CreateRepositoryRequest>;

export const CreateRepositoryResponse = RepositoryResource;
export type CreateRepositoryResponse = z.infer<typeof CreateRepositoryResponse>;

export const CreateRepository = {
  path: "/repository/{owner}/{repo}",
  method: "POST",
  request: CreateRepositoryRequest,
  response: CreateRepositoryResponse,
} as const satisfies Endpoint;
export type CreateRepository = typeof CreateRepository;
