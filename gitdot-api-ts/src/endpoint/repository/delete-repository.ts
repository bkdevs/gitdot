import { z } from "zod";

export const DeleteRepositoryRequest = z.object({});
export type DeleteRepositoryRequest = z.infer<typeof DeleteRepositoryRequest>;

export const DeleteRepository = {
  path: "/repository/{owner}/{repo}",
  method: "DELETE",
  request: DeleteRepositoryRequest,
  response: z.void(),
} as const;
export type DeleteRepository = typeof DeleteRepository;
