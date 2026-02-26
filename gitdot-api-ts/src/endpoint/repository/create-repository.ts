import { z } from "zod";

import { RepositoryResource } from "../../resource";

export const CreateRepositoryRequest = z.object({
  owner_type: z.string(),
  visibility: z.string(),
});
export type CreateRepositoryRequest = z.infer<typeof CreateRepositoryRequest>;

export const CreateRepository = {
  path: "/repository/{owner}/{repo}",
  method: "POST",
  request: CreateRepositoryRequest,
  response: RepositoryResource,
} as const;
export type CreateRepository = typeof CreateRepository;
